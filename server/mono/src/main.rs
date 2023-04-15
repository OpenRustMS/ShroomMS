use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use data::services::{meta::meta_service::MetaService, server_info::ServerInfo, SharedServices};
use dotenv::dotenv;
use login::{config::LoginConfig, LoginHandler};
use proto95::send_opcodes::SendOpcodes;
use shroom_net::{
    crypto::{ig_cipher::IgContext, CryptoContext},
    net::{
        service::{
            handler::MakeServerSessionHandler,
            server_sess::{SharedSessionHandle, ShroomServer, ShroomServerConfig},
            BasicHandshakeGenerator, HandshakeGenerator,
        },
        ShroomSession,
    },
    PacketWriter, ShroomPacket,
};
use tokio::{net::TcpStream, task::JoinSet};

use shrooming::{FileIndex, FileSvr};

use crate::config::Environment;

mod config;

static LOGIN_CFG: &LoginConfig = &LoginConfig {
    enable_pic: true,
    enable_pin: false,
};

#[derive(Clone, Debug)]
pub struct Shared;

#[derive(Debug, Clone)]
pub struct MakeLoginHandler {
    services: SharedServices,
}

#[async_trait::async_trait]
impl MakeServerSessionHandler for MakeLoginHandler {
    type Transport = TcpStream;

    type Error = anyhow::Error;

    type Handler = LoginHandler;

    async fn make_handler(
        &mut self,
        sess: &mut ShroomSession<Self::Transport>,
        _broadcast_tx: SharedSessionHandle,
    ) -> Result<Self::Handler, Self::Error> {
        Ok(LoginHandler::new(
            self.services.clone(),
            LOGIN_CFG,
            sess.peer_addr()?.ip(),
        ))
    }
}

async fn srv_login_server(
    cfg: ShroomServerConfig,
    addr: impl tokio::net::ToSocketAddrs,
    handshake_gen: impl HandshakeGenerator,
    services: SharedServices,
) -> anyhow::Result<()> {
    let mut login_server = ShroomServer::new(cfg, handshake_gen, MakeLoginHandler { services });
    login_server.serve_tcp(addr).await?;
    Ok(())
}

async fn srv_game_server(
    cfg: ShroomServerConfig,
    addr: impl tokio::net::ToSocketAddrs,
    handshake_gen: impl HandshakeGenerator,
    services: SharedServices,
    world_id: u32,
    channel_id: u16,
) -> anyhow::Result<()> {
    let mut game_server = ShroomServer::new(
        cfg,
        handshake_gen,
        game::MakeGameHandler::new(services, channel_id, world_id),
    );
    game_server.serve_tcp(addr).await?;
    Ok(())
}

async fn srv_shrooming(addr: SocketAddr) -> anyhow::Result<()> {
    let file_ix = FileIndex::build_index(
        [
            "notes.txt",
            "../../client/shroom_hook/target/i686-pc-windows-gnu/release/dinput8.dll",
            "../../target/i686-pc-windows-gnu/release/shroom_launchar.exe",
        ]
        .iter(),
    )?;

    FileSvr::new(file_ix).serve(addr).await?;

    Ok(())
}

fn get_ping_packet() -> ShroomPacket {
    let mut pw = PacketWriter::default();
    pw.write_opcode(SendOpcodes::AliveReq).expect("Ping opcode");
    pw.into_packet()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    // Load configuration
    let settings = config::get_configuration().expect("Failed to load configuration");
    log::info!("{0} - Mono - {1}", settings.server_name, settings.version);

    //TODO add crypto keys to config
    let crypto_ctx = CryptoContext {
        aes_key: *include_bytes!("../../../keys/net/aes_key.bin"),
        ig_ctx: IgContext::new(
            *include_bytes!("../../../keys/net/round_shifting_key.bin"),
            *include_bytes!("../../../keys/net/initial_round_key.bin"),
        ),
    };

    let shared_ctx = Arc::new(crypto_ctx);

    let server_addr: IpAddr = settings.external_ip.parse()?;
    let bind_addr: IpAddr = settings.bind_ip.parse()?;

    tokio::spawn(srv_shrooming(SocketAddr::new(
        bind_addr,
        settings.shrooming_port,
    )));

    let servers = [ServerInfo::new(
        server_addr,
        settings.base_port,
        settings.server_name,
        settings.num_channels,
    )];

    // Create login server
    let handshake_gen = match settings.client_version {
        83 => BasicHandshakeGenerator::v83(),
        95 => BasicHandshakeGenerator::v95(),
        _ => anyhow::bail!("unexpected client version"),
    };

    // Meta will be available all the time
    let meta = Box::new(MetaService::load_from_dir("../../game_data/rbin")?);

    let services = match config::get_environment() {
        Environment::Local => data::services::Services::seeded_in_memory(servers, Box::leak(meta))
            .await?
            .as_shared(),
        _ => data::services::Services::seeded_in_db(servers, Box::leak(meta))
            .await?
            .as_shared(),
    };
    match config::get_environment() {
        Environment::Local => {
            let (acc_id, char_id) = services.seed_acc_char().await?;
            log::info!("Created test account {acc_id} - char: {char_id}");
        }
        _ => {}
    }

    let mut set = JoinSet::new();
    set.spawn(srv_login_server(
        ShroomServerConfig {
            crypto_ctx: shared_ctx.clone(),
            migrate_delay: Duration::from_millis(7500),
            ping_packet: get_ping_packet(),
            ping_interval: Duration::from_secs(45),
        },
        SocketAddr::new(bind_addr, settings.base_port),
        handshake_gen.clone(),
        services.clone(),
    ));
    for ch in 0..settings.num_channels {
        set.spawn(srv_game_server(
            ShroomServerConfig {
                crypto_ctx: shared_ctx.clone(),
                migrate_delay: Duration::from_millis(7500),
                ping_packet: get_ping_packet(),
                ping_interval: Duration::from_secs(45),
            },
            SocketAddr::new(bind_addr, settings.base_port + 1 + ch as u16),
            handshake_gen.clone(),
            services.clone(),
            0,
            ch as u16,
        ));
    }

    log::info!("Listening ...");
    while let Some(res) = set.join_next().await {
        let _ = res?;
    }

    Ok(())
}
