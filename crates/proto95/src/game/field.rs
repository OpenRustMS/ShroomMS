use shroom_pkt::{
    list::ShroomListLen, packet_with_opcode, CondEither, ShroomList, ShroomList16, ShroomOption8,
    ShroomPacket, ShroomTime,
};

use crate::{
    id::FieldId,
    send_opcodes::SendOpcodes,
    shared::{
        char::{CharDataHeader, CharacterId},
        TagPoint,
    },
};

use super::user::char::{CharDataFlags, CharDataAll};

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
/// First entry is chatblock reason
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
#[derive(ShroomPacket, Debug, Default)]
pub struct NotificationList(ShroomList<PlusOneListIndex, String>);

impl NotificationList {
    pub fn chat_banned<'s>(ban_reason: &str, extra: impl Iterator<Item = &'s str>) -> Self {
        let mut list = Self::default();
        list.0.push(ban_reason.to_string());
        list.0.extend(extra.map(|s| s.to_string()));
        list
    }

    pub fn ban_reason(&self) -> Option<&str> {
        self.0.first().map(|s| s.as_str())
    }

    pub fn extra(&self) -> impl Iterator<Item = &str> {
        self.0.iter().skip(1).map(|s| s.as_str())
    }
}

#[derive(ShroomPacket, Debug)]
pub struct FieldCharData {
    pub seed: CrcSeed,
    pub char_data_flags: CharDataFlags,
    pub char_data_hdr: CharDataHeader,
    pub char_data: CharDataAll,
    pub logout_gift_config: LogoutGiftConfig,
}

#[derive(ShroomPacket, Debug)]
pub struct FieldTransferData {
    pub revive: bool,
    pub map: FieldId,
    pub portal: u8,
    pub hp: u32,
    pub chase_target_pos: ShroomOption8<TagPoint>,
}

impl FieldTransferData {
    pub fn is_chase_enabled(&self) -> bool {
        self.chase_target_pos.opt.is_some()
    }
}

fn is_true(b: &bool) -> bool {
    *b
}

#[derive(ShroomPacket, Debug)]
pub struct SetFieldResp {
    pub client_option: ShroomList16<ClientOption>,
    pub channel_id: u32,
    pub old_driver_id: CharacterId,
    pub unknown_flag_1: u8,
    pub has_char_data: bool,
    pub notifications: NotificationList,
    #[pkt(either(field = "has_char_data", cond = "is_true"))]
    pub char_data: CondEither<FieldCharData, FieldTransferData>,
    pub server_time: ShroomTime,
}
packet_with_opcode!(SetFieldResp, SendOpcodes::SetField);
