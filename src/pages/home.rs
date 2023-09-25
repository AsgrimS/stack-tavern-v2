use crate::components::{navbar::Navbar, stack_card::StackCard};
use leptos::*;

#[server(GetStacks, "/api")]
pub async fn get_stacks() -> Result<(), ServerFnError> {
    use crate::db::get_connection_pool;
    use crate::models::technology::Technology;
    let pool = get_connection_pool().await;

    match sqlx::query_as!(Technology, "SELECT * FROM technologies")
        .fetch_all(pool)
        .await
    {
        Ok(_row) => {
            println!("{:?}", _row);
            Ok(())
        }
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // let technologies = create_resource(cx, || (), |_| async move { get_stacks().await });

    view! { cx,
        <main>
            <Navbar/>
            <button
                class="btn"
                on:click=move |_| {
                    spawn_local(async {
                        get_stacks().await;
                    });
                }
            >

                "Add Todo"
            </button>
            <StackCard/>
        </main>
    }
}
