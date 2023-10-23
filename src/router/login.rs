use std::sync::Arc;

use askama_axum::{IntoResponse, Response};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Form,
};
use serde::Deserialize;
use tower_cookies::{cookie::time::OffsetDateTime, Cookie, Cookies};

#[derive(Deserialize)]
pub struct GetQuery {
    pub magic: Option<String>,
}

pub async fn get(
    Query(query): Query<GetQuery>,
    State(state): State<Arc<crate::router::State>>,
    cookies: Cookies,
) -> Redirect {
    let magic = match query.magic {
        Some(magic) => magic,
        None => return Redirect::temporary("/"),
    };

    match state.auth.verify_magic(magic) {
        Some((magic, expires_at)) => {
            let mut cookie = Cookie::new(crate::services::COOKIE_NAME, magic);
            cookie.set_path("/");
            let offset = OffsetDateTime::from_unix_timestamp(expires_at as i64).unwrap();
            cookie.set_expires(offset);
            cookies.add(cookie);
        }
        None => {
            let mut cookie = Cookie::new(crate::services::COOKIE_NAME, "");
            cookie.set_path("/");
            let offset = OffsetDateTime::from_unix_timestamp(0_i64).unwrap();
            cookie.set_expires(offset);
            cookies.add(cookie);
            return Redirect::to("/");
        }
    }

    tracing::debug!("login user with magic link");
    Redirect::temporary("/")
}

#[derive(Deserialize)]
pub struct PostParams {
    email: String,
}

pub async fn post(
    State(state): State<Arc<crate::router::State>>,
    Form(params): Form<PostParams>,
) -> Response {
    if params.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            super::shared::ErrorTemplate {
                title: "email is required".to_string(),
                message: "Please enter your email address.".to_string(),
                back_path: "/".to_string(),
            },
        )
            .into_response();
    }

    if let Err((msg, status)) = state.auth.send_magic_link(&params.email).await {
        return (
            status,
            super::shared::ErrorTemplate {
                title: "failed to send magic link".to_string(),
                message: msg,
                back_path: "/".to_string(),
            },
        )
            .into_response();
    }

    super::shared::InfoTemplate {
        title: format!("email sent to {}", params.email),
        message: format!("Magic link has been sent to {}. Please open the link in the email to login to this site.", params.email),
        back_path: "/".to_string(),
    }.into_response()
}
