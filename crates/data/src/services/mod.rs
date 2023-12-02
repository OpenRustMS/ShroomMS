pub mod character;
pub mod data;
pub mod field;
pub mod game;
pub mod helper;
pub mod item;
pub mod server_info;
pub mod session;

use std::{sync::Arc, time::Duration};

use plugins::npc::NpcScriptHandle;
use proto95::{
    game::MigrateInGameReq,
    id::{job_id::JobGroup, FaceId, FieldId, HairId, Skin},
    shared::Gender,
};
use sea_orm::{DatabaseConnection, DbErr};
use server_info::{ServerInfo, ServerService};
use shroom_pkt::{util::packet_buf::PacketBuf, ShroomPacket, ShroomPacketBytes};
use shroom_srv::{
    srv::{
        server_socket::ServerSocketHandle, server_system::{SystemHandler, ServerSystemTx},
    },
    util::clock::GameClockRef,
    Context,
};

use crate::{
    entities::sea_orm_active_enums::GenderTy,
    services::{
        game::GameSession,
        session::{shroom_session_backend::ShroomSessionData, ShroomMigrationKey},
    },
};

use meta::MetaService;

use self::{
    data::{
        account::{AccountId, Region},
        character::{CharacterCreateDTO, CharacterID, ItemStarterSet},
        DataServices,
    },
    field::{FieldHandler, FieldService, SharedFieldState},
    session::{ShroomSessionBackend, ShroomSessionManager},
};

pub struct GameSystem {
    pub services: Arc<Services>,
}

impl SystemHandler for GameSystem {
    type Ctx = GameCtx;
    type Msg = ();
    type RoomHandler = FieldHandler;
    type RoomId = FieldId;
    type SessionHandler = GameSession;

    async fn create_session(
        ctx: &Self::Ctx,
        sck: &mut ServerSocketHandle,
    ) -> anyhow::Result<Self::SessionHandler> {
        log::info!("New session: {:?}", sck.addr());
        // Read handshake packet
        let pkt = sck
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Handshake packet not received"))?;
        let mut pr = pkt.into_reader();
        let req = MigrateInGameReq::decode_packet(&mut pr)?;

        let migrate_key = ShroomMigrationKey::new(req.client_key, sck.addr().ip());
        let session = ctx
            .services
            .session_manager
            .claim_migration_session(migrate_key)
            .await?;

        // TODO, add a try_map function to owned session
        let session = session.try_map(|sess| match sess {
            ShroomSessionData::Login(_) => anyhow::bail!("Expected ingame session"),
            ShroomSessionData::Ingame(sess) => Ok(sess),
        })?;

        log::info!("Claimed session");

        log::info!(
            "Game session for acc: {} - char: {}",
            session.acc.username,
            session.char.name
        );
        log::info!("Spawning");

        let field_id = session.char.map_id;

        let sess = GameSession {
            services: ctx.services.clone(),
            session,
            addr: sck.addr().ip(),
            channel_id: 0,
            world_id: 0,
            client_key: req.client_key,
            script_handle: NpcScriptHandle::default(),
            field_id,
            field_meta: ctx.services.game.meta.get_field_data(field_id).unwrap()
        };
        Ok(sess)
    }

    fn create_room(&mut self, room_id: Self::RoomId) -> anyhow::Result<Self::RoomHandler> {
        log::info!("Creating room: {room_id}");
        let meta = self.services.game.meta;
        let field_meta = meta.get_field_data(room_id).unwrap();
        let field_fh = meta.get_field_fh_data(room_id).unwrap();
        Ok(FieldHandler::new(
            meta,
            SharedFieldState {
                field_meta,
                field_fh,
            }
            .into(),
        ))
    }

    fn create_ctx(&mut self, clock: GameClockRef, tx: ServerSystemTx<Self>) -> anyhow::Result<Self::Ctx> {
        log::info!("Creating new ctx");
        Ok(GameCtx {
            services: self.services.clone(),
            clock,
            tx,
        })
    }

    fn on_update(&mut self, _ctx: &mut Self::Ctx) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum SessionMsg {
    Pkt(ShroomPacketBytes),
    PktBuf(PacketBuf),
}

pub type SharedServices = Arc<Services>;
pub type SharedGameServices = Arc<GameServices>;

#[derive(Debug)]
pub struct GameServices {
    pub data: DataServices,
    pub server_info: ServerService,
    pub field: FieldService,
    pub meta: &'static MetaService,
}

pub struct GameCtx {
    pub services: Arc<Services>,
    pub clock: GameClockRef,
    pub tx: ServerSystemTx<GameSystem>
}

impl Context for GameCtx {
    fn create(_clock_ref: GameClockRef) -> Self {
        todo!()
    }

    fn time(&self) -> shroom_srv::util::clock::GameTime {
        self.clock.time()
    }

    fn wait_tick(&mut self) -> impl futures::prelude::Future<Output = ()> + Send {
        self.clock.wait_tick()
    }
}

#[derive(Debug)]
pub struct Services {
    pub game: Arc<GameServices>,
    pub session_manager: ShroomSessionManager<ShroomSessionBackend>,
}

impl Services {
    pub fn new(
        db: DatabaseConnection,
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
    ) -> Self {
        let game = Arc::new(GameServices {
            data: DataServices::new(db, meta),
            server_info: ServerService::new(servers),
            field: FieldService::new(meta),
            meta,
        });

        let session_backend = ShroomSessionBackend::new(game.clone());

        Self {
            game,
            session_manager: ShroomSessionManager::new(session_backend, Duration::from_secs(30)),
        }
    }

    pub async fn seeded_in_memory(
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
    ) -> Result<Self, DbErr> {
        let db = crate::gen_sqlite(crate::SQL_OPT_MEMORY).await?;
        Ok(Self::new(db, servers, meta))
    }

    pub async fn seeded_in_db(
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
        db_url: &str,
    ) -> Result<Self, DbErr> {
        let db = crate::gen_psql(db_url).await?;
        Ok(Self::new(db, servers, meta))
    }

    pub fn as_shared(self) -> SharedServices {
        Arc::new(self)
    }

    pub async fn seed_acc_char(&self) -> anyhow::Result<(AccountId, CharacterID)> {
        let acc_id = self
            .game
            .data
            .account
            .create(
                "admin",
                "test1234",
                Region::Europe,
                true,
                Some(GenderTy::Female),
            )
            .await?;

        let job = JobGroup::Legend;
        let _char_id = self
            .game
            .data
            .char
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran".to_string(),
                    job_group: JobGroup::Adventurer,
                    face: FaceId::LEISURE_LOOK_M,
                    skin: Skin::Normal,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet {
                        bottom: job.get_starter_bottoms().next().unwrap(),
                        shoes: job.get_starter_shoes().next().unwrap(),
                        top: job.get_starter_tops().next().unwrap(),
                        weapon: job.get_starter_weapons().next().unwrap(),
                        guide: job.get_guide_item(),
                    },
                    gender: Gender::Male,
                },
                &self.game.data.item,
            )
            .await?;

        let job = JobGroup::Legend;
        let char_id = self
            .game
            .data
            .char
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran2".to_string(),
                    job_group: JobGroup::Adventurer,
                    face: FaceId::LEISURE_LOOK_M,
                    skin: Skin::Normal,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet {
                        bottom: job.get_starter_bottoms().next().unwrap(),
                        shoes: job.get_starter_shoes().next().unwrap(),
                        top: job.get_starter_tops().next().unwrap(),
                        weapon: job.get_starter_weapons().next().unwrap(),
                        guide: job.get_guide_item(),
                    },
                    gender: Gender::Male,
                },
                &self.game.data.item,
            )
            .await?;

        Ok((acc_id, char_id))
    }
}
