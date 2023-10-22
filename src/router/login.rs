use std::sync::Arc;

use askama_axum::{IntoResponse, Response};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetQuery {
    pub magic: Option<String>,
}

pub async fn get(Query(query): Query<GetQuery>) -> Redirect {
    let magic = match query.magic {
        Some(magic) => magic,
        None => return Redirect::temporary("/"),
    };

    println!("Login attempt with magic link: {}", magic);
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
            },
        )
            .into_response();
    }

    super::shared::InfoTemplate {
        title: format!("email sent to {}", params.email),
        message: format!("Magic link has been sent to {}. Please open the link in the email to login to this site.", params.email),
    }.into_response()
}
