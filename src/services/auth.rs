use std::sync::Arc;

use axum::http::StatusCode;

#[derive(Debug, Clone)]
pub struct Auth {
    email_validator: Arc<EmailValidator>,
    sendgrid_api_key: String,
}

// TODO implement using
// - orion for encryption

impl Auth {
    pub fn new(_private_key: String, raw_auth_emails: String, sendgrid_api_key: String) -> Self {
        Self {
            email_validator: Arc::new(EmailValidator::new(raw_auth_emails)),
            sendgrid_api_key,
        }
    }

    pub async fn send_magic_link(&self, email: &str) -> Result<(), (String, StatusCode)> {
        if !self.email_validator.validate(email) {
            return Err((
                "Email is not authorized to login. Please enter another email or ask an invite to @glendc.".to_string(),
                StatusCode::UNAUTHORIZED,
            ));
        }

        // TODO create actual magic here...
        let magic = "hello";

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
