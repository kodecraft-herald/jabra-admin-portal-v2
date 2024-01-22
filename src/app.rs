use crate::{
    components::page::Page,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/islands-arch-test.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main class="font-poppins">
                <div class="min-h-full">
                    <Routes>
                        <Route path="/" view=Page/>
                        <Route path="/quote_builder" view=Page/>
                        <Route path="/active_quotes" view=Page/>
                        <Route path="/positions" view=Page/>
                        <Route path="/trade_history" view=Page/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
