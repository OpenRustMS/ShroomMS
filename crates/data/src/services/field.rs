use std::{sync::Arc, time::Duration};

use dashmap::{mapref::entry::Entry, DashMap};
use meta::{field::FhTree, FieldLife, FieldMeta, MetaService};
use proto95::{
    game::{
        chat::UserChatMsgResp,
        drop::{DropId, DropOwner},
        ObjectId, life::mob::{MobLeaveType, MobMoveReq},
    },
    id::{FieldId, ItemId, SkillId},
    shared::{
        char::{AvatarData, Money},
        movement::MovePath,
        FootholdId, Range2, Vec2,
    },
};
use shroom_net::server::{
    room::{Room, RoomJoinHandle, RoomSet, RoomState},
    tick::Tick,
};
use shroom_pkt::{
    util::packet_buf::PacketBuf, EncodePacket, HasOpcode, PacketWriter, ShroomPacketData,
};
use tokio::sync::{mpsc, oneshot};

use super::{
    data::character::CharacterID,
    helper::{
        delay_queue::DelayQueue,
        pool::{
            drop::{DropLeaveParam, DropTypeValue},
            mob::MobPool,
            reactor::Reactor,
            summoned::Summon,
            user::User,
            Drop, Mob, Npc, Pool, SimplePool,
        },
    },
};

#[derive(Clone, Debug)]
pub enum SessionMsg {
    Pkt(ShroomPacketData),
    PktBuf(Arc<PacketBuf>),
}

impl SessionMsg {
    pub fn from_packet<T: EncodePacket + HasOpcode>(pkt: T) -> Self {
        let mut pw = PacketWriter::default();
        pw.write_opcode(T::OPCODE).expect("op");
        pkt.encode_packet(&mut pw).expect("pw");

        Self::Pkt(ShroomPacketData::from_writer(pw))
    }
}

impl From<PacketBuf> for SessionMsg {
    fn from(pkt: PacketBuf) -> Self {
        Self::PktBuf(Arc::new(pkt))
    }
}

#[derive(Debug)]
pub struct SharedFieldState {
    field_meta: FieldMeta,
    field_fh: &'static FhTree,
    drops: DashMap<DropId, Drop>,
}

