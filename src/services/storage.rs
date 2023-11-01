use sqlx::PgPool;

use crate::data::Shortlink;

#[derive(Debug, Clone)]
pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_domain_blocked(&self, _domain: &str) -> bool {
        false // todo
    }

    pub async fn add_shortlink(&self, shortlink: &Shortlink) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO bckt_links (link_hash, link_long, owner_email) VALUES ($1, $2, $3)",
        )
        .bind(shortlink.link_hash())
        .bind(shortlink.link_long())
        .bind(shortlink.owner_email())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())
        .map(|_| ())
    }

    pub async fn get_shortlink(&self, id: &str) -> Option<Shortlink> {
        sqlx::query_as::<_, Shortlink>("SELECT * FROM bckt_links WHERE link_hash = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .ok()
    }

    pub async fn get_shortlinks_for_owner(&self, owner_email: &str) -> Vec<Shortlink> {
        sqlx::query_as::<_, Shortlink>("SELECT * FROM bckt_links WHERE owner_email = $1")
            .bind(owner_email)
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
    }

    pub async fn delete_shortlink(&self, id: &str, owner_email: &str) -> Result<(), String> {
        let len = sqlx::query("DELETE FROM bckt_links WHERE owner_email = $1 AND link_hash = $2")
            .bind(owner_email)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())
            .map(|result| result.rows_affected())?;
        if len == 0 {
            Err(format!("no shortlink '{id}' exists for current owner"))
        } else {
            Ok(())
        }
    }
}
