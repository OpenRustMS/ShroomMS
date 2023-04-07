use std::collections::BTreeMap;

use data::entities::skill;
use shroom_net::packet::proto::partial::PartialFlag;
use proto95::{id::SkillId, shared::char::CharStatPartial};

pub type PartialCharStats = PartialFlag<(), CharStatPartial>;

#[derive(Debug)]
pub struct CharState {
    _skills: BTreeMap<SkillId, skill::Model>,
    _char_data: PartialCharStats,
}
