use shroom_pkt::{packet_opcode, shroom_enum_code, ShroomOption8, ShroomPacket};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

shroom_enum_code!(
    CheckPinResp,
    u8,
    Accepted = 0,
    RegisterNewPin = 1,
    InvalidPin = 2,
    SystemError = 3,
    EnterPin = 4,
    //TODO valid?
    ResetLogin = 7
);
packet_opcode!(CheckPinResp, SendOpcodes::CheckPinCodeResult);

#[derive(ShroomPacket, Debug)]
pub struct UpdatePinResp {
    pub success: bool,
}
packet_opcode!(UpdatePinResp, SendOpcodes::UpdatePinCodeResult);

#[derive(Debug, ShroomPacket)]
pub struct CheckPinData {
    //TODO: set to true in CheckPasswordResult and OnSelectWorldResult why?
    /// Somehow set to one for CLogin::OnSelectWorldResult, elsewise 0
    pub is_on_select_world_result_request: bool,
    pub pin: String,
}

#[derive(Debug, ShroomPacket)]
pub struct CheckPinReq {
    pub pin: ShroomOption8<CheckPinData>,
}
packet_opcode!(CheckPinReq, RecvOpcodes::CheckPinCode);

#[derive(Debug, ShroomPacket)]
pub struct UpdatePinReq {
    pub pin: ShroomOption8<String>,
}
packet_opcode!(UpdatePinReq, RecvOpcodes::UpdatePinCode);
