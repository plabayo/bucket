use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{
    extract::{Host, State},
    http::StatusCode,
    response::Redirect,
    Form,
};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::data::Shortlink;

#[derive(Template)]
#[template(path = "../templates/content/link.html")]
pub struct GetTemplate {
    pub email: String,
    pub long: Option<String>,
    pub shortlinks: Vec<Shortlink>,
    pub scheme: String,
    pub host: String,
}

#[derive(Deserialize)]
pub struct GetParams {
    long: Option<String>,
}

pub async fn get(
    State(state): State<Arc<crate::router::State>>,
    Host(host): Host,
    cookies: Cookies,
    Form(params): Form<GetParams>,
) -> Response {
    if let Some(cookie) = cookies.get(crate::services::COOKIE_NAME) {
        if let Some(identity) = state.auth.verify_cookie(cookie.value()) {
            let shortlinks = state
                .storage
                .get_shortlinks_for_owner(identity.email_hash())
                .await;
            return GetTemplate {
                email: identity.email().to_owned(),
                long: params.long,
                shortlinks,
                scheme: if host.to_lowercase().contains("bckt.xyz") {
                    "https"
                } else {
                    "http"
                }
                .to_owned(),
                host: host.to_owned(),
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
    long: Option<String>,
    short: Option<String>,
}

pub async fn post(
    State(state): State<Arc<crate::router::State>>,
    cookies: Cookies,
    Host(host): Host,
    Form(params): Form<PostParams>,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get(crate::services::COOKIE_NAME) {
        if let Some(identity) = state.auth.verify_cookie(cookie.value()) {
            if let Some(short) = params.short {
                return LinkPostResponse::Other(match state.storage.delete_shortlink(&short).await {
                    Ok(_) => {
                        crate::router::shared::InfoTemplate {
                            title: "Shortlink Deleted".to_string(),
                            message: format!("The shortlink '{}' has been deleted.", short),
                            back_path: "/link".to_string(),
                        }.into_response()
                    }
                    Err(err) => {
                        crate::router::shared::ErrorTemplate {
                            title: "Failed to Delete Shortlink".to_string(),
                            message: format!("The shortlink '{}' could not be deleted. {}. Please try again later.", short, err),
                            back_path: "/link".to_string(),
                        }.into_response()
                    }
                });
            }

            let long = params.long.unwrap_or_default();
            if long.is_empty() {
                return LinkPostResponse::BadRequest {
                    reason: "URL is not specified.",
                    long,
                };
            }

            // default to https
            let long: String = if long.contains("://") {
                long.clone()
            } else {
                format!("https://{}", long)
            };

            // validate url
            let url = match reqwest::Url::parse(&long) {
                Ok(url) => url,
                Err(_) => {
                    return LinkPostResponse::BadRequest {
                        reason: "URL is invalid.",
                        long,
                    };
                }
            };

            // only allow http and https
            if url.scheme() != "https" && url.scheme() != "http" {
                return LinkPostResponse::BadRequest {
                    reason: "Schema (protocol) is not supported.",
                    long,
                };
            }

            // validate domains
            let domain = match url.domain() {
                Some(domain) => domain,
                None => {
                    return LinkPostResponse::BadRequest {
                        reason: "No domain found.",
                        long,
                    };
                }
            };
            // ...only allow second level domains or higher
            if domain.split('.').count() == 1 {
                return LinkPostResponse::BadRequest {
                    reason: "Bare top level domains are not allowed.",
                    long,
                };
            }
            // ...only allow domains that are not blocked
            if state.storage.is_domain_blocked(domain).await {
                return LinkPostResponse::BadRequest {
                    reason: "The domain is blocked.",
                    long,
                };
            }

            // create shortlink
            let shortlink = Shortlink::new(url.to_string(), identity.email_hash().to_owned());

            // store shortlink
            if let Err(err) = state.storage.add_shortlink(&shortlink).await {
                tracing::error!(
                    "Failed to store shortlink for long url {} by {}: {}",
                    shortlink.owner_email(),
                    shortlink.link_long(),
                    err
                );
                return LinkPostResponse::Exception {
                    reason: "Failed to store shortlink",
                    long,
                };
            };

            return LinkPostResponse::Ok {
                email: identity.email().to_owned(),
                long: shortlink.link_long().to_string(),
                short: shortlink.link_short(
                    if host.to_lowercase().contains("bckt.xyz") {
                        "https"
                    } else {
                        "http"
                    },
                    &host,
                ),
            };
        }
    }
    LinkPostResponse::Forbidden
}

enum LinkPostResponse {
    BadRequest {
        reason: &'static str,
        long: String,
    },
    Forbidden,
    Exception {
        reason: &'static str,
        long: String,
    },
    Ok {
        email: String,
        long: String,
        short: String,
    },
    Other(Response),
}

impl IntoResponse for LinkPostResponse {
    fn into_response(self) -> Response {
        match self {
            LinkPostResponse::BadRequest { reason, long } => (
                StatusCode::BAD_REQUEST,
                super::shared::ErrorTemplate {
                    title: "Invalid Long URL".to_string(),
                    message: if long.is_empty() {
                        format!("The long URL is invalid. {}", reason)
                    } else {
                        format!("The long URL '{}' is invalid. {}", long, reason)
                    },
                    back_path: format!("/link?long={}", long),
                },
            )
                .into_response(),
            LinkPostResponse::Forbidden => (
                StatusCode::FORBIDDEN,
                super::shared::ErrorTemplate {
                    title: "Forbidden".to_string(),
                    message: "You are not authorized for creating shortlinks.".to_string(),
                    back_path: "/".to_string(),
                },
            )
                .into_response(),
            LinkPostResponse::Exception { reason, long } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                super::shared::ErrorTemplate {
                    title: reason.to_string(),
                    message: format!("{} for '{}'. Please try again later.", reason, long),
                    back_path: format!("/link?long={}", long),
                },
            )
                .into_response(),
            LinkPostResponse::Ok { email, long, short } => {
                PostOkTemplate { email, long, short }.into_response()
            }
            LinkPostResponse::Other(response) => response,
        }
    }
}
