use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::home::HomePage;
use crate::pages::login_callback::LoginCallbackPage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/stack-tavern-v2.css"/>

        // DaisyUI
        <Stylesheet href="https://cdn.jsdelivr.net/npm/daisyui@3.9.4/dist/full.css"/>
        <Script src="https://cdn.tailwindcss.com"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Html attr:data-theme="night"/>
                <Body class="min-h-screen"/>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/login/callback" view=LoginCallbackPage/>
                </Routes>
            </main>
        </Router>
    }
}
