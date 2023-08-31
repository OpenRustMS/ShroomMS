use std::hash::Hash;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use tokio::sync::watch;
use tokio::time::sleep;

/// Represents the context for the migration
/// containing the data and a timeout
#[derive(Debug, Clone)]
struct MigrationContext<V> {
    data: V,
    timeout: Instant,
}

impl<V> MigrationContext<V> {
    pub fn new(data: V, timeout_dur: Duration) -> Self {
        Self {
            data,
            timeout: Instant::now() + timeout_dur,
        }
    }

    /// Checks whether the context timed out
    pub fn is_timeout(&self) -> bool {
        self.timeout < Instant::now()
    }
}

/// Manager for migrations
/// 
#[derive(Debug)]
pub struct MigrationManager<K, V>
where
    K: Eq + Hash,
{
    timeout: Duration,
    pending: DashMap<K, MigrationContext<V>>,
    watch_tx: watch::Sender<Option<K>>,
    watch_rx: watch::Receiver<Option<K>>
}

impl<K, V> MigrationManager<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(timeout: Duration) -> Self {
        let (watch_tx, watch_rx) = watch::channel(None);
        Self {
            timeout,
            pending: DashMap::default(),
            watch_tx,
            watch_rx
        }
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    pub async fn take_with_timeout(&self, key: &K, dur: Duration) -> anyhow::Result<V> {
        tokio::select! {
            _ = sleep(dur) => {
                anyhow::bail!("Timeout reached")
            },
            v = self.take(key) => {
                Ok(v)
            }
        }
    }

    /// Attempts to take the item by key, until It's suceeds
    /// Cancel Safety: This method is cancel safe
    pub async fn take(&self, key: &K) -> V {
        let mut watch_rx = self.watch_rx.clone();
        loop {
            if let Some(data) = self.try_take(key) {
                break data;
            }

            // Sender exist as long the manager exists
            watch_rx.changed().await.expect("Watch recv");
        }
    }

    pub fn try_take(&self, key: &K) -> Option<V> {
        let ctx = self.pending.remove(key);

        match ctx {
            Some((_, v)) if !v.is_timeout() => Some(v.data),
            _ => None,
        }
    }

    pub fn insert(&self, key: K, data: V) {
        self.pending
            .insert(key.clone(), MigrationContext::new(data, self.timeout));
        
        // The manager holds a receiver so this never fails
        self.watch_tx.send(Some(key)).expect("watch send");
    }

    pub fn clean(&self) {
        self.pending.retain(|_, v| !v.is_timeout())
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::services::session::migration::MigrationManager;


    const TIMEOUT: Duration = Duration::from_millis(100);

    #[test]
    fn test_insert_remove() {
        let svc = MigrationManager::<u32, u32>::new(TIMEOUT);

        let key_1 = 1;
        let key_2 = 2;

        // Test insert/remove
        assert_eq!(svc.try_take(&key_1), None);
        svc.insert(key_1, 10);
        assert_eq!(svc.try_take(&key_1), Some(10));
        assert_eq!(svc.try_take(&key_1), None);
        assert_eq!(svc.try_take(&key_2), None);

        //Test timeout
        svc.insert(key_1, 10);
        assert_eq!(svc.pending(), 1);
        sleep(TIMEOUT * 2);
        assert_eq!(svc.try_take(&key_1), None);

        // Test clean
        svc.insert(key_1, 10);
        assert_eq!(svc.pending(), 1);
        sleep(TIMEOUT * 2);
        assert_eq!(svc.pending(), 1);
        svc.clean();
        assert_eq!(svc.pending(), 0);
    }

    #[tokio::test]
    async fn timeout() {
        let svc = MigrationManager::<u32, u32>::new(TIMEOUT);

        assert!(svc.take_with_timeout(&1, TIMEOUT).await.is_err());
        svc.insert(1, 1);
        assert!(svc.take_with_timeout(&1, TIMEOUT).await.is_ok());
    }
}
