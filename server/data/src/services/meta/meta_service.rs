use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::Context;
use game_data::{map, wz2};
use proto95::{
    game::mob::MobId,
    id::{job_id::JobId, ItemId, MapId, SkillId},
};
use rand::Rng;

use super::{fh_tree::FhTree, skill::SkillInfo};

#[derive(Debug)]
pub struct DropEntry {
    pub item: ItemId,
    pub max_quantity: usize,
    pub chance: f32,
}

#[derive(Debug)]
pub struct DropPool {
    pub entries: Vec<DropEntry>,
    pub money: u32,
    pub money_variance: u32,
}

impl DropPool {
    pub fn get_item_drops<R: Rng>(&self, rng: &mut R) -> Vec<(ItemId, usize)> {
        let mut drops = Vec::new();
        for entry in self.entries.iter() {
            if !rng.gen_bool(entry.chance.into()) {
                continue;
            }

            drops.push((entry.item, rng.gen_range(1..=entry.max_quantity)))
        }
        drops
    }

    pub fn get_money_drop<R: Rng>(&self, rng: &mut R) -> u32 {
        rng.gen_range((self.money - self.money_variance)..=self.money)
    }
}

#[derive(Debug)]
pub struct FieldMetaData {
    pub map: map::Map,
    pub fh_tree: FhTree,
}

#[derive(Debug)]
pub struct MetaData {
    pub maps0: BTreeMap<i64, map::Map>,
    pub maps0_fh: BTreeMap<i64, FhTree>,
    pub mobs: BTreeMap<u32, wz2::Mob>,
    pub items: BTreeMap<u32, wz2::Item>,
    pub equips: BTreeMap<u32, wz2::Item>,
    pub skills: BTreeMap<SkillId, SkillInfo>,
}

pub type FieldMeta = &'static map::Map;
pub type MobMeta = &'static wz2::Mob;
pub type ItemMeta = &'static wz2::Item;
pub type DropsMeta = &'static DropPool;
pub type SkillMeta = &'static SkillInfo;

impl MetaData {
    fn load_from_file<T: serde::de::DeserializeOwned>(file: impl AsRef<Path>) -> anyhow::Result<T> {
        let file = File::open(file)?;
        Ok(bincode::deserialize_from(file)?)
    }

    fn load_from_json<T: serde::de::DeserializeOwned>(file: impl AsRef<Path>) -> anyhow::Result<T> {
        let file = File::open(file)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn load_from_dir(dir: PathBuf) -> anyhow::Result<Self> {
        let maps0: BTreeMap<i64, map::Map> = Self::load_from_file(dir.join("maps0.rbin")).context("Map")?;
        let skills: BTreeMap<u32, SkillInfo> =
            Self::load_from_json(dir.join("warrion_skill_bundle.json")).context("Skill")?;

        Ok(Self {
            maps0_fh: maps0
                .iter()
                .map(|(id, map)| (*id, FhTree::from_meta(map)))
                .collect(),
            maps0,
            mobs: wz2::load_all(dir.join("wz/Mob"))?,
            items: wz2::load_all(dir.join("wz/Item"))?,
            equips: wz2::load_all(dir.join("wz/Equip"))?,
            skills: skills
                .into_iter()
                .map(|(id, skill)| (SkillId(id), skill))
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct MetaService {
    meta_data: MetaData,
    hard_coded_drop_pool: DropPool,
}

impl MetaService {
    pub fn new(meta_data: MetaData) -> Self {
        let hard_coded_drop_pool = DropPool {
            entries: vec![
                DropEntry {
                    item: ItemId::ADVANCED_MONSTER_CRYSTAL_1,
                    max_quantity: 5,
                    chance: 0.5,
                },
                DropEntry {
                    item: ItemId::PINK_ADVENTURER_CAPE,
                    max_quantity: 1,
                    chance: 0.7,
                },
                DropEntry {
                    item: ItemId::CHAOS_SCROLL_60,
                    max_quantity: 5,
                    chance: 0.7,
                },
            ],
            money: 1_000,
            money_variance: 970,
        };

        Self {
            meta_data,
            hard_coded_drop_pool,
        }
    }

    pub fn load_from_dir(dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self::new(MetaData::load_from_dir(
            dir.as_ref().to_path_buf(),
        )?))
    }

    pub fn get_field_data(&self, field_id: MapId) -> Option<&map::Map> {
        self.meta_data.maps0.get(&(field_id.0 as i64))
    }

    pub fn get_field_fh_data(&self, field_id: MapId) -> Option<&FhTree> {
        self.meta_data.maps0_fh.get(&(field_id.0 as i64))
    }

    pub fn get_mob_data(&self, mob_id: MobId) -> Option<&wz2::Mob> {
        self.meta_data.mobs.get(&mob_id)
    }

    pub fn get_item_data(&self, id: ItemId) -> Option<&wz2::Item> {
        self.meta_data.items.get(&id.0)
    }

    pub fn get_eq_data(&self, id: ItemId) -> Option<&wz2::Item> {
        self.meta_data.equips.get(&id.0)
    }

    pub fn get_drops_for_mob(&self, _id: MobId) -> Option<&DropPool> {
        Some(&self.hard_coded_drop_pool)
    }

    pub fn get_skill(&self, id: SkillId) -> Option<&SkillInfo> {
        self.meta_data.skills.get(&id)
    }

    pub fn get_skills_for_job(&self, job: JobId) -> impl Iterator<Item = (SkillId, &SkillInfo)> {
        self.meta_data
            .skills
            .range(job.skill_range())
            .map(|(id, skill)| (*id, skill))
    }
}
