use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use data::services::{server_info::ServerInfo, Services, GameSystem};
use dotenv::dotenv;
use login::LoginService;

use shroom_net::{
    codec::legacy::{handshake_gen::BasicHandshakeGenerator, LegacyCodec},
    crypto::{ig_cipher::IgContext, CryptoContext},
};
use shroom_srv::{rpc::RpcListener, srv::server_system::ServerSystem};
use tokio::net::TcpListener;

use crate::config::Environment;

mod config;

/*static LOGIN_CFG: &LoginConfig = &LoginConfig {
    enable_pic: true,
    enable_pin: false,
};*/

/*


server_name: settings.server_name.clone(),
external_ip: server_addr,
listen_ip: bind_addr,
login_port: settings.base_port,
game_ports: (settings.base_port + 1
    ..=settings.base_port + 1 + (settings.num_channels as u16)),
tick_duration: Duration::from_millis(50),
ping_dur: Duration::from_secs(15),
migration_delay: Duration::from_secs(10),
msg_cap: 32,*/

pub struct Mono {
    data_dir: PathBuf,
    env: Environment,
    external_ip: IpAddr,
    login_port: u16,
    game_ports: std::ops::RangeInclusive<u16>,
    server_name: String,
}

/*
#[async_trait::async_trait]
impl ShroomServerHandler for Mono {
    type Codec = LegacyCodec<TcpStream>;

    type GameHandler = GameHandler;

    type LoginHandler = LoginService;

    type Services = Services;

    async fn build_services(
        &self,
        ticker: &shroom_net::server::tick::Ticker,
        cfg: Arc<ShroomRuntimeConfig>,
    ) -> anyhow::Result<Self::Services> {
        let meta = Box::new(meta::MetaService::load_from_dir(
            self.data_dir.join("game_data/rbin"),
            meta::MetaOption::Full
        )?);
        log::info!("Loaded meta data");

        let static_meta = Box::leak(meta);
        let tick = ticker.get_tick();

        let servers = [ServerInfo::new(
            cfg.external_ip,
            cfg.login_port,
            cfg.server_name.clone(),
            cfg.game_ports.clone().count(),
        )];

        let services = match self.env {
            Environment::Local => {
                data::services::Services::seeded_in_memory(servers, tick, static_meta).await?
            }
            _ => {
                // Wait for db to start
                tokio::time::sleep(Duration::from_secs(5)).await;
                let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
                log::info!("db url: {db_url}");
                data::services::Services::seeded_in_db(servers, tick, static_meta, &db_url).await?
            }
        };
        match self.env {
            Environment::Local => {
                let (acc_id, char_id) = services.seed_acc_char().await?;
                log::info!("Created test account {acc_id} - char: {char_id}");
            }
            _ => {
                let (acc_id, char_id) = services.seed_acc_char().await?;
                log::info!("Created test account {acc_id} - char: {char_id}");
            }
        }

        Ok(services)
    }

    fn make_login_handler(
        &self,
        services: Arc<Self::Services>,
        _tick: Tick,
    ) -> anyhow::Result<
        <Self::LoginHandler as shroom_net::server::server_conn::ShroomConnHandler>::MakeState,
    > {
        Ok(LoginMakeState {
            services,
            cfg: Arc::new(LOGIN_CFG.clone()),
        })
    }

    fn make_game_handler(
        &self,
        services: Arc<Self::Services>,
        _tick: Tick,
        channel_id: usize,
    ) -> anyhow::Result<
        <Self::GameHandler as shroom_net::server::server_conn::ShroomConnHandler>::MakeState,
    > {
        Ok(MakeGameHandler::new(services, channel_id as u16, 0))
    }
}

async fn srv_tuf(addr: impl Into<SocketAddr>, tuf_repo: impl Into<PathBuf>) -> anyhow::Result<()> {
    let p = tuf_repo.into();
    let metadata = warp::path("metadata").and(warp::fs::dir(p.join("metadata")));
    let targets = warp::path("targets").and(warp::fs::dir(p.join("targets")));
    warp::serve(metadata.or(targets)).run(addr).await;

    Ok(())
}*/

