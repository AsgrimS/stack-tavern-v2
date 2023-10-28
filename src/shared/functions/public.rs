use leptos::*;

#[server(GetLoginUrl, "/api/public")]
pub async fn get_login_url() -> Result<String, ServerFnError> {
    use crate::api::auth::get_authorization_url;

    let url = get_authorization_url().await;
    Ok(url)
}
