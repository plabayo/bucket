use std::sync::Arc;

use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

mod data;
mod router;
mod services;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../scripts/storage.sql"))
        .await
        .expect("migrate Postgres database");

    let auth = Arc::new(services::Auth::new(
        secret_store.get("AUTH_PRIVATE_KEY").unwrap(),
        secret_store.get("AUTHORIZED_EMAILS").unwrap(),
        secret_store.get("SENDGRID_API_KEY").unwrap(),
    ));

    let storage = services::Storage::new(pool);

    let state = router::State { auth, storage };
    let router = router::new(state);

    tracing::debug!("starting axum router");
    Ok(router.into())
}
