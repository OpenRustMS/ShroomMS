use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::Context;
use proto95::{
    game::{mob::MobId, npc::NpcId},
    id::{job_id::JobId, ItemId, MapId, SkillId},
};

use crate::{
    drops::{DropPool, NpcShop, NpcShops},
    field::FhTree,
    skill, wz2, Field, FIELD_REGIONS,
};

#[derive(Debug)]
pub struct FieldMetaData {
    pub field: Field,
    pub fh_tree: FhTree,
}

#[derive(Debug)]
pub struct MetaData {
    pub fields: BTreeMap<MapId, FieldMetaData>,
    pub mobs: BTreeMap<u32, wz2::Mob>,
    pub items: BTreeMap<u32, wz2::Item>,
    pub equips: BTreeMap<u32, wz2::Item>,
    pub skills: BTreeMap<SkillId, skill::Skill>,
    pub npc_shops: NpcShops,
    pub drop_pool: DropPool,
}

pub type FieldMeta = &'static Field;
pub type MobMeta = &'static wz2::Mob;
pub type ItemMeta = &'static wz2::Item;
pub type DropsMeta = &'static DropPool;
pub type SkillMeta = &'static skill::Skill;

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
        let mut fields = BTreeMap::new();
        for region in FIELD_REGIONS.iter() {
            let f: BTreeMap<u32, Field> =
                Self::load_from_file(dir.join(format!("fields/fields{region}.bincode")))
                    .context("Map")?;

            fields.extend(f.into_iter().map(|(id, field)| {
                let fh_tree = FhTree::from_meta(&field);
                (MapId(id), FieldMetaData { field, fh_tree })
            }))
        }
        let skills: BTreeMap<u32, skill::Skill> =
            Self::load_from_json(dir.join("skill.json")).context("Skill")?;

        let drop_pool = DropPool::from_drop_lists(
            Self::load_from_json(dir.join("ext/mob_drops.json")).context("Mob Drops")?,
            Self::load_from_json(dir.join("ext/reactor_drops.json")).context("Reactor Drops")?,
        );

        Ok(Self {
            fields,
            mobs: wz2::load_all(dir.join("wz/Mob"))?,
            items: wz2::load_all(dir.join("wz/Item"))?,
            equips: wz2::load_all(dir.join("wz/Equip"))?,
            skills: skills
                .into_iter()
                .map(|(id, skill)| (SkillId(id), skill))
                .collect(),
            npc_shops: Self::load_from_json(dir.join("ext/npc_shop.json")).context("Shops")?,
            drop_pool,
        })
    }
}

#[derive(Debug)]
pub struct MetaService {
    meta_data: MetaData,
}

impl MetaService {
    pub fn new(meta_data: MetaData) -> Self {
        Self { meta_data }
    }

    pub fn load_from_dir(dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self::new(MetaData::load_from_dir(
            dir.as_ref().to_path_buf(),
        )?))
    }

    pub fn get_field_data(&self, field_id: MapId) -> Option<&Field> {
        self.meta_data.fields.get(&field_id).map(|v| &v.field)
    }

    pub fn get_field_fh_data(&self, field_id: MapId) -> Option<&FhTree> {
        self.meta_data.fields.get(&field_id).map(|v| &v.fh_tree)
    }

    pub fn get_portal_map_spawn(
        &self,
        field_id: MapId,
        field: &Field,
        portal_name: &str,
    ) -> Option<(MapId, u8)> {
        let (_, portal) = field.get_portal_by_name(portal_name)?;
        let map_id = if portal.tm == Some(MapId(999999)) {
            field_id
        } else {
            portal.tm.unwrap()
        };
        let next_map = self.get_field_data(map_id)?;
        let (portal, _) = next_map.get_portal_by_name(portal.tn.as_deref().unwrap())?;
        Some((map_id, portal))
    }

    pub fn get_return_field_spawn(&self, field: &Field) -> Option<(MapId, u8)> {
        let map_id = field.get_return_field();
        let next_map = self.get_field_data(map_id)?;
        let target_sp = next_map.get_first_portal_id()?;
        Some((map_id, target_sp))
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

    pub fn get_drops_for_mob(&self, id: MobId) -> Vec<(ItemId, usize)> {
        self.meta_data
            .drop_pool
            .get_drops_for_mob(id, &mut rand::thread_rng())
    }

    pub fn get_money_drops_for_mob(&self, _id: MobId) -> u32 {
        self.meta_data
            .drop_pool
            .get_money_drop(&mut rand::thread_rng())
    }

    pub fn get_skill(&self, id: SkillId) -> Option<&skill::Skill> {
        self.meta_data.skills.get(&id)
    }

    pub fn get_skills_for_job(&self, job: JobId) -> impl Iterator<Item = (SkillId, &skill::Skill)> {
        self.meta_data
            .skills
            .range(job.skill_range())
            .map(|(id, skill)| (*id, skill))
    }

    pub fn get_npc_shop(&self, npc_id: NpcId) -> Option<&NpcShop> {
        self.meta_data.npc_shops.get(&npc_id)
    }
}
