use axum::{
    routing::{get, post},
    Router,
};

mod index;
mod link;
mod not_found;
mod redirect;

pub fn new() -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/link", get(link::get))
        .route("/link", post(link::post))
        .route("/{TODO_HOW_POS_ARG?}", get(redirect::get))
        .fallback(not_found::any)
}
