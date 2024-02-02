use super::wrapper::check_server_cookie;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn ResourceProtectedRoute() -> impl IntoView {
    let refetcher = create_rw_signal(false);
    let auth_resource: Resource<bool, Result<bool, ServerFnError>> =
        create_local_resource(refetcher, move |_| async move {
            check_server_cookie("jabra-admin-portal-v2".to_string()).await
        });

    view! {
      <Suspense fallback=move || view! {<p>"Loading..."</p> }>
        {move || auth_resource.get().map(|data| {
          // once the data has loaded, we provide its value via context and
          // then render the nested route's outlet
          provide_context(data);
          view! {<Outlet/> }
        })}
      </Suspense>
    }
}
