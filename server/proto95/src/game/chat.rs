use shroom_net::{
    packet::proto::{time::Ticks, ShroomList8},
    packet_opcode, shroom_enum_code, shroom_packet_enum,
};
use shroom_net_derive::ShroomPacket;

use crate::{
    id::ItemId, recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::char::CharacterId,
};

#[derive(Debug, ShroomPacket)]
pub struct GeneralChatPacket {
    message: String,
    show: bool,
}

#[derive(Debug, ShroomPacket)]
pub struct SpouseChatPacket {
    message: String,
}

shroom_enum_code!(
    MultiChatPacketType,
    u8,
    Buddy = 1,
    Party = 2,
    Guild = 3,
    Alliance = 4
);

#[derive(Debug, ShroomPacket)]
pub struct MultiChatPacket {
    ty: MultiChatPacketType,
    recipients: ShroomList8<u32>,
    message: String,
}

#[derive(Debug, ShroomPacket)]
pub struct WispherData {
    name: String,
    message: String,
}

shroom_packet_enum!(
    #[derive(Debug)]
    pub enum WispherMessageType: u8 {
        Location(String) = 1,
        Whispher(WispherData) = 2,
        Request(String) = 0x04,
        Result(String) = 0x08,
        Receiver(String) = 0x10,
        Blocked(String) = 0x20,
        LocationFriend(String) = 0x40
    }
);

#[derive(Debug, ShroomPacket)]
pub struct ItemGainInfoData {
    path: String,
    unknown1: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct ItemGainItemData {
    mode2: u8,
    item_id: ItemId,
    quantity: u32,
}


shroom_packet_enum!(
    #[derive(Debug)]
    pub enum SlashChatMsgType: u8 {
        CmdStrF9(()) = 0x3A,
        //lvl
        CmdStr725(u8) = 0x1E,
        // som id?
        Create(u32) = 0,
        // /
        CmdStr717(u8) = 1,
        // /ex
        CmdStr718(u32) = 2
    }
);

#[derive(Debug, ShroomPacket)]
pub struct SlashChatMsg {
    msg: SlashChatMsgType,
}

#[derive(Debug, ShroomPacket)]
pub struct ChatMsgReq {
    pub ticks: Ticks,
    pub msg: String,
    pub only_balloon: bool,
}
packet_opcode!(ChatMsgReq, RecvOpcodes::UserChat);

#[derive(Debug, ShroomPacket)]
pub struct WhisperData {
    pub ticks: Ticks,
    pub target: String,
    pub msg: String,
}

#[derive(Debug, ShroomPacket)]
pub struct WhisperFindData {
    pub ticks: Ticks,
    pub target: String,
}

shroom_packet_enum!(
    pub enum WhiperMsgReq: u8 {
        Unknown(WhisperData) = 0x86,
        Whisper(WhisperData) = 6,
        WhisperFind(WhisperFindData) = 5
    }
);
packet_opcode!(WhiperMsgReq, RecvOpcodes::Whisper);

#[derive(ShroomPacket, Debug)]
pub struct UserChatMsgResp {
    pub char: CharacterId,
    pub is_admin: bool,
    pub msg: String,
    pub only_balloon: bool,
}
packet_opcode!(UserChatMsgResp, SendOpcodes::UserChat);
