use std::sync::Arc;

use askama_axum::{IntoResponse, Response};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};

pub async fn get(
    State(state): State<Arc<crate::router::State>>,
    Path(hash): Path<String>,
) -> Response {
    match hash.as_str() {
        "code" => Redirect::permanent("https://github.com/plabayo/bucket").into_response(),
        "author" => Redirect::permanent("https://plabayo.tech").into_response(),
        "og-image" => Redirect::permanent(
            "https://upload.wikimedia.org/wikipedia/commons/3/3b/Sand_bucket.jpg",
        )
        .into_response(),
        hash => {
            if let Some(link) = state.storage.get_shortlink(hash).await {
                Redirect::temporary(link.long_url()).into_response()
            } else {
                (StatusCode::NOT_FOUND, crate::router::shared::ErrorTemplate {
                    title: "Not Found".to_string(),
                    message: "The requested shortlink does not exist. It might have been deleted or perhaps it never existed to begin with. Please try with another one.".to_string(),
                    back_path: "/".to_string(),
                }).into_response()
            }
        }
    }
}
