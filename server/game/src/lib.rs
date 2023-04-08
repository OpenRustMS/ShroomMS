pub mod repl;
pub mod state;

use std::ops::Neg;

use std::{net::IpAddr, time::Duration};

use async_trait::async_trait;

use data::entities::character;
use data::services::field::FieldJoinHandle;
use data::services::helper::pool::drop::{DropLeaveParam, DropTypeValue};
use data::services::session::session_data::OwnedShroomSession;
use data::services::session::{ClientKey, ShroomMigrationKey};
use data::services::SharedServices;
use shroom_net::net::service::handler::{
    MakeServerSessionHandler, SessionHandleResult, ShroomServerSessionHandler, ShroomSessionHandler,
};
use shroom_net::net::service::resp::{
    MigrateResponse, PacketOpcodeExt, PongResponse, ResponsePacket,
};
use shroom_net::net::service::server_sess::SharedSessionHandle;
use shroom_net::net::ShroomSession;
use shroom_net::{shroom_router_handler, HasOpcode};

use shroom_net::packet::EncodePacket;

use shroom_net::packet::proto::list::{ShroomIndexList8, ShroomIndexListZ};
use shroom_net::packet::proto::partial::PartialFlag;
use shroom_net::packet::proto::time::ShroomExpiration;
use shroom_net::packet::{
    proto::{
        list::{ShroomIndexListZ16, ShroomIndexListZ8},
        time::ShroomTime,
        ShroomList16,
    },
    DecodePacket, PacketReader, PacketWriter, ShroomPacket,
};

use data::services::helper::pool::Drop;

use proto95::game::mob::{MobMoveCtrlAckResp, MobMoveReq};
use proto95::game::user::{
    ChangeSkillRecordResp, UpdatedSkillRecord, UserDropMoneyReq, UserDropPickUpReq, UserHitReq,
    UserMeleeAttackReq, UserSkillUpReq, UserStatChangeReq,
};

use proto95::id::{FaceId, HairId, ItemId, Skin};
use proto95::shared::char::{AvatarData, AvatarEquips, PetIds, SkillInfo, TeleportRockInfo};
use proto95::shared::inventory::InvChangeSlotPosReq;
use proto95::shared::{ClientDumpLogReq, FootholdId, PongReq, Vec2};
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
    send_opcodes::SendOpcodes,
    shared::{
        char::{
            CharDataAll, CharDataEquipped, CharDataFlagsAll, CharDataHeader, CharDataStat,
            CharStatChangedResp, CharStatPartial,
        },
        item::Item,
        UpdateScreenSettingReq,
    },
};
use repl::GameRepl;
use tokio::net::TcpStream;

pub type GameResponse<T> = ResponsePacket<SendOpcodes, T>;
pub type GameResult<T> = Result<GameResponse<T>, anyhow::Error>;

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

#[async_trait::async_trait]
impl MakeServerSessionHandler for MakeGameHandler {
    type Transport = TcpStream;

    type Error = anyhow::Error;

    type Handler = GameHandler;

    async fn make_handler(
        &mut self,
        sess: &mut ShroomSession<Self::Transport>,
        sess_handle: SharedSessionHandle,
    ) -> Result<Self::Handler, Self::Error> {
        let mut handler = GameHandler::from_session(
            sess,
            self.services.clone(),
            self.channel_id,
            self.world_id,
            sess_handle,
        )
        .await?;
        sess.send_packet(handler.set_field()).await?;
        handler.init_char(sess).await?;

        Ok(handler)
    }
}

pub struct GameHandler {
    session: OwnedShroomSession,
    channel_id: ChannelId,
    world_id: WorldId,
    services: SharedServices,
    addr: IpAddr,
    client_key: ClientKey,
    sess_handle: SharedSessionHandle,
    pos: Vec2,
    fh: FootholdId,
    field: FieldJoinHandle,
    repl: GameRepl,
    avatar_data: AvatarData,
}

