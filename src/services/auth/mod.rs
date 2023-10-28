use axum::http::StatusCode;
use orion::aead::SecretKey;

pub const COOKIE_NAME: &str = "bckt-auth";

mod email;
mod magic;

#[derive(Debug)]
pub struct Auth {
    secret_key: SecretKey,
    magic_sender: magic::MagicSender,
}

impl Auth {
    pub fn new(private_key: String, raw_auth_emails: String, sendgrid_api_key: String) -> Self {
        let secret_key =
            SecretKey::from_slice(private_key.as_bytes()).expect("invalid private key");
        Self {
            secret_key,
            magic_sender: magic::MagicSender::new(sendgrid_api_key, raw_auth_emails),
        }
    }

    pub async fn send_magic_link(&self, email: &str) -> Result<(), (String, StatusCode)> {
        self.magic_sender
            .send_magic_link(email, &self.secret_key)
            .await
    }

    pub fn verify_magic(&self, magic: impl AsRef<str>) -> Option<(String, u64)> {
        let mut identity = match magic::MagicIdentity::decrypt(magic, &self.secret_key) {
            Ok(identity) => identity,
            Err(e) => {
                tracing::debug!("failed decrypting magic: {:?}", e);
                return None;
            }
        };

        if identity.expired() {
            tracing::debug!("magic identity expired");
            return None;
        }
        if identity.verified() {
            tracing::debug!("magic identity already verified");
            return None;
        }

        // make it verified and allow it to be used for a fixed period of time
        identity.verify();

        // encrypt the magic identity again and return it together with the expiration time
        let magic = identity.encrypt(&self.secret_key).ok()?;
        let expires_at = identity.expires_at();
        Some((magic, expires_at))
    }

    pub fn verify_cookie(&self, magic: impl AsRef<str>) -> Option<String> {
        let identity = match magic::MagicIdentity::decrypt(magic, &self.secret_key) {
            Ok(identity) => identity,
            Err(e) => {
                tracing::debug!("failed decrypting magic: {:?}", e);
                return None;
            }
        };

        if identity.expired() {
            tracing::debug!("magic expired");
            return None;
        }
        if !identity.verified() {
            tracing::debug!("magic not yet verified");
            return None;
        }

        Some(identity.email().to_owned())
    }
}
