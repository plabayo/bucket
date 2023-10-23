use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::data::Shortlink;

#[derive(Debug)]
pub struct Storage {
    inner: Arc<Mutex<InnerStorage>>,
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Debug)]
pub struct InnerStorage {
    shortlinks: HashMap<String, Shortlink>,
    blocked_domains: HashSet<String>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerStorage {
                shortlinks: HashMap::new(),
                blocked_domains: HashSet::new(),
            })),
        }
    }

    pub async fn is_domain_blocked(&self, domain: &str) -> bool {
        self.inner.lock().unwrap().blocked_domains.contains(domain)
    }

    pub async fn add_shortlink(&self, shortlink: &Shortlink) -> Result<(), String> {
        let id = shortlink.id().to_string();
        self.inner
            .lock()
            .unwrap()
            .shortlinks
            .entry(id)
            .or_insert(shortlink.clone());
        Ok(())
    }

    pub async fn get_shortlink(&self, id: &str) -> Option<Shortlink> {
        self.inner.lock().unwrap().shortlinks.get(id).cloned()
    }
}
