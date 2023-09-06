pub mod repl;

use std::ops::Neg;

use std::net::IpAddr;

use async_trait::async_trait;

use data::services::character::Character;
use data::services::field::FieldJoinHandle;

use data::services::helper::pool::drop::{DropLeaveParam, DropTypeValue};
use data::services::session::shroom_session_manager::{ClientKey, OwnedShroomGameSession};
use data::services::session::ShroomMigrationKey;
use data::services::SharedServices;

use proto95::id::SkillId;
use shroom_net::net::service::handler::{MakeServerSessionHandler, ShroomSessionHandler};
use shroom_net::net::service::resp::MigrateResponse;
use shroom_net::net::service::SharedSessionHandle;
use shroom_net::net::service::{SessionHandleResult, ShroomContext};
use shroom_net::net::ShroomSession;
use shroom_net::{shroom_router_fn, HasOpcode};

use shroom_net::packet::proto::partial::PartialFlag;
use shroom_net::packet::proto::time::ShroomExpirationTime;
use shroom_net::packet::{
    proto::{
        list::{ShroomIndexListZ16, ShroomIndexListZ8},
        time::ShroomTime,
        ShroomList16,
    },
    DecodePacket, PacketReader, ShroomPacket,
};

use data::services::helper::pool::Drop;

use proto95::game::mob::{MobMoveCtrlAckResp, MobMoveReq};
use proto95::game::user::{
    ChangeSkillRecordResp, UpdatedSkillRecord, UserDropMoneyReq, UserDropPickUpReq, UserHitReq,
    UserMeleeAttackReq, UserSkillUpReq, UserSkillUseReq, UserStatChangeReq,
};

use proto95::shared::char::{SkillInfo, TeleportRockInfo};
use proto95::shared::inventory::{InvChangeSlotPosReq, InventoryOperationsResp};
use proto95::shared::{ClientDumpLogReq, PongReq};
use proto95::{
    game::{
        chat::{ChatMsgReq, UserChatMsgResp},
        field::{
            CrcSeed, LogoutGiftConfig, NotificationList, SetFieldCharData, SetFieldResp,
            SetFieldResult,
        },
        friend::{FriendList, FriendResultResp},
        keymaps::FuncKeyMapInitResp,
        user::{UserMoveReq, UserPortalScriptReq, UserTransferFieldReq},
        BroadcastMessageResp, ClaimSvrStatusChangedResp, CtxSetGenderResp, MigrateCommandResp,
        MigrateInGameReq, TransferChannelReq,
    },
    id::MapId,
    login::world::{ChannelId, WorldId},
    recv_opcodes::RecvOpcodes,
    shared::{
        char::{
            CharDataAll, CharDataEquipped, CharDataFlagsAll, CharDataHeader, CharDataStat,
            CharStatChangedResp,
        },
        item::Item,
        UpdateScreenSettingReq,
    },
};
use repl::GameRepl;
use tokio::net::TcpStream;

pub type GameResult<T> = Result<T, anyhow::Error>;

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

type Ctx = ShroomContext<GameHandler>;

#[async_trait::async_trait]
impl MakeServerSessionHandler for MakeGameHandler {
    type Transport = TcpStream;
    type Error = anyhow::Error;
    type Handler = GameHandler;

    async fn make_handler(
        &mut self,
        mut sess: ShroomSession<Self::Transport>,
        session_handle: SharedSessionHandle,
    ) -> Result<Ctx, Self::Error> {
        let handler = GameHandler::from_session(
            &mut sess,
            self.services.clone(),
            self.channel_id,
            self.world_id,
            session_handle.clone(),
        )
        .await?;
        sess.send_encode_packet(handler.set_field()).await?;
        let mut ctx = ShroomContext::new(sess, handler, session_handle);
        GameHandler::init_char(&mut ctx).await?;

        Ok(ctx)
    }
}

pub struct GameHandler {
    session: OwnedShroomGameSession,
    channel_id: ChannelId,
    world_id: WorldId,
    services: SharedServices,
    addr: IpAddr,
    client_key: ClientKey,
    field: FieldJoinHandle,
    repl: GameRepl,
}

