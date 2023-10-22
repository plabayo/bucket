use axum::response::Redirect;
use serde::Deserialize;
use tower_cookies::{cookie::time::OffsetDateTime, Cookie, Cookies};

#[derive(Deserialize)]
pub struct GetQuery {
    pub magic: Option<String>,
}

pub async fn get(cookies: Cookies) -> Redirect {
    let mut cookie = Cookie::new(crate::services::COOKIE_NAME, "");
    cookie.set_path("/");
    let offset = OffsetDateTime::from_unix_timestamp(0_i64).unwrap();
    cookie.set_expires(offset);
    cookies.add(cookie);

    tracing::debug!("logout user upon their request");
    Redirect::temporary("/")
}