impl Mono {
    async fn build_services(&self) -> anyhow::Result<Services> {
        let meta = Box::new(meta::MetaService::load_from_dir(
            self.data_dir.join("game_data/rbin"),
            meta::MetaOption::Full,
        )?);
        log::info!("Loaded meta data");

        let static_meta = Box::leak(meta);

        let servers = [ServerInfo::new(
            self.external_ip,
            self.login_port,
            self.server_name.clone(),
            self.game_ports.clone().count(),
        )];

        let services = match self.env {
            Environment::Local => {
                data::services::Services::seeded_in_memory(servers, static_meta).await?
            }
            _ => {
                // Wait for db to start
                tokio::time::sleep(Duration::from_secs(5)).await;
                let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
                log::info!("db url: {db_url}");
                data::services::Services::seeded_in_db(servers, static_meta, &db_url).await?
            }
        };
        match self.env {
            Environment::Local => {
                let (acc_id, char_id) = services.seed_acc_char().await?;
                log::info!("Created test account {acc_id} - char: {char_id}");
            }
            _ => {
                let (acc_id, char_id) = services.seed_acc_char().await?;
                log::info!("Created test account {acc_id} - char: {char_id}");
            }
        }

        Ok(services)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    let data_dir: PathBuf = std::env::var("DATA_DIR")
        .unwrap_or("/home/jonas/Dokumente/projects/open-rust-ms/ShroomMS".to_string())
        .into();

    // Load configuration
    let settings = config::get_configuration(&data_dir).expect("Failed to load configuration");
    log::info!("{0} - Mono - {1}", settings.server_name, settings.version);

    let ext_ip = std::env::var("EXTERNAL_IP")
        .ok()
        .or(settings.external_ip)
        .ok_or_else(|| anyhow::format_err!("No external IP set"))?;

    log::info!("External IP: {0}", ext_ip);

    let server_addr: IpAddr = ext_ip.parse()?;
    let bind_addr: IpAddr = settings.bind_ip.parse()?;

    //TODO add crypto keys to config
    let crypto_ctx = Arc::new(CryptoContext {
        aes_key: *include_bytes!("../../../keys/net/aes_key.bin"),
        ig_ctx: IgContext::new(
            *include_bytes!("../../../keys/net/round_shifting_key.bin"),
            *include_bytes!("../../../keys/net/initial_round_key.bin"),
        ),
    });

    log::info!("Loaded crypto context");

    // Create login server
    let handshake_gen = match settings.client_version {
        83 => BasicHandshakeGenerator::v83(),
        95 => BasicHandshakeGenerator::v95(),
        _ => anyhow::bail!("unexpected client version"),
    };

    // Meta will be available all the time
    let mono = Mono {
        data_dir,
        env: config::get_environment(),
        external_ip: server_addr,
        login_port: 8484,
        game_ports: 8485..=8485 + (settings.num_channels as u16),
        server_name: settings.server_name.clone(),
    };
    let services = mono.build_services().await?.as_shared();
    let mut login = RpcListener::<LoginService>::new(
        LegacyCodec::new(crypto_ctx.clone(), handshake_gen.clone()),
        services.clone(),
    );
    let login_port = mono.login_port;
    tokio::spawn(async move {
        if let Err(err) = login
            .run_tcp(SocketAddr::new(
                bind_addr,
                login_port,
            ))
            .await
        {
            log::error!("Login server error: {:?}", err);
        }
    });

    let mut sys = ServerSystem::new(GameSystem {
        services
    });

    let acceptor = sys.create_acceptor(LegacyCodec::new(crypto_ctx.clone(), handshake_gen.clone()))?;

    let game_listener = TcpListener::bind(SocketAddr::new(
        bind_addr,
        login_port + 1,
    )).await?;


    tokio::spawn(async move {
        if let Err(err) = acceptor
            .run_tcp(game_listener)
            .await
        {
            log::error!("Game server error: {:?}", err);
        }
    });

    sys.run().await?;
    /*

    let mut runtime = ShroomServerRuntime::create(
        LegacyCodec::new(crypto_ctx, handshake_gen),
        ShroomRuntimeConfig {
            server_name: settings.server_name.clone(),
            external_ip: server_addr,
            listen_ip: bind_addr,
            login_port: settings.base_port,
            game_ports: (settings.base_port + 1
                ..=settings.base_port + 1 + (settings.num_channels as u16)),
            tick_duration: Duration::from_millis(50),
            ping_dur: Duration::from_secs(15),
            migration_delay: Duration::from_secs(10),
            msg_cap: 32,
        },
        mono,
    )
    .await?;

    let _session_task = runtime.spawn_task("Session Lifecycle", |svc| async move {
        loop {
            svc.session_manager.clean().await.expect("Clean");
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });

    runtime.spawn_login_server().await?;
    runtime.spawn_game_servers().await?;

    runtime.run().await?;*/

    Ok(())
}
