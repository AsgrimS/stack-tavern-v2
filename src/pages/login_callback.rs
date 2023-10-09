use crate::components::navbar::Navbar;
use leptos::*;
use leptos_router::*;
use log::debug;

#[server(Login, "/api")]
pub async fn login(code: String) -> Result<Option<String>, ServerFnError> {
    use crate::auth::get_token;

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

    view! {
        <main>
            <Navbar/>
            {move || match token.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(token) => {
                    let Ok(token) = token else { return view! { <p>"Error"</p> }.into_view() };
                    let Some(token) = token else { return view! { <Redirect path="/"/> }.into_view()
                };
                    debug!("{:?}", token);
                    view! { <Redirect path="/success"/> }.into_view()
                }
            }}

        </main>
    }
}
