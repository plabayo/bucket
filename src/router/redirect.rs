use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Redirect,
};

pub async fn get(
    State(state): State<Arc<crate::router::State>>,
    Path(hash): Path<String>,
) -> Redirect {
    match hash.as_str() {
        "code" => Redirect::permanent("https://github.com/plabayo/bucket"),
        "author" => Redirect::permanent("https://plabayo.tech"),
        "og-image" => Redirect::permanent(
            "https://upload.wikimedia.org/wikipedia/commons/3/3b/Sand_bucket.jpg",
        ),
        hash => {
            if let Some(link) = state.storage.get_shortlink(hash).await {
                Redirect::temporary(link.long_url())
            } else {
                Redirect::temporary("/")
            }
        }
    }
}