impl GameHandler {
    pub async fn from_session(
        net_session: &mut ShroomSession<TcpStream>,
        services: SharedServices,
        channel_id: ChannelId,
        world_id: WorldId,
        session_handle: SharedSessionHandle,
    ) -> anyhow::Result<Self> {
        // Read handshake packet
        let pkt = net_session.read_packet().await?;
        let mut pr = pkt.into_reader();
        let op = pr.read_opcode::<RecvOpcodes>()?;
        if op != MigrateInGameReq::OPCODE {
            anyhow::bail!("Wrong client hello packet: {op:?}")
        }
        let req = MigrateInGameReq::decode_packet(&mut pr)?;

        // Look up session in the migration manager
        let peer_addr = net_session.peer_addr()?.ip();
        let migrate_key = ShroomMigrationKey::new(req.client_key, peer_addr);
        let session = services
            .session_manager
            .claim_migration_session(migrate_key)
            .await?
            .as_ingame();

        log::info!(
            "Game session for acc: {} - char: {}",
            session.acc.username,
            session.char.name
        );

        // Join field
        let join_field = services
            .field
            .join_field(
                session.char.id,
                session.char.get_avatar_data(),
                session_handle,
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
        })
    }
}

#[async_trait]
impl ShroomSessionHandler for GameHandler {
    type Transport = TcpStream;
    type Error = anyhow::Error;
    type Msg = ();

    async fn handle_msg(_ctx: &mut Ctx, _msg: Self::Msg) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn handle_packet(
        ctx: &mut Ctx,
        packet: ShroomPacket,
    ) -> Result<SessionHandleResult, Self::Error> {
        let op = packet.read_opcode()?;
        if op == u16::from(PongReq::OPCODE) {
            return Ok(SessionHandleResult::Pong);
        }

        shroom_router_fn!(
            handler,
            GameHandler,
            anyhow::Error,
            GameHandler::handle_default,
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
        );

        handler(ctx, packet.into_reader()).await?;

        if ctx.is_migrating() {
            ctx.set_migrate(false);
            return Ok(SessionHandleResult::Migrate);
        }

        Self::update_char_stats(ctx).await?;

        Ok(SessionHandleResult::Ok)
    }

    async fn finish(self, is_migrating: bool) -> Result<(), Self::Error> {
        log::info!("Finishing game session...");
        if is_migrating {
            self.services.session_manager.migrate_session(
                ShroomMigrationKey::new(self.client_key, self.addr),
                self.session.unmap(),
            )?;
        } else {
            self.services
                .session_manager
                .close_session(self.session.unmap())
                .await?;
        }

        Ok(())
    }
}

impl GameHandler {
    async fn handle_use_skill(ctx: &mut Ctx, req: UserSkillUseReq) -> anyhow::Result<()> {
        dbg!(&req);
        ctx.char_mut().use_skill(req.skill_id);

        Ok(())
    }

    async fn handle_client_dump_log(_ctx: &mut Ctx, req: ClientDumpLogReq) -> anyhow::Result<()> {
        dbg!(req);
        Ok(())
    }

    async fn handle_user_hit(ctx: &mut Ctx, req: UserHitReq) -> GameResult<()> {
        ctx.session.char.update_hp((req.dmg_internal as i16).neg());

        let stats = ctx.session.char.get_stats_partial();

        ctx.send(CharStatChangedResp {
            excl: false,
            stats: PartialFlag {
                hdr: (),
                data: stats,
            },
            secondary_stat: false,
            battle_recovery: false,
        })
        .await
    }

    async fn handle_stat_change(ctx: &mut Ctx, req: UserStatChangeReq) -> GameResult<()> {
        //TODO ?
        ctx.session.char.update_hp(req.hp as i16);
        ctx.session.char.update_mp(req.mp as i16);
        let stats = ctx.session.char.get_stats_partial();

        ctx.send(CharStatChangedResp {
            excl: false,
            stats: PartialFlag {
                hdr: (),
                data: stats,
            },
            secondary_stat: false,
            battle_recovery: false,
        })
        .await
    }

    fn char_mut(&mut self) -> &mut Character {
        &mut self.session.char
    }

    fn char(&self) -> &Character {
        &self.session.char
    }

    async fn handle_inv_change_slot(ctx: &mut Ctx, req: InvChangeSlotPosReq) -> anyhow::Result<()> {
        let count = (req.count != u16::MAX).then_some(req.count as usize);
        // Check for drop
        if req.to == 0 {
            let slot = (req.from as i16).try_into()?;
            if req.inv_type.is_equip() {
                ctx.char_mut().inventory.drop_equip_item(slot)?;
            } else {
                ctx.char_mut()
                    .inventory
                    .drop_stack_item(req.inv_type, slot, count)?;
            }
        } else {
            let src = (req.from as i16).try_into()?;
            let dst = (req.to as i16).try_into()?;
            ctx.session
                .char
                .inventory
                .move_item(req.inv_type, src, dst, count)?;
        }
        dbg!(&ctx.session.char.inventory.pending_operations);

        ctx.session.char.unlock_char();
        Ok(())
    }

    async fn handle_skill_up(ctx: &mut Ctx, req: UserSkillUpReq) -> GameResult<()> {
        ctx.session.char.skills.skill_up(req.skill_id)?;
        Ok(())
    }

