use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, response::Redirect, Form};
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Template)]
#[template(path = "../templates/content/link.html")]
pub struct GetTemplate {
    pub email: String,
    pub long: Option<String>,
}

#[derive(Deserialize)]
pub struct GetParams {
    long: Option<String>,
}

pub async fn get(
    State(state): State<Arc<crate::router::State>>,
    cookies: Cookies,
    Form(params): Form<GetParams>,
) -> Response {
    if let Some(cookie) = cookies.get(crate::services::COOKIE_NAME) {
        if let Some(email) = state.auth.verify_cookie(cookie.value()) {
            return GetTemplate {
                email,
                long: params.long,
            }
            .into_response();
        }
    }
    Redirect::temporary("/").into_response()
}

#[derive(Template)]
#[template(path = "../templates/content/link_ok.html")]
pub struct PostOkTemplate {
    pub email: String,
    pub long: String,
    pub short: String,
}

#[derive(Deserialize)]
pub struct PostParams {
    long: String,
}

pub async fn post(
    State(state): State<Arc<crate::router::State>>,
    cookies: Cookies,
    Form(params): Form<PostParams>,
) -> Response {
    if let Some(cookie) = cookies.get(crate::services::COOKIE_NAME) {
        if let Some(email) = state.auth.verify_cookie(cookie.value()) {
            if params.long.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    super::shared::ErrorTemplate {
                        title: "Long URL Missing".to_string(),
                        message: "The long URL must be specified.".to_string(),
                        back_path: "/link".to_string(),
                    },
                )
                    .into_response();
            }

            // default to https
            let long: String =
                if params.long.starts_with("http://") || params.long.starts_with("https://") {
                    params.long.clone()
                } else {
                    format!("https://{}", params.long)
                };

            // validate url
            let url = match reqwest::Url::parse(&long) {
                Ok(url) => url,
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        super::shared::ErrorTemplate {
                            title: "Invalid Long URL".to_string(),
                            message: "The long URL is invalid.".to_string(),
                            back_path: format!("/link?long={}", long),
                        },
                    )
                        .into_response();
                }
            };

            // only allow http and https
            if url.scheme() != "https" && url.scheme() != "http" {
                return (
                    StatusCode::BAD_REQUEST,
                    super::shared::ErrorTemplate {
                        title: "Invalid Long URL".to_string(),
                        message: "The long URL is invalid.".to_string(),
                        back_path: format!("/link?long={}", long),
                    },
                )
                    .into_response();
            }

            // TODO validate domain

            return PostOkTemplate {
                email,
                long,
                short: "example.com".to_string(),
            }
            .into_response();
        }
    }
    (
        StatusCode::FORBIDDEN,
        super::shared::ErrorTemplate {
            title: "action forbidden".to_string(),
            message: "You are not authorized for creating shortlinks.".to_string(),
            back_path: "/".to_string(),
        },
    )
        .into_response()
}
