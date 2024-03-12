use crate::{
    components::page::Page,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Copy, Clone)]
pub struct Refetcher(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct HasError(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct CheckCookie(pub Resource<bool, Result<bool, ServerFnError>>);

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    _ = crate::providers::color_scheme::provide_color_scheme();
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let color_scheme = use_context::<crate::providers::color_scheme::ColorScheme>()
        .expect("Failed to find ColorScheme");
    let theme = move || match color_scheme.prefers_dark.get() {
        true => "darkpurple",
        false => "lightpurple",
    };

    let refetcher = create_rw_signal(false);
    let has_error = create_rw_signal(false);

    let auth_resource: Resource<bool, Result<bool, ServerFnError>> =
        create_local_resource(refetcher, move |_| async move {
            crate::components::wrapper::check_server_cookie("jabra-admin-portal-v2".to_string())
                .await
        });

    provide_context(Refetcher(refetcher));
    provide_context(HasError(has_error));
    provide_context(CheckCookie(auth_resource));

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/jabra-admin-portal-v2.css"/>

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
            <main class="font-poppins" data-theme = theme>
                <div class="min-h-screen">
                    <Routes>
                        <Route path="/" view=Page/>
                        <Route path="/login" view=Page/>
                        <Route path="/quote_builder" view=Page/>
                        <Route path="/active_quotes" view=Page/>
                        <Route path="/positions" view=Page/>
                        <Route path="/trade_history" view=Page/>
                        <Route path="/components" view=Page/>
                        <Route path="/perp-aggregator" view=Page/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
