use axum::{
    routing::{get, post},
    Router,
};

mod index;
mod link;
mod redirect;

pub mod not_found;

pub fn new() -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/link", get(link::get))
        .route("/link", post(link::post))
        .route("/:hash", get(redirect::get))
}