    pub fn enable_char(&mut self) {
        self.session.char.unlock_char()
    }

    pub async fn handle_default(
        _ctx: &mut Ctx,
        op: RecvOpcodes,
        pr: PacketReader<'_>,
    ) -> anyhow::Result<()> {
        log::info!("Unhandled packet: {:?} {:?}", op, pr.into_inner());
        Ok(())
    }

    async fn init_char(ctx: &mut Ctx) -> anyhow::Result<()> {
        ctx.send(FriendResultResp::Reset3(FriendList::empty()))
            .await?;
        ctx.send(FuncKeyMapInitResp::default_map()).await?;
        ctx.send(ClaimSvrStatusChangedResp { connected: true })
            .await?;
        ctx.send(CtxSetGenderResp {
            gender: ctx.session.char.gender,
        })
        .await?;

        ctx.send(BroadcastMessageResp::PinkMessage("Hello".to_string()))
            .await?;

        ctx.session.char.unlock_char();

        Ok(())
    }

    async fn update_char_stats(ctx: &mut Ctx) -> GameResult<()> {
        if ctx.session.char.stats_changed() {
            let partial = ctx.session.char.get_stats_partial();
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

        if ctx.session.char.is_inventory_changed() {
            let ops = ctx.session.char.get_inventory_ops();
            ctx.send(InventoryOperationsResp {
                reset_excl: true,
                operations: ops.into(),
                secondary_stat_changed: false,
            })
            .await?;
        }

        if let Some(skills) = ctx.session.char.skills.get_updates() {
            ctx.send(ChangeSkillRecordResp {
                reset_excl: true,
                skill_records: skills.into(),
                updated_secondary_stat: false,
            })
            .await?;
        }

        Ok(())
    }

    async fn handle_update_screen_setting(
        _ctx: &mut Ctx,
        _req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_melee_attack(ctx: &mut Ctx, req: UserMeleeAttackReq) -> anyhow::Result<()> {
        for target in req.targets {
            let dmg = target.hits.iter().sum::<u32>();
            let mut sess_handle = ctx.session_handle.clone();
            ctx.field
                .attack_mob(target.mob_id, dmg, ctx.session.char.id, &mut sess_handle)
                .await?;
        }

        Ok(())
    }

    async fn handle_drop_pick_up(ctx: &mut Ctx, req: UserDropPickUpReq) -> GameResult<()> {
        /*ctx.field
        .handle_pickup(req.drop_id, &mut ctx.session.char)?;*/
        let drop = ctx.field.remove_drop(
            req.drop_id,
            DropLeaveParam::UserPickup(ctx.session.char.id as u32),
        )?;

        match drop.value {
            DropTypeValue::Mesos(money) => {
                ctx.session.char.update_mesos(money as i32);
            }
            DropTypeValue::Item(item_id) => {
                let inv_ty = item_id.get_inv_type()?;
                if !inv_ty.is_stack() {
                    let item = ctx.services.data.item.get_eq_item_from_id(item_id)?;

                    ctx.session.char.inventory.try_add_equip(item)?;
                } else {
                    let item = ctx
                        .services
                        .data
                        .item
                        .get_stack_item_from_id(item_id, drop.quantity)?;

                    ctx.session
                        .char
                        .inventory
                        .try_add_stack_item(item, inv_ty)?;
                };
            }
        }

        Ok(())
    }

    async fn handle_drop_money(ctx: &mut Ctx, req: UserDropMoneyReq) -> GameResult<()> {
        let ok = ctx.session.char.update_mesos((req.money as i32).neg());
        let char = &ctx.session.char;
        if ok {
            ctx.field.add_drop(Drop {
                owner: proto95::game::drop::DropOwner::User(char.id as u32),
                pos: char.pos,
                start_pos: char.pos,
                value: DropTypeValue::Mesos(req.money),
                quantity: 1,
            })?;
        }

        let stats = ctx.session.char.get_stats_partial();
        ctx.send(CharStatChangedResp {
            excl: true,
            stats: PartialFlag {
                hdr: (),
                data: stats,
            },
            secondary_stat: false,
            battle_recovery: false,
        })
        .await
    }

    async fn handle_chat_msg(ctx: &mut Ctx, req: ChatMsgReq) -> anyhow::Result<()> {
        let admin = false;
        if let Some(s) = req.msg.strip_prefix('@') {
            let repl_resp = Self::handle_repl(ctx, s).await?;
            let Some(msg) = repl_resp else {
                return Ok(())
            };
            let resp = UserChatMsgResp {
                char: ctx.session.char.id as u32,
                is_admin: admin,
                msg,
                only_balloon: false,
            };

            ctx.send(resp).await?;
        } else {
            ctx.field.add_chat(UserChatMsgResp {
                char: ctx.session.char.id as u32,
                is_admin: admin,
                msg: req.msg,
                only_balloon: req.only_balloon,
            })?;
        };
        Ok(())
    }

    async fn handle_mob_move(ctx: &mut Ctx, req: MobMoveReq) -> GameResult<()> {
        let ctrl_sn = req.ctrl_sn;
        let id = req.id;

        ctx.field.update_mob_pos(req, ctx.session.char.id)?;

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

    async fn handle_portal_script(ctx: &mut Ctx, _req: UserPortalScriptReq) -> GameResult<()> {
        ctx.enable_char();
        Ok(())
    }

    fn set_field(&self) -> SetFieldResp {
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

        let skill_records: ShroomList16<SkillInfo> =
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
            skillrecords: skill_records,
            skllcooltime: ShroomList16::default(),
            quests: ShroomList16::default(),
            questscompleted: ShroomList16::default(),
            minigamerecords: ShroomList16::default(),
            socialrecords: ShroomList16::default(),
            teleportrockinfo: TeleportRockInfo::default(),
            newyearcards: ShroomList16::default(),
            questrecordsexpired: ShroomList16::default(),
            questcompleteold: ShroomList16::default(),
            visitorquestloginfo: ShroomList16::default(),
        };

        let char_data = SetFieldCharData {
            notifications: NotificationList::default(),
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
            char_data_flags: CharDataFlagsAll,
        };

        SetFieldResp {
            client_option: ShroomList16::default(),
            channel_id: self.channel_id as u32,
            old_driver_id: 0,
            unknown_flag_1: 0,
            set_field_result: SetFieldResult::CharData(char_data),
            timestamp: ShroomTime::now(),
            extra: 0,
        }
    }

    async fn join_field(ctx: &mut Ctx, map: MapId, spawn_point: Option<u8>) -> GameResult<()> {
        ctx.session.char.transfer_map(map, spawn_point.unwrap_or(0));

        // Only get a new handle, when the map is new
        if map != ctx.field.get_field_id() {
            ctx.field = ctx
                .services
                .field
                .join_field(
                    ctx.session.char.id,
                    ctx.session.char.get_avatar_data(),
                    ctx.session_handle.clone(),
                    ctx.session.char.map_id,
                )
                .await?;
        }

        ctx.send(ctx.set_field()).await
    }

    async fn handle_field_transfer(ctx: &mut Ctx, req: UserTransferFieldReq) -> GameResult<()> {
        let (map, spawn) = if ctx.session.char.is_dead() {
            let return_map = MapId(ctx.field.get_meta().info.return_map.unwrap_or_default() as u32);

            let spawn_point = ctx
                .services
                .meta
                .get_field_data(return_map)
                .unwrap()
                .portal
                .first_key_value()
                .map(|(k, _)| *k)
                .unwrap_or_default() as u8;
            ctx.session.char.respawn();

            (return_map, spawn_point)
        } else {
            let portal = ctx
                .field
                .get_meta()
                .portal
                .values()
                .find(|p| p.pn == req.portal)
                .ok_or_else(|| anyhow::format_err!("Invalid portal"))?;

            // TODO(!) tm should be an option as mapid 999999 is invalid
            let map_id = if portal.tm == 999999 {
                ctx.field.get_field_id()
            } else {
                MapId(portal.tm as u32)
            };
            let spawn_point = ctx
                .services
                .meta
                .get_field_data(map_id)
                .unwrap()
                .portal
                .iter()
                .find(|(_, p)| p.pn == portal.tn)
                .map(|(id, _)| *id as u8)
                .unwrap_or(0);

            (map_id, spawn_point)
        };

        Self::join_field(ctx, map, Some(spawn)).await
    }

    async fn handle_movement(ctx: &mut Ctx, req: UserMoveReq) -> anyhow::Result<()> {
        ctx.char_mut().pos = req.move_path.pos;
        let last = req.move_path.get_last_pos_fh();

        if let Some((pos, fh)) = last {
            ctx.char_mut().pos = pos;
            ctx.char_mut().fh = fh.unwrap_or(ctx.char().fh);
        }

        ctx.field.update_user_pos(req, ctx.session.char.id)?;
        Ok(())
    }

    async fn handle_channel_transfer(ctx: &mut Ctx, req: TransferChannelReq) -> GameResult<()> {
        log::info!("Transfer channel: {:?}", req);
        let addr = ctx
            .services
            .server_info
            .get_channel_addr(ctx.world_id, req.channel_id as ChannelId)?;

        ctx.reply(MigrateResponse(MigrateCommandResp {
            unknown: true,
            addr: addr.try_into()?,
        }))
        .await
    }
}
