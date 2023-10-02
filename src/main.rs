use std::path::PathBuf;

use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;

async fn page_index() -> Html<&'static str> {
    // TODO: support template
    Html(
        r#"
    <!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <title>bckt.xzy</title>

    <link rel="icon"
        href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%2210 0 100 100%22><text y=%22.90em%22 font-size=%2290%22>🪣</text></svg>">

    <script src="/static/js/htmx.min.js?v=1.9.6"></script>
    <link rel="stylesheet" href="/static/css/missing.min.css?v=1.1.1" />
</head>

<body hx-boost="true">
    <header class="navbar f-switch" style="margin: 0">
        <h1 class="heading-1 align-self:start">
            bckt.xzy
        </h1>
    </header>

    <main class="width:100% height:100%">
        <h1>Hello World!</h1>
        <p>This is not very much yet...</p>
    </main>

    <footer class="text-align:center">
        Made with ❤️ by <a href="https://plabayo.tech">plabayo</a>.
    </footer>
</body>

</html>
    "#,
    )
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(page_index))
        .nest_service("/static", ServeDir::new(PathBuf::from("static")));

    Ok(router.into())
}
