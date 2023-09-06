use std::collections::{BTreeMap, HashSet};

use proto95::{game::user::UpdatedSkillRecord, id::SkillId, shared::char::SkillInfo};
use sea_orm::prelude::DateTimeUtc;
use shroom_net::packet::ShroomExpirationTime;

use crate::services::meta::meta_service::SkillMeta;

#[derive(Debug)]
pub struct SkillData {
    pub id: SkillId,
    pub level: usize,
    pub mastery_level: Option<usize>,
    pub expires_at: Option<DateTimeUtc>,
    pub cooldown: Option<DateTimeUtc>,
    pub meta: &'static SkillMeta,
}

impl SkillData {
    pub fn max_level(&self) -> usize {
        self.mastery_level.unwrap_or(self.meta.levels.len())
    }

    pub fn level_up(&mut self) -> anyhow::Result<()> {
        if self.level >= self.max_level() {
            return Err(anyhow::anyhow!("skill already at max level"));
        }

        self.level += 1;
        Ok(())
    }
}

impl From<&SkillData> for SkillInfo {
    fn from(value: &SkillData) -> Self {
        Self {
            id: value.id,
            level: value.level as u32,
            expiration: ShroomExpirationTime::never(),
            master_level: value.mastery_level.map(|n| n as u32).into(),
        }
    }
}

#[derive(Debug)]
pub struct SkillSet {
    pub skills: BTreeMap<SkillId, SkillData>,
    pub updated_skills: HashSet<SkillId>,
}

impl SkillSet {
    pub fn new() -> Self {
        Self {
            skills: BTreeMap::new(),
            updated_skills: HashSet::new(),
        }
    }

    pub fn get_skill_info(&self) -> Vec<SkillInfo> {
        self.skills.values().map(|data| data.into()).collect()
    }

    pub fn add_skill(&mut self, data: SkillData) {
        self.skills.insert(data.id, data);
    }

    pub fn skills(&self) -> impl Iterator<Item = &SkillData> {
        self.skills.values()
    }

    pub fn skill_up(&mut self, skill_id: SkillId) -> anyhow::Result<()> {
        let skill = self
            .skills
            .get_mut(&skill_id)
            .ok_or_else(|| anyhow::anyhow!("skill not found: {}", skill_id))?;

        skill.level_up()?;
        self.updated_skills.insert(skill_id);
        dbg!(skill);

        Ok(())
    }

    pub fn get_updates(&mut self) -> Option<Vec<UpdatedSkillRecord>> {
        if self.updated_skills.len() > 0 {
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
}