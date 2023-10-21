use axum::{
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

mod index;
mod link;
mod login;
mod redirect;
mod shared;

pub mod not_found;

pub async fn get_robots_txt() -> &'static str {
    r"User-agent: *
Allow: /
Sitemap: https://bckt.xyz/sitemap.xml
"
}

pub async fn get_sitemap_xml() -> impl IntoResponse {
    let body = r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://bckt.xyz/</loc>
    </url>
</urlset>"#;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/xml".parse().unwrap());

    (headers, body)
}

pub fn new() -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/robots.txt", get(get_robots_txt))
        .route("/sitemap.xml", get(get_sitemap_xml))
        .route("/link", get(link::get))
        .route("/link", post(link::post))
        .route("/login", get(login::get))
        .route("/login", post(login::post))
        .route("/:hash", get(redirect::get))
}
