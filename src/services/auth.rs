use axum::http::StatusCode;
use base64::{engine::general_purpose, Engine as _};
use orion::aead::SecretKey;

#[derive(Debug)]
pub struct Auth {
    email_validator: EmailValidator,
    sendgrid_api_key: String,
    secret_key: SecretKey,
}

// TODO implement using
// - orion for encryption

impl Auth {
    pub fn new(private_key: String, raw_auth_emails: String, sendgrid_api_key: String) -> Self {
        let secret_key =
            SecretKey::from_slice(private_key.as_bytes()).expect("invalid private key");
        Self {
            email_validator: EmailValidator::new(raw_auth_emails),
            sendgrid_api_key,
            secret_key,
        }
    }

    pub async fn send_magic_link(&self, email: &str) -> Result<(), (String, StatusCode)> {
        if !self.email_validator.validate(email) {
            return Err((
                "Email is not authorized to login. Please enter another email or ask an invite to @glendc.".to_string(),
                StatusCode::UNAUTHORIZED,
            ));
        }

        // create magic
        let magic = AuthTokenMagic::new(email.to_string())
            .map_err(|e| {
                tracing::error!("failed making auth token magic: {:?}", e);
                (
                    "failed making auth token magic".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?
            .to_string();
        let cipher_text = orion::aead::seal(&self.secret_key, magic.as_bytes()).map_err(|e| {
            tracing::error!("failed encrypting magic: {:?}", e);
            (
                "failed encrypting magic".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;
        let magic = general_purpose::STANDARD_NO_PAD.encode(&cipher_text);

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

struct AuthTokenMagic {
    email: String,
    token: Vec<u8>,
    expires_at: u64,
}

impl AuthTokenMagic {
    pub fn new(email: String) -> Result<Self, String> {
        let mut token = [0u8; 16];
        orion::util::secure_rand_bytes(&mut token).map_err(|e| e.to_string())?;
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .ok_or("failed to calculate expires_at")?
            .timestamp() as u64;
        Ok(Self {
            email,
            token: token.to_vec(),
            expires_at,
        })
    }
}

impl std::fmt::Display for AuthTokenMagic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::json!({
            "email": self.email,
            "token": general_purpose::STANDARD_NO_PAD.encode(&self.token),
            "expires_at": self.expires_at,
        })
        .to_string();
        write!(f, "{value}")
    }
}

impl TryFrom<&str> for AuthTokenMagic {
    type Error = String;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let value: serde_json::Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
        let email = value
            .get("email")
            .ok_or("missing email")?
            .as_str()
            .ok_or("invalid email")?
            .to_owned();
        let token = value
            .get("token")
            .ok_or("missing token")?
            .as_str()
            .ok_or("invalid token")?;
        let token = general_purpose::STANDARD_NO_PAD
            .decode(token.as_bytes())
            .map_err(|e| e.to_string())?;
        let expires_at = value
            .get("expires_at")
            .ok_or("missing expires_at")?
            .as_u64()
            .ok_or("invalid expires_at")?;
        Ok(Self {
            email,
            token,
            expires_at,
        })
    }
}

#[derive(Debug)]
struct EmailValidator {
    filters: Vec<String>,
}

impl EmailValidator {
    pub fn new(raw_emails: impl AsRef<str>) -> Self {
        let filters = raw_emails
            .as_ref()
            .split(',')
            .map(|email| email.trim().to_string())
            .collect();
        Self { filters }
    }

    pub fn validate(&self, email: &str) -> bool {
        let email = email.to_lowercase();
        for filter in &self.filters {
            if filter.starts_with('@') {
                if email.ends_with(filter) {
                    return true;
                }
            } else if &email == filter {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validator() {
        let validator = EmailValidator::new("@example.com");
        assert!(validator.validate("foo@example.com"));
        assert!(!validator.validate("foo@example.org"));
        assert!(!validator.validate("foo@sub.example.org"));
    }

    #[test]
    fn test_email_validator_with_multiple_filters() {
        let validator = EmailValidator::new("john@smith.me,@example.com");
        assert!(validator.validate("foo@example.com"));
        assert!(!validator.validate("foo@smith.me"));
        assert!(validator.validate("john@smith.me"));
    }
}
