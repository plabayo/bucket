use askama_axum::{IntoResponse, Response};
use axum::{extract::Query, http::StatusCode, response::Redirect, Form};
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

pub async fn post(Form(params): Form<PostParams>) -> Response {
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
    super::shared::InfoTemplate {
        title: format!("email sent to {}", params.email),
        message: format!("Magic link has been sent to {}. Please open the link in the email to login to this site.", params.email),
    }.into_response()
}
