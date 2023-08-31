use dashmap::DashMap;
use futures::FutureExt;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    panic::AssertUnwindSafe,
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;

/// Backend for the sessions, to load and save the session data
#[async_trait::async_trait]
pub trait SessionBackend {
    /// Session Data
    type Data;
    /// Parameter for the loading the session data
    type LoadParam;

    type TransitionInput;

    type Error;

    /// Loads the session data with the given parameter
    async fn load(&self, param: Self::LoadParam) -> Result<Self::Data, Self::Error>;
    /// Saves the session data
    async fn save(&self, session: &mut Self::Data) -> Result<(), Self::Error>;

    async fn close(&self, session: Self::Data) -> Result<(), Self::Error>;

    async fn transition(
        &self,
        session: &mut Self::Data,
        input: Self::TransitionInput,
    ) -> Result<(), Self::Error>;
}

/// Transparent wrapper for an owned session
/// Derefs into the owned session
#[derive(Debug)]
pub struct OwnedSession<Key, Data> {
    session: tokio::sync::OwnedMutexGuard<Data>,
    key: Key,
}

impl<Key, Data> OwnedSession<Key, Data> {
    pub fn map<Mapped, F>(mut self, f: F) -> OwnedMappedSession<Key, Data, Mapped>
    where
        F: FnOnce(&mut Data) -> &mut Mapped,
    {
        let mapped = f(&mut self.session) as *mut Mapped;
        OwnedMappedSession {
            session: self,
            mapped,
        }
    }
}

impl<Key, SessionData> Deref for OwnedSession<Key, SessionData> {
    type Target = SessionData;

    fn deref(&self) -> &Self::Target {
        self.session.deref()
    }
}

impl<Key, SessionData> DerefMut for OwnedSession<Key, SessionData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.session.deref_mut()
    }
}

#[derive(Debug)]
pub struct OwnedMappedSession<Key, Data, Mapped> {
    session: OwnedSession<Key, Data>,
    mapped: *mut Mapped,
}

impl<Key, Data, Mapped> OwnedMappedSession<Key, Data, Mapped> {
    pub fn unmap(self) -> OwnedSession<Key, Data> {
        self.session
    }
}

impl<Key, Data, Mapped> Deref for OwnedMappedSession<Key, Data, Mapped> {
    type Target = Mapped;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mapped }
    }
}

impl<Key, Data, Mapped> DerefMut for OwnedMappedSession<Key, Data, Mapped> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mapped }
    }
}

unsafe impl<Key, Data, Mapped> Send for OwnedMappedSession<Key, Data, Mapped>
where
    Key: Send,
    Data: Send,
    Mapped: Send,
{
}
unsafe impl<Key, Data, Mapped> Sync for OwnedMappedSession<Key, Data, Mapped>
where
    Key: Sync,
    Data: Sync,
    Mapped: Sync,
{
}

pub type SharedSessionHandle<SessionData> = Arc<Mutex<SessionData>>;

impl<Key, SessionData> OwnedSession<Key, SessionData> {
    /// Create a new owned session, from the locked session and the key
    pub fn new(session: tokio::sync::OwnedMutexGuard<SessionData>, key: Key) -> Self {
        Self { session, key }
    }

    /// Obtain the key of the owned session
    pub fn key(&self) -> &Key {
        &self.key
    }
}

#[derive(Debug)]
pub struct SessionManager<Key: Eq + Hash, Backend: SessionBackend> {
    sessions: DashMap<Key, SharedSessionHandle<Backend::Data>>,
    backend: Backend,
}

#[derive(Error, Debug)]
pub enum SessionError<B: SessionBackend> {
    #[error("data store disconnected")]
    Backend(B::Error),
    #[error("send panic")]
    SavePanic,
    #[error("session key already exists")]
    SessionKeyAlreadyExists,
    #[error("unable to lock session")]
    UnableToLockSession,
    #[error("Session for key does not exist")]
    SessionKeyNotExists,
}

pub type SessionResult<T, B> = Result<T, SessionError<B>>;

