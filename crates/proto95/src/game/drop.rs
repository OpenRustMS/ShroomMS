use shroom_pkt::{
    packet_with_opcode, shroom_enum_code, CondOption, PacketResult, PacketTryWrapped,
    ShroomDurationMs16, ShroomExpirationTime, ShroomPacket, ShroomPacketEnum,
};

use crate::{
    id::ItemId,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, Vec2},
};

pub type DropId = u32;

#[derive(Debug, Clone, Copy)]
pub enum DropOwner {
    User(CharacterId),
    // TODO: Party ID
    Party(u32),
    None,
    Explosive,
}

impl PacketTryWrapped for DropOwner {
    type Inner = (u32, u8);
    type IntoValue<'a> = Self::Inner;  

    fn packet_into_inner(&self) -> Self::Inner {
        match self {
            DropOwner::User(user) => (*user, 0),
            DropOwner::Party(party) => (*party, 1),
            DropOwner::None => (0, 2),
            DropOwner::Explosive => (0, 3),
        }
    }

    fn packet_try_from(v: Self::Inner) -> PacketResult<Self> {
        Ok(match v.1 {
            0 => Self::User(v.0),
            1 => Self::Party(v.0),
            2 => Self::None,
            3 => Self::Explosive,
            _ => return Err(shroom_pkt::Error::InvalidEnumPrimitive(v.1 as u32)),
        })
    }
}

shroom_enum_code!(
    DropEnterType,
    u8,
    Default = 0,
    Create = 1,     // Basic floating
    OnFoothold = 2, // Instant attached to fh
    FadingOut = 3,  // Fading away
    Unknown4 = 4    // ?
);

impl DropEnterType {
    fn has_start_pos(&self) -> bool {
        matches!(
            self,
            Self::Default | Self::Create | Self::FadingOut | Self::Unknown4
        )
    }
}
#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum DropType {
    Item(ItemId) = 0,
    Money(u32) = 1,
}
impl DropType {
    fn has_expiration(&self) -> bool {
        !matches!(self, DropType::Money(_))
    }
}

#[derive(ShroomPacket, Debug)]
pub struct DropEnterFieldResp {
    pub enter_type: DropEnterType,
    pub id: DropId,
    pub drop_type: DropType,
    pub drop_owner: DropOwner,
    pub pos: Vec2,
    pub src_id: u32,
    #[pkt(check(field = "enter_type", cond = "DropEnterType::has_start_pos"))]
    pub start_pos: CondOption<(Vec2, ShroomDurationMs16)>,
    #[pkt(check(field = "drop_type", cond = "DropType::has_expiration"))]
    pub drop_expiration: CondOption<ShroomExpirationTime>,
    //TODO: ? ownerCharId == 0
    pub by_pet: bool,
    // If this is set to true It throws an exception
    pub u1_flag: bool,
}
packet_with_opcode!(DropEnterFieldResp, SendOpcodes::DropEnterField);

shroom_enum_code!(
    DropLeaveType,
    u8,
    TimeOut = 0,
    ScreenScroll = 1,
    UserPickup = 2,
    MobPickup = 3,
    Explode = 4,
    PetPickup = 5,
    PassConvex = 6,
    PetSkill = 7
);

impl DropLeaveType {
    fn has_pickup_id(&self) -> bool {
        matches!(self, Self::UserPickup | Self::MobPickup | Self::PetSkill)
    }
}

#[derive(ShroomPacket, Debug)]
pub struct DropLeaveFieldResp {
    pub leave_type: DropLeaveType,
    pub id: DropId,
    #[pkt(check(field = "leave_type", cond = "DropLeaveType::has_pickup_id"))]
    pub pickup_id: CondOption<u32>,
}
packet_with_opcode!(DropLeaveFieldResp, SendOpcodes::DropLeaveField);

#[cfg(test)]
mod tests {
    use shroom_pkt::{DecodePacket, PacketReader};

    use super::*;

    #[test]
    fn drop_enter() {
        let data = [0x01, 0x3B, 0x00, 0x00, 0x00, 0x01, 0x5E, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x59, 0x01, 0xC7, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xEC, 0xFF, 0x64, 0x00, 0x01, 0x00];
        let drop = DropEnterFieldResp::decode_complete(&mut PacketReader::new(&data)).unwrap();
        dbg!(&drop);
    }

    #[test]
    fn drop_enter2() {
        let data = [01,0x68,0x00,0x00,0x00,0x01,0x5E,0x02,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0xC3,0x02,0x31,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0xEC,0xFF,0x64,0x00,0x01,0x00];
        let drop = DropEnterFieldResp::decode_complete(&mut PacketReader::new(&data)).unwrap();
        dbg!(&drop);
    }
}