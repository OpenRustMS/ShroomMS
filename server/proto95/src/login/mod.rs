pub mod pin;
use shroom_net::{shroom_enum_code, packet_opcode, packet::CondOption};
use shroom_net_derive::ShroomPacket;

use crate::recv_opcodes::RecvOpcodes;

pub mod account;
pub mod char;
pub mod world;

#[derive(Debug, ShroomPacket)]
pub struct MachineId(pub [u8; 0x10]);
pub type ClientKey = [u8; 8];

#[derive(ShroomPacket, Debug)]
pub struct CreateSecurityHandleReq;
packet_opcode!(CreateSecurityHandleReq, RecvOpcodes::CreateSecurityHandle);

shroom_enum_code!(StartMode, u8, WebStart = 0, Unknown1 = 1, GameLaunching = 2);

impl StartMode {
    pub fn has_system_info(&self) -> bool {
        self == &Self::Unknown1
    }
}

#[derive(ShroomPacket, Debug)]
pub struct StartModeInfo {
    start_mode: StartMode,
    #[pkt(if(field = "start_mode", cond = "StartMode::has_system_info"))]
    system_info: CondOption<SystemInfo>,
}

#[derive(ShroomPacket, Debug)]
pub struct SystemInfo {
    // SupportID?
    unknown: String,
    machine_id: MachineId,
    game_room_client: u32,
    start_mode: u8,
}

shroom_enum_code!(
    RegStateId,
    u8,
    // Both work `Registered` fine as success codes
    default(Registered0 = 0),
    Registered1 = 1,
    // Opens a verify code urlin the browser
    Verify2 = 2,
    Verify3 = 3,
);

#[derive(Debug, ShroomPacket, Default)]
pub struct LoginResultHeader {
    pub reg: RegStateId,
    // Unused variable
    pub unknown: u32,
}

shroom_enum_code!(
    LoginOpt,
    u8,
    EnableSecondPassword = 0,
    CheckSecondPassword = 1,
    NoSecondPassword1 = 2,
    NoSecondPassword2 = 3
);

/*
63, c7 => blocked for typing

*/
pub type BanReason = u8;

#[derive(Debug, ShroomPacket)]
pub struct HardwareInfo {
    mac: String,
    hdd_serial_no: String,
}

#[derive(Debug, ShroomPacket)]
pub struct SSOErrorLog {
    unknown1: u8,
    auth_reply_code: u32,
}
