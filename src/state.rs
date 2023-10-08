use axum::extract::FromRef;
use leptos::{use_context, LeptosOptions, ServerFnError};
use leptos_router::RouteListing;
use sqlx::postgres::PgPool;
use zitadel::axum::introspection::IntrospectionState;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
    pub routes: Vec<RouteListing>,
}

pub async fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}

pub async fn is() -> Result<IntrospectionState, ServerFnError> {
    use_context::<IntrospectionState>()
        .ok_or_else(|| ServerFnError::ServerError("IntrospectionState missing.".into()))
}
