use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn Content1() -> impl IntoView {
    view! {
        <div class="p-4">
            <div class = "text-xl font-bold pb-5 ml-2">
                <span>Content 1</span>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Content2() -> impl IntoView {
    view! {
        <div class="p-4">
            <div class = "text-xl font-bold pb-5 ml-2">
                <span>Content 2</span>
            </div>
        </div>
    }
}
