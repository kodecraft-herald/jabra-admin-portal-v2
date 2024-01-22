use leptos::*;
use leptos_router::use_location;

use crate::components::{
    content::{Content1, Content2},
    sidebar::Sidebar,
};

#[component]
pub fn Page() -> impl IntoView {
    let location = use_location().pathname;
    
    view! {
        <div class = "main-content gap-2 flex">
            <div class = "flex-none basis-1/12 md:basis-1/6 bg-base-200 rounded-xl min-h-full">
                <Sidebar />
            </div>
            <div class = "flex-1 bg-base-200 rounded-xl min-h-full">
                {
                    move || {
                        match location.get().as_str() {
                            "/" => view!{<Content1 />},
                            "/quote_builder" => view!{<Content1 />},
                            "/active_quotes" => view!{<Content1 />},
                            "/positions" => view!{<Content2 />},
                            "/trade_history" => view!{<Content2 />},

                            _ => view!{<div></div>}.into_view(),
                        }
                    }
                }
            </div>
        </div>
    }
}
