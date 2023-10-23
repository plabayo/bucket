#[derive(Debug, Clone)]
pub struct Shortlink {
    id: String,
    long_url: String,
    _owner: String,
}

impl Shortlink {
    pub fn new(long_url: String, owner: String) -> Self {
        let id = nanoid::nanoid!(8);
        Self {
            id,
            long_url,
            _owner: owner,
        }
    }

    pub fn _owner(&self) -> &str {
        &self._owner
    }

    pub fn long_url(&self) -> &str {
        &self.long_url
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn short_url(&self) -> String {
        format!("/{}", self.id,)
    }
}
