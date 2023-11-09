use std::env;

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IntrospectionUrl,
    RedirectUrl, Scope, TokenIntrospectionResponse, TokenResponse, TokenUrl,
};

use axum::{
    headers::Cookie,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use tokio::sync::OnceCell;

use crate::shared::dto::user::UserInfoDto;

async fn initalize_oauth_client() -> BasicClient {
    let client_id = env::var("CLIENT_ID").expect("Missing the CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing the CLIENT_SECRET environment variable.");
    let issuer = env::var("ISSUER").expect("Missing the ISSUER environment variable.");
    let auth_url = format!("{}/oauth/v2/authorize", issuer);
    let token_url = format!("{}/oauth/v2/token", issuer);
    let introspection_url = format!("{}/oauth/v2/introspect", issuer);

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).expect("Could not parse auth url"),
        Some(TokenUrl::new(token_url).expect("Could not parse token url")),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:3000/login/callback".to_string())
            .expect("Could not parse redirect url"),
    )
    .set_introspection_uri(
        IntrospectionUrl::new(introspection_url).expect("COuld not parse introspection url"),
    )
}

static AUTH_CLIENT: OnceCell<BasicClient> = OnceCell::const_new();

async fn get_oauth_client<'a>() -> &'a BasicClient {
    AUTH_CLIENT.get_or_init(initalize_oauth_client).await
}

pub async fn get_authorization_url() -> String {
    let client = get_oauth_client().await;
    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    auth_url.to_string()
}

pub async fn get_token(code: String) -> Option<String> {
    let client = get_oauth_client().await;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;

    let Ok(token) = token_response else {
        return None;
    };

    Some(token.access_token().secret().clone())
}

pub async fn verify_token(token: &str) -> Option<UserInfoDto> {
    let client = get_oauth_client().await;
    let token = AccessToken::new(token.to_string());

    let introspection_response = client
        .introspect(&token)
        .expect("IntrospectionUrl was not configured")
        .request_async(async_http_client)
        .await;

    let Ok(introspection_response) = introspection_response else {
        return None;
    };

    let Some(username) = introspection_response.username() else {
        return None;
    };

    let Some(user_id) = introspection_response.sub() else {
        return None;
    };

    Some(UserInfoDto {
        username: username.to_string(),
        user_id: user_id.to_string(),
    })
}

pub async fn require_token<B>(
    TypedHeader(cookie): TypedHeader<Cookie>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let Some(access_token) = cookie.get("access_token") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let Some(_) = verify_token(access_token).await else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(request).await)
}
