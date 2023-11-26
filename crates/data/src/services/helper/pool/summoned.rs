use proto95::{
    game::{
        life::summon::{SummonCreateResp, SummonDeleteResp, SummonInitData, SummonMoveAction, SummonMoveAbility, SummonEnterType, SummonLeaveType, SummonAssistType},
        ObjectId,
    },
    id::SkillId,
    shared::{char::CharacterId, FootholdId, Vec2},
};

use super::{next_id, PoolItem};

#[derive(Debug)]
pub struct Summon {
    pub pos: Vec2,
    pub fh: FootholdId,
    pub skill_id: SkillId,
    pub skill_level: u8,
    pub char_level: u8,
    pub char_id: CharacterId,
}

impl PoolItem for Summon {
    type Id = ObjectId;

    type EnterPacket = SummonCreateResp;
    type LeavePacket = SummonDeleteResp;

    type LeaveParam = ();

    fn get_id(&self) -> Self::Id {
        next_id()
    }

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        SummonCreateResp {
            char: self.char_id,
            summon_id: id,
            skill_id: self.skill_id,
            char_level: self.char_level,
            skill_level: self.skill_level,
            init: SummonInitData {
                pos: self.pos,
                move_action: SummonMoveAction::Fly1,
                cur_fh: self.fh,
                move_ability: SummonMoveAbility::Follow,
                assist_type: SummonAssistType::None,
                enter_type: SummonEnterType::CreateSummon,
                avatar: None.into(),
            },
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeavePacket {
        SummonDeleteResp {
            char: self.char_id,
            summon_id: id,
            leave: SummonLeaveType::Default,
        }
    }
}
