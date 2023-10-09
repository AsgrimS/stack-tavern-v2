use std::env;

// use axum::{
//     headers::{authorization::Bearer, Authorization},
//     http::{Request, StatusCode},
//     middleware::Next,
//     response::Response,
//     TypedHeader,
// };
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
// use sqlx::types::Uuid;
use tokio::sync::OnceCell;

async fn initalize_oauth_client() -> BasicClient {
    let client_id = env::var("CLIENT_ID").expect("Missing the CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing the CLIENT_SECRET environment variable.");
    let issuer = env::var("ISSUER").expect("Missing the ISSUER environment variable.");
    let auth_url = format!("{}/oauth/v2/authorize", issuer);
    let token_url = format!("{}/oauth/v2/token", issuer);

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

    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;

    let Ok(token) = token_result else {
        return None;
    };

    Some(token.access_token().secret().clone())
}

// async fn verify_token(token: &str) -> Option<Uuid> {
//     let token = AccessToken::new(token.to_string());
//
//     let client = get_openid_client().await;
//
//     let introspec_request = client.introspect(&token).unwrap();
//     let introspect = introspec_request
//         .request_async(async_http_client)
//         .await
//         .unwrap();
//
//     if !introspect.active() {
//         return None;
//     };
//
//     let (Some(sub), Some(name)) = (introspect.sub(), introspect.username()) else {
//         return None;
//     };
//
//     let Ok(user_uuid) = Uuid::try_parse(sub) else {
//         return None;
//     };
//
//     // if (User::get_by_uuid(&user_uuid).await).is_err() {
//     //     User::create(name, &user_uuid).await.ok();
//     // };
//
//     Some(user_uuid)
// }

// pub async fn require_token<B>(
//     TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
//     mut request: Request<B>,
//     next: Next<B>,
// ) -> Result<Response, StatusCode> {
//     let Some(user_uuid) = verify_token(auth.token()).await else {
//         return Err(StatusCode::UNAUTHORIZED);
//     };
//
//     request.extensions_mut().insert(user_uuid);
//
//     Ok(next.run(request).await)
// }
