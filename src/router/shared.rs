use askama::Template;

#[derive(Template)]
#[template(path = "../templates/content/shared/info.html")]
pub struct InfoTemplate {
    pub title: String,
    pub message: String,
    pub back_path: String,
}

#[derive(Template)]
#[template(path = "../templates/content/shared/error.html")]
pub struct ErrorTemplate {
    pub title: String,
    pub message: String,
    pub back_path: String,
}
