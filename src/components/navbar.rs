use leptos::*;

#[server(GetLoginUrl, "/api")]
pub async fn get_login_url() -> Result<String, ServerFnError> {
    use crate::auth::get_authorization_url;

    let url = get_authorization_url().await;
    Ok(url)
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
            <div class="flex-1">
                <a href="/" class="text-xl normal-case btn btn-ghost">
                    Stack Tavern
                </a>
            </div>
            <div class="flex-none gap-2">
                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                        <div class="w-10 rounded-full">
                            <img src="/images/leptos_circle.svg"/>
                        </div>
                    </label>
                    <ul
                        tabindex="0"
                        class="p-2 mt-3 w-52 shadow z-[1] menu menu-sm dropdown-content bg-base-100 rounded-box"
                    >
                        <li>
                            <a class="justify-between">
                                Profile
                            </a>
                        </li>
                        <li>
                            <a>
                                Settings
                            </a>
                        </li>
                        <li>
                            <a>
                                Logout
                            </a>
                        </li>
                        <li>
                            <a on:click=move |_| {
                                spawn_local(async {
                                    let url = get_login_url().await.unwrap();
                                    window().location().set_href(url.as_str()).unwrap();
                                });
                            }>
                                Login
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
