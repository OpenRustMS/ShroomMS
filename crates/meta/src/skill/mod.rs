pub mod eval;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::shared;
use crate::shared::{ElementAttribute, EvalExpr};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDotData {
    pub dmg: EvalExpr,
    pub time: EvalExpr,
    pub interval: EvalExpr,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum SkillType {
    Normal = 0, // TODO: does not exist in the files, but we default to it
    Mastery = 1,
    Booster = 2,
    FinalAttack = 3,
    DarkSight = 4, // TODO confirm
}

impl SkillType {
    pub fn is_mastery(&self) -> bool {
        matches!(self, Self::Mastery)
    }

    pub fn is_booster(&self) -> bool {
        matches!(self, Self::Booster)
    }
}

impl TryFrom<i64> for SkillType {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Normal,
            1 => Self::Mastery,
            2 => Self::Booster,
            3 => Self::FinalAttack,
            4 => Self::DarkSight,
            _ => anyhow::bail!("Invalid skill type: {}", value),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillCost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub money: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<(i64, u32)>,
    pub bullets: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack_count: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_time: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_prop: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evasion: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub morph: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastery: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad_x: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad_x: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_mob_p_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_max_hp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_max_mp: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_pad: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_pdd: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_mdd: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_hp_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_mp_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub money_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_damage_min: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_damage_max: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evasion_ratio: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abnormal_status_res: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr_atk_status_res: Option<EvalExpr>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<EvalExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<EvalExpr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassiveSkillData {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub skills: BTreeSet<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillSummonAttack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_range: Option<shared::Rect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circular_range: Option<shared::Circ>,
    pub ty: u32,
    pub attack_after: u32,
    pub mob_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillSummonDieAttack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_range: Option<shared::Rect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circular_range: Option<shared::Circ>,
    pub attack_after: u32,
    pub mob_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillSummonData {
    pub fly: bool,
    pub attack: Option<SkillSummonAttack>,
    pub die_attack: Option<SkillSummonDieAttack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_attr: Option<ElementAttribute>,
    pub invisible: bool,
    pub disable: bool,
    pub has_affected: bool,
    pub skill_type: SkillType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot: Option<SkillDotData>,
    pub cost: SkillCost,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_weapon: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_level: Option<u32>,
    pub max_level: u32,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub req_skills: BTreeMap<u32, u32>,
    pub stats: SkillStats,
    pub combat_orders: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passive: Option<PassiveSkillData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summon: Option<SkillSummonData>,
}