impl SharedFieldState {
    /// Fast way to loot items
    pub fn try_claim_drop(
        &self,
        drop_id: DropId,
        check: impl FnOnce(&Drop) -> bool,
    ) -> Option<Drop> {
        let Entry::Occupied(drop) = self.drops.entry(drop_id) else {
            return None;
        };

        if check(drop.get()) {
            Some(drop.remove())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum FieldEvent {
    DropTimeout(DropId),
}

#[derive(Debug)]
pub struct FieldData {
    shared: Arc<SharedFieldState>,
    drop_pool: SimplePool<Drop>,
    mob_pool: MobPool,
    npc_pool: SimplePool<Npc>,
    reactor_pool: SimplePool<Reactor>,
    user_pool: SimplePool<User>,
    summon_pool: SimplePool<Summon>,
    sessions: FieldRoomSet,
    drop_spam: Option<Vec2>,
    field_events: DelayQueue<FieldEvent>,
    meta: &'static MetaService,
}

pub type FieldRoomSet = RoomSet<CharacterID, SessionMsg>;
pub type FieldSessionHandle = mpsc::Sender<SessionMsg>;

pub enum FieldMsg {
    UserUpdatePos(MovePath),
    NpcAdd(Npc),
    NpcRemove(ObjectId),
    MobAdd(Mob),
    MobRemove(ObjectId, MobLeaveType),
    MobUpdatePos(MobMoveReq, CharacterID),
    MobAssignController(CharacterID),
    MobAttack {
        id: ObjectId,
        dmg: u32,
    },
    DropAdd(Drop),
    DropRemove(DropId, DropLeaveParam),
    Chat(UserChatMsgResp),
    SlowLoot(DropId, DropLeaveParam, oneshot::Sender<Option<Drop>>),
    StartSpamDrop(Vec2),
    StopSpamDrop,
    ReactorAttack {
        id: ObjectId,
        attacker: CharacterID,
    },
    SummonSpawn {
        char_id: CharacterID,
        char_level: u8,
        skill_id: SkillId,
        skill_level: u8,
        pos: Vec2,
        fh: FootholdId,
    },
}

impl RoomState for FieldData {
    type ConnMsg = SessionMsg;
    type Key = CharacterID;
    type Msg = FieldMsg;
    type JoinData = AvatarData;
    type CreateData = (&'static MetaService, Arc<SharedFieldState>);

    fn create(
        create_data: Self::CreateData,
        conns: RoomSet<Self::Key, Self::ConnMsg>,
    ) -> anyhow::Result<Self> {
        Ok(FieldData::new(create_data.0, create_data.1, conns))
    }

    fn session_mut(&mut self) -> &mut RoomSet<Self::Key, Self::ConnMsg> {
        &mut self.sessions
    }

    fn sessions(&self) -> &RoomSet<Self::Key, Self::ConnMsg> {
        &self.sessions
    }

    fn handle_leave(&mut self, id: Self::Key) -> anyhow::Result<()> {
        self.leave_field(id);
        Ok(())
    }

    fn handle_join(&mut self, id: Self::Key, data: Self::JoinData) -> anyhow::Result<()> {
        self.enter_field(id, data)
    }

    fn handle_msg(&mut self, src: Option<Self::Key>, msg: Self::Msg) -> anyhow::Result<()> {
        match msg {
            FieldMsg::UserUpdatePos(move_path) => {
                self.update_user_pos(move_path, src.expect("user pos"))?;
            }
            FieldMsg::NpcAdd(npc) => {
                self.add_npc(npc)?;
            }
            FieldMsg::NpcRemove(id) => {
                self.remove_npc(id, ())?;
            }
            FieldMsg::MobAdd(mob) => {
                self.add_mob(mob)?;
            }
            FieldMsg::MobRemove(id, leave) => {
                self.remove_mob(id, leave)?;
            }
            FieldMsg::MobUpdatePos(movement, id) => {
                self.update_mob_pos(movement, id)?;
            }
            FieldMsg::MobAssignController(id) => {
                self.assign_mob_controller(id)?;
            }
            FieldMsg::MobAttack { id, dmg } => {
                self.attack_mob(id, dmg, src.expect("attacker"))?;
            }
            FieldMsg::DropAdd(drop) => {
                self.add_drop(drop)?;
            }
            FieldMsg::DropRemove(id, param) => {
                self.remove_drop(id, param)?;
            }
            FieldMsg::Chat(msg) => {
                self.add_chat(msg)?;
            }
            FieldMsg::SlowLoot(id, reason, tx) => {
                let drop = self.remove_drop(id, reason).ok();
                tx.send(drop).ok();
            }
            FieldMsg::StartSpamDrop(pos) => self.drop_spam = Some(pos),
            FieldMsg::StopSpamDrop => {
                self.drop_spam = None;
            }
            FieldMsg::ReactorAttack { id, attacker } => {
                let reactor = self.reactor_pool.must_get_mut(&id)?;
                reactor.state = reactor.state.saturating_sub(1);
                if reactor.state == 0 {
                    let drops = self.meta.get_reactor_drops(reactor.tmpl_id);
                    let pos = reactor.pos.clone();
                    self.reactor_pool.remove(id, (), &self.sessions)?;
                    self.spread_drops(pos, DropOwner::User(attacker as u32), &drops, 10)?;
                }
            }
            FieldMsg::SummonSpawn {
                char_id,
                char_level,
                skill_id,
                skill_level,
                pos,
                fh
            } => {
                self.summon_pool.add(
                    Summon {
                        pos,
                        fh,
                        skill_id,
                        skill_level,
                        char_level,
                        char_id: char_id as u32,
                    },
                    &self.sessions,
                )?;
            }
        }

        Ok(())
    }

    fn handle_tick(&mut self) -> anyhow::Result<()> {
        self.mob_pool.respawn(&self.sessions)?;

        for event in self.field_events.drain_expired() {
            match event {
                FieldEvent::DropTimeout(id) => {
                    // Remove fail is not a problem
                    let _ = self
                        .drop_pool
                        .remove(id, DropLeaveParam::TimeOut, &self.sessions);
                }
            }
        }

        if let Some(pos) = self.drop_spam {
            for _ in 0..10 {
                self.drop_pool.add(
                    Drop {
                        owner: proto95::game::drop::DropOwner::None,
                        pos,
                        start_pos: pos,
                        value: DropTypeValue::Mesos(100),
                        quantity: 1,
                    },
                    &self.sessions,
                )?;
            }
        }

        Ok(())
    }
}

impl FieldData {
    pub fn new(
        meta_svc: &'static MetaService,
        shared: Arc<SharedFieldState>,
        sessions: FieldRoomSet,
    ) -> Self {
        let meta = shared.field_meta;
        let npcs = meta
            .life
            .values()
            .filter_map(|life| match &life {
                FieldLife::Npc(n) => Some(n),
                _ => None,
            })
            .map(|npc| Npc {
                tmpl_id: npc.id,
                pos: npc.pos,
                fh: npc.fh,
                move_action: 0,
                range_horz: Range2 {
                    low: *npc.range_x.start(),
                    high: *npc.range_x.end(),
                },
                enabled: true,
            });

        let mobs = meta
            .life
            .values()
            .filter_map(|life| match life {
                FieldLife::Mob(m) if !m.hide => Some(m),
                _ => None,
            })
            .map(|m| {
                let meta = meta_svc.get_mob_data(m.id).unwrap();
                (m.id, meta, m)
            });

        let reactors = meta.reactors.values().map(|r| Reactor {
            name: r.name.clone(),
            pos: r.pos,
            tmpl_id: r.id,
            state: 0,
        });

        Self {
            shared,
            drop_pool: SimplePool::new(meta_svc),
            mob_pool: MobPool::from_spawns(meta_svc, mobs),
            npc_pool: SimplePool::from_elems(meta_svc, npcs),
            reactor_pool: SimplePool::from_elems(meta_svc, reactors),
            user_pool: SimplePool::new(meta_svc),
            summon_pool: SimplePool::new(meta_svc),
            sessions,
            drop_spam: None,
            field_events: DelayQueue::new(),
            meta: meta_svc,
        }
    }

    pub fn enter_field(
        &mut self,
        char_id: CharacterID,
        avatar_data: AvatarData,
    ) -> anyhow::Result<()> {
        self.user_pool.add_filter(
            User {
                char_id: char_id as u32,
                pos: Vec2::from((0, 0)),
                fh: 1,
                avatar_data,
            },
            &self.sessions,
            char_id,
        )?;
        let mut buf = PacketBuf::default();
        self.user_pool.on_enter(&mut buf)?;
        self.drop_pool.on_enter(&mut buf)?;
        self.npc_pool.on_enter(&mut buf)?;
        self.mob_pool.on_enter(&mut buf)?;
        self.reactor_pool.on_enter(&mut buf)?;
        self.sessions
            .send_to(&char_id, SessionMsg::PktBuf(Arc::new(buf)))?;

        Ok(())
    }

    pub fn leave_field(&mut self, id: CharacterID) {
        self.user_pool
            .remove(id as u32, (), &self.sessions)
            .expect("Must remove user");
    }

    pub fn add_user(&mut self, user: User) -> anyhow::Result<()> {
        self.user_pool.add(user, &self.sessions)?;
        Ok(())
    }

    pub fn remove_user(&mut self, id: CharacterID) -> anyhow::Result<()> {
        self.user_pool.remove(id as u32, (), &self.sessions)?;
        Ok(())
    }

    pub fn add_npc(&mut self, npc: Npc) -> anyhow::Result<()> {
        self.npc_pool.add(npc, &self.sessions)?;
        Ok(())
    }

    pub fn remove_npc(&mut self, id: u32, param: ()) -> anyhow::Result<()> {
        self.npc_pool.remove(id, param, &self.sessions)?;
        Ok(())
    }

    pub fn add_mob(&mut self, drop: Mob) -> anyhow::Result<()> {
        self.mob_pool.add(drop, &self.sessions)?;
        Ok(())
    }

    pub fn remove_mob(&mut self, id: u32, param: MobLeaveType) -> anyhow::Result<()> {
        self.mob_pool.remove(id, param, &self.sessions)?;
        Ok(())
    }

    pub fn update_user_pos(&mut self, move_path: MovePath, id: CharacterID) -> anyhow::Result<()> {
        self.user_pool.user_move(id, move_path, &self.sessions)?;
        Ok(())
    }

    pub fn update_mob_pos(
        &mut self,
        movement: MobMoveReq,
        controller: CharacterID,
    ) -> anyhow::Result<()> {
        self.mob_pool
            .mob_move(movement.id, movement, controller, &self.sessions)?;

        Ok(())
    }

    pub fn add_drop(&mut self, drop: Drop) -> anyhow::Result<()> {
        let drop_id = self.drop_pool.add(drop, &self.sessions)?;
        self.field_events
            .push_after(FieldEvent::DropTimeout(drop_id), Duration::from_secs(60));
        Ok(())
    }

    pub fn remove_drop(&mut self, id: DropId, param: DropLeaveParam) -> anyhow::Result<Drop> {
        self.drop_pool.remove(id, param, &self.sessions)
    }

    pub fn assign_mob_controller(&self, session_id: CharacterID) -> anyhow::Result<()> {
        self.mob_pool
            .assign_controller(session_id, &self.sessions)?;
        Ok(())
    }

    pub fn add_chat(&self, chat: UserChatMsgResp) -> anyhow::Result<()> {
        self.sessions.broadcast(SessionMsg::from_packet(chat))?;
        Ok(())
    }

    fn spread_drops(
        &mut self,
        pos: Vec2,
        owner: DropOwner,
        drops: &[(ItemId, usize)],
        money: Money,
    ) -> anyhow::Result<()> {
        let fh = self
            .shared
            .field_fh
            .get_foothold_below((pos.x as f32, pos.y as f32 - 20.).into());

        self.drop_pool
            .add_drops(drops, money, pos, fh, owner, &self.sessions)?;

        Ok(())
    }

    pub fn attack_mob(
        &mut self,
        id: ObjectId,
        dmg: u32,
        attacker: CharacterID,
    ) -> anyhow::Result<()> {
        let mut buf = PacketBuf::default();
        let killed = self
            .mob_pool
            .attack_mob(attacker, id, dmg, &mut buf, &self.sessions)?;
        self.sessions
            .send_to(&attacker, SessionMsg::PktBuf(Arc::new(buf)))?;

        if killed {
            let mob = self.mob_pool.kill_mob(id, &self.sessions)?;
            let drops = self.reactor_pool.meta.get_drops_for_mob(mob.tmpl_id);
            let money = self.reactor_pool.meta.get_money_drops_for_mob(mob.tmpl_id);

            let fh = self
                .shared
                .field_fh
                .get_foothold_below((mob.pos.x as f32, mob.pos.y as f32 - 20.).into());

            self.drop_pool.add_drops(
                &drops,
                money,
                mob.pos,
                fh,
                DropOwner::User(attacker as u32),
                &self.sessions,
            )?;
        }

        Ok(())
    }
}

pub type FieldRoom = Room<FieldData>;

pub struct FieldJoinHandle {
    room_handle: RoomJoinHandle<FieldData>,
    pub field_id: FieldId,
    pub field_meta: FieldMeta,
}

impl FieldJoinHandle {
    pub async fn send(&self, msg: FieldMsg) -> anyhow::Result<()> {
        self.room_handle.send(msg).await?;
        Ok(())
    }

    pub async fn leave(self) -> anyhow::Result<()> {
        self.room_handle.leave().await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<SessionMsg> {
        self.room_handle.recv().await
    }
}

#[derive(Debug)]
pub struct FieldService {
    fields: DashMap<FieldId, Arc<FieldRoom>>,
    meta: &'static MetaService,
    tick: Tick,
}

impl FieldService {
    pub fn new(tick: Tick, meta: &'static MetaService) -> Self {
        Self {
            fields: DashMap::new(),
            meta,
            tick,
        }
    }

    fn create_field(&self, field_id: FieldId) -> anyhow::Result<Arc<FieldRoom>> {
        let field_meta = self
            .meta
            .get_field_data(field_id)
            .ok_or_else(|| anyhow::format_err!("Invalid field id: {field_id:?}"))?;

        let shared = Arc::new(SharedFieldState {
            field_meta,
            field_fh: self.meta.get_field_fh_data(field_id).expect("Field fh"),
            drops: DashMap::new(),
        });
        Ok(Arc::new(FieldRoom::spawn(
            (self.meta, shared),
            self.tick.clone(),
            32,
            128,
        )?))
    }

    pub fn get_field(&self, field_id: FieldId) -> anyhow::Result<Arc<FieldRoom>> {
        Ok(self
            .fields
            .entry(field_id)
            .or_try_insert_with(|| self.create_field(field_id))?
            .clone())
    }

    pub async fn join_field(
        &self,
        char_id: CharacterID,
        avatar_data: AvatarData,
        field_id: FieldId,
    ) -> anyhow::Result<FieldJoinHandle> {
        let field = self.get_field(field_id)?;
        let field_meta = self.meta.get_field_data(field_id).unwrap();
        let room_handle = field.join_with_channel(char_id, avatar_data).await?;

        Ok(FieldJoinHandle {
            room_handle,
            field_id,
            field_meta,
        })
    }

    pub async fn switch_field(
        &self,
        avatar_data: AvatarData,
        field_id: FieldId,
        handle: &mut FieldJoinHandle,
    ) -> anyhow::Result<()> {
        let field = self.get_field(field_id)?;
        let field_meta = self.meta.get_field_data(field_id).unwrap();

        handle.room_handle.switch_to(&field, avatar_data).await?;
        handle.field_id = field_id;
        handle.field_meta = field_meta;
        Ok(())
    }
}
