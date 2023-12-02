use std::{num::Saturating, sync::Arc};

use meta::{field::FhTree, FieldLife, FieldMeta, MetaService};
use proto95::{
    game::{
        drop::{DropId, DropOwner},
        life::{
            mob::{MobLeaveType, MobMoveReq},
            npc::NpcId,
        },
        ObjectId,
    },
    id::ItemId,
    shared::{
        char::{AvatarData, Money},
        movement::MovePath,
        Range2, Vec2,
    },
};
use shroom_pkt::util::packet_buf::PacketBuf;
use shroom_srv::{
    srv::{server_room::{RoomCtx, RoomHandler, RoomSessionHandler}, room_set::ServerSessionData, server_socket::ServerSocketHandle},
    util::delay_queue::DelayQueue,
    Context,
};

use super::{
    data::character::CharacterID,
    game::GameSession,
    helper::pool::{
        drop::{Drop, DropLeaveParam, DropTypeValue},
        mob::MobPool,
        reactor::Reactor,
        summoned::Summon,
        user::User,
        Mob, Npc, Pool, SimplePool,
    },
    GameCtx,
};

#[derive(Debug)]
pub struct FieldService;

impl FieldService {
    pub fn new(_meta: &'static MetaService) -> Self {
        Self
    }
}

#[derive(Debug)]
pub enum FieldEvent {
    DropTimeout(DropId),
}

#[derive(Debug)]
pub struct SharedFieldState {
    pub field_meta: FieldMeta,
    pub field_fh: &'static FhTree,
}

#[derive(Debug)]
pub struct FieldHandler {
    shared: Arc<SharedFieldState>,
    drop_pool: SimplePool<Drop>,
    mob_pool: MobPool,
    npc_pool: SimplePool<Npc>,
    reactor_pool: SimplePool<Reactor>,
    user_pool: SimplePool<User>,
    summon_pool: SimplePool<Summon>,
    drop_spam: Option<Vec2>,
    field_events: DelayQueue<FieldEvent>,
    meta: &'static MetaService,
}

impl RoomHandler for FieldHandler {
    type Ctx = GameCtx;
    type SessionHandler = GameSession;

    fn on_enter(
        &mut self,
        ctx: &mut Self::Ctx,
        session: &mut ServerSessionData<Self::SessionHandler>,
    ) -> anyhow::Result<()> {
        log::info!("Session entering field");
        Ok(())
    }
    fn on_leave(
        ctx: &mut RoomCtx<'_, Self::SessionHandler>,
        id: <Self::SessionHandler as RoomSessionHandler>::SessionId,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn on_update(ctx: &mut RoomCtx<'_, Self::SessionHandler>) -> anyhow::Result<()> {
        ctx.handle_update()
    }
}

pub trait FieldRoomCtxExt {
    fn ctx(&self) -> &RoomCtx<'_, GameSession>;
    fn ctx_mut(&mut self) -> &mut RoomCtx<'_, GameSession>;

    fn update_user_pos(&mut self, move_path: MovePath, id: CharacterID) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room
            .user_pool
            .user_move(id, move_path, &mut ctx.room_sessions)?;
        Ok(())
    }

