use askama::Template;
use axum::Form;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct GetTemplate {
    pub email: String,
}

pub async fn get() -> GetTemplate {
    GetTemplate {
        email: "foo@example.com".to_string(),
    }
}

#[derive(Template)]
#[template(path = "../templates/fragments/link_ok.html")]
pub struct PostTemplate {
    long: String,
    short: String,
}

#[derive(Deserialize)]
pub struct PostParams {
    link: String,
}

pub async fn post(Form(params): Form<PostParams>) -> PostTemplate {
    PostTemplate {
        long: params.link,
        short: "/abc123".to_string(),
    }
}
