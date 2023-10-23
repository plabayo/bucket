use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use crate::router::shared::ErrorTemplate;

pub async fn any(request: Request<Body>) -> (StatusCode, ErrorTemplate) {
    (
        StatusCode::NOT_FOUND,
        ErrorTemplate {
            title: "404 — Not Found".to_string(),
            message: format!("The page '{}' does not exist.", request.uri().path()),
            back_path: "/".to_string(),
        },
    )
}
