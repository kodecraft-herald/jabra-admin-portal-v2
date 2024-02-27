use leptos::*;

use crate::components::quote_builder::{future::page::FuturesGeneratorActivity, option::page::OptionsGeneratorActivity, spot::page::SpotGeneratorActivity};

#[allow(non_snake_case)]
#[component]
pub fn QuoteBuilder() -> impl IntoView {
    let quote_type = create_rw_signal("Spot".to_string());
    view! {
        <div class="p-4">
            <div class = "flex justify-between bg-base-300 p-4 rounded-xl">
                <div class = "flex-1 text-xl font-semibold text-success">
                    <span>[QUOTE BUILDER]</span>
                </div>
                <div class = "flex flex-1 justify-end">
                    <div class = "join flex-0 border-gray-800 bg-base-100 shadow-md">
                        <button class = "join-item btn pointer-events-none btn-sm rounded-l-lg text-opacity-70">SELECT TYPE</button>
                        <select class = "join-item select-sm text-xs text-success hover:shadow-sm hover:shadow-success rounded-r-lg bg-base-100" 
                            on:change = move |event| {
                            let val: String = event_target_value(&event);
                            quote_type.set(val);
                        }>
                            <option value = "Spot">Spot</option>
                            <option value = "Option">Option</option>
                            <option value = "Future">Future</option>
                            <option value = "Perpetual Future">Perpetual Future</option>
                        </select>
                    </div>
                </div>
            </div>

            <div>
                <QuoteBuilderPageManager quote_type = quote_type/>
            </div>

            <div class="pt-5">
                <crate::components::common::components::generated_quotes::GeneratedQuotes/>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn QuoteBuilderPageManager(
    quote_type: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div>
            {
                move || {
                    match quote_type.get().as_str() {
                        "Spot" => view!{<SpotGeneratorActivity/>},
                        "Future" => view!{<FuturesGeneratorActivity/>},
                        "Option" => view!{<OptionsGeneratorActivity/>},
                        _ => view! {
                            <div class = "skeleton h-104 flex sm:basis-full lg:basis-auto border border-gray-800 rounded-md bg-base-300 justify-center items-center">
                                <span class = "opacity-50 text-success">Page not available!</span>
                            </div>
                        }.into_view(),
                    }
                }
            }
        </div>
    }
}