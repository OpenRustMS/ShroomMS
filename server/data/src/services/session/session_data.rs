use std::{collections::BTreeMap, sync::Arc};

use proto95::id::SkillId;

use crate::{
    entities::{self, skill},
    services::{
        character::Character,
        data::{character::CharacterID, DataServices},
    },
};

use super::session_manager::{OwnedSession, SessionBackend};

#[derive(Debug, Clone)]
pub struct ShroomSessionData {
    pub acc: entities::account::Model,
    pub char: Character,
    pub skills: BTreeMap<SkillId, skill::Model>,
}

pub type OwnedShroomSession = OwnedSession<uuid::Uuid, ShroomSessionData>;

#[derive(Debug)]
pub struct ShroomSessionBackend {
    pub(crate) data: Arc<DataServices>,
}

#[async_trait::async_trait]
impl SessionBackend for ShroomSessionBackend {
    type SessionData = ShroomSessionData;
    type SessionLoadParam = (entities::account::Model, CharacterID);

    async fn load(&self, param: Self::SessionLoadParam) -> anyhow::Result<Self::SessionData> {
        let (acc, char_id) = param;
        //TODO: important verify that char belongs to the account
        let char = Character::new(self.data.char.must_get(char_id).await?, self.data.item.load_inventory_for_character(char_id).await?);

        let skills = self
            .data
            .char
            .load_skills(char_id)
            .await?
            .into_iter()
            .map(|skill| (SkillId(skill.id as u32), skill))
            .collect();
        Ok(ShroomSessionData { acc, char, skills })
    }
    async fn save(&self, session: Self::SessionData) -> anyhow::Result<()> {
        let char_id = session.char.model.id;
        self.data
            .item
            .save_inventory(session.char.inventory, char_id)
            .await?;

        Ok(())
    }
}
