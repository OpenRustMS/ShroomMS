use std::{collections::BTreeMap, sync::Arc};

use dashmap::DashSet;
use proto95::id::SkillId;

use crate::{
    entities::{self, skill},
    services::{
        character::Character,
        data::{account::AccountId, character::CharacterID, DataServices},
    },
};

use super::session_manager::SessionBackend;

#[derive(Debug)]
pub struct SessionIngameData {
    pub acc: entities::account::Model,
    pub char: Character,
    pub skills: BTreeMap<SkillId, skill::Model>,
}

#[derive(Debug)]
pub struct SessionLoginData {
    pub acc: entities::account::Model,
}

#[derive(Debug)]
pub enum ShroomSessionData {
    Ingame(SessionIngameData),
    Login(SessionLoginData),
}

impl ShroomSessionData {
    pub async fn transition_ingame(
        &mut self,
        char_id: CharacterID,
        data: &DataServices,
    ) -> anyhow::Result<()> {
        let Self::Login(login) = self else {
            anyhow::bail!("Session is not in login state")
        };

        //TODO: important verify that char belongs to the account
        let char = Character::new(
            data.char.must_get(login.acc.id).await?,
            data.item.load_inventory_for_character(char_id).await?,
        );

        let skills = data
            .char
            .load_skills(char_id)
            .await?
            .into_iter()
            .map(|skill| (SkillId(skill.id as u32), skill))
            .collect();

        *self = Self::Ingame(SessionIngameData {
            acc: login.acc.clone(),
            char,
            skills,
        });

        Ok(())
    }

    fn get_acc(&self) -> &entities::account::Model {
        match self {
            Self::Ingame(ingame) => &ingame.acc,
            Self::Login(login) => &login.acc,
        }
    }
}

#[derive(Debug)]
pub struct ShroomSessionBackend {
    pub(crate) data: Arc<DataServices>,
    logged_in: DashSet<AccountId>,
}

impl ShroomSessionBackend {
    pub fn new(data: Arc<DataServices>) -> Self {
        Self {
            data,
            logged_in: DashSet::new(),
        }
    }
}

#[async_trait::async_trait]
impl SessionBackend for ShroomSessionBackend {
    type Data = ShroomSessionData;
    type LoadParam = entities::account::Model;
    type Error = anyhow::Error;
    type TransitionInput = CharacterID;

    async fn load(&self, param: Self::LoadParam) -> anyhow::Result<Self::Data> {
        if !self.logged_in.insert(param.id) {
            anyhow::bail!("Account is already logged in");
        }

        Ok(ShroomSessionData::Login(SessionLoginData { acc: param }))
    }
    async fn save(&self, session: &mut Self::Data) -> anyhow::Result<()> {
        log::info!("Saving session for account {}", session.get_acc().id);
        match session {
            ShroomSessionData::Ingame(ingame) => {
                let char_id = ingame.char.id;
                self.data
                    .item
                    .save_inventory(&mut ingame.char.inventory.invs, char_id)
                    .await?;
            }
            ShroomSessionData::Login(_login) => {}
        };

        Ok(())
    }

    async fn close(&self, session: Self::Data) -> anyhow::Result<()> {
        log::info!("Closing session for account {}", session.get_acc().id);
        self.logged_in.remove(&session.get_acc().id);

        Ok(())
    }

    async fn transition(
        &self,
        session: &mut Self::Data,
        input: Self::TransitionInput,
    ) -> Result<(), Self::Error> {
        session.transition_ingame(input, &self.data).await?;
        Ok(())
    }
}