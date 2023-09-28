use crate::components::{navbar::Navbar, stack_card::StackCard};
use crate::dto::technology::TechnologyDto;
use leptos::*;

#[server(GetStacks, "/api")]
pub async fn get_stacks() -> Result<Vec<TechnologyDto>, ServerFnError> {
    use crate::db::get_connection_pool;
    use crate::models::technology::Technology;
    let pool = get_connection_pool().await;

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
pub fn HomePage() -> impl IntoView {
    let technologies = create_resource(|| (), |_| async move { get_stacks().await });

    view! {
        <main>
            <Navbar/>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    technologies
                        .get()
                        .map(|response| match response {
                            Err(_) => view! { <p>"Error"</p> }.into_view(),
                            Ok(technologies) => {
                                technologies
                                    .into_iter()
                                    .map(move |_t| {
                                        view! { <StackCard/> }
                                    })
                                    .collect_view()
                            }
                        })
                }}

            </Suspense>
        </main>
    }
}
