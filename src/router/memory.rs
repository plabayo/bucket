use axum::http::HeaderMap;

pub async fn get_robots_txt() -> &'static str {
    r"User-agent: *
Allow: /
Sitemap: https://bckt.xyz/sitemap.xml
"
}

pub async fn get_sitemap_xml() -> impl axum::response::IntoResponse {
    let body = r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://bckt.xyz/</loc>
    </url>
</urlset>"#;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/xml".parse().unwrap());

    (headers, body)
}
