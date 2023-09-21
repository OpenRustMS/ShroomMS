use shroom_pkt::{packet_opcode, ShroomList8, ShroomPacket};

use crate::{id::SkillId, send_opcodes::SendOpcodes};

#[derive(ShroomPacket, Debug)]
pub struct SingleMacro {
    pub name: String,
    pub mute: bool,
    pub skills: [SkillId; 3],
}

pub type MacroSysData = ShroomList8<SingleMacro>;

#[derive(ShroomPacket, Debug)]
pub struct MacroSysDataInitResp {
    pub data: MacroSysData,
}
packet_opcode!(MacroSysDataInitResp, SendOpcodes::MacroSysDataInit);
