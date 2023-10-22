use axum::{extract::Path, response::Redirect};

pub async fn get(Path(hash): Path<String>) -> Redirect {
    match hash.as_str() {
        "code" => Redirect::permanent("https://github.com/plabayo/bucket"),
        "author" => Redirect::permanent("https://plabayo.tech"),
        "og-image" => Redirect::permanent(
            "https://upload.wikimedia.org/wikipedia/commons/3/3b/Sand_bucket.jpg",
        ),
        hash => Redirect::temporary(&format!("https://{hash}")),
    }
}
