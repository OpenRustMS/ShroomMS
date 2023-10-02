use proto95::{
    id::{job_id::JobGroup, FaceId, HairId, ItemId, SkillId, Skin},
    login::char::{DeleteCharResult, SelectCharResultCode},
    shared::Gender,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QuerySelect, Set,
};

use crate::{
    created_at,
    entities::{
        account,
        character::{self, ActiveModel, Column, Entity, Model},
        skill,
    },
    services::{
        character::skill::{SkillData, SkillSet},
        meta::meta_service::MetaService,
    },
};

use super::{account::{AccountService, AccountId}, item::{ItemService, CharacterEquippedItemIds}};

#[derive(Debug, Clone)]
pub struct ItemStarterSet {
    pub bottom: ItemId,
    pub shoes: ItemId,
    pub top: ItemId,
    pub weapon: ItemId,
    pub guide: ItemId,
}

impl ItemStarterSet {
    pub fn validate(&self, job: JobGroup) -> anyhow::Result<()> {
        //TODO: update to v95
        let _bottom = check_contains(job.get_starter_bottoms(), self.bottom, "Bottom ID")?;
        let _shoes = check_contains(job.get_starter_shoes(), self.shoes, "Shoes ID")?;
        let _top = check_contains(job.get_starter_tops(), self.top, "Top ID")?;
        let _weapon = check_contains(job.get_starter_weapons(), self.weapon, "Weapon ID")?;
        if self.guide != job.get_guide_item() {
            anyhow::bail!("Invalid starter guide");
        }

        Ok(())
    }

    pub fn default_starter_set(job: JobGroup) -> Self {
        Self {
            shoes: ItemId::LEATHER_SANDALS,
            bottom: ItemId::BLUE_JEAN_SHORTS,
            top: ItemId::WHITE_UNDERSHIRT,
            weapon: ItemId::SWORD,
            guide: job.get_guide_item(),
        }
    }
}

pub type CharacterID = i32;

#[derive(Debug, Clone)]
pub struct CharacterCreateDTO {
    pub name: String,
    pub job_group: JobGroup,
    pub face: FaceId,
    pub skin: Skin,
    pub hair: HairId,
    pub starter_set: ItemStarterSet,
    pub gender: Gender,
}

impl CharacterCreateDTO {
    pub fn get_starter_set(&self) -> ItemStarterSet {
        self.starter_set.clone()
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(())
        /*  de-uglify and test this
        let job = self.job_group;
        let _face = check_contains(job.get_starter_face(), self.face, "Face ID")?;
        let _hair = check_contains(job.get_starter_hair(), self.hair, "Hair")?;
        self.starter_set.validate(job)?;

        Ok(())*/
    }
}

fn is_valid_char_name(name: &str) -> bool {
    //TODO error messages
    if !(3..13).contains(&name.len()) {
        return false;
    }

    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }

    true
}

