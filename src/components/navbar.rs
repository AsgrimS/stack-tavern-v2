use leptos::*;

use crate::components::user_badge::UserBadge;
use crate::shared::dto::user::UserInfoDto;

#[server(GetStacks, "/api")]
pub async fn get_current_user() -> Result<Option<UserInfoDto>, ServerFnError> {
    let Some(user_info_dto) = use_context::<Option<UserInfoDto>>() else {
        return Ok(None);
    };

    Ok(user_info_dto)
}

#[component]
pub fn Navbar() -> impl IntoView {
    let current_user_resource = create_resource(|| (), |_| async move { get_current_user().await });

    view! {
        <div class="navbar bg-base-100">
            <div class="flex-1">
                <a href="/" class="text-xl normal-case btn btn-ghost">
                    "Stack Tavern"
                </a>
            </div>
            <div class="flex-none gap-2">
                <Suspense fallback=move || {
                    view! { <span class="loading loading-spinner loading-lg"></span> }
                }>
                    {move || {
                        current_user_resource
                            .get()
                            .map(|response| match response {
                                Err(_) => view! { <UserBadge current_user=None/> },
                                Ok(current_user) => view! { <UserBadge current_user/> },
                            })
                    }}

                </Suspense>
            </div>
        </div>
    }
}
