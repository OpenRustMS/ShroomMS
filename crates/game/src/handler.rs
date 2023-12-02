use std::future::Future;
use std::ops::Neg;

use std::net::IpAddr;

use async_trait::async_trait;

use data::scripts::NpcScriptHandle;
use data::services::character::Character;
use data::services::field::{FieldJoinHandle, FieldMsg, SessionMsg};

use data::services::helper::pool::drop::{DropLeaveParam, DropTypeValue};

use data::services::session::shroom_session_backend::ShroomSessionData;
use data::services::session::shroom_session_manager::{ClientKey, OwnedShroomGameSession};
use data::services::session::ShroomMigrationKey;
use data::services::SharedServices;

use either::Either;
use plugins::npc::{npc_script_1000, NpcAction};
use plugins::poll_state::StateRef;
use proto95::game::field::FieldTransferData;
use proto95::game::life::npc::UserSelectNpcReq;
use proto95::game::life::reactor::ReactorHitReq;
use proto95::game::script::{ScriptAnswerReq, ScriptMessageResp};
use proto95::game::user::char::{CharDataAll, CharDataFlags};
use proto95::game::user::secondary_stats::LocalSecondaryStatResetResp;
use shroom_net::codec::legacy::LegacyCodec;

use shroom_net::server::server_conn::{ShroomConnEvent, ShroomConnHandler};
use shroom_net::server::{ServerHandleResult, SharedConnHandle};
use shroom_net::shroom_router_fn;

use data::services::helper::pool::Drop;

use proto95::game::life::mob::{MobMoveCtrlAckResp, MobMoveReq};
use proto95::game::user::{
    ChangeSkillRecordResp, UserDropMoneyReq, UserDropPickUpReq, UserHitReq, UserMeleeAttackReq,
    UserSkillUpReq, UserSkillUseReq, UserStatChangeReq,
};

use super::repl::GameRepl;
use proto95::shared::char::{SkillInfo, SocialRecords, TeleportRockInfo};
use proto95::shared::inventory::{InvChangeSlotPosReq, InventoryOperationsResp};
use proto95::shared::{ClientDumpLogReq, PingResp, PongReq};
use proto95::{
    game::{
        chat::{ChatMsgReq, UserChatMsgResp},
        field::{CrcSeed, FieldCharData, LogoutGiftConfig, NotificationList, SetFieldResp},
        friend::{FriendList, FriendResultResp},
        keymaps::FuncKeyMapInitResp,
        user::{UserMoveReq, UserPortalScriptReq, UserTransferFieldReq},
        BroadcastMessageResp, ClaimSvrStatusChangedResp, CtxSetGenderResp, MigrateCommandResp,
        MigrateInGameReq, TransferChannelReq,
    },
    id::FieldId,
    login::world::{ChannelId, WorldId},
    recv_opcodes::RecvOpcodes,
    shared::{
        char::{CharDataEquipped, CharDataHeader, CharDataStat, CharStatChangedResp},
        item::Item,
        UpdateScreenSettingReq,
    },
};
use shroom_pkt::partial::PartialFlag;

use shroom_pkt::{
    DecodePacket, HasOpcode, PacketReader, ShroomExpirationTime, ShroomIndexListZ16,
    ShroomIndexListZ8, ShroomList16, ShroomPacketData, ShroomTime,
};
use tokio::net::TcpStream;
use tokio::sync::oneshot;

pub type GameResult<T> = Result<T, anyhow::Error>;
pub type GameMsg = SessionMsg;

#[derive(Debug, Clone)]
pub struct MakeGameHandler {
    services: SharedServices,
    channel_id: ChannelId,
    world_id: WorldId,
}

impl MakeGameHandler {
    pub fn new(services: SharedServices, channel_id: ChannelId, world_id: WorldId) -> Self {
        Self {
            services,
            channel_id,
            world_id,
        }
    }
}

pub type Ctx = shroom_net::server::ServerConnCtx<GameHandler>;

