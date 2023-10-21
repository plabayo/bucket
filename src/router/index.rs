use askama::Template;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct GetTemplate;

#[derive(Template)]
#[template(path = "../templates/content/login.html")]
pub struct IndexLoginTemplate;

pub async fn get() -> IndexLoginTemplate {
    IndexLoginTemplate
}
