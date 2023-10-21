use std::path::PathBuf;

use axum::Router;
use shuttle_secrets::SecretStore;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer, services::ServeDir,
    trace::TraceLayer,
};

mod auth;
mod router;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // TODO:
    // - add `authorizer to state` ???
    // - make it compile again...

    let _auth = auth::Auth::new(secret_store.get("AUTHORIZED_EMAILS").unwrap());

    let router = Router::new()
        .nest_service("/static", ServeDir::new(PathBuf::from("static")))
        .nest("/", router::new())
        .fallback(router::not_found::any)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(NormalizePathLayer::trim_trailing_slash()),
        );

    Ok(router.into())
}
