use leptos::*;
use leptos_router::ActionForm;

use crate::app::{HasError, Refetcher};

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="h-full lg:grid lg:grid-cols-3">
            <div class = "h-full flex items-center justify-center px-4">
                <div class="card flex-shrink-0 w-full max-w-sm shadow-lg bg-light">
                    <div class="card-body">
                        <LoginIsland>
                                <div class="form-control">
                                    <label for = "userid" class="label">
                                    <span class="label-text">Email</span>
                                    </label>
                                    <input type="text" id = "userid" name = "userid" class="input input-sm bg-white rounded hover:shadow-md text-black border-gray-800 shadow-md" autocomplete required
                                    />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                    <span for = "password" class="label-text">Password</span>
                                    </label>
                                    <input type="password" id = "password" name = "password" class="input input-sm bg-white rounded hover:shadow-md text-black border-gray-800 shadow-md" autocomplete required
                                    />
                                    <label class="label">
                                    <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
                                    </label>
                                </div>
                                // <div class="form-control mt-6">
                                //     <ButtonWithState/>
                                //     // <button type="submit" class="btn rounded btn-block btn-success">LOGIN</button>
                                // </div>
                        </LoginIsland>
                    </div>
                </div>
            </div>
            <div class = "h-full lg:flex items-center hidden lg:block lg:col-span-2">
                <div class = "flex flex-col">
                    <div class="flex items-center">
                        <h1 class="text-6xl text-white">
                        "Bespoke Structured Products"
                        <br/>
                        "for your"
                        <span class="font-bold">" digital assets"</span>
                        </h1>
                    </div>
                    <p class="mt-4">A tailored solution for your investment thesis.</p>
                </div>
            </div>
        </div>
    }
}

#[island]
pub fn LoginIsland(children: Children) -> impl IntoView {
    let login_action: Action<DirectusLogin, Result<bool, ServerFnError>> =
        create_server_action::<DirectusLogin>();
    let is_pending = login_action.pending();

    create_effect(move |_| {
        log::info!("Is_pending: {:?}", is_pending());

        let value = login_action.value();

        if let Some(data) = value.get() {
            match data {
                Ok(bool) => {
                    if bool {
                        use_context::<Refetcher>().unwrap().0.set(true);
                        use_context::<HasError>().unwrap().0.set(false);
                    } else {
                        use_context::<Refetcher>().unwrap().0.set(false);
                        use_context::<HasError>().unwrap().0.set(true);
                    }
                }
                Err(_) => {
                    use_context::<Refetcher>().unwrap().0.set(false);
                    use_context::<HasError>().unwrap().0.set(true);
                }
            }
        }
    });

    view! {
        <ActionForm action = login_action>
            {children()}
            {
                move || match is_pending() {
                    true => view! {
                        <div class="form-control mt-6">
                            <button type="submit" class="btn btn-block btn-success"><span class="loading loading-spinner loading-sm"></span></button>
                        </div>
                    }.into_any(),
                    false => view! {
                        <div class="form-control mt-6">
                            <button type="submit" class="btn rounded btn-block btn-success">LOGIN</button>
                        </div>
                    }.into_any(),
                }
            }
        </ActionForm>
    }
}

// #[island]
// pub fn ButtonWithState() -> impl IntoView {
//     let is_pending = expect_context::<IsPending>().0;
//     create_effect(move |_| {
//         log::info!("Is_pending_button: {}", is_pending());
//     });
//     view! {
//         {
//             move || match is_pending() {
//                 true => view! {
//                     <div class="form-control mt-6">
//                         <button type="submit" class="btn btn-block btn-success"><span class="loading loading-spinner loading-sm"></span></button>
//                     </div>
//                 }.into_any(),
//                 false => view! {
//                     <div class="form-control mt-6">
//                         <button type="submit" class="btn rounded btn-block btn-success">LOGIN</button>
//                     </div>
//                 }.into_any(),
//             }
//         }
//     }
// }

#[server(DirectusLogin, "/api")]
pub async fn directus_login(userid: String, password: String) -> Result<bool, ServerFnError> {
    use super::models::{DirectusLoginRequest, DirectusLoginResponse};
    use super::wrapper::{self, call_and_parse, HttpMethod, JabraCookie};
    use crate::errors::JabraError;

    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/auth/login", url);
    let email = userid.clone();
    let login_request = DirectusLoginRequest::new(userid.into(), password.into());
    let response = call_and_parse::<DirectusLoginRequest, DirectusLoginResponse>(
        Some(login_request),
        path,
        reqwest::header::HeaderMap::new(),
        HttpMethod::POST,
    )
    .await;

    match response {
        Ok(res) => {
            // Calculate expiration time in millis, subract 2 minute to be safe
            // Why 10 minutes? There are other api resource that are automatically when users navigate to a certain page
            // Only those API calls in action will have the refresh token
            // Which means during the manual submit, the refresh token is used
            // 10 minutes will act as a buffer for those action

            let expiration_time =
                chrono::Utc::now().timestamp_millis() + res.data.expires - 600_000;
            // log::info!("Expiration Time: {}", expiration_time);
            let jabra_cookie = JabraCookie::new(
                email,
                res.data.access_token,
                res.data.refresh_token,
                expiration_time,
            );
            wrapper::set_jabra_cookie(Some(jabra_cookie), "jabra-admin-portal-v2".to_string())
                .await;
            leptos_axum::redirect("/quote_builder");
            Ok(true)
        }
        Err(e) => {
            log::info!("Login Error: {}", e.to_string());
            Err(ServerFnError::ServerError(
                JabraError::LoginError.to_string(),
            ))
        }
    }
}
