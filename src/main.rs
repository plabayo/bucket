use shuttle_secrets::SecretStore;

mod router;
mod services;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let auth = services::Auth::new(
        secret_store.get("AUTH_PRIVATE_KEY").unwrap(),
        secret_store.get("AUTHORIZED_EMAILS").unwrap(),
        secret_store.get("SENDGRID_API_KEY").unwrap(),
    );

    let state = router::State { auth };
    let router = router::new(state);

    Ok(router.into())
}
