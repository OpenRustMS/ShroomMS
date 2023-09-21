use std::sync::Arc;

use dashmap::DashSet;

use crate::{
    entities::{self, character},
    services::{
        character::Character,
        data::{account::AccountId, DataServices},
    },
};

use super::session_manager::SessionBackend;

#[derive(Debug)]
pub struct SessionIngameData {
    pub acc: entities::account::Model,
    pub char: Character,
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
        char: character::Model,
        data: &DataServices,
    ) -> anyhow::Result<()> {
        let Self::Login(login) = self else {
            anyhow::bail!("Session is not in login state")
        };
        let acc_id = login.acc.id;
        let char_id = char.id;
        if char.acc_id != acc_id {
            anyhow::bail!("Chracter with id: {char_id}, does not belong to account: {acc_id}");
        }

        //TODO: important verify that char belongs to the account
        let char = Character::new(
            data.char.must_get(login.acc.id).await?,
            data.item.load_inventory_for_character(char_id).await?,
            data.char.load_skills(char_id).await?,
        );

        *self = Self::Ingame(SessionIngameData {
            acc: login.acc.clone(),
            char,
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
    type TransitionInput = character::Model;

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
                self.data
                    .char
                    .save_skills(char_id, &ingame.char.skills)
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
