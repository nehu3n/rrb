use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng};
use std::sync::{Arc, RwLock};

use super::db::ProxyDatabase;

pub struct ProxyManager {
    proxies: Arc<RwLock<Vec<String>>>,
    db: Arc<ProxyDatabase>,
}

impl ProxyManager {
    pub fn new() -> Result<Self, sled::Error> {
        let db = Arc::new(ProxyDatabase::new()?);
        let proxies = Arc::new(RwLock::new(Vec::new()));

        if let Ok(initial_proxies) = db.get_all_proxies() {
            let mut proxies_write = proxies.write().unwrap();
            *proxies_write = initial_proxies;
        }

        Ok(Self { proxies, db })
    }

    pub async fn get_next_proxy(&self) -> Option<String> {
        let proxies = self.proxies.read().unwrap();
        proxies.choose(&mut thread_rng()).cloned()
    }

    pub async fn add_proxy(&self, proxy: String) -> Result<(), sled::Error> {
        self.db.add_proxy(&proxy)?;
        let mut proxies = self.proxies.write().unwrap();
        proxies.push(proxy);
        Ok(())
    }

    pub async fn get_all_proxies(&self) -> Vec<String> {
        let proxies = self.proxies.read().unwrap();
        proxies.clone()
    }

    pub async fn shuffle_proxies(&self) {
        let mut proxies = self.proxies.write().unwrap();
        proxies.shuffle(&mut thread_rng());
    }
}

pub static PROXY_MANAGER: Lazy<ProxyManager> = Lazy::new(|| ProxyManager::new().unwrap());
