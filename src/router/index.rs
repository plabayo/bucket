use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::extract::State;
use tower_cookies::Cookies;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct GetTemplate {
    pub email: String,
}

#[derive(Template)]
#[template(path = "../templates/content/login.html")]
pub struct IndexLoginTemplate;

pub async fn get(State(state): State<Arc<crate::router::State>>, cookies: Cookies) -> Response {
    if let Some(cookie) = cookies.get(crate::services::COOKIE_NAME) {
        if let Some(email) = state.auth.verify_cookie(cookie.value()) {
            return GetTemplate { email }.into_response();
        }
    }
    IndexLoginTemplate.into_response()
}
