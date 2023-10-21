use askama::Template;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct GetTemplate;

pub async fn get() -> GetTemplate {
    GetTemplate
}
