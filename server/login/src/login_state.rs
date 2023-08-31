use std::future::Future;

use data::{entities::account, services::session::{ClientKey, shroom_session_manager::OwnedShroomLoginSession}};
use proto95::login::world::{ChannelId, WorldId};

#[derive(Debug, Clone, Default, PartialEq)]
enum LoginStage {
    #[default]
    Unauthorized,
    AcceptTOS,
    Pin,
    SetGender,
    ServerSelection,
    CharSelection {
        world: WorldId,
        channel: ChannelId,
    },
}

/// State for the login server
/// the core idea is that the whole login logic
/// is handled in the state and illegal operations
/// will result in an error
#[derive(Debug, Default)]
pub struct LoginState {
    stage: LoginStage,
    login_session: Option<OwnedShroomLoginSession>,
    client_key: Option<ClientKey>,
}

impl LoginState {
    pub fn new() -> Self {
        Self {
            stage: LoginStage::Unauthorized,
            login_session: None,
            client_key: None,
        }
    }


    fn check_stage(&self, stage: LoginStage) -> anyhow::Result<()> {
        if self.stage != stage {
            anyhow::bail!("Expected stage: {stage:?}, current stage: {:?}", self.stage);
        }

        Ok(())
    }

    /// Returns an immutable reference to the account
    fn get_account(&self) -> anyhow::Result<&account::Model> {
        self.login_session
            .as_ref()
            .map(|login| &login.acc)
            .ok_or_else(|| anyhow::format_err!("Not authorized"))
    }

    pub fn get_client_key(&self) -> anyhow::Result<ClientKey> {
        self.client_key
            .ok_or_else(|| anyhow::anyhow!("No client key"))
    }

    /// Claim account so It can not be used by the state any longer
    pub fn claim_session(&mut self) -> anyhow::Result<OwnedShroomLoginSession> {
        self.stage = LoginStage::Unauthorized;
        Ok(self.login_session.take().unwrap())
    }

    
    pub fn reset(&mut self) {
        self.login_session = None;
        self.client_key = None;
        self.stage = LoginStage::default();
    }

    pub fn get_char_select(&self) -> anyhow::Result<(&account::Model, WorldId, ChannelId)> {
        if let LoginStage::CharSelection { world, channel } = self.stage {
            return Ok((self.get_account().unwrap(), world, channel));
        }
        anyhow::bail!(
            "Expected stage: CharSelect, current stage: {:?}",
            self.stage
        );
    }

    pub fn get_pin(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::Pin)?;
        self.get_account()
    }

    pub fn get_set_gender(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::SetGender)?;
        self.get_account()
    }

    pub fn get_accept_tos(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::AcceptTOS)?;
        self.get_account()
    }

    pub fn get_unauthorized(&self) -> anyhow::Result<()> {
        self.check_stage(LoginStage::Unauthorized)?;
        Ok(())
    }

    pub fn get_server_selection(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::ServerSelection)
            .or_else(|_| self.check_stage(LoginStage::Pin))?; //Char select
        self.get_account()
    }

    /// Updates the account with the given update operation
    /// ensures that database and local model are in-sync
    pub async fn update_account<F, Fut>(&mut self, update: F) -> anyhow::Result<&account::Model>
    where
        F: FnOnce(account::Model) -> Fut,
        Fut: Future<Output = anyhow::Result<account::Model>>,
    {
        let login = self
            .login_session
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let new_acc = update(login.acc.clone()).await?;
        login.acc = new_acc;
        Ok(&login.acc)
    }

    pub fn is_accept_tos_stage(&self) -> bool {
        matches!(self.stage, LoginStage::AcceptTOS)
    }

    pub fn is_set_gender_stage(&self) -> bool {
        matches!(self.stage, LoginStage::SetGender)
    }


    /// Transitions the stage with the given account
    pub fn transition_login_with_session(&mut self, session: OwnedShroomLoginSession) -> anyhow::Result<()> {
        self.client_key = Some((session.acc.id as u64).to_le_bytes());
        let has_gender = session.acc.gender.is_some();
        let accepted_tos = session.acc.accepted_tos;
        self.stage = if !accepted_tos {
            LoginStage::AcceptTOS
        } else if !has_gender {
            LoginStage::SetGender
        } else {
            LoginStage::Pin
        };

        self.login_session = Some(session);

        Ok(())
    }

    pub fn transition_char_select(
        &mut self,
        world: WorldId,
        channel: ChannelId,
    ) -> anyhow::Result<()> {
        self.stage = LoginStage::CharSelection { world, channel };
        Ok(())
    }

    pub fn transition_server_select(&mut self) -> anyhow::Result<()> {
        self.stage = LoginStage::ServerSelection;
        Ok(())
    }
}
