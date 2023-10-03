use std::path::PathBuf;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod pages {
    use askama::Template;

    #[derive(Template)]
    #[template(path = "../templates/index.html")]
    pub struct IndexTemplate;

    pub async fn index() -> IndexTemplate {
        IndexTemplate
    }

    #[derive(Template)]
    #[template(path = "../templates/404.html")]
    pub struct NotFoundTemplate;

    pub async fn not_found() -> NotFoundTemplate {
        NotFoundTemplate
    }
}

mod api {
    use askama::Template;
    // use axum::extract::Query;

    #[derive(Template)]
    #[template(path = "../templates/fragments/link_ok.html")]
    pub struct LinkOkFragment {
        original_link: String,
        short_link: String,
    }

    // pub struct CreateLinkParams {
    //     link: String,
    // }

    // pub async fn create_link(Query(params): Query<CreateLinkParams>) -> LinkOkFragment {
    //     LinkOkFragment {
    //         original_link: params.link,
    //         short_link: "/abc123".to_string(),
    //     }
    // }
}

mod tmp {
    use axum::response::Redirect;

    pub async fn redirect_link_example() -> Redirect {
        Redirect::temporary("https://www.example.com")
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(pages::index))
        // .route("/api/link", post(api::create_link))
        .nest_service("/static", ServeDir::new(PathBuf::from("static")))
        .route("/abc123", get(tmp::redirect_link_example))
        .fallback(pages::not_found);

    Ok(router.into())
}