pub fn check_contains<T: PartialEq + std::fmt::Debug>(
    mut iter: impl Iterator<Item = T>,
    check_id: T,
    name: &str,
) -> anyhow::Result<T> {
    if !iter.any(|id| id == check_id) {
        anyhow::bail!("Invalid {name} ({check_id:?}) for char creation ")
    }

    Ok(check_id)
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharWithEquips {
    pub char: Model,
    pub equips: CharacterEquippedItemIds
}

#[derive(Debug)]
pub struct CharacterService {
    db: DatabaseConnection,
    account: AccountService,
    meta: &'static MetaService,
}

impl CharacterService {
    pub fn new(db: DatabaseConnection, meta: &'static MetaService) -> Self {
        Self {
            db: db.clone(),
            account: AccountService::new(db),
            meta,
        }
    }

    pub async fn check_name(&self, name: &str) -> anyhow::Result<bool> {
        if !is_valid_char_name(name) {
            return Ok(false);
        }

        let other_id = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(Column::Name.eq(name))
            .one(&self.db)
            .await?;

        Ok(other_id.is_none())
    }

    pub async fn get_characters_for_account(&self, acc_id: i32) -> anyhow::Result<Vec<Model>> {
        Ok(Entity::find()
            .filter(Column::AccId.eq(acc_id))
            .all(&self.db)
            .await?)
    }

    pub async fn get_characters_with_equips(&self, acc_id: AccountId) -> anyhow::Result<Vec<CharWithEquips>> {
        let inv_svc = ItemService::new(self.db.clone(), self.meta);
        // TODO should be a single query + caching
        let chars = self.get_characters_for_account(acc_id).await?;
        let mut res = Vec::with_capacity(chars.len());
        for char in chars {
            let equips = inv_svc.load_equipped_items(char.id).await?;
            res.push(CharWithEquips {
                char,
                equips,
            });
        }
        Ok(res)
    }

    pub async fn get(&self, char_id: CharacterID) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find_by_id(char_id).one(&self.db).await?)
    }

    pub async fn must_get(&self, char_id: CharacterID) -> anyhow::Result<Model> {
        self.get(char_id)
            .await?
            .ok_or_else(|| anyhow::format_err!("No char for id: {char_id}"))
    }

    pub async fn create_character(
        &self,
        acc_id: i32,
        create: CharacterCreateDTO,
        item_svc: &ItemService,
    ) -> anyhow::Result<CharacterID> {
        create.validate()?;

        if !self.check_name(&create.name).await? {
            anyhow::bail!("Name is not valid");
        }

        let job = create.job_group;
        let map_id = job.get_start_map().0 as i32;
        let job_id = job.get_noob_job_id() as u32;

        let char = ActiveModel {
            acc_id: Set(acc_id),
            created_at: created_at(&self.db),
            gender: Set((create.gender).into()),
            name: Set(create.name),
            map_id: Set(map_id),
            job: Set(job_id as i32),
            level: Set(50),
            str: Set(13),
            dex: Set(4),
            int: Set(4),
            luk: Set(4),
            hp: Set(50 * 100),
            max_hp: Set(50 * 100),
            mp: Set(50 * 100),
            max_mp: Set(50 * 100),
            equip_slots: Set(24),
            use_slots: Set(24),
            setup_slots: Set(24),
            etc_slots: Set(24),
            cash_slots: Set(24),
            storage_slots: Set(16),
            buddy_capacity: Set(20),
            skin: Set(create.skin as u8 as i32),
            face: Set(create.face.0 as i32),
            hair: Set(create.hair.0 as i32),
            exp: Set(0),
            gacha_exp: Set(0),
            mesos: Set(50_000),
            fame: Set(0),
            ap: Set(5),
            sp: Set(10),
            spawn_point: Set(0),
            skill_points: Set(vec![0; 20]),
            play_time: Set(0),
            ..Default::default()
        };

        let char_id = Entity::insert(char).exec(&self.db).await?.last_insert_id;
        item_svc
            .create_starter_set(char_id, create.starter_set)
            .await?;

        let skills = self
            .meta
            .get_skills_for_job(job.get_noob_job_id())
            .map(SkillData::from);
        let skill_set = SkillSet::from_skills(skills)?;
        self.save_skills(char_id, &skill_set).await?;

        Ok(char_id)
    }

    pub async fn delete_character(
        &self,
        acc: &account::Model,
        char_id: CharacterID,
        pic: &str,
    ) -> anyhow::Result<DeleteCharResult> {
        if !self.account.check_pic(acc, pic)? {
            return Ok(DeleteCharResult::InvalidPic);
        }

        let char = self.must_get(char_id).await?;
        if char.acc_id != acc.id {
            return Ok(DeleteCharResult::UnknownErr);
        }

        /* Check:
        - world transfer
        - family
        - guild
        */

        Ok(DeleteCharResult::Success)
    }

    pub async fn select_char_with_pic(
        &self,
        acc: &account::Model,
        char_id: CharacterID,
        pic: &str,
    ) -> anyhow::Result<SelectCharResultCode> {
        if !self.account.check_pic(acc, pic)? {
            return Ok(SelectCharResultCode::InvalidPic);
        }

        self.select_char(acc, char_id).await
    }

    pub async fn select_char(
        &self,
        acc: &account::Model,
        char_id: CharacterID,
    ) -> anyhow::Result<SelectCharResultCode> {
        let char = self.must_get(char_id).await?;
        if char.acc_id != acc.id {
            return Ok(SelectCharResultCode::UnknownErr);
        }
        Ok(SelectCharResultCode::Success)
    }

    pub async fn load_skills(&self, id: CharacterID) -> anyhow::Result<SkillSet> {
        let skills = skill::Entity::find()
            .filter(skill::Column::CharId.eq(id))
            .all(&self.db)
            .await?;

        SkillSet::from_skills(skills.iter().map(|skill| {
            let id = SkillId(skill.skill_id as u32);
            let meta = self.meta.get_skill(id).unwrap();
            SkillData {
                id,
                level: skill.skill_level as usize,
                mastery_level: (skill.master_level != 0).then_some(skill.master_level as usize),
                expires_at: skill.expires_at.map(|t| t.and_utc()),
                cooldown: skill.expires_at.map(|t| t.and_utc()),
                meta,
            }
        }))
    }

    pub async fn save_skills(&self, char_id: CharacterID, skills: &SkillSet) -> anyhow::Result<()> {
        // Remove all skills
        skill::Entity::delete_many()
            .filter(skill::Column::CharId.eq(char_id))
            .exec(&self.db)
            .await?;

        // Insert new skills
        let skills: Vec<_> = skills
            .skills()
            .map(|skill| skill::ActiveModel {
                id: NotSet,
                skill_id: Set(skill.id.0 as i32),
                skill_level: Set(skill.level as i32),
                master_level: Set(skill.mastery_level.unwrap_or(0) as i32),
                expires_at: Set(skill.expires_at.map(|t| t.naive_utc())),
                cooldown: Set(skill.cooldown.map(|t| t.naive_utc())),
                char_id: Set(char_id),
            })
            .collect();
        skill::Entity::insert_many(skills).exec(&self.db).await?;

        Ok(())
    }

    pub async fn save_char(&self, char: character::ActiveModel) -> anyhow::Result<()> {
        char.save(&self.db).await?;
        Ok(())
    }
}
