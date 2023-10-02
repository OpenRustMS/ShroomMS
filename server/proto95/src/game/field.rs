use shroom_pkt::{
    list::ShroomListLen, packet_opcode, ShroomList, ShroomList16, ShroomOption8, ShroomPacket,
    ShroomPacketEnum, ShroomTime,
};

use crate::{
    id::MapId,
    send_opcodes::SendOpcodes,
    shared::{
        char::{CharDataHeader, CharacterId},
        TagPoint,
    },
};

use super::user::char::{CharDataAll, CharDataFlags};

#[derive(ShroomPacket, Debug)]
pub struct ClientOption {
    pub key: u32,
    pub value: u32,
}

#[derive(ShroomPacket, Debug, Default)]
pub struct CrcSeed {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct LogoutGiftConfig {
    pub predict_quit: u32,
    pub gift_commodity_id: [u32; 3],
}

/// Dirty hack to work around the problem
/// that when there's a notification, there's always n + 1
#[derive(ShroomPacket, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlusOneListIndex(pub u16);

impl ShroomListLen for PlusOneListIndex {
    fn to_len(&self) -> usize {
        match self.0 {
            0 => 0,
            n => (n + 1) as usize,
        }
    }

    fn from_len(ix: usize) -> Self {
        PlusOneListIndex(ix as u16)
    }
}

pub type NotificationList = ShroomList<PlusOneListIndex, String>;

#[derive(ShroomPacket, Debug)]
pub struct SetFieldCharData {
    pub notifications: NotificationList,
    pub seed: CrcSeed,
    pub char_data_flags: CharDataFlags,
    pub char_data_hdr: CharDataHeader,
    pub char_data: CharDataAll,
    pub logout_gift_config: LogoutGiftConfig,
}

#[derive(ShroomPacket, Debug)]
pub struct SetFieldOtherData {
    pub notifications: NotificationList,
    pub revive: bool,
    pub map: MapId,
    pub portal: u8,
    pub hp: u32,
    pub chase_target_pos: ShroomOption8<TagPoint>,
}

impl SetFieldOtherData {
    pub fn is_chase_enabled(&self) -> bool {
        self.chase_target_pos.opt.is_some()
    }
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum SetFieldResult {
    TransferField(SetFieldOtherData) = 0,
    CharData(SetFieldCharData) = 1,
}

#[derive(ShroomPacket, Debug)]
pub struct SetFieldResp {
    pub client_option: ShroomList16<ClientOption>,
    pub channel_id: u32,
    pub old_driver_id: CharacterId,
    pub unknown_flag_1: u8,
    pub set_field_result: SetFieldResult,
    pub timestamp: ShroomTime,
    pub extra: u32,
}
packet_opcode!(SetFieldResp, SendOpcodes::SetField);
