use std::{
    collections::{BTreeMap, HashSet},
    time::{Duration, Instant},
};

use meta::{shared::EvalExpr, SkillMeta};
use proto95::{
    game::user::{SkillCooldownSetResp, UpdatedSkillRecord},
    id::SkillId,
    shared::char::SkillInfo,
};
use sea_orm::prelude::DateTimeUtc;
use shroom_pkt::ShroomExpirationTime;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cooldown(pub Instant);

impl Cooldown {
    pub fn from_duration(d: Duration) -> Self {
        Self(Instant::now() + d)
    }

    pub fn remaining_cooldown(&self) -> Option<Duration> {
        self.remaining_cooldown_from(Instant::now())
    }

    pub fn remaining_cooldown_from(&self, earlier: Instant) -> Option<Duration> {
        self.0.checked_duration_since(earlier)
    }
}

#[derive(Debug)]
pub struct SkillData {
    pub id: SkillId,
    pub level: usize,
    pub mastery_level: Option<usize>,
    pub expires_at: Option<DateTimeUtc>,
    pub cooldown: Option<DateTimeUtc>,
    pub meta: SkillMeta,
}

impl SkillData {
    fn eval_helper(&self, v: &Option<EvalExpr>) -> Option<i32> {
        v.as_ref().map(|v| v.eval(self.level as i32))
    }

    pub fn max_level(&self) -> usize {
        self.meta.max_level as usize
    }

    pub fn level_up(&mut self, pts: u16) -> anyhow::Result<()> {
        let l = self.level + pts as usize;
        if l > self.max_level() {
            return Err(anyhow::anyhow!("skill already at max level"));
        }

        self.level = l;
        Ok(())
    }

    pub fn mastery_up(&mut self) -> anyhow::Result<()> {
        let mastery = self
            .mastery_level
            .ok_or_else(|| anyhow::anyhow!("skill has no mastery"))?;
        if mastery + 10 > self.max_level() {
            return Err(anyhow::anyhow!("skill already at max level"));
        }

        self.mastery_level = Some(mastery + 10);
        Ok(())
    }

    pub fn mp_cost(&self) -> Option<usize> {
        self.eval_helper(&self.meta.cost.mp).map(|n| n as usize)
    }

    pub fn is_buff(&self) -> bool {
        //!self.meta.buff_stat.is_empty()
        // TODO
        true
    }

    pub fn max(&mut self) -> bool {
        let max = self.max_level();
        if self.level != max {
            self.level = max;
            true
        } else {
            false
        }
    }
}

impl From<&SkillData> for SkillInfo {
    fn from(value: &SkillData) -> Self {
        let master_level = if value.id.has_master_level() {
            dbg!((value.mastery_level, value.id));
            Some(value.mastery_level.map(|n| n as u32).unwrap_or(10))
        } else {
            None
        };
        Self {
            id: value.id,
            level: value.level as u32,
            expiration: ShroomExpirationTime::never(),
            master_level: master_level.into()
        }
    }
}

impl From<(SkillId, SkillMeta)> for SkillData {
    fn from((id, skill): (SkillId, SkillMeta)) -> Self {
        SkillData {
            id,
            level: 0,
            mastery_level: skill.master_level.map(|n| n as usize),
            expires_at: None,
            cooldown: None,
            meta: skill,
        }
    }
}

#[derive(Debug)]
pub struct SkillSet {
    pub skills: BTreeMap<SkillId, SkillData>,
    pub skill_cooldowns: BTreeMap<SkillId, Cooldown>,
    pub updated_skills: HashSet<SkillId>,
    pub updated_cooldowns: HashSet<SkillId>,
}

impl Default for SkillSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillSet {
    pub fn new() -> Self {
        Self {
            skills: BTreeMap::new(),
            updated_skills: HashSet::new(),
            skill_cooldowns: BTreeMap::new(),
            updated_cooldowns: HashSet::new(),
        }
    }

    fn update_skill(
        &mut self,
        skill_id: SkillId,
        update: impl FnOnce(&mut SkillData) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let skill = self.get_mut(skill_id)?;
        update(skill)?;
        self.updated_skills.insert(skill_id);
        Ok(())
    }

    pub fn from_skills(skills: impl Iterator<Item = SkillData>) -> anyhow::Result<Self> {
        let mut set = Self::new();
        set.add_skills(skills, true);
        Ok(set)
    }

    pub fn get_skill_info(&self) -> Vec<SkillInfo> {
        self.skills.values().map(|data| data.into()).collect()
    }

    pub fn add_skills(&mut self, skills: impl Iterator<Item = SkillData>, initial: bool) {
        for skill in skills {
            let id = skill.id;
            self.skills.insert(id, skill);
            if !initial {
                self.updated_skills.insert(id);
            }
        }
    }

    pub fn max_skills(&mut self) {
        for skill in self.skills.values_mut() {
            if skill.max() {
                self.updated_skills.insert(skill.id);
            }
        }
    }

    pub fn skills(&self) -> impl Iterator<Item = &SkillData> {
        self.skills.values()
    }

    pub fn get(&self, skill_id: SkillId) -> anyhow::Result<&SkillData> {
        self.skills
            .get(&skill_id)
            .ok_or_else(|| anyhow::anyhow!("skill not found: {:?}", skill_id))
    }

    pub fn get_mut(&mut self, skill_id: SkillId) -> anyhow::Result<&mut SkillData> {
        self.skills
            .get_mut(&skill_id)
            .ok_or_else(|| anyhow::anyhow!("skill not found: {:?}", skill_id))
    }

    pub fn skill_up(&mut self, skill_id: SkillId, d: u16) -> anyhow::Result<()> {
        self.update_skill(skill_id, |skill| skill.level_up(d))
    }

    pub fn mastery_up(&mut self, skill_id: SkillId) -> anyhow::Result<()> {
        self.update_skill(skill_id, |skill| skill.mastery_up())
    }

    pub fn set_cooldown(&mut self, skill_id: SkillId, dur: Duration) {
        if let Some(cd) = self.skill_cooldowns.get_mut(&skill_id) {
            *cd = Cooldown::from_duration(dur);
        } else {
            self.skill_cooldowns
                .insert(skill_id, Cooldown::from_duration(dur));
        }

        self.updated_cooldowns.insert(skill_id);
    }

    pub fn get_cooldowns(&self) -> impl Iterator<Item = (SkillId, Duration)> + '_ {
        let now = Instant::now();
        self.skill_cooldowns
            .iter()
            .filter_map(move |(k, v)| v.remaining_cooldown_from(now).map(|d| (*k, d)))
    }

    pub fn get_updates(&mut self) -> Option<Vec<UpdatedSkillRecord>> {
        if !self.updated_skills.is_empty() {
            Some(
                self.updated_skills
                    .drain()
                    .filter_map(|id| self.skills.get(&id))
                    .map(|data| UpdatedSkillRecord {
                        id: data.id,
                        level: data.level as u32,
                        master_level: data.mastery_level.unwrap_or(0) as u32,
                        expiration: ShroomExpirationTime::never(),
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn get_cooldown_updates(&mut self) -> Option<Vec<SkillCooldownSetResp>> {
        if !self.updated_cooldowns.is_empty() {
            Some(
                self.updated_cooldowns
                    .drain()
                    .filter_map(|id| {
                        let dur = self
                            .skill_cooldowns
                            .get(&id)
                            .and_then(Cooldown::remaining_cooldown);
                        dur.map(|d| SkillCooldownSetResp {
                            skill_id: id,
                            cooldown_s: d.as_secs() as u16,
                        })
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}
