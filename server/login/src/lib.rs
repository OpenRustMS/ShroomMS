pub mod config;
pub mod login_state;

use std::net::IpAddr;

use async_trait::async_trait;
use config::LoginConfig;
use data::services::data::account::AccountServiceError;
use data::services::data::character::{CharacterCreateDTO, ItemStarterSet};
use data::services::session::ShroomMigrationKey;
use data::{entities::character, services};
use login_state::LoginState;

use proto95::shared::char::AvatarEquips;
use proto95::shared::{ExceptionLogReq, PongReq};
use proto95::{
    id::{FaceId, HairId, ItemId, Skin},
    login::{
        account::{
            BlockedIp, CheckPasswordReq, CheckPasswordResp, ConfirmEULAReq, ConfirmEULAResp,
            LoginAccountData, LoginInfo, SetGenderReq, SuccessResult,
        },
        char::{
            CharRankInfo, CheckDuplicateIDReq, CheckDuplicateIDResp, CheckDuplicateIDResult,
            CreateCharReq, CreateCharResp, DeleteCharReq, DeleteCharResp, DeleteCharResult,
            MigrateStageInfo, SelectCharReq, SelectCharResp, SelectCharResult, SelectWorldCharList,
            SelectWorldResp, ViewChar, ViewCharWithRank,
        },
        pin::{CheckPinReq, CheckPinResp, UpdatePinReq, UpdatePinResp},
        world::{
            ChannelId, LogoutWorldReq, SelectWorldReq, WorldCheckUserLimitReq,
            WorldCheckUserLimitResp, WorldId, WorldInfoReq, WorldInfoResp, WorldReq,
        },
        CreateSecurityHandleReq, LoginOpt, LoginResultHeader,
    },
    recv_opcodes::RecvOpcodes,
    shared::{
        char::{AvatarData, CharStat, PetIds},
        UpdateScreenSettingReq,
    },
};
use shroom_net::net::service::handler::ShroomSessionHandler;
use shroom_net::net::service::resp::MigrateResponse;
use shroom_net::net::service::{SessionHandleResult, ShroomContext};

use shroom_net::packet::list::ShroomIndexList8;
use shroom_net::packet::time::ShroomTime;
use shroom_net::packet::ShroomList8;
use shroom_net::{shroom_router_fn, HasOpcode, PacketReader, ShroomPacket};
use tokio::net::TcpStream;

pub type LoginResult<T> = Result<T, anyhow::Error>;

pub struct LoginHandler {
    services: services::SharedServices,
    addr: IpAddr,
    cfg: &'static LoginConfig,
    login_state: LoginState,
}

impl LoginHandler {
    pub fn new(
        services: services::SharedServices,
        cfg: &'static LoginConfig,
        addr: IpAddr,
    ) -> Self {
        Self {
            services,
            cfg,
            addr,
            login_state: LoginState::new(),
        }
    }
}

type Ctx = ShroomContext<LoginHandler>;

#[async_trait]
impl ShroomSessionHandler for LoginHandler {
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
            LoginHandler,
            anyhow::Error,
            LoginHandler::handle_default,
            PongReq => LoginHandler::handle_pong,
            CreateSecurityHandleReq => LoginHandler::handle_create_security_handle,
            UpdateScreenSettingReq => LoginHandler::handle_update_screen_setting,
            CheckPasswordReq => LoginHandler::handle_check_password,
            SetGenderReq => LoginHandler::handle_set_gender,
            CheckPinReq => LoginHandler::handle_check_pin,
            UpdatePinReq => LoginHandler::handle_register_pin,
            ConfirmEULAReq => LoginHandler::handle_accept_tos,
            WorldInfoReq => LoginHandler::handle_world_information,
            LogoutWorldReq => LoginHandler::handle_world_logout,
            WorldReq => LoginHandler::handle_world_request,
            WorldCheckUserLimitReq => LoginHandler::handle_world_check_user_limit,
            SelectWorldReq => LoginHandler::handle_select_world,
            CheckDuplicateIDReq => LoginHandler::handle_check_duplicate_id,
            CreateCharReq => LoginHandler::handle_create_char,
            DeleteCharReq => LoginHandler::handle_delete_character,
            SelectCharReq => LoginHandler::handle_select_char,
            ExceptionLogReq => LoginHandler::handle_exception_log
        );

        handler(ctx, packet.into_reader()).await?;
        Ok(SessionHandleResult::Ok)
    }
}

impl LoginHandler {
    pub async fn handle_default(
        _ctx: &mut Ctx,
        _op: RecvOpcodes,
        pr: PacketReader<'_>,
    ) -> anyhow::Result<()> {
        log::info!("Unhandled packet: {:?}", pr.into_inner());
        Ok(())
    }

    async fn handle_pong(_ctx: &mut Ctx, _req: PongReq) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_exception_log(_ctx: &mut Ctx, _req: ExceptionLogReq) -> LoginResult<()> {
        dbg!(&_req);
        Ok(())
    }

    async fn handle_create_security_handle(
        _ctx: &mut Ctx,
        _req: CreateSecurityHandleReq,
    ) -> anyhow::Result<()> {
        dbg!(&_req);
        Ok(())
    }