pub struct GameHandler {
    pub services: SharedServices,
    pub session: OwnedShroomGameSession,
    pub field: FieldJoinHandle,
    pub addr: IpAddr,
    pub channel_id: ChannelId,
    pub world_id: WorldId,
    pub client_key: ClientKey,
    pub repl: GameRepl,
    pub script_handle: NpcScriptHandle,
}

impl GameHandler {
    pub async fn from_ctx(
        ctx: &mut Ctx,
        services: SharedServices,
        channel_id: ChannelId,
        world_id: WorldId,
        _session_handle: SharedConnHandle<GameMsg>,
    ) -> anyhow::Result<Self> {
        log::info!("New game session");
        // Read handshake packet
        let pkt = ctx.session_mut().read_packet().await?;
        let mut pr = pkt.into_reader();
        let op = pr.read_opcode::<RecvOpcodes>()?;
        if op != MigrateInGameReq::OPCODE {
            anyhow::bail!("Wrong client hello packet: {op:?}")
        }
        let req = MigrateInGameReq::decode(&mut pr)?;

        // Look up session in the migration manager
        let peer_addr = ctx.session().peer_addr().ip();

        log::info!("Claiming session for {peer_addr}");

        let migrate_key = ShroomMigrationKey::new(req.client_key, peer_addr);
        let session = services
            .session_manager
            .claim_migration_session(migrate_key)
            .await?;

        // TODO, add a try_map function to owned session
        let session = session.map(|sess| match sess {
            ShroomSessionData::Login(_) => panic!("Session is not a game session"),
            ShroomSessionData::Ingame(sess) => sess,
        });

        log::info!("Claimed session");

        log::info!(
            "Game session for acc: {} - char: {}",
            session.acc.username,
            session.char.name
        );

        // Join field
        let join_field = services
            .game
            .field
            .join_field(
                session.char.id,
                session.char.get_avatar_data(),
                session.char.map_id,
            )
            .await?;

        Ok(Self {
            session,
            services,
            channel_id,
            world_id,
            addr: peer_addr,
            client_key: req.client_key,
            field: join_field,
            repl: GameRepl::new(),
            script_handle: Default::default(),
        })
    }

    pub fn char_mut(&mut self) -> &mut Character {
        &mut self.session.char
    }

    pub fn char(&self) -> &Character {
        &self.session.char
    }

    pub fn enable_char(&mut self) {
        self.session.char.unlock_char()
    }

    fn set_field(&self, char_data: bool, sp: u8) -> SetFieldResp {
        let field_data = if char_data {
            let char = &self.session.char;

            let equipped: ShroomIndexListZ16<Item> = self
                .session
                .char
                .inventory
                .invs
                .equipped
                .item_with_slots()
                .map(|(slot, item)| (slot as u16, Item::Equip(item.0.item.as_ref().into())))
                .collect();

            let equip: ShroomIndexListZ16<Item> = self
                .session
                .char
                .inventory
                .invs
                .equip
                .item_with_slots()
                .map(|(slot, item)| (slot as u16 + 1, Item::Equip(item.item.as_ref().into())))
                .collect();

            let etc: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .etc
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let setup: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .misc
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let cash: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .cash
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let use_: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .use_
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let char_equipped = CharDataEquipped {
                equipped,
                equip,
                ..Default::default()
            };

            let skillrecords: ShroomList16<SkillInfo> =
                self.session.char.skills.get_skill_info().into();

            let char_data = CharDataAll {
                stat: CharDataStat {
                    stat: char.get_all_stats(),
                    friend_max: 30,
                    linked_character: None.into(),
                },
                money: char.money(),
                invsize: char.get_inv_slots(),
                equipextslotexpiration: ShroomExpirationTime::never(),
                equipped: char_equipped,
                useinv: use_,
                setupinv: setup,
                etcinv: etc,
                cashinv: cash,
                skillrecords,
                skllcooltime: ShroomList16::default(),
                quests: ShroomList16::default(),
                questscompleted: ShroomList16::default(),
                minigamerecords: ShroomList16::default(),
                socialrecords: SocialRecords::default(),
                teleportrockinfo: TeleportRockInfo::default(),
                newyearcards: ShroomList16::default(),
                questrecordsexpired: ShroomList16::default(),
                questcompleteold: ShroomList16::default(),
                visitorquestloginfo: ShroomList16::default(),
            };

            Either::Left(FieldCharData {
                seed: CrcSeed {
                    s1: 1,
                    s2: 2,
                    s3: 3,
                },
                logout_gift_config: LogoutGiftConfig {
                    predict_quit: 0,
                    gift_commodity_id: [0; 3],
                },
                char_data_hdr: CharDataHeader {
                    combat_orders: 0,
                    extra_data: None.into(),
                },
                char_data,
                char_data_flags: CharDataFlags::all(),
            })
        } else {
            Either::Right(FieldTransferData {
                revive: false,
                map: self.field.field_id,
                portal: sp,
                hp: self.char().stats.hp.value as u32,
                chase_target_pos: None.into(),
            })
        };

        SetFieldResp {
            client_option: ShroomList16::default(),
            channel_id: self.channel_id as u32,
            has_char_data: field_data.is_left(),
            char_data: field_data.into(),
            notifications: NotificationList::default(),
            old_driver_id: 0,
            unknown_flag_1: 0,
            server_time: ShroomTime::now(),
        }
    }

