use leptos::*;

// #[server(GetStack, "/api")]
// pub async fn get_stack() -> Result<(), ServerFnError> {
//     use crate::db::get_connection_pool;
//
//     let pool = get_connection_pool().await;
//
//     let stack = sqlx::query!("SELECT * FROM stacks");
//
//     return ();
// }
#[server(AddTodo, "/api")]
pub async fn add_todo() -> Result<(), ServerFnError> {
    use crate::db::get_connection_pool;
    let pool = get_connection_pool().await;

    match sqlx::query!("INSERT INTO technologies (name, purpose) VALUES ('test', 'test')")
        .execute(pool)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn StackCard(cx: Scope) -> impl IntoView {
    view! { cx,
        <button
            class="btn"
            on:click=move |_| {
                spawn_local(async {
                    add_todo().await;
                })
            }
        >

            "Add Stack"
        </button>
        <div class="w-96 card bg-neutral text-neutral-content">
            <div class="card-body">
                <h2 class="card-title">
                    Card title!
                </h2>
                <p>
                    If a dog chews shoes whose shoes does he choose?
                </p>
                <div class="justify-end card-actions">
                    <div class="badge badge-outline">
                        Fashion
                    </div>
                    <div class="badge badge-outline">
                        Products
                    </div>
                </div>
            </div>
        </div>
    }
}
