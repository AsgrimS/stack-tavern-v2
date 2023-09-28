use leptos::*;

#[component]
pub fn StackCard() -> impl IntoView {
    view! {
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