    pub async fn join_field(
        &mut self,
        ctx: &mut Ctx,
        map: FieldId,
        spawn_point: Option<u8>,
    ) -> GameResult<()> {
        let sp = spawn_point.unwrap_or(0);
        self.session.char.transfer_map(map, sp);

        // Only get a new handle, when the map is new
        if map != self.field.field_id {
            self.services
                .game
                .field
                .switch_field(self.session.char.get_avatar_data(), map, &mut self.field)
                .await?;
        }

        ctx.send(self.set_field(false, sp)).await
    }

    async fn init_char(&mut self, ctx: &mut Ctx) -> anyhow::Result<()> {
        ctx.send(self.set_field(true, 0)).await?;

        ctx.send(FriendResultResp::Reset3(FriendList::empty()))
            .await?;
        ctx.send(FuncKeyMapInitResp::default_map()).await?;
        ctx.send(ClaimSvrStatusChangedResp { connected: true })
            .await?;
        ctx.send(CtxSetGenderResp {
            gender: self.session.char.gender,
        })
        .await?;

        ctx.send(BroadcastMessageResp::PinkMessage("Hello".to_string()))
            .await?;

        self.session.char.unlock_char();

        Ok(())
    }
}

#[async_trait]
impl ShroomConnHandler for GameHandler {
    type Codec = LegacyCodec<TcpStream>;
    type Error = anyhow::Error;
    type Msg = SessionMsg;
    type MakeState = MakeGameHandler;

    async fn make_handler(
        state: &Self::MakeState,
        ctx: &mut Ctx,
        handle: SharedConnHandle<Self::Msg>,
    ) -> Result<Self, Self::Error> {
        let mut handler = GameHandler::from_ctx(
            ctx,
            state.services.clone(),
            state.channel_id,
            state.world_id,
            handle.clone(),
        )
        .await?;
        handler.init_char(ctx).await?;

        //Ok(ctx)

        Ok(handler)
    }

    async fn recv_msg(&mut self) -> Option<Self::Msg> {
        self.field.recv().await.ok()
    }

    async fn handle_msg(
        &mut self,
        ctx: &mut Ctx,
        msg: ShroomConnEvent<Self::Msg>,
    ) -> Result<ServerHandleResult, Self::Error> {
        Ok(match msg {
            ShroomConnEvent::IncomingPacket(pkt) => self.handle_packet(&pkt, ctx).await?,
            ShroomConnEvent::Message(msg) => {
                match msg {
                    SessionMsg::Pkt(pkt) => ctx.session_mut().send_packet(pkt.as_ref()).await?,
                    SessionMsg::PktBuf(pkt_buf) => {
                        ctx.session_mut().send_packet_buffer(&pkt_buf).await?
                    }
                }
                ServerHandleResult::Ok
            }
            ShroomConnEvent::Ping => {
                ctx.send(PingResp).await?;
                ServerHandleResult::Ok
            }
            ShroomConnEvent::Tick(_) => {
                self.tick(ctx).await?;
                ServerHandleResult::Ok
            }
        })
    }

