use base64::{engine::general_purpose, Engine as _};
use base64_serde::base64_serde_type;
use orion::{aead::SecretKey, hash};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::email::EmailValidator;

base64_serde_type!(Base64Standard, general_purpose::URL_SAFE);

#[derive(Debug)]
pub enum MagicError {
    Encrypt(String),
    Decrypt(String),
    Encode(String),
    Decode(String),
}

impl std::fmt::Display for MagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicError::Encrypt(msg) => write!(f, "magic error: encrypt: {}", msg),
            MagicError::Decrypt(msg) => write!(f, "magic error: decrypt: {}", msg),
            MagicError::Encode(msg) => write!(f, "magic error: encode: {}", msg),
            MagicError::Decode(msg) => write!(f, "magic error: decode: {}", msg),
        }
    }
}

impl std::error::Error for MagicError {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIdentity {
    email: String,
    email_hash: String,
    #[serde(with = "Base64Standard")]
    token: Vec<u8>,
    expires_at: u64,
    verified: bool,
}

impl MagicIdentity {
    pub fn new(email: &str) -> Result<Self, String> {
        let email = email.to_lowercase();
        let email_hash = hex::encode(
            hash::digest(email.as_bytes())
                .expect("hashing email")
                .as_ref(),
        );
        let mut token = [0u8; 16];
        orion::util::secure_rand_bytes(&mut token).map_err(|e| e.to_string())?;
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(1))
            .ok_or("failed to calculate expires_at")?
            .timestamp() as u64;
        Ok(Self {
            email,
            email_hash,
            token: token.to_vec(),
            expires_at,
            verified: false,
        })
    }

    pub fn decrypt(cipher: impl AsRef<str>, secret_key: &SecretKey) -> Result<Self, MagicError> {
        let cipher = cipher.as_ref();
        let cipher_bytes = general_purpose::URL_SAFE
            .decode(cipher.as_bytes())
            .map_err(|e| MagicError::Decode(e.to_string()))?;
        let magic_bytes = orion::aead::open(secret_key, &cipher_bytes)
            .map_err(|e| MagicError::Decrypt(e.to_string()))?;
        serde_json::from_slice(&magic_bytes).map_err(|e| MagicError::Decode(e.to_string()))
    }

    pub fn encrypt(&self, secret_key: &SecretKey) -> Result<String, MagicError> {
        let magic_bytes =
            serde_json::to_vec(self).map_err(|e| MagicError::Encode(e.to_string()))?;
        let cipher_bytes = orion::aead::seal(secret_key, &magic_bytes)
            .map_err(|e| MagicError::Encrypt(e.to_string()))?;
        let cipher = general_purpose::URL_SAFE.encode(cipher_bytes);
        Ok(cipher)
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn email_hash(&self) -> &str {
        &self.email_hash
    }

    pub fn verified(&self) -> bool {
        self.verified
    }

    pub fn expired(&self) -> bool {
        self.expires_at < chrono::Utc::now().timestamp() as u64
    }

    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }

    pub fn verify(&mut self) {
        self.verified = true;
        self.expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(7))
            .unwrap()
            .timestamp() as u64;
    }

    #[allow(dead_code)]
    pub fn id(&self) -> String {
        format!("{:x?}", hash::digest(self.email.as_bytes()).unwrap(),)
    }
}

#[derive(Debug)]
pub struct MagicSender {
    sendgrid_api_key: String,
    email_validator: EmailValidator,
}

impl MagicSender {
    pub fn new(sendgrid_api_key: String, raw_auth_emails: String) -> Self {
        Self {
            sendgrid_api_key,
            email_validator: EmailValidator::new(raw_auth_emails),
        }
    }

    pub async fn send_magic_link(
        &self,
        email: &str,
        key: &SecretKey,
    ) -> Result<(), (String, StatusCode)> {
        if !self.email_validator.validate(email) {
            return Err((format!(
                "Email '{}' is not authorized to login. Please enter another email or ask an invite to @glendc.", email),
                StatusCode::UNAUTHORIZED,
            ));
        }

        // create magic
        let magic: MagicIdentity = MagicIdentity::new(email).map_err(|e| {
            tracing::error!("failed making auth token magic: {:?}", e);
            (
                "failed making auth token magic".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;
        let magic = magic.encrypt(key).map_err(|e| {
            tracing::error!("failed encrypting magic: {:?}", e);
            (
                "failed encrypting magic".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

        // send magic
        let client = reqwest::Client::new();
        let result = client
            .post("https://api.sendgrid.com/v3/mail/send")
            .header("Authorization", format!("Bearer {}", self.sendgrid_api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
               "personalizations": [
                  {
                     "to": [
                        {
                           "email": email
                        }
                     ],
                     "dynamic_template_data": {
                        "magic": magic
                     }
                  }
               ],
               "from": {
                  "email": "hello@bckt.xyz",
                  "name": "bckt.xyz"
               },
               "reply_to": {
                  "email": "hello@bckt.xyz",
                  "name": "bckt.xyz"
               },
               "mail_settings": {
                  "bypass_list_management": {
                     "enable": false
                  },
                  "footer": {
                     "enable": false
                  },
                  "sandbox_mode": {
                     "enable": false
                  }
               },
               "tracking_settings": {
                  "click_tracking": {
                     "enable": false,
                     "enable_text": false
                  },
                  "open_tracking": {
                     "enable": false
                  },
                  "subscription_tracking": {
                     "enable": false
                  }
               },
               "template_id": "d-3bf522f04d47411489abe38342be66a4"
            }))
            .send()
            .await;
        let resp = match result {
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                return Err((
                    "Error sending magic link.".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
            Ok(response) => response,
        };

        if resp.status() != StatusCode::ACCEPTED {
            tracing::error!("Error: {:?}: {}", resp.status(), resp.text().await.unwrap());
            return Err((
                "Error sending magic link.".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_identity_crypto() {
        let magic = MagicIdentity::new("john@example.com").unwrap();
        let key = SecretKey::default();
        let cipher = magic.encrypt(&key).unwrap();
        let magic2 = MagicIdentity::decrypt(cipher, &key).unwrap();
        assert_eq!(magic, magic2);
    }

    #[test]
    fn test_magic_identity_encrypt_different_nonce() {
        let magic = MagicIdentity::new("john@example.com").unwrap();
        let key = SecretKey::default();
        let cipher1 = magic.encrypt(&key).unwrap();
        let cipher2 = magic.encrypt(&key).unwrap();
        assert_ne!(cipher1, cipher2);
    }
}
