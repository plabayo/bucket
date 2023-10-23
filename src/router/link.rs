use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode, response::Redirect, Form};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::data::Shortlink;

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

            // validate domains
            let domain = match url.domain() {
                Some(domain) => domain,
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        super::shared::ErrorTemplate {
                            title: "Invalid Long URL".to_string(),
                            message: "The long URL is invalid. No domain found.".to_string(),
                            back_path: format!("/link?long={}", long),
                        },
                    )
                        .into_response();
                }
            };
            // ...only allow second level domains or higher
            if domain.split('.').count() < 2 {
                return (
                    StatusCode::BAD_REQUEST,
                    super::shared::ErrorTemplate {
                        title: "Invalid Long URL".to_string(),
                        message: "The long URL is invalid. Bare top level domains are not allowed."
                            .to_string(),
                        back_path: format!("/link?long={}", long),
                    },
                )
                    .into_response();
            }
            // ...only allow domains that are not blocked
            if state.storage.is_domain_blocked(domain).await {
                return (
                    StatusCode::BAD_REQUEST,
                    super::shared::ErrorTemplate {
                        title: "Invalid Long URL".to_string(),
                        message: "The long URL is invalid. The domain is blocked.".to_string(),
                        back_path: format!("/link?long={}", long),
                    },
                )
                    .into_response();
            }

            // create shortlink
            let shortlink = Shortlink::new(url.to_string(), email.clone());

            // store shortlink
            if let Err(err) = state.storage.add_shortlink(&shortlink).await {
                tracing::error!("Failed to store shortlink for long url {}: {}", long, err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    super::shared::ErrorTemplate {
                        title: "Failed to store shortlink".to_string(),
                        message: format!(
                            "Failed to store shortlink for long url '{}'. Please try again later.",
                            long
                        ),
                        back_path: format!("/link?long={}", long),
                    },
                )
                    .into_response();
            };

            return PostOkTemplate {
                email,
                long: shortlink.long_url().to_string(),
                short: shortlink.short_url(),
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
