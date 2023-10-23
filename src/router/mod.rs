use std::{path::PathBuf, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer, services::ServeDir,
    trace::TraceLayer,
};

mod index;
mod link;
mod login;
mod logout;
mod memory;
mod not_found;
mod redirect;
mod shared;

#[derive(Debug, Clone)]
pub struct State {
    pub auth: Arc<crate::services::Auth>,
    pub storage: crate::services::Storage,
}

fn new_root(state: State) -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/robots.txt", get(memory::get_robots_txt))
        .route("/sitemap.xml", get(memory::get_sitemap_xml))
        .route("/link", get(link::get))
        .route("/link", post(link::post))
        .route("/login", get(login::get))
        .route("/login", post(login::post))
        .route("/logout", get(logout::get))
        .route("/:hash", get(redirect::get))
        .with_state(Arc::new(state))
        .layer(CookieManagerLayer::new())
}

pub fn new(state: State) -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new(PathBuf::from("static")))
        .nest("/", new_root(state))
        .fallback(not_found::any)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(NormalizePathLayer::trim_trailing_slash()),
        )
}
