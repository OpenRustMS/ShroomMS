use shroom_pkt::{packet_with_opcode, ShroomPacket};

use crate::{game::ObjectId, id::ItemId, send_opcodes::SendOpcodes, shared::Vec2};

#[derive(ShroomPacket, Debug)]
pub struct MessageBoxCreateResp {
    pub id: ObjectId,
    pub item_id: ItemId,
    pub message: String,
    pub char_name: String,
    pub pos: Vec2,
}
packet_with_opcode!(MessageBoxCreateResp, SendOpcodes::MessageBoxEnterField);

#[derive(ShroomPacket, Debug)]
pub struct MessageBoxCreateFailedResp;
packet_with_opcode!(MessageBoxCreateFailedResp, SendOpcodes::CreateMessgaeBoxFailed);

#[derive(ShroomPacket, Debug)]
pub struct MessageBoxRemoveResp {
    pub no_fade_out: bool,
    pub id: ObjectId,
}
packet_with_opcode!(MessageBoxRemoveResp, SendOpcodes::MessageBoxLeaveField);