impl<Key, Backend> SessionManager<Key, Backend>
where
    Key: Eq + Hash + Clone,
    Backend: SessionBackend + Send + 'static + std::fmt::Debug,
    Backend::Error: std::fmt::Debug,
    Backend::Data: std::fmt::Debug,
{
    pub fn new(backend: Backend) -> Self {
        Self {
            sessions: DashMap::new(),
            backend,
        }
    }

    /// Transition a session into a new state, using the session input
    pub async fn transition(
        &self,
        session: &mut OwnedSession<Key, Backend::Data>,
        input: Backend::TransitionInput,
    ) -> SessionResult<(), Backend> {
        self.backend
            .transition(session, input)
            .await
            .map_err(SessionError::Backend)?;

        Ok(())
    }

    /// Helper function to close a session
    async fn close_session_inner(
        &self,
        session: SharedSessionHandle<Backend::Data>,
    ) -> SessionResult<(), Backend> {
        let session = Arc::try_unwrap(session).expect("arc session data");
        let mut session_data = session.into_inner();

        // After session is removed save It
        self.backend
            .save(&mut session_data)
            .await
            .map_err(SessionError::Backend)?;

        self.backend
            .close(session_data)
            .await
            .map_err(SessionError::Backend)?;

        Ok(())
    }

    async fn safe_close(
        &self,
        session: SharedSessionHandle<Backend::Data>,
    ) -> SessionResult<(), Backend> {
        let res = AssertUnwindSafe(self.close_session_inner(session))
            .catch_unwind()
            .await;
        // TODO handle errors properly and add fallback handling via the backend
        match res {
            Err(_panic) => Err(SessionError::SavePanic),
            Ok(res) => res,
        }
    }

    /// Remove
    pub async fn remove_unowned_session(&self) -> anyhow::Result<()> {
        let mut removed = vec![];

        // Remove all un-owned sessions, by trying to lock them
        self.sessions.retain(|_, v| {
            if v.clone().try_lock_owned().is_ok() {
                removed.push(v.clone());
                false
            } else {
                true
            }
        });

        // We are now the last holder of the lock, so we are allowed
        for session in removed {
            let res = self.safe_close(session).await;
            if let Err(err) = res {
                log::error!("Error during saving Session: {err:?}");
            }
        }

        Ok(())
    }

    fn create_session_from_data(
        &self,
        key: Key,
        data: Backend::Data,
    ) -> SessionResult<(), Backend> {
        let mut inserted = false;
        self.sessions.entry(key).or_insert_with(|| {
            inserted = true;
            Arc::new(Mutex::new(data))
        });

        if !inserted {
            return Err(SessionError::SessionKeyAlreadyExists);
        }

        Ok(())
    }

    pub async fn close_session(
        &self,
        owned_session: OwnedSession<Key, Backend::Data>,
    ) -> SessionResult<(), Backend> {
        // Remove session, If the session exist It must be in the map
        let (_, session) = self.sessions.remove(&owned_session.key).expect("Session");

        // Release lock, thus decrement the reference count
        drop(owned_session);

        self.close_session_inner(session).await
    }

    pub async fn create_session(
        &self,
        key: Key,
        param: Backend::LoadParam,
    ) -> SessionResult<(), Backend> {
        let data = self
            .backend
            .load(param)
            .await
            .map_err(SessionError::Backend)?;
        self.create_session_from_data(key, data)
    }

    pub async fn create_claimed_session(
        &self,
        key: Key,
        param: Backend::LoadParam,
    ) -> SessionResult<OwnedSession<Key, Backend::Data>, Backend>
    where
        Key: Clone,
    {
        self.create_session(key.clone(), param).await?;
        self.try_claim_session(&key)
    }

    pub fn try_claim_session(
        &self,
        key: &Key,
    ) -> SessionResult<OwnedSession<Key, Backend::Data>, Backend> {
        let data = self
            .sessions
            .get(key)
            .ok_or_else(|| SessionError::SessionKeyNotExists)?
            .value()
            .clone();

        Ok(OwnedSession {
            session: data
                .try_lock_owned()
                .map_err(|_| SessionError::UnableToLockSession)?,
            key: key.clone(),
        })
    }
}
