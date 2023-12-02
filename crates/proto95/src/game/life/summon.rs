use num_enum::{TryFromPrimitive, IntoPrimitive};
use shroom_pkt::{packet_with_opcode, ShroomList8, ShroomOption8, ShroomPacket, mark_shroom_enum};

use crate::{
    id::SkillId,
    send_opcodes::SendOpcodes,
    shared::{
        char::{AvatarData, CharacterId},
        movement::MovePath,
        FootholdId, Vec2,
    }, game::{ObjectId, user::ForeActionDir},
};

pub type SummonId = u32;

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonEnterType {
    Default = 0,
    CreateSummon = 1,
    ReregisterSummon  = 2
}
mark_shroom_enum!(SummonEnterType);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonLeaveType {
    Update = 0,
    Die = 1,
    Mystery = 2,
    Default = 3,
    LeaveField = 4,
    SelfDestruct = 5,
    Gabiota = 6,
    EnterForbidenMap = 7,
    EnterEventField = 8,
    UserDead = 9,
    OnRemove = 10,
    TeslaCoilError = 11,
    NotAbleMultiple = 12,
    NotSelfDestruct = 13,
    SummonCountOver = 14
}
mark_shroom_enum!(SummonLeaveType);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonAssistType {
    None = 0,
    Attack = 1,
    Heal = 2,
    AttackEx = 3,
    AttackEx2 = 4,
    ManualAttack = 5
}
mark_shroom_enum!(SummonAssistType);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonMoveAbility {
    NoMove = 0,
    Follow = 1,
    WalkRandom = 2,
    Jump = 3,
    CircleFollow = 4,
    FlyRandom = 5,
    Escort = 6
}
mark_shroom_enum!(SummonMoveAbility);

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SummonMoveAction {
    Walk = 0,
    Move = 1,
    Stand = 2,
    Jump = 3,
    Alert = 4,
    Prone = 5,
    Fly1 = 6,
    Ladder = 7,
    Rope = 8,
    Dead = 9,
    Sit = 10,
    Stand0 = 11,
    Hungry = 12,
    Rest0 = 13,
    Rest1 = 14,
    Hang = 15,
    Chase = 16,
    Fly2 = 17,
    Fly2Move = 18,
    Dash2 = 19,
    RocketBooster = 20,
    TeslaCoilTriangle = 21,
    NoMove = 22,
}
mark_shroom_enum!(SummonMoveAction);

#[derive(Debug, ShroomPacket)]
pub struct SummonTeslaCoilInitData {
    //TODO first u8 is coil state
    pub state: ShroomOption8<[Vec2; 3]>,
}

#[derive(Debug, ShroomPacket)]
pub struct SummonInitData {
    pub pos: Vec2,
    pub move_action: SummonMoveAction,
    pub cur_fh: FootholdId,
    pub move_ability: SummonMoveAbility,
    pub assist_type: SummonAssistType,
    pub enter_type: SummonEnterType,
    pub avatar: ShroomOption8<AvatarData>,
    // TODO if skill id is tesla coil then there's extra data
}

#[derive(Debug, ShroomPacket)]
pub struct SummonCreateResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub skill_id: SkillId,
    pub char_level: u8,
    pub skill_level: u8,
    pub init: SummonInitData,
}
packet_with_opcode!(SummonCreateResp, SendOpcodes::SummonedEnterField);

#[derive(Debug, ShroomPacket)]
pub struct SummonDeleteResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub leave: SummonLeaveType
}
packet_with_opcode!(SummonDeleteResp, SendOpcodes::SummonedLeaveField);

#[derive(Debug, ShroomPacket)]
pub struct SummonMoveResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub path: MovePath,
}
packet_with_opcode!(SummonMoveResp, SendOpcodes::SummonedMove);

#[derive(Debug, ShroomPacket)]
pub struct SummonAttackInfo {
    pub target_mob: ObjectId,
    pub hit_action: u8,
    pub dmg: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct SummonAttackResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub fore_action: ForeActionDir,
    pub attack_info: ShroomList8<SummonAttackInfo>,
    pub u: u8, // TODO
}
packet_with_opcode!(SummonAttackResp, SendOpcodes::SummonedAttack);

#[derive(Debug, ShroomPacket)]
pub struct SummonSkillResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub attack_action: bool, // TODO: this is used as bool 0x7F
}
packet_with_opcode!(SummonSkillResp, SendOpcodes::SummonedSkill);

#[derive(Debug, ShroomPacket)]
pub struct SummonHitResp {
    pub char: CharacterId,
    pub summon_id: SummonId,
    pub atk_index: i8,
    pub damage: u32,
    // The following is only encdoed if atk_index > -i
    pub mob_tmpl_id: u32,
    pub left: bool,
}
packet_with_opcode!(SummonHitResp, SendOpcodes::SummonedHit);
