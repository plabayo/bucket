#[derive(Debug)]
pub struct Auth;

// TODO implement using
// - orion for encryption
// - reqwest to send magic link over sendgrid api v3

impl Auth {
    pub fn new(_private_key: String, _raw_auth_emails: String, _sendgrid_api_key: String) -> Auth {
        Auth
    }
}
