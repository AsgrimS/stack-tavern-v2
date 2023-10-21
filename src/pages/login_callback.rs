use crate::components::navbar::Navbar;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

const ACCESS_TOKEN_STORAGE_KEY: &str = "access_token";

#[server(Login, "/api")]
pub async fn login(code: String) -> Result<Option<String>, ServerFnError> {
    use crate::api::auth::get_token;

    let token = get_token(code).await;

    Ok(token)
}

#[derive(Params, PartialEq, Clone)]
struct LoginCallbackQueryParams {
    code: String,
}

/// Renders the home page of your application.
#[component]
pub fn LoginCallbackPage() -> impl IntoView {
    let navigate = use_navigate();
    let query = use_query::<LoginCallbackQueryParams>();
    let (code, set_code) = create_signal("".to_string());

    let token = create_resource(
        code,
        // every time `count` changes, this will run
        |code| async move { login(code).await },
    );

    query.with_untracked(|query| match query {
        Ok(query) => {
            set_code(query.code.clone());
        }
        Err(_) => {
            navigate("/", Default::default());
        }
    });

    create_effect(move |_| {
        let token = token.get().unwrap();

        if let Ok(Some(token)) = token {
            LocalStorage::set(ACCESS_TOKEN_STORAGE_KEY, token).expect("LocalStorage::set");
            navigate("/yay", Default::default());
        };
    });

    view! {
        <main>
            <Navbar/>
            {move || match token.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(token) => {
                    let Ok(token) = token else { return view! { <p>"Error"</p> }.into_view() };
                    let Some(_) = token else { return view! { <Redirect path="/"/> }.into_view() };
                    view! { <p>"Success!"</p> }.into_view()
                }
            }}

        </main>
    }
}
