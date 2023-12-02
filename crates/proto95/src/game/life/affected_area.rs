use num_enum::{TryFromPrimitive, IntoPrimitive};
use shroom_pkt::{packet_with_opcode, ShroomDurationMs16, ShroomPacket, mark_shroom_enum};

use crate::{
    game::ObjectId,
    id::SkillId,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, Rect32},
};

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AffectedAreaType {
    MobSkill = 0,
    UserSkill = 1,
    Smoke = 2,
    Buff = 3,
    BlessedMist = 4
}
mark_shroom_enum!(AffectedAreaType);

#[derive(ShroomPacket, Debug)]
pub struct AffectedAreaCreateResp {
    pub id: ObjectId,
    pub ty: AffectedAreaType,
    pub owner_id: CharacterId,
    pub skill_id: SkillId,
    pub skill_level: u8,
    pub skill_dur: ShroomDurationMs16, // TODO is that really ms?
    pub area: Rect32,                  // TODO check for correct rect
    pub phase: u32,
}
packet_with_opcode!(AffectedAreaCreateResp, SendOpcodes::AffectedAreaCreated);


#[derive(ShroomPacket, Debug)]
pub struct AffectedAreaRemoveResp {
    pub id: ObjectId,
}
packet_with_opcode!(AffectedAreaRemoveResp, SendOpcodes::AffectedAreaRemoved);
