use std::env;

use zitadel::axum::introspection::{IntrospectionState, IntrospectionStateBuilder};

pub async fn initialize_introspect_state() -> IntrospectionState {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");

    IntrospectionStateBuilder::new("http://localhost:8080")
        .with_basic_auth(client_id.as_str(), client_secret.as_str())
        .build()
        .await
        .unwrap()
}

// https://leptos-rs.github.io/leptos/server/27_response.html