impl GameHandler {
    pub async fn from_session(
        net_session: &mut ShroomSession<TcpStream>,
        services: SharedServices,
        channel_id: ChannelId,
        world_id: WorldId,
        sess_handle: SharedSessionHandle,
    ) -> anyhow::Result<Self> {
        let addr = net_session.peer_addr()?;
        log::info!("Game sess: {} - waiting abit for session to be free", addr);

        let pkt = net_session.read_packet().await?;
        log::info!("Migration: {:?}", pkt);
        let mut pr = pkt.into_reader();

        let op = pr.read_opcode::<RecvOpcodes>()?;
        log::info!("New client with opcode: {:?}", op);
        if op != MigrateInGameReq::OPCODE {
            anyhow::bail!("Wrong client hello packet: {op:?}")
        }

        let req = MigrateInGameReq::decode_packet(&mut pr)?;
        let addr = net_session.peer_addr()?.ip();

        dbg!(ShroomMigrationKey::new(req.client_key, addr));

        let session = services
            .session_manager
            .claim_migration_session(ShroomMigrationKey::new(req.client_key, addr))
            .await?;

        log::info!(
            "Session for acc: {} - char: {}",
            session.acc.username,
            session.char.model.name
        );

        let avatar_data = map_char_to_avatar(&session.char.model);

        let join_field = services
            .field
            .join_field(
                session.char.model.id,
                avatar_data.clone(),
                sess_handle.clone(),
                MapId(session.char.model.map_id as u32),
            )
            .await?;

        Ok(Self {
            session,
            services,
            channel_id,
            world_id,
            addr,
            client_key: req.client_key,
            pos: Vec2::default(),
            fh: 0,
            sess_handle,
            field: join_field,
            repl: GameRepl::new(),
            avatar_data,
        })
    }
}

#[async_trait]
impl ShroomSessionHandler for GameHandler {
    type Transport = TcpStream;
    type Error = anyhow::Error;

    async fn handle_packet(
        &mut self,
        packet: ShroomPacket,
        session: &mut ShroomSession<Self::Transport>,
    ) -> Result<SessionHandleResult, Self::Error> {
        shroom_router_handler!(
            handler,
            GameHandler,
            ShroomSession<TcpStream>,
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
        );

        Ok(handler(self, session, packet.into_reader()).await?)
    }

    async fn finish(self, is_migrating: bool) -> Result<(), Self::Error> {
        log::info!("Finishing game session...");
        if is_migrating {
            self.services.session_manager.migrate_session(
                ShroomMigrationKey::new(self.client_key, self.addr),
                self.session,
            )?;
        } else {
            self.services
                .session_manager
                .close_session(self.session)
                .await?;
        }

        Ok(())
    }
}

impl ShroomServerSessionHandler for GameHandler {
    fn get_ping_interval() -> std::time::Duration {
        Duration::from_secs(30)
    }

    fn get_ping_packet(&mut self) -> Result<ShroomPacket, Self::Error> {
        let mut pw = PacketWriter::default();
        pw.write_opcode(SendOpcodes::AliveReq)?;
        Ok(pw.into_packet())
    }
}

impl GameHandler {
    async fn handle_client_dump_log(&mut self, req: ClientDumpLogReq) -> anyhow::Result<()> {
        dbg!(req);
        Ok(())
    }

    async fn handle_user_hit(&mut self, req: UserHitReq) -> GameResult<CharStatChangedResp> {
        self.session.char.update_hp((req.dmg_internal as i32).neg());

        Ok(CharStatChangedResp {
            excl: false,
            stats: PartialFlag {
                hdr: (),
                data: self.session.char.get_char_partial(),
            },
            secondary_stat: false,
            battle_recovery: false,
        }
        .into())
    }

