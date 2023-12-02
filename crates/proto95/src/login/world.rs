use shroom_pkt::{packet_with_opcode, CondOption, ShroomList16, ShroomList8, ShroomPacket};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::Vec2};

use super::StartModeInfo;

pub type WorldId = u32;
pub type WorldId16 = u16;
pub type ChannelId = u16;

#[derive(ShroomPacket, Debug)]
pub struct LogoutWorldReq;
packet_with_opcode!(LogoutWorldReq, RecvOpcodes::LogoutWorld);

#[derive(Debug, ShroomPacket)]
pub struct WorldInfoReq;
packet_with_opcode!(WorldInfoReq, RecvOpcodes::WorldInfoRequest);

#[derive(Debug, ShroomPacket)]
pub struct WorldReq;
packet_with_opcode!(WorldReq, RecvOpcodes::WorldRequest);

#[derive(Debug, ShroomPacket)]
pub struct WorldCheckUserLimitReq {
    pub world: WorldId16,
}
packet_with_opcode!(WorldCheckUserLimitReq, RecvOpcodes::CheckUserLimit);

#[derive(Debug, ShroomPacket)]
pub struct WorldCheckUserLimitResp {
    pub over_user_limit: bool,
    //TODO seems like a bool
    pub populate_level: u8,
}
packet_with_opcode!(WorldCheckUserLimitResp, SendOpcodes::CheckUserLimitResult);

#[derive(Debug, ShroomPacket)]
pub struct RecommendWorldMessage {
    world_id: WorldId,
    message: String,
}

#[derive(Debug, ShroomPacket)]
pub struct RecommendWorldMessageResp {
    messages: ShroomList8<RecommendWorldMessage>,
}
packet_with_opcode!(
    RecommendWorldMessageResp,
    SendOpcodes::RecommendWorldMessage
);

#[derive(Debug, ShroomPacket)]
pub struct LastConnectedWorldResp {
    last_world: WorldId,
}
packet_with_opcode!(LastConnectedWorldResp, SendOpcodes::LatestConnectedWorld);

#[derive(Debug, ShroomPacket)]
pub struct ChannelItem {
    pub name: String,
    pub user_number: u32,
    pub world_id: u8,
    pub id: u8,
    pub adult_channel: bool,
}

#[derive(Debug, ShroomPacket)]
pub struct WorldBalloon {
    pub pos: Vec2,
    pub message: String,
}

#[derive(Debug, ShroomPacket)]
pub struct WorldItem {
    pub name: String,
    pub state: u8, // 0 = normal, 1 = hot?, 2 = new
    pub event_desc: String,
    pub event_exp: u16,
    pub event_drop_rate: u16,
    pub block_char_creation: bool,
    pub channels: ShroomList8<ChannelItem>,
    pub balloons: ShroomList16<WorldBalloon>,
}

fn has_world_info(world_id: &u8) -> bool {
    *world_id != 0xff
}

#[derive(Debug, ShroomPacket)]
pub struct WorldInfoResp {
    pub world_id: u8,
    #[pkt(check(field = "world_id", cond = "has_world_info"))]
    pub world: CondOption<WorldItem>,
}
packet_with_opcode!(WorldInfoResp, SendOpcodes::WorldInformation);

impl WorldInfoResp {
    pub fn end() -> Self {
        Self {
            world_id: 0xff,
            world: CondOption(None),
        }
    }

    pub fn world(id: u8, world: WorldItem) -> Self {
        Self {
            world_id: id,
            world: CondOption(Some(world)),
        }
    }
}

#[derive(ShroomPacket, Debug)]
pub struct SelectWorldReq {
    pub start_mode: StartModeInfo,
    pub world_id: u8,
    pub channel_id: u8,
    // TODO: 2-5 of sa_data
    pub sa_data: u32,
}
packet_with_opcode!(SelectWorldReq, RecvOpcodes::SelectWorld);