    async fn handle_update_screen_setting(
        _ctx: &mut Ctx,
        req: UpdateScreenSettingReq,
    ) -> anyhow::Result<()> {
        dbg!(&req);
        Ok(())
    }

    async fn handle_accept_tos(ctx: &mut Ctx, req: ConfirmEULAReq) -> LoginResult<()> {
        ctx.login_state.get_accept_tos()?;

        if !req.accepted {
            anyhow::bail!("Should accept the TOS");
        }

        let svc = ctx.services.clone();

        ctx.login_state
            .update_account(|acc| svc.data.account.accept_tos(acc))
            .await?;
        ctx.login_state.reset();

        ctx.send(ConfirmEULAResp { success: true }).await
    }

    async fn handle_check_pin(ctx: &mut Ctx, req: CheckPinReq) -> LoginResult<()> {
        let acc = ctx.login_state.get_pin()?;

        ctx.send(if ctx.cfg.enable_pin {
            match req.pin.opt {
                Some(pin) => {
                    if ctx.services.data.account.check_pin(acc, &pin.pin)? {
                        CheckPinResp::Accepted
                    } else {
                        CheckPinResp::InvalidPin
                    }
                }
                _ => CheckPinResp::EnterPin,
            }
        } else {
            CheckPinResp::Accepted
        })
        .await
    }

    async fn handle_register_pin(ctx: &mut Ctx, req: UpdatePinReq) -> LoginResult<()> {
        ctx.login_state.get_pin()?;

        let Some(pin) = req.pin.opt else {
            //TODO handle a login reset here not a dc
            anyhow::bail!("Pin registration cancelled");
        };

        let svc = ctx.services.clone();

        ctx.login_state
            .update_account(|acc| svc.data.account.set_pin(acc, pin))
            .await?;

        ctx.send(UpdatePinResp { success: true }).await
    }

    async fn handle_set_gender(ctx: &mut Ctx, req: SetGenderReq) -> LoginResult<()> {
        let _ = ctx.login_state.get_set_gender()?;

        let gender = req
            .gender
            .opt
            .ok_or_else(|| anyhow::format_err!("Gender not set"))?;

        let svc = ctx.services.clone();

        ctx.login_state
            .update_account(|acc| svc.data.account.set_gender(acc, gender.into()))
            .await?;
        /*
        ctx.login_state.transition_login().unwrap();

        //TODO this doesn't set the client key, maybe make it dc?
        Ok(SetGenderResp {
            gender,
            success: true,
        })
        */

        todo!("Set gender");
    }

    async fn handle_world_logout(ctx: &mut Ctx, _req: LogoutWorldReq) -> LoginResult<()> {
        ctx.login_state.get_char_select()?;
        ctx.login_state.transition_server_select()?;

        Ok(())
    }

    async fn handle_world_check_user_limit(
        ctx: &mut Ctx,
        _req: WorldCheckUserLimitReq,
    ) -> LoginResult<()> {
        let _acc = ctx.login_state.get_server_selection()?;

        ctx.send(WorldCheckUserLimitResp {
            over_user_limit: false,
            populate_level: 0,
        })
        .await
    }

    fn get_world_info(&self) -> Vec<WorldInfoResp> {
        self.services
            .server_info
            .get_world_info_packets()
            .into_iter()
            .collect()
    }

    async fn handle_world_information(ctx: &mut Ctx, _req: WorldInfoReq) -> LoginResult<()> {
        ctx.reply(ctx.get_world_info()).await
    }

    async fn handle_world_request(ctx: &mut Ctx, _req: WorldReq) -> LoginResult<()> {
        ctx.reply(ctx.get_world_info()).await
    }

    pub async fn handle_check_password(ctx: &mut Ctx, req: CheckPasswordReq) -> LoginResult<()> {
        let login_result = ctx.services.data.account.try_login(&req.id, &req.pw).await;
        let hdr = LoginResultHeader::default();

        let resp = match login_result {
            Err(AccountServiceError::UsernameNotFound) => CheckPasswordResp::InvalidUserName(hdr),
            Err(AccountServiceError::PasswordMismatch) => CheckPasswordResp::InvalidPassword(hdr),
            Err(AccountServiceError::AccountIsBanned) => CheckPasswordResp::BlockedIp(BlockedIp {
                hdr,
                reason: 0,
                ban_time: ShroomTime::now(), // TODO
            }),
            Ok(acc) => {
                let account_info = (&acc).into();
                let login_session = ctx
                    .services
                    .session_manager
                    .create_claimed_session(acc)
                    .await?;

                ctx.login_state
                    .transition_login_with_session(login_session.as_login())?;
                let client_key = ctx
                    .login_state
                    .get_client_key()
                    .expect("Must have client key after login");

                let login_info = (!ctx.login_state.is_set_gender_stage())
                    .then_some(LoginInfo {
                        skip_pin: false,
                        login_opt: proto95::login::LoginOpt::EnableSecondPassword,
                        client_key,
                    })
                    .into();

                if ctx.login_state.is_accept_tos_stage() {
                    CheckPasswordResp::TOS(hdr)
                } else {
                    CheckPasswordResp::Success(SuccessResult {
                        hdr,
                        account: LoginAccountData {
                            account_info,
                            login_info,
                        },
                    })
                }
            }
            _ => todo!("Unhandled Account Service Login Result: {:?}", login_result),
        };

        ctx.send(resp).await
    }