    fn add_npc(&mut self, npc: Npc) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room.npc_pool.add(npc, &mut ctx.room_sessions)?;
        Ok(())
    }

    fn remove_npc(&mut self, id: NpcId) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room.npc_pool.remove(id, (), &mut ctx.room_sessions)?;
        Ok(())
    }

    fn add_mob(&mut self, mob: Mob) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room.mob_pool.add(mob, &mut ctx.room_sessions)?;
        Ok(())
    }

    fn remove_mob(&mut self, id: ObjectId, leave: MobLeaveType) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room
            .mob_pool
            .remove(id, leave, &mut ctx.room_sessions)?;
        Ok(())
    }

    fn update_mob_pos(
        &mut self,
        movement: MobMoveReq,
        controller: CharacterID,
    ) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room
            .mob_pool
            .mob_move(movement.id, movement, controller, &mut ctx.room_sessions)?;
        Ok(())
    }

    fn assign_mob_controller(&mut self, session_id: CharacterID) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room
            .mob_pool
            .assign_controller(session_id, &mut ctx.room_sessions)?;
        Ok(())
    }

    fn attack_mob(&mut self, id: ObjectId, dmg: u32, attacker: CharacterID) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        let killed = ctx
            .room
            .mob_pool
            .attack_mob(attacker, id, dmg, &mut ctx.room_sessions)?;

        if killed {
            let mob = ctx.room.mob_pool.kill_mob(id, &mut ctx.room_sessions)?;
            let meta = ctx.room.meta;
            let drops = meta.get_drops_for_mob(mob.tmpl_id);
            let money = meta.get_money_drops_for_mob(mob.tmpl_id);

            let fh = ctx
                .room
                .shared
                .field_fh
                .get_foothold_below((mob.pos.x as f32, mob.pos.y as f32 - 20.).into());

            ctx.room.drop_pool.add_drops(
                &drops,
                money,
                mob.pos,
                fh,
                DropOwner::User(attacker as u32),
                &mut ctx.room_sessions,
            )?;
        }

        Ok(())
    }

    fn add_drop(&mut self, drop: Drop) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        let drop_id = ctx.room.drop_pool.add(drop, &mut ctx.room_sessions)?;
        ctx.room.field_events.push(
            FieldEvent::DropTimeout(drop_id),
            ctx.room_ctx.time().add_ms(60_000),
        );
        Ok(())
    }

    fn remove_drop(&mut self, id: DropId, param: DropLeaveParam) -> anyhow::Result<Drop> {
        let ctx = self.ctx_mut();
        ctx.room.drop_pool.remove(id, param, &mut ctx.room_sessions)
    }

    fn chat(&mut self, msg: proto95::game::chat::UserChatMsgResp) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room_sessions.broadcast_encode(msg)?;
        Ok(())
    }

    fn start_spam_drop(&mut self, pos: Vec2) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room.drop_spam = Some(pos);
        Ok(())
    }

    fn stop_spam_drop(&mut self) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room.drop_spam = None;
        Ok(())
    }

    fn attack_reactor(&mut self, id: ObjectId, attacker: CharacterID) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();

        let reactor = ctx.room.reactor_pool.must_get_mut(&id)?;
        reactor.state -= 1;
        if reactor.state.0 == 0 {
            let drops = ctx.room.meta.get_reactor_drops(reactor.tmpl_id);
            let pos = reactor.pos;
            ctx.room
                .reactor_pool
                .remove(id, (), &mut ctx.room_sessions)?;
            self.spread_drops(pos, DropOwner::User(attacker as u32), &drops, 10)?;
        }

        Ok(())
    }

    fn summon_spawn(
        &mut self,
        char_id: CharacterID,
        char_level: u8,
        skill_id: proto95::id::SkillId,
        skill_level: u8,
        pos: Vec2,
        fh: u16,
    ) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        ctx.room.summon_pool.add(
            Summon {
                pos,
                fh,
                skill_id,
                skill_level,
                char_level,
                char_id: char_id as u32,
            },
            &mut ctx.room_sessions,
        )?;
        Ok(())
    }

    fn handle_update(&mut self) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        let t = ctx.time();

        ctx.room.mob_pool.respawn(&mut ctx.room_sessions)?;

        for event in ctx.room.field_events.drain_expired(t) {
            match event {
                FieldEvent::DropTimeout(id) => {
                    // Remove fail is not a problem
                    let _ = ctx.room.drop_pool.remove(
                        id,
                        DropLeaveParam::TimeOut,
                        &mut ctx.room_sessions,
                    );
                }
            }
        }

        if let Some(pos) = ctx.room.drop_spam {
            for _ in 0..10 {
                ctx.room.drop_pool.add(
                    Drop {
                        owner: proto95::game::drop::DropOwner::None,
                        pos,
                        start_pos: pos,
                        value: DropTypeValue::Mesos(100),
                        quantity: 1,
                    },
                    &mut ctx.room_sessions,
                )?;
            }
        }

        Ok(())
    }

    fn spread_drops(
        &mut self,
        pos: Vec2,
        owner: DropOwner,
        drops: &[(ItemId, usize)],
        money: Money,
    ) -> anyhow::Result<()> {
        let ctx = self.ctx_mut();
        let fh = ctx
            .room
            .shared
            .field_fh
            .get_foothold_below((pos.x as f32, pos.y as f32 - 20.).into());

        ctx.room
            .drop_pool
            .add_drops(drops, money, pos, fh, owner, &mut ctx.room_sessions)?;

        Ok(())
    }

    fn enter_field(&mut self, char_id: CharacterID, avatar_data: AvatarData, sck: &mut ServerSocketHandle) -> anyhow::Result<()> {
        log::info!("Char entering field1");
        let ctx = self.ctx_mut();
        ctx.room.user_pool.add_filter(
            User {
                char_id: char_id as u32,
                pos: Vec2::from((0, 0)),
                fh: 1,
                avatar_data,
            },
            &mut ctx.room_sessions,
            char_id,
        )?;
        log::info!("Char entering field2");

        let mut buf = PacketBuf::default();
        ctx.room.user_pool.on_enter(&mut buf)?;
        ctx.room.drop_pool.on_enter(&mut buf)?;
        ctx.room.npc_pool.on_enter(&mut buf)?;
        ctx.room.mob_pool.on_enter(&mut buf)?;
        ctx.room.reactor_pool.on_enter(&mut buf)?;
        log::info!("sending enter field pkt: {}", buf.packets().count());
        sck.send_buf(buf);

        log::info!("Char entering field3");

        Ok(())
    }

    fn leave_field(&mut self, id: CharacterID) {
        let ctx = self.ctx_mut();
        ctx.room
            .user_pool
            .remove(id as u32, (), &mut ctx.room_sessions)
            .expect("Must remove user");
    }
}

impl FieldHandler {
    pub fn new(meta_svc: &'static MetaService, shared: Arc<SharedFieldState>) -> Self {
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
            state: Saturating(0),
        });

        Self {
            shared,
            drop_pool: SimplePool::new(meta_svc),
            mob_pool: MobPool::from_spawns(meta_svc, mobs),
            npc_pool: SimplePool::from_elems(meta_svc, npcs),
            reactor_pool: SimplePool::from_elems(meta_svc, reactors),
            user_pool: SimplePool::new(meta_svc),
            summon_pool: SimplePool::new(meta_svc),
            drop_spam: None,
            field_events: DelayQueue::new(),
            meta: meta_svc,
        }
    }
}

impl FieldRoomCtxExt for RoomCtx<'_, GameSession> {
    fn ctx(&self) -> &RoomCtx<'_, GameSession> {
        self
    }
    fn ctx_mut(&mut self) -> &mut RoomCtx<'_, GameSession> {
        //TODO find a better way
        unsafe { std::mem::transmute(self) }
    }
}
/*
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
*/
