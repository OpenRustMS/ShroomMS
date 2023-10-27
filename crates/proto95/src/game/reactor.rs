use shroom_pkt::{packet_opcode, ShroomDurationMs16, ShroomPacket};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::Vec2};

use super::ObjectId;

pub type ReactorId = u32;

#[derive(ShroomPacket, Debug)]
pub struct ReactorEnterFieldResp {
    pub id: ObjectId,
    pub tmpl_id: ReactorId,
    pub state: u8,
    pub pos: Vec2,
    pub flipped: bool,
    pub name: String,
}
packet_opcode!(ReactorEnterFieldResp, SendOpcodes::ReactorEnterField);

#[derive(ShroomPacket, Debug)]
pub struct ReactorLeaveFieldResp {
    pub id: ObjectId,
    pub state: u8,
    pub pos: Vec2,
}
packet_opcode!(ReactorLeaveFieldResp, SendOpcodes::ReactorLeaveField);

#[derive(ShroomPacket, Debug)]
pub struct ReactorMoveResp {
    pub id: ObjectId,
    pub pos: Vec2,
}
packet_opcode!(ReactorMoveResp, SendOpcodes::ReactorMove);

#[derive(ShroomPacket, Debug)]
pub struct ReactorChangeStateResp {
    pub id: ObjectId,
    pub state: u8,
    pub pos: Vec2,
    pub animation_delay: ShroomDurationMs16,
    pub proper_event_id: u8,
    pub end_state: u8,
}
packet_opcode!(ReactorChangeStateResp, SendOpcodes::ReactorChangeState);

#[derive(ShroomPacket, Debug)]
pub struct ReactorHitReq {
    pub id: ObjectId,
    pub skill_reactor: u32,
    pub hit_option: u32,
    pub action_delay: ShroomDurationMs16,
    pub skill_id: u32,
}
packet_opcode!(ReactorHitReq, RecvOpcodes::ReactorHit);

#[derive(ShroomPacket, Debug)]
pub struct ReactorTouchReq {
    pub id: ObjectId,
    pub has_reactor: bool, // TODO
}
packet_opcode!(ReactorTouchReq, RecvOpcodes::ReactorTouch);
