use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::extract::{Query, State};
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct GetTemplate {
    pub email: String,
}

#[derive(Template)]
#[template(path = "../templates/content/login.html")]
pub struct IndexLoginTemplate {
    email: Option<String>,
}

#[derive(Deserialize)]
pub struct GetQuery {
    email: Option<String>,
}

pub async fn get(
    State(state): State<Arc<crate::router::State>>,
    cookies: Cookies,
    Query(query): Query<GetQuery>,
) -> Response {
    if let Some(cookie) = cookies.get(crate::services::COOKIE_NAME) {
        if let Some(identity) = state.auth.verify_cookie(cookie.value()) {
            return GetTemplate {
                email: identity.email().to_owned(),
            }
            .into_response();
        }
    }
    IndexLoginTemplate { email: query.email }.into_response()
}
