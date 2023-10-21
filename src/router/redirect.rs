use axum::{extract::Path, response::Redirect};

pub async fn get(Path(hash): Path<String>) -> Redirect {
    Redirect::temporary(&format!("https://{hash}"))
}
