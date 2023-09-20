use crate::components::{navbar::Navbar, stack_card::StackCard};
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <main>
            <Navbar/>
            <StackCard/>
        </main>
    }
}
