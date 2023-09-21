use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use data::services::{meta::meta_service::MetaService, server_info::ServerInfo, Services};
use dotenv::dotenv;
use game::{GameHandler, MakeGameHandler};
use login::{config::LoginConfig, LoginHandler, LoginMakeState};

use shroom_net::{
    codec::legacy::{handshake_gen::BasicHandshakeGenerator, LegacyCodec},
    crypto::{ig_cipher::IgContext, CryptoContext},
    server::{
        runtime::{ShroomRuntimeConfig, ShroomServerHandler, ShroomServerRuntime},
        tick::Tick,
    },
};
use tokio::net::TcpStream;
use warp::Filter;

use crate::config::Environment;

mod config;

static LOGIN_CFG: &LoginConfig = &LoginConfig {
    enable_pic: true,
    enable_pin: false,
};

pub struct Mono {
    data_dir: PathBuf,
    env: Environment,
}

#[async_trait::async_trait]
impl ShroomServerHandler for Mono {
    type Codec = LegacyCodec<TcpStream>;

    type GameHandler = GameHandler;

    type LoginHandler = LoginHandler;

    type Services = Services;

    async fn build_services(
        &self,
        ticker: &shroom_net::server::tick::Ticker,
        cfg: Arc<ShroomRuntimeConfig>,
    ) -> anyhow::Result<Self::Services> {
        let meta = Box::new(MetaService::load_from_dir(
            self.data_dir.join("game_data/rbin"),
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    let data_dir: PathBuf = std::env::var("DATA_DIR")
        .unwrap_or("../..".to_string())
        .into();

    // Load configuration
    let settings = config::get_configuration(&data_dir).expect("Failed to load configuration");
    log::info!("{0} - Mono - {1}", settings.server_name, settings.version);

    let ext_ip = std::env::var("EXTERNAL_IP")
        .ok()
        .or_else(|| settings.external_ip)
        .ok_or_else(|| anyhow::format_err!("No external IP set"))?;

    log::info!("External IP: {0}", ext_ip);

    let server_addr: IpAddr = ext_ip.parse()?;
    let bind_addr: IpAddr = settings.bind_ip.parse()?;

    tokio::spawn(srv_tuf(
        (bind_addr, settings.tuf_repo_port),
        data_dir.join("client_repo/data/tuf-repo"),
    ));

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
    };

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
            ping_dur: Duration::from_secs(30),
            msg_cap: 32,
        },
        mono,
    )
    .await?;

    let _ = runtime.spawn_task("Session Lifecycle", |svc| async move {
        loop {
            svc.session_manager.clean().await.expect("Clean");
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });

    runtime.spawn_login_server().await?;
    runtime.spawn_game_servers().await?;

    runtime.run().await?;

    Ok(())
}
