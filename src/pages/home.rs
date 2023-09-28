use crate::components::{navbar::Navbar, stack_card::StackCard};
use crate::dto::technology::TechnologyDto;
use leptos::*;

#[server(GetStacks, "/api")]
pub async fn get_stacks() -> Result<Vec<TechnologyDto>, ServerFnError> {
    use crate::db::get_connection_pool;
    use crate::models::technology::Technology;
    let pool = get_connection_pool().await;
    std::thread::sleep(std::time::Duration::from_secs(1));

    match sqlx::query_as!(Technology, "SELECT * FROM technologies")
        .fetch_all(pool)
        .await
    {
        Ok(technologies) => Ok(technologies.into_iter().map(|t| t.into()).collect()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let technologies = create_resource(cx, || (), |_| async move { get_stacks().await });

    view! { cx,
        <main>
            <Navbar/>
            <Suspense fallback=move || {
                view! { cx, <p>"Loading..."</p> }
                }>{move || { technologies.read(cx).map(|response| match response{
                    Err(_) => { view! {cx, <p>"Error"</p>}.into_view(cx)},
                    Ok(technologies) => technologies.into_iter().map(move |t| {view! {cx, <StackCard/>}}).collect_view(cx),
                }) }}
            </Suspense>
        </main>
    }
}
