pub mod char;
pub mod inventory;
pub mod item;
pub mod job;
pub mod movement;

use std::net::{Ipv4Addr, SocketAddr};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_pkt::{
    mark_shroom_enum, packet_opcode, string::FixedPacketString, time::DurationMs, PacketWrapped,
    ShroomPacket,
};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

pub type NameStr = FixedPacketString<13>;

#[derive(ShroomPacket, Debug)]
pub struct ClientDumpLogReq {
    call_type: u16,
    error_code: u32,
    unknown1: u16,
    unknown2: u32,
    clear_stack_log: u32,
    unknown3: u32,
    //TODO: data: ShroomList16<u8>,
}
packet_opcode!(ClientDumpLogReq, RecvOpcodes::ClientDumpLog);

#[derive(ShroomPacket, Debug)]
pub struct ExceptionLogReq {
    pub log: String,
}
packet_opcode!(ExceptionLogReq, RecvOpcodes::ExceptionLog);

#[derive(ShroomPacket, Debug)]
pub struct UpdateScreenSettingReq {
    large_screen: bool,
    window_mode: bool,
}
packet_opcode!(UpdateScreenSettingReq, RecvOpcodes::UpdateScreenSetting);

#[derive(ShroomPacket, Debug)]
pub struct PongReq;
packet_opcode!(PongReq, RecvOpcodes::AliveAck);

#[derive(ShroomPacket, Debug, Clone)]
pub struct PingResp;
packet_opcode!(PingResp, SendOpcodes::AliveReq);

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum Gender {
    #[default]
    Male = 0,
    Female = 1,
}
mark_shroom_enum!(Gender);

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum OptionGender {
    #[default]
    Male = 0,
    Female = 1,
    Unset = 10,
}

impl OptionGender {
    pub fn is_set(&self) -> bool {
        !self.is_unset()
    }

    pub fn is_unset(&self) -> bool {
        matches!(self, OptionGender::Unset)
    }
}

impl<T> From<Option<T>> for OptionGender
where
    T: Into<Gender>,
{
    fn from(value: Option<T>) -> Self {
        match value.map(Into::into) {
            None => OptionGender::Unset,
            Some(Gender::Female) => OptionGender::Female,
            Some(Gender::Male) => OptionGender::Male,
        }
    }
}

impl From<OptionGender> for Option<Gender> {
    fn from(val: OptionGender) -> Self {
        match val {
            OptionGender::Female => Some(Gender::Female),
            OptionGender::Male => Some(Gender::Male),
            OptionGender::Unset => None,
        }
    }
}
mark_shroom_enum!(OptionGender);

pub type Vec2 = euclid::default::Vector2D<i16>;
pub type Rect2D = euclid::default::Box2D<i16>;
pub type TagPoint = euclid::default::Point2D<i32>;

pub type FootholdId = u16;

#[derive(Debug, ShroomPacket, Copy, Clone)]
pub struct Range2 {
    pub low: i16,
    pub high: i16,
}

#[derive(Debug, ShroomPacket)]
pub struct Rect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[derive(Debug, ShroomPacket)]
pub struct Rect32 {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}


#[derive(Debug, Clone)]
pub struct ServerAddr(pub Ipv4Addr);

#[derive(Debug, Clone, ShroomPacket)]
pub struct ServerSocketAddr {
    pub addr: ServerAddr,
    pub port: u16,
}

impl TryFrom<SocketAddr> for ServerSocketAddr {
    type Error = anyhow::Error;

    fn try_from(value: SocketAddr) -> Result<Self, Self::Error> {
        match value {
            SocketAddr::V4(addr) => Ok(Self {
                addr: ServerAddr(*addr.ip()),
                port: addr.port(),
            }),
            _ => Err(anyhow::format_err!("Ipv6 not supported")),
        }
    }
}

impl PacketWrapped for ServerAddr {
    type Inner = [u8; 4];

    fn packet_into_inner(&self) -> Self::Inner {
        self.0.octets()
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self(Ipv4Addr::from(v))
    }
}

//TODO: should this go into the net crate
/// This is the the offset from time::getTime
/// in milliseconds
/// in_future means the encoded time is behind time::getTime
#[derive(Debug, Clone)]
pub struct ShroomTimeOffset(pub DurationMs<i32>);

impl PacketWrapped for ShroomTimeOffset {
    type Inner = (bool, u32);

    fn packet_into_inner(&self) -> Self::Inner {
        let v = self.0 .0;
        (v >= 0, v.unsigned_abs())
    }
    fn packet_from(v: Self::Inner) -> Self {
        if v.0 {
            Self(DurationMs(-(v.1 as i32)))
        } else {
            Self(DurationMs(v.1 as i32))
        }
    }
}
