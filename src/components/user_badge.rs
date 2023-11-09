use leptos::*;
use leptos_icons::{Icon, RiIcon};

use crate::shared::dto::user::UserInfoDto;
use crate::shared::functions::public::get_login_url;

#[server(Logout, "/api/public")]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::shared::consts::ACCESS_TOKEN_COOKIE;
    use axum::http::header;
    use cookie::Cookie;
    use leptos_axum::ResponseOptions;

    let response = expect_context::<ResponseOptions>();

    let cookie = Cookie::build((ACCESS_TOKEN_COOKIE, ""))
        .path("/")
        .secure(true)
        .http_only(true)
        .expires(time::OffsetDateTime::now_utc())
        .to_string();

    response.insert_header(
        header::SET_COOKIE,
        header::HeaderValue::from_str(cookie.as_str()).unwrap(),
    );

    Ok(())
}

#[component]
pub fn UserBadge(current_user: Option<UserInfoDto>) -> impl IntoView {
    view! {
        {match current_user {
            Some(user_info) => {
                view! {
                    <div class="flex items-center">
                        <p class="mr-1">{user_info.username}</p>
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
                                    <a on:click=move |_| {
                                        spawn_local(async {
                                            logout().await.unwrap();
                                            window().location().set_href("/").unwrap();
                                        });
                                    }>
                                        Logout
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                }
                    .into_view()
            }
            None => {
                view! {
                    <div class="dropdown dropdown-end">
                        <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                            <div class="w-10 rounded-full">
                                <Icon
                                    icon=Icon::from(RiIcon::RiAccountCircleUserFacesLine)
                                    width="2.5rem"
                                    height="2.5rem"
                                />
                            </div>
                        </label>
                        <ul
                            tabindex="0"
                            class="p-2 mt-3 w-52 shadow z-[1] menu menu-sm dropdown-content bg-base-100 rounded-box"
                        >
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
                }
                    .into_view()
            }
        }}
    }
}
