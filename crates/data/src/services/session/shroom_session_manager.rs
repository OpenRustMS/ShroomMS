use std::{fmt::Debug, net::IpAddr, time::Duration};

use uuid::Uuid;

use super::shroom_session_backend::{
    SessionIngameData, SessionLoginData, ShroomSessionData, ShroomSessionError,
};

use shroom_srv::session::{
    migration::MigrationManager, OwnedMappedSession, OwnedSession, SessionBackend, SessionManager,
    SessionResult,
};

pub type OwnedShroomSession = OwnedSession<Uuid, ShroomSessionData>;

pub type OwnedShroomLoginSession = OwnedMappedSession<Uuid, ShroomSessionData, SessionLoginData>;
pub type OwnedShroomGameSession = OwnedMappedSession<Uuid, ShroomSessionData, SessionIngameData>;

/*
impl OwnedShroomSession {
    pub fn as_login(self) -> OwnedShroomLoginSession {
        Self::map(self, |session| match session {
            ShroomSessionData::Login(login) => login,
            _ => panic!("Session is not a login session"),
        })
    }

    pub fn as_ingame(self) -> OwnedShroomGameSession {
        Self::map(self, |session| match session {
            ShroomSessionData::Ingame(ingame) => ingame,
            _ => panic!("Session is not an ingame session"),
        })
    }
}*/

// Client uses a 8 byte session id
pub type ClientKey = [u8; 8];

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct ShroomMigrationKey {
    client_key: ClientKey,
    peer_addr: IpAddr,
}

impl ShroomMigrationKey {
    pub fn new(client_key: ClientKey, peer_addr: IpAddr) -> Self {
        Self {
            client_key,
            peer_addr,
        }
    }
}

/// Manages all sessions
pub struct ShroomSessionManager<Backend: SessionBackend> {
    session_man: SessionManager<Uuid, Backend>,
    migration: MigrationManager<ShroomMigrationKey, OwnedSession<Uuid, Backend::Data>>,
}

impl<B: SessionBackend + std::fmt::Debug> Debug for ShroomSessionManager<B>
where
    B::Data: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShroomSessionManager")
            .field("session_man", &self.session_man)
            .finish()
    }
}

impl<Backend> ShroomSessionManager<Backend>
where
    Backend: SessionBackend<Error = ShroomSessionError> + std::fmt::Debug + Send + 'static,
    Backend::Data: std::fmt::Debug,
{
    pub fn new(backend: Backend, migration_timeout: Duration) -> Self {
        ShroomSessionManager {
            session_man: SessionManager::new(backend),
            migration: MigrationManager::new(migration_timeout),
        }
    }

    fn gen_key() -> uuid::Uuid {
        Uuid::new_v4()
    }

    pub async fn create_claimed_session(
        &self,
        param: Backend::LoadParam,
    ) -> SessionResult<OwnedSession<uuid::Uuid, Backend::Data>, Backend> {
        self.session_man
            .create_claimed_session(Self::gen_key(), param)
            .await
    }

    pub async fn clean(&self) -> anyhow::Result<()> {
        // Remove timed out migrations and free up the sessions
        self.migration.clean();

        // Clean up all un-owned sessions
        self.session_man.remove_unowned_session().await?;

        Ok(())
    }

    /// Closes a the session
    pub async fn close_session(
        &self,
        session: OwnedSession<uuid::Uuid, Backend::Data>,
    ) -> anyhow::Result<()> {
        Ok(self.session_man.close_session(session).await?)
    }

    pub async fn transition_migrate_session(
        &self,
        migration_key: ShroomMigrationKey,
        mut session: OwnedSession<uuid::Uuid, Backend::Data>,
        param: Backend::TransitionInput,
    ) -> anyhow::Result<()> {
        self.transition_session(&mut session, param).await?;
        self.migrate_session(migration_key, session)?;

        Ok(())
    }

    /// Creates a new sessions, which is set into a migration state
    pub async fn transition_session(
        &self,
        session: &mut OwnedSession<uuid::Uuid, Backend::Data>,
        param: Backend::TransitionInput,
    ) -> anyhow::Result<()> {
        self.session_man.transition(session, param).await?;
        Ok(())
    }

    /// Puts a session into a migration state
    pub fn migrate_session(
        &self,
        migration_key: ShroomMigrationKey,
        session: OwnedSession<uuid::Uuid, Backend::Data>,
    ) -> anyhow::Result<()> {
        self.migration.insert(migration_key, session);
        Ok(())
    }

    /// Tries to claim a session in migration, with the given key
    pub async fn claim_migration_session(
        &self,
        migration_key: ShroomMigrationKey,
    ) -> anyhow::Result<OwnedSession<uuid::Uuid, Backend::Data>> {
        self.migration
            .take_with_timeout(&migration_key, self.migration.timeout())
            .await
    }
}