    async fn handle_select_world(ctx: &mut Ctx, req: SelectWorldReq) -> LoginResult<()> {
        let acc = ctx.login_state.get_server_selection()?;
        let char_list = ctx
            .services
            .data
            .char
            .get_characters_for_account(acc.id)
            .await?;
        let characters: ShroomList8<_> = char_list.iter().map(map_char_with_rank).collect();

        let char_list = SelectWorldCharList {
            characters,
            //TODO pic handling
            login_opt: LoginOpt::NoSecondPassword1,
            slot_count: acc.character_slots as u32,
            //TODO get buy count
            buy_char_count: 3,
        };
        ctx.login_state
            .transition_char_select(req.world_id as WorldId, req.channel_id as ChannelId)?;

        ctx.send(SelectWorldResp::Success(char_list)).await
    }

    async fn handle_check_duplicate_id(ctx: &mut Ctx, req: CheckDuplicateIDReq) -> LoginResult<()> {
        let _ = ctx.login_state.get_char_select()?;
        let name_used = !ctx.services.data.char.check_name(&req.name).await?;

        let resp = if name_used {
            CheckDuplicateIDResp {
                name: "".to_string(),
                result: CheckDuplicateIDResult::Error1,
            }
        } else {
            CheckDuplicateIDResp {
                name: req.name,
                result: CheckDuplicateIDResult::Success,
            }
        };

        ctx.send(resp).await
    }

    async fn handle_create_char(ctx: &mut Ctx, req: CreateCharReq) -> LoginResult<()> {
        let (acc, _, _) = ctx.login_state.get_char_select()?;

        let starter_set = ItemStarterSet {
            shoes: req.starter_set.shoes,
            bottom: req.starter_set.bottom,
            weapon: req.starter_set.weapon,
            top: req.starter_set.top,
            guide: req.job.get_guide_item(),
        };

        let char_id = ctx
            .services
            .data
            .char
            .create_character(
                acc.id,
                CharacterCreateDTO {
                    name: req.name,
                    job_group: req.job,
                    face: req.starter_set.face,
                    skin: (req.starter_set.skin_color as u8).try_into()?,
                    hair: req.starter_set.hair,
                    //TODO hair color
                    starter_set,
                    gender: req.gender,
                },
                &ctx.services.data.item,
            )
            .await?;

        let char = ctx.services.data.char.get(char_id).await?.unwrap();
        ctx.send(CreateCharResp::Success(map_char(&char))).await
    }

    async fn handle_delete_character(ctx: &mut Ctx, req: DeleteCharReq) -> LoginResult<()> {
        let (acc, _, _) = ctx.login_state.get_char_select()?;
        let status = ctx
            .services
            .data
            .char
            .delete_character(acc, req.char_id as i32, &req.pic)
            .await?;

        let result = match status {
            DeleteCharResult::Success => DeleteCharResult::Success,
            //TODO add more
            _ => DeleteCharResult::UnknownErr,
        };

        ctx.send(DeleteCharResp {
            char_id: req.char_id,
            result,
        })
        .await
    }

    async fn handle_select_char(ctx: &mut Ctx, req: SelectCharReq) -> LoginResult<()> {
        let (_, world, channel) = ctx.login_state.get_char_select()?;

        let mut session = ctx.login_state.claim_session()?.unmap();
        let client_key = ctx.login_state.get_client_key()?;

        ctx.services
            .session_manager
            .transition_session(&mut session, req.char_id as i32)
            .await?;

        ctx.services
            .session_manager
            .migrate_session(ShroomMigrationKey::new(client_key, ctx.addr), session)?;

        let addr = ctx.services.server_info.get_channel_addr(world, channel)?;
        let migrate = MigrateStageInfo {
            socket_addr: addr.try_into()?,
            char_id: req.char_id,
            premium: false,
            premium_arg: 0,
        };

        let pkt = SelectCharResp {
            error_code: 0,
            result: SelectCharResult::Success(migrate),
        };

        ctx.reply(MigrateResponse::<SelectCharResp>(pkt)).await
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

pub fn map_rank_info(_char: &character::Model) -> CharRankInfo {
    CharRankInfo {
        world_rank: 0,
        rank_move: 0,
        job_rank: 0,
        job_rank_mode: 0,
    }
}

pub fn map_char(char: &character::Model) -> ViewChar {
    let stats: CharStat = char.into();
    let avatar_data = map_char_to_avatar(char);

    ViewChar { stats, avatar_data }
}

fn map_char_with_rank(char: &character::Model) -> ViewCharWithRank {
    ViewCharWithRank {
        view_char: map_char(char),
        u1: 0,
        rank_info: Some(map_rank_info(char)).into(),
    }
}
