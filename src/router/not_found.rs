use askama::Template;

#[derive(Template)]
#[template(path = "../templates/404.html")]
pub struct GetTemplate;

pub async fn any() -> GetTemplate {
    GetTemplate
}
