#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Shortlink {
    owner_email: String,
    link_hash: String,
    link_long: String,
}

impl Shortlink {
    pub fn new(link_long: String, owner_email: String) -> Self {
        let link_hash = nanoid::nanoid!(8);
        Self {
            owner_email,
            link_hash,
            link_long,
        }
    }

    pub fn owner_email(&self) -> &str {
        &self.owner_email
    }

    pub fn link_long(&self) -> &str {
        &self.link_long
    }

    pub fn link_hash(&self) -> &str {
        &self.link_hash
    }

    pub fn link_short(&self, scheme: &str, host: &str) -> String {
        format!("{}://{}/{}", scheme, host, self.link_hash)
    }
}
