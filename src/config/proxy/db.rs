use std::path::Path;
use sled::Db;

pub struct ProxyDatabase {
    db: Db,
}

impl ProxyDatabase {
    pub fn new() -> Result<Self, sled::Error> {
        let db = sled::open(Path::new("proxy_db"))?;
        Ok(Self { db })
    }

    pub fn add_proxy(&self, proxy: &str) -> Result<(), sled::Error> {
        self.db.insert(proxy, &[])?;
        self.db.flush()?;
        Ok(())
    }

   pub fn get_all_proxies(&self) -> Result<Vec<String>, sled::Error> {
        let proxies: Vec<String> = self
            .db
            .iter()
            .filter_map(|item| {
                item.ok()
                    .map(|(key, _)| String::from_utf8_lossy(&key).into_owned())
            })
            .collect();
        Ok(proxies)
    }

    pub fn remove_proxy(&self, proxy: &str) -> Result<(), sled::Error> {
        self.db.remove(proxy)?;
        self.db.flush()?;
        Ok(())
    }
}