    async fn handle_stat_change(
        &mut self,
        req: UserStatChangeReq,
    ) -> GameResult<CharStatChangedResp> {
        self.session.char.update_hp(req.hp as i32);
        self.session.char.update_mp(req.mp as i32);

        Ok(CharStatChangedResp {
            excl: false,
            stats: PartialFlag {
                hdr: (),
                data: self.session.char.get_char_partial(),
            },
            secondary_stat: false,
            battle_recovery: false,
        }
        .into())
    }

    async fn handle_inv_change_slot(&mut self, _req: InvChangeSlotPosReq) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_pong(&mut self, _req: PongReq) -> anyhow::Result<PongResponse> {
        Ok(PongResponse)
    }

    async fn handle_skill_up(&mut self, req: UserSkillUpReq) -> GameResult<ChangeSkillRecordResp> {
        Ok(ChangeSkillRecordResp {
            reset_excl: true,
            skill_records: vec![UpdatedSkillRecord {
                id: req.skill_id,
                level: 1,
                master_level: 0,
                expiration: ShroomExpiration::never(),
            }]
            .into(),
            updated_secondary_stat: false,
        }
        .into())
    }

    pub fn enable_char(&mut self) -> CharStatChangedResp {
        CharStatChangedResp {
            excl: true,
            stats: PartialFlag {
                hdr: (),
                data: CharStatPartial {
                    ..CharStatPartial::default()
                },
            },
            secondary_stat: false,
            battle_recovery: false,
        }
    }

    pub async fn handle_default(
        &mut self,
        op: RecvOpcodes,
        pr: PacketReader<'_>,
    ) -> anyhow::Result<SessionHandleResult> {
        log::info!("Unhandled packet: {:?} {:?}", op, pr.into_inner());
        Ok(SessionHandleResult::Ok)
    }

    async fn init_char(&mut self, sess: &mut ShroomSession<TcpStream>) -> anyhow::Result<()> {
        sess.send_packet(FriendResultResp::Reset3(FriendList::empty()))
            .await?;
        sess.send_packet(FuncKeyMapInitResp::default_map()).await?;
        sess.send_packet(ClaimSvrStatusChangedResp { connected: true })
            .await?;
        sess.send_packet(CtxSetGenderResp {
            gender: (&self.session.char.model.gender).into(),
        })
        .await?;

        sess.send_packet(BroadcastMessageResp::PinkMessage("Hello".to_string()))
            .await?;

        sess.send_packet(self.enable_char()).await?;

        Ok(())
    }