    async fn finish(self, is_migrating: bool) -> Result<(), Self::Error> {
        log::info!("Finishing game session...");
        let session = self.session.unmap();
        if is_migrating {
            self.services
                .session_manager
                .migrate_session(ShroomMigrationKey::new(self.client_key, self.addr), session)?;
        } else {
            self.services.session_manager.close_session(session).await?;
        }

        self.field.leave().await?;

        Ok(())
    }
}

impl GameHandler {
    pub async fn poll_npc(&mut self, ctx: &mut Ctx, input: NpcAction) -> anyhow::Result<()> {
        if !self.script_handle.is_finished() {
            self.script_handle
                .run(input, &mut self.session.char)
                .await?;

            if let Some(msg) = self.char_mut().npc_msg.pop_front() {
                const MAPLE_ADMIN: u32 = 9010000;
                ctx.send(ScriptMessageResp {
                    script_flag: 0x4, // Replace ByNpc
                    speaker_id: MAPLE_ADMIN,
                    msg,
                })
                .await?;
            }

            if self.script_handle.is_finished() {
                self.char_mut().npc_msg.clear();
                self.enable_char();
            }
        }

        Ok(())
    }

    pub async fn start_script<Fut, F>(&mut self, ctx: &mut Ctx, script: F) -> anyhow::Result<()>
    where
        Fut: Future<Output = anyhow::Result<()>> + Send + Sync + 'static,
        F: FnOnce(StateRef<Character, NpcAction>) -> Fut,
    {
        self.script_handle
            .start_fn(
                |n: StateRef<Character, NpcAction>| Box::pin(script(n)),
                &mut self.session.char,
            )
            .await?;

        if let Some(msg) = self.char_mut().npc_msg.pop_front() {
            const MAPLE_ADMIN: u32 = 9010000;
            ctx.send(ScriptMessageResp {
                script_flag: 0x4, // Replace ByNpc
                speaker_id: MAPLE_ADMIN,
                msg,
            })
            .await?;
        }

        if self.script_handle.is_finished() {
            self.char_mut().npc_msg.clear();
            self.enable_char();
        }
        Ok(())
    }

