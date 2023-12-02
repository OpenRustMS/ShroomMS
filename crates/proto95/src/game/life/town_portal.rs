use shroom_pkt::{ShroomPacket, packet_with_opcode, CondOption};

use crate::{id::FieldId, shared::Vec2, game::ObjectId, send_opcodes::SendOpcodes, recv_opcodes::RecvOpcodes};

#[derive(ShroomPacket, Debug)]
pub struct TownPortalSkill {
    pub skill_id: u32,
    pub pos: Vec2
}

#[derive(ShroomPacket, Debug)]
pub struct TownPortalChangedResp {
    pub town_id: u32,
    pub field_id: FieldId,
    // TODO: check town id aswell for 999999999
    #[pkt(either(field = "field_id", cond = "FieldId::is_none"))]
    pub skill: CondOption<TownPortalSkill>
    
}
packet_with_opcode!(TownPortalChangedResp, SendOpcodes::TownPortal);

#[derive(ShroomPacket, Debug)]
pub struct TownPortalCreateResp {
    pub state: u8,
    pub id: ObjectId,
    pub pos: Vec2,

}
packet_with_opcode!(TownPortalCreateResp, SendOpcodes::TownPortalCreated);

#[derive(ShroomPacket, Debug)]
pub struct TownPortalRemoveResp {
    pub display: bool, // TODO seems to be a flag to keep the portal rendered
    pub id: ObjectId,

}
packet_with_opcode!(TownPortalRemoveResp, SendOpcodes::TownPortalRemoved);

#[derive(ShroomPacket, Debug)]
pub struct TownPortalEnterReq {
    pub char_or_party_id: u32, // TODO check what this is
    pub u: bool, // TODO: always true?
}
packet_with_opcode!(TownPortalEnterReq, RecvOpcodes::EnterTownPortalRequest);