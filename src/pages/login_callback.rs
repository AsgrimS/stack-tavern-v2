use crate::components::navbar::Navbar;
use leptos::*;
use leptos_router::*;

const ACCESS_TOKEN_COOKIE: &str = "access_token";

#[server(Login, "/api/auth")]
pub async fn login(code: String) -> Result<bool, ServerFnError> {
    use crate::api::auth::get_token;
    use axum::http::header;
    use cookie::Cookie;
    use leptos_axum::ResponseOptions;

    let token = get_token(code).await;
    let response = expect_context::<ResponseOptions>();

    if let Some(token) = token {
        let cookie = Cookie::build((ACCESS_TOKEN_COOKIE, token))
            .path("/")
            .secure(true)
            .http_only(true)
            .to_string();

        response.insert_header(
            header::SET_COOKIE,
            header::HeaderValue::from_str(cookie.as_str()).unwrap(),
        );

        return Ok(true);
    }

    Ok(false)
}

#[derive(Params, PartialEq)]
struct LoginCallbackQueryParams {
    code: String,
}

/// Renders the home page of your application.
#[component]
pub fn LoginCallbackPage() -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query_map();

    let code = move || {
        query
            .with(|q| q.get("code").and_then(|code| code.parse::<String>().ok()))
            .unwrap()
    };

    let login = create_action(|input: &String| {
        let input = input.clone();
        async move { login(input).await }
    });
    let logged_in = login.value();

    create_effect(move |_| {
        let code = code();
        login.dispatch(code);
    });

    create_effect(move |_| {
        let logged_in = logged_in();

        if let Some(Ok(true)) = logged_in {
            navigate("/yay", Default::default());
        };
    });

    view! {
        <main>
            <Navbar/>
        </main>
    }
}