    fn set_field(&mut self) -> SetFieldResp {
        let char = &self.session.char;

        let equipped: ShroomIndexListZ16<Item> = self
            .session
            .char
            .inventory
            .equipped
            .iter()
            .map(|(slot, item)| (slot as u16, Item::Equip(item.item.as_ref().into())))
            .collect();

        let etc: ShroomIndexListZ8<Item> = self
            .session
            .char
            .inventory
            .etc
            .iter()
            .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.item.as_ref().into())))
            .collect();

        let invsize = [
            char.model.equip_slots as u8,
            char.model.use_slots as u8,
            char.model.setup_slots as u8,
            char.model.etc_slots as u8,
            char.model.cash_slots as u8,
        ];

        let char_equipped = CharDataEquipped {
            equipped,
            ..Default::default()
        };

        let skill_records: ShroomList16<SkillInfo> = self
            .session
            .skills
            .iter()
            .map(|(id, skill)| SkillInfo {
                id: *id,
                level: skill.skill_level as u32,
                expiration: skill.expires_at.into(),
                master_level: skill.master_level as u32,
            })
            .collect();

        let char_stat: &character::Model = &char.model.clone();

        let char_data = CharDataAll {
            stat: CharDataStat {
                stat: char_stat.into(),
                friend_max: 30,
                linked_character: None.into(),
            },
            money: char.model.mesos as u32,
            invsize,
            equipextslotexpiration: ShroomExpiration::never(),
            equipped: char_equipped,
            useinv: ShroomIndexListZ::default(),
            setupinv: ShroomIndexListZ::default(),
            etcinv: etc,
            cashinv: ShroomIndexListZ::default(),
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
            timestamp: ShroomTime::utc_now(),
            extra: 0,
        }
    }

    async fn handle_update_screen_setting(
        &mut self,
        _req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_melee_attack(&mut self, req: UserMeleeAttackReq) -> anyhow::Result<()> {
        for target in req.targets {
            let dmg = target.hits.iter().sum::<u32>();
            self.field
                .attack_mob(
                    target.mob_id,
                    dmg,
                    self.session.char.model.id,
                    &mut self.sess_handle,
                )
                .await?;
        }

        Ok(())
    }

    async fn handle_drop_pick_up(
        &mut self,
        req: UserDropPickUpReq,
    ) -> GameResult<CharStatChangedResp> {
        dbg!(&req);

        self.field
            .handle_pickup(req.drop_id, &mut self.session.char)?;
        self.field.remove_drop(
            req.drop_id,
            DropLeaveParam::UserPickup(self.session.char.model.id as u32),
        )?;
        Ok(CharStatChangedResp {
            excl: true,
            stats: PartialFlag {
                hdr: (),
                data: self.session.char.get_char_partial(),
            },
            secondary_stat: false,
            battle_recovery: false,
        }
        .into())
    }

    async fn handle_drop_money(
        &mut self,
        req: UserDropMoneyReq,
    ) -> GameResult<CharStatChangedResp> {
        let ok = self.session.char.update_mesos((req.money as i32).neg());
        if ok {
            self.field.add_drop(Drop {
                owner: proto95::game::drop::DropOwner::User(self.session.char.model.id as u32),
                pos: self.pos,
                start_pos: self.pos,
                value: DropTypeValue::Mesos(req.money),
                quantity: 1,
            })?;
        }
        Ok(CharStatChangedResp {
            excl: true,
            stats: PartialFlag {
                hdr: (),
                data: self.session.char.get_char_partial(),
            },
            secondary_stat: false,
            battle_recovery: false,
        }
        .into())
    }

    async fn handle_chat_msg(&mut self, req: ChatMsgReq) -> anyhow::Result<()> {
        let admin = false;
        if let Some(s) = req.msg.strip_prefix('@') {
            let repl_resp = self.handle_repl(s).await?;
            let Some(msg) = repl_resp else {
                return Ok(())
            };
            let resp = UserChatMsgResp {
                char: self.session.char.model.id as u32,
                is_admin: admin,
                msg,
                only_balloon: false,
            };
            let mut pw = PacketWriter::default();
            pw.write_opcode(UserChatMsgResp::OPCODE)?;
            resp.encode_packet(&mut pw)?;

            self.sess_handle.tx.try_send(pw.into_packet().as_ref())?;
        } else {
            self.field.add_chat(UserChatMsgResp {
                char: self.session.char.model.id as u32,
                is_admin: admin,
                msg: req.msg,
                only_balloon: req.only_balloon,
            })?;
        };
        Ok(())
    }

    async fn handle_mob_move(&mut self, req: MobMoveReq) -> GameResult<MobMoveCtrlAckResp> {
        let ctrl_sn = req.ctrl_sn;
        let id = req.id;

        self.field.update_mob_pos(req, self.session.char.model.id)?;

        Ok(MobMoveCtrlAckResp {
            id,
            ctrl_sn,
            next_atk_possible: false,
            mp: 0,
            skill_id: 0,
            slv: 0,
        }
        .into())
    }

    async fn handle_portal_script(
        &mut self,
        _req: UserPortalScriptReq,
    ) -> GameResult<CharStatChangedResp> {
        Ok(self.enable_char().into())
    }

    async fn handle_field_transfer(
        &mut self,
        req: UserTransferFieldReq,
    ) -> GameResult<SetFieldResp> {
        if self.session.char.model.hp.le(&0) {
            let return_map =
                MapId(self.field.get_meta().info.return_map.unwrap_or_default() as u32);

            self.session.char.model.hp = 1;
            self.session.char.model.mp = 1;
            self.session.char.model.map_id = return_map.0 as i32;
            self.session.char.model.spawn_point = self
                .services
                .meta
                .get_field_data(return_map)
                .unwrap()
                .portal
                .first_key_value()
                .map(|(k, _)| *k)
                .unwrap_or_default() as i32;

            self.field = self
                .services
                .field
                .join_field(
                    self.session.char.model.id,
                    self.avatar_data.clone(),
                    self.sess_handle.clone(),
                    MapId(self.session.char.model.map_id as u32),
                )
                .await?;

            self.field = self
                .services
                .field
                .join_field(
                    self.session.char.model.id,
                    self.avatar_data.clone(),
                    self.sess_handle.clone(),
                    MapId(self.session.char.model.map_id as u32),
                )
                .await?;

            Ok(self.set_field().into())
        } else {
            let portal = self
                .field
                .get_meta()
                .portal
                .values()
                .find(|p| p.pn == req.portal)
                .ok_or_else(|| anyhow::format_err!("Invalid portal"))?;

            // TODO(!) tm should be an option as mapid 999999 is invalid
            let map_id = MapId(portal.tm as u32);
            self.session.char.model.map_id = map_id.0 as i32;
            self.session.char.model.spawn_point = self
                .services
                .meta
                .get_field_data(map_id)
                .unwrap()
                .portal
                .iter()
                .find(|(_, p)| p.pn == portal.tn)
                .map(|(id, _)| *id as u8)
                .unwrap_or(0) as i32;

            self.field = self
                .services
                .field
                .join_field(
                    self.session.char.model.id,
                    self.avatar_data.clone(),
                    self.sess_handle.clone(),
                    MapId(self.session.char.model.map_id as u32),
                )
                .await?;

            let transfer_field = self.set_field();
            Ok(transfer_field.into())
        }
    }

    async fn handle_movement(&mut self, req: UserMoveReq) -> anyhow::Result<()> {
        self.pos = req.move_path.pos;
        let last = req.move_path.get_last_pos_fh();

        if let Some((pos, fh)) = last {
            self.pos = pos;
            self.fh = fh.unwrap_or(self.fh);
        }

        self.field
            .update_user_pos(req, self.session.char.model.id)?;
        Ok(())
    }

    async fn handle_channel_transfer(
        &mut self,
        req: TransferChannelReq,
    ) -> anyhow::Result<MigrateResponse<ResponsePacket<SendOpcodes, MigrateCommandResp>>> {
        log::info!("Transfer channel: {:?}", req);
        let addr = self
            .services
            .server_info
            .get_channel_addr(self.world_id, req.channel_id as ChannelId)?;

        let pkt: ResponsePacket<_, _> = MigrateCommandResp {
            unknown: true,
            addr: addr.try_into()?,
        }
        .into_response(MigrateCommandResp::OPCODE);

        Ok(MigrateResponse(pkt))
    }
}

pub fn map_char_to_avatar(char: &character::Model) -> AvatarData {
    AvatarData {
        gender: (&char.gender).into(),
        skin: Skin::try_from(char.skin as u8).unwrap(),
        mega: true,
        face: FaceId(char.face as u32),
        hair: HairId(char.hair as u32),
        equips: AvatarEquips {
            equips: ShroomIndexList8::from(vec![
                (5, ItemId(1040006)),
                (6, ItemId(1060006)),
                (7, ItemId(1072005)),
                (11, ItemId(1322005)),
            ]),
            masked_equips: ShroomIndexList8::from(vec![]),
            weapon_sticker_id: ItemId(0),
        },
        pets: PetIds::default(),
    }
}