    pub async fn tick(&mut self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn handle_npc_action(
        &mut self,
        ctx: &mut Ctx,
        npc_action: plugins::npc::NpcAction,
    ) -> anyhow::Result<()> {
        self.poll_npc(ctx, npc_action).await?;
        Ok(())
    }

    pub async fn send_field_msg(&mut self, msg: FieldMsg) -> anyhow::Result<()> {
        self.field.send(msg).await?;
        Ok(())
    }

    pub async fn handle_packet(
        &mut self,
        pkt: &ShroomPacketData,
        ctx: &mut Ctx,
    ) -> anyhow::Result<ServerHandleResult> {
        //dbg!(&pkt.as_ref());

        shroom_router_fn!(
            handler,
            GameHandler,
            anyhow::Error,
            GameHandler::handle_default,
            PongReq => GameHandler::handle_pong,
            UpdateScreenSettingReq => GameHandler::handle_update_screen_setting,
            ChatMsgReq => GameHandler::handle_chat_msg,
            UserMoveReq => GameHandler::handle_movement,
            UserPortalScriptReq => GameHandler::handle_portal_script,
            UserTransferFieldReq => GameHandler::handle_field_transfer,
            TransferChannelReq => GameHandler::handle_channel_transfer,
            UserDropPickUpReq => GameHandler::handle_drop_pick_up,
            UserDropMoneyReq => GameHandler::handle_drop_money,
            MobMoveReq => GameHandler::handle_mob_move,
            UserMeleeAttackReq => GameHandler::handle_melee_attack,
            UserSkillUpReq => GameHandler::handle_skill_up,
            UserHitReq => GameHandler::handle_user_hit,
            UserStatChangeReq => GameHandler::handle_stat_change,
            InvChangeSlotPosReq => GameHandler::handle_inv_change_slot,
            ClientDumpLogReq => GameHandler::handle_client_dump_log,
            UserSkillUseReq => GameHandler::handle_use_skill,
            UserSelectNpcReq => GameHandler::handle_select_npc,
            ScriptAnswerReq => GameHandler::handle_script_action,
            ReactorHitReq => GameHandler::handle_reactor_hit,
        );

        let res = handler(self, ctx, pkt.into_reader()).await?;
        self.update_char_stats(ctx).await?;
        Ok(res)
    }

    async fn handle_pong(
        &mut self,
        _ctx: &mut Ctx,
        _req: PongReq,
    ) -> anyhow::Result<ServerHandleResult> {
        Ok(ServerHandleResult::Pong)
    }

    pub async fn handle_default(
        &mut self,
        _ctx: &mut Ctx,
        op: RecvOpcodes,
        pr: PacketReader<'_>,
    ) -> anyhow::Result<ServerHandleResult> {
        log::info!("Unhandled packet: {:?} {:?}", op, pr.into_inner());
        Ok(ServerHandleResult::Ok)
    }

    async fn handle_script_action(
        &mut self,
        ctx: &mut Ctx,
        req: ScriptAnswerReq,
    ) -> anyhow::Result<()> {
        self.handle_npc_action(ctx, req.into()).await?;
        Ok(())
    }

    async fn handle_select_npc(
        &mut self,
        ctx: &mut Ctx,
        _req: UserSelectNpcReq,
    ) -> anyhow::Result<()> {
        self.enable_char();
        self.start_script(ctx, npc_script_1000).await?;

        Ok(())
    }

    async fn handle_use_skill(
        &mut self,
        _ctx: &mut Ctx,
        req: UserSkillUseReq,
    ) -> anyhow::Result<()> {
        self.char_mut().use_skill(req.skill_id)?;

        if let Some(summon) = self.char_mut().do_summon.take() {
            self.send_field_msg(FieldMsg::SummonSpawn {
                char_id: summon.char_id as i32,
                char_level: summon.char_level,
                skill_id: summon.skill_id,
                skill_level: summon.skill_level,
                pos: summon.pos,
                fh: summon.fh,
            })
            .await?;
        }

        Ok(())
    }

    async fn handle_client_dump_log(
        &mut self,
        _ctx: &mut Ctx,
        req: ClientDumpLogReq,
    ) -> anyhow::Result<()> {
        dbg!(req);
        Ok(())
    }

    async fn handle_user_hit(&mut self, _ctx: &mut Ctx, req: UserHitReq) -> GameResult<()> {
        self.char_mut()
            .stats
            .update_hp((req.dmg_internal as i16).neg());
        Ok(())
    }

    async fn handle_reactor_hit(&mut self, _ctx: &mut Ctx, req: ReactorHitReq) -> GameResult<()> {
        dbg!(&req);
        self.send_field_msg(FieldMsg::ReactorAttack {
            id: req.id,
            attacker: self.char().id,
        })
        .await?;
        Ok(())
    }

    async fn handle_stat_change(
        &mut self,
        _ctx: &mut Ctx,
        req: UserStatChangeReq,
    ) -> GameResult<()> {
        //TODO ?
        self.session.char.stats.update_hp(req.hp as i16);
        self.session.char.stats.update_mp(req.mp as i16);

        Ok(())
    }

    async fn handle_inv_change_slot(
        &mut self,
        _ctx: &mut Ctx,
        req: InvChangeSlotPosReq,
    ) -> anyhow::Result<()> {
        let count = (req.count != u16::MAX).then_some(req.count as usize);
        let drop = req.to == 0;
        let from = (req.inv_type, req.from).try_into()?;
        // Check for drop
        if drop {
            let item = self.session.char.inventory.drop_item(from, count)?;
            // TODO handle persistent equip items
            let msg = match item {
                Either::Left(eq) => FieldMsg::DropAdd(Drop {
                    owner: proto95::game::drop::DropOwner::User(self.session.char.id as u32),
                    pos: self.session.char.pos,
                    start_pos: self.session.char.pos,
                    value: DropTypeValue::Item(eq.item_id),
                    quantity: 1,
                }),
                Either::Right(stack) => FieldMsg::DropAdd(Drop {
                    owner: proto95::game::drop::DropOwner::User(self.session.char.id as u32),
                    pos: self.session.char.pos,
                    start_pos: self.session.char.pos,
                    value: DropTypeValue::Item(stack.item_id),
                    quantity: stack.quantity as usize,
                }),
            };
            self.send_field_msg(msg).await?;
        } else {
            let to = (req.inv_type, req.to).try_into()?;
            self.session.char.inventory.move_item(from, to, count)?;
        }

        self.session.char.unlock_char();
        Ok(())
    }

    async fn handle_skill_up(&mut self, _ctx: &mut Ctx, req: UserSkillUpReq) -> GameResult<()> {
        self.session.char.skills.skill_up(req.skill_id, 1)?;
        Ok(())
    }

    async fn update_char_stats(&mut self, ctx: &mut Ctx) -> GameResult<()> {
        if let Some(partial) = self.session.char.get_stats_update() {
            ctx.send(CharStatChangedResp {
                excl: true, //TODO handle this
                stats: PartialFlag {
                    hdr: (),
                    data: partial,
                },
                secondary_stat: false,
                battle_recovery: false,
            })
            .await?;
        }

        if let Some(ops) = self.session.char.get_inv_op_updates() {
            ctx.send(InventoryOperationsResp {
                reset_excl: true,
                operations: ops.into(),
                secondary_stat_changed: false,
            })
            .await?;
        }

        if let Some(skills) = self.session.char.skills.get_updates() {
            ctx.send(ChangeSkillRecordResp {
                reset_excl: true,
                skill_records: skills.into(),
                updated_secondary_stat: false,
            })
            .await?;
        }

        if let Some(skill_cd) = self.char_mut().skills.get_cooldown_updates() {
            for cd in skill_cd {
                ctx.send(cd).await?;
            }
        }

        if let Some(secondary_stat) = self.char_mut().get_secondary_stats_update() {
            ctx.send(secondary_stat).await?;
        }

        // Handle timer events

        if let Some(flags) = self.char_mut().handle_timer_events() {
            ctx.send(LocalSecondaryStatResetResp {
                flags,
                movement_affecting: true,
            })
            .await?;
        }

        Ok(())
    }

    async fn handle_update_screen_setting(
        &mut self,
        _ctx: &mut Ctx,
        _req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_melee_attack(
        &mut self,
        _ctx: &mut Ctx,
        req: UserMeleeAttackReq,
    ) -> anyhow::Result<()> {
        for target in req.targets {
            let dmg = target.hits.iter().sum::<u32>();
            self.send_field_msg(FieldMsg::MobAttack {
                id: target.mob_id,
                dmg,
            })
            .await?;
        }
        self.char_mut().handle_attack()?;

        Ok(())
    }

    async fn handle_drop_pick_up(
        &mut self,
        _ctx: &mut Ctx,
        req: UserDropPickUpReq,
    ) -> GameResult<()> {
        let (tx, rx) = oneshot::channel();
        self.send_field_msg(FieldMsg::SlowLoot(
            req.drop_id,
            DropLeaveParam::UserPickup(self.session.char.id as u32),
            tx,
        ))
        .await?;
        let drop = rx.await?.unwrap();

        let chr = self.char_mut();

        match drop.value {
            DropTypeValue::Mesos(money) => {
                chr.update_mesos(money as i32);
            }
            DropTypeValue::Item(item_id) => {
                let inv_ty = item_id.get_inv_type()?;
                if !inv_ty.is_stack() {
                    chr.add_equip_item(item_id)?;
                } else {
                    chr.add_stack_item(inv_ty, item_id, drop.quantity)?;
                };
            }
        }

        Ok(())
    }

    async fn handle_drop_money(&mut self, _ctx: &mut Ctx, req: UserDropMoneyReq) -> GameResult<()> {
        let ok = self.session.char.update_mesos((req.money as i32).neg());
        if ok {
            let char = &self.session.char;
            self.send_field_msg(FieldMsg::DropAdd(Drop {
                owner: proto95::game::drop::DropOwner::User(char.id as u32),
                pos: char.pos,
                start_pos: char.pos,
                value: DropTypeValue::Mesos(req.money),
                quantity: 1,
            }))
            .await?;
        }

        self.session.char.unlock_char();
        Ok(())
    }

    async fn handle_chat_msg(&mut self, ctx: &mut Ctx, req: ChatMsgReq) -> anyhow::Result<()> {
        let admin = false;
        if let Some(s) = req.msg.strip_prefix('@') {
            let repl_resp = self.handle_repl(ctx, s).await?;
            let Some(msg) = repl_resp else { return Ok(()) };
            let resp = UserChatMsgResp {
                char: self.session.char.id as u32,
                is_admin: admin,
                msg,
                only_balloon: false,
            };

            ctx.send(resp).await?;
        } else {
            self.send_field_msg(FieldMsg::Chat(UserChatMsgResp {
                char: self.session.char.id as u32,
                is_admin: admin,
                msg: req.msg,
                only_balloon: req.only_balloon,
            }))
            .await?;
        };
        Ok(())
    }

    async fn handle_mob_move(&mut self, ctx: &mut Ctx, req: MobMoveReq) -> GameResult<()> {
        let ctrl_sn = req.ctrl_sn;
        let id = req.id;

        self.send_field_msg(FieldMsg::MobUpdatePos(req, self.session.char.id))
            .await?;

        ctx.send(MobMoveCtrlAckResp {
            id,
            ctrl_sn,
            next_atk_possible: false,
            mp: 0,
            skill_id: 0,
            slv: 0,
        })
        .await
    }

    async fn handle_portal_script(
        &mut self,
        _ctx: &mut Ctx,
        _req: UserPortalScriptReq,
    ) -> GameResult<()> {
        self.enable_char();
        Ok(())
    }

    async fn handle_field_transfer(
        &mut self,
        ctx: &mut Ctx,
        req: UserTransferFieldReq,
    ) -> GameResult<()> {
        let meta = &self.services.game.meta;
        let field_meta = &self.field.field_meta;
        let (map, spawn) = if self.session.char.is_dead() {
            self.session.char.respawn();
            // TODO the portal is not correct
            meta.get_return_field_spawn(field_meta).unwrap_or_else(|| {
                (
                    self.field.field_id,
                    field_meta.get_first_portal_id().unwrap(),
                )
            })
        } else {
            meta.get_portal_map_spawn(self.field.field_id, self.field.field_meta, &req.portal)
                .ok_or_else(|| anyhow::format_err!("Invalid portal"))?
        };

        self.join_field(ctx, map, Some(spawn)).await
    }

    async fn handle_movement(&mut self, _ctx: &mut Ctx, req: UserMoveReq) -> anyhow::Result<()> {
        self.char_mut().pos = req.move_path.pos;
        let last = req.move_path.get_last_pos_fh();

        if let Some((pos, fh)) = last {
            self.char_mut().pos = pos;
            self.char_mut().fh = fh.unwrap_or(self.char().fh);
        }

        self.send_field_msg(FieldMsg::UserUpdatePos(req.move_path))
            .await?;
        Ok(())
    }

    async fn handle_channel_transfer(
        &mut self,
        ctx: &mut Ctx,
        req: TransferChannelReq,
    ) -> GameResult<ServerHandleResult> {
        let addr = self
            .services
            .game
            .server_info
            .get_channel_addr(self.world_id, req.channel_id as ChannelId)?;

        ctx.send(MigrateCommandResp {
            unknown: true,
            addr: addr.try_into()?,
        })
        .await?;

        Ok(ServerHandleResult::Migrate)
    }
}
