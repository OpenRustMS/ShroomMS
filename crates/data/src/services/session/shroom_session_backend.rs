use dashmap::DashSet;
use thiserror::Error;

use crate::{
    entities::{self, character},
    services::{
        character::Character,
        data::account::{AccountId, AccountServiceError},
        SharedGameServices,
    },
};

use shroom_srv::session::SessionBackend;

#[derive(Debug, Error)]
pub enum ShroomSessionError {
    #[error("Invalid login session")]
    InvalidLoginSession,
    #[error("Char not belonging to account")]
    CharNotBelongingToAccount,
    #[error("Other error: {0:?}")]
    Anyhow(anyhow::Error),
    #[error("Account error: {0:?}")]
    Account(#[from] AccountServiceError),
}

#[derive(Debug)]
pub enum AccountAuth {
    UsernamePassword(String, String),
}

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
        svc: &SharedGameServices,
    ) -> Result<(), ShroomSessionError> {
        let Self::Login(login) = self else {
            return Err(ShroomSessionError::InvalidLoginSession);
        };
        let acc_id = login.acc.id;
        let char_id = char.id;
        if char.acc_id != acc_id {
            return Err(ShroomSessionError::CharNotBelongingToAccount);
        }

        //TODO: important verify that char belongs to the account
        let char = Character::new(
            svc.clone(),
            svc.data
                .char
                .must_get(login.acc.id)
                .await
                .map_err(ShroomSessionError::Anyhow)?,
            svc.data
                .item
                .load_inventory_for_character(char_id)
                .await
                .map_err(ShroomSessionError::Anyhow)?,
            svc.data
                .char
                .load_skills(char_id)
                .await
                .map_err(ShroomSessionError::Anyhow)?,
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
    pub(crate) game: SharedGameServices,
    logged_in: DashSet<AccountId>,
}

impl ShroomSessionBackend {
    pub fn new(game: SharedGameServices) -> Self {
        Self {
            game,
            logged_in: DashSet::new(),
        }
    }
}

impl SessionBackend for ShroomSessionBackend {
    type Data = ShroomSessionData;
    type LoadParam = AccountAuth;
    type Error = ShroomSessionError;
    type TransitionInput = character::Model;

    async fn load(&self, param: Self::LoadParam) -> Result<Self::Data, ShroomSessionError> {
        let acc = match param {
            AccountAuth::UsernamePassword(username, password) => self
                .game
                .data
                .account
                .try_login(&username, &password)
                .await
                .unwrap(),
        };

        if !self.logged_in.insert(acc.id) {
            return Err(AccountServiceError::AccountAlreadyLoggedIn.into());
        }

        Ok(ShroomSessionData::Login(SessionLoginData { acc }))
    }

    async fn save(&self, session: &mut Self::Data) -> Result<(), ShroomSessionError> {
        log::info!("Saving session for account {}", session.get_acc().id);
        match session {
            ShroomSessionData::Ingame(ingame) => {
                let char_id = ingame.char.id;
                self.game
                    .data
                    .item
                    .save_inventory(&mut ingame.char.inventory.invs, char_id)
                    .await
                    .map_err(ShroomSessionError::Anyhow)?;
                self.game
                    .data
                    .char
                    .save_skills(char_id, &ingame.char.skills)
                    .await
                    .map_err(ShroomSessionError::Anyhow)?;
            }
            ShroomSessionData::Login(_login) => {}
        };

        Ok(())
    }

    async fn close(&self, session: &mut Self::Data) -> Result<(), ShroomSessionError> {
        log::info!("Closing session for account {}", session.get_acc().id);
        self.logged_in.remove(&session.get_acc().id);

        Ok(())
    }

    async fn transition(
        &self,
        session: &mut Self::Data,
        input: Self::TransitionInput,
    ) -> Result<(), Self::Error> {
        session.transition_ingame(input, &self.game).await?;
        Ok(())
    }
}
