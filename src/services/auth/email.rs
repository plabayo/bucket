#[derive(Debug)]
pub struct EmailValidator {
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
