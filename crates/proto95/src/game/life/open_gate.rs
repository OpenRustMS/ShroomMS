use shroom_pkt::{packet_with_opcode, ShroomPacket};

use crate::{
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, Vec2}, recv_opcodes::RecvOpcodes, game::party::PartyID,
};

#[derive(ShroomPacket, Debug)]
pub struct OpenGateCreateResp {
    pub state: u8,
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub first: bool, // Either first or second gate
    pub party_id: PartyID,
}
packet_with_opcode!(OpenGateCreateResp, SendOpcodes::OpenGateCreated);

#[derive(ShroomPacket, Debug)]
pub struct OpenGateRemoveResp {
    pub leave: u8,
    pub char_id: CharacterId,
    pub first: bool,
}
packet_with_opcode!(OpenGateRemoveResp, SendOpcodes::OpenGateRemoved);

#[derive(ShroomPacket, Debug)]
pub struct OpenGateEntryReq {
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub first: bool
}
packet_with_opcode!(OpenGateEntryReq, RecvOpcodes::EnterOpenGateRequest);
