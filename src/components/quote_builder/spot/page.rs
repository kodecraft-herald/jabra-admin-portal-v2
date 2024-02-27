use leptos::*;

use crate::components::{common::{components::modals::{ErrorModal, SuccessModalRefetch}, functions::{helpers::{calculate_time_difference, get_expiry}, precision::{format_with_specs, RoundType}, utils::fetch_unified_configuration}, models::{
        common_models::{Currency, CurrencyPair, UnifiedCurrencyPairConfigurationResponse}, quote_model::QuotesTab, quote_option_model::{ApproveTradeQuoteRequest, ApproveTradeQuoteResponse}
    }}, quote_builder::spot::model::{sort, SpotQuote, SpotSort}};

use super::model::{ExtractedSpotQuote, SpotQuoteHistory};

#[allow(non_snake_case)]
#[component]
pub fn SpotGeneratorActivity() -> impl IntoView {
    let currency_pair = RwSignal::new(CurrencyPair::default());
    let config_resource = create_blocking_resource(|| (), move |_| fetch_unified_configuration());
    let configuration = RwSignal::new(UnifiedCurrencyPairConfigurationResponse::default());
    view! {
        <Suspense
            fallback = move || view! {
                <div class = "skeleton h-104 flex sm:basis-full lg:basis-auto border border-gray-800 rounded-md bg-base-300 justify-center items-center">
                    <span class="loading loading-bars loading-sm text-success"></span>
                </div>
            }
        >
            {
                config_resource.and_then(|conf| {
                    configuration.set(conf.clone());
                    view!{
                        <Spot config = configuration currency_pair = currency_pair/>
                    }.into_view()
                })
            }
        </Suspense>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Spot(
    config: RwSignal<UnifiedCurrencyPairConfigurationResponse>,
    currency_pair: RwSignal<CurrencyPair>,
) -> impl IntoView {
    let default_pair = move || {
        config
            .get()
            .data
            .first()
            .unwrap_or(&CurrencyPair::default())
            .clone()
    };

    let currency = RwSignal::new(Currency::default());

    let notional_amount = create_rw_signal(0.0);
    let price = create_rw_signal(1.0);

    let expiry_in_min = RwSignal::new(0 as u16);
    let gtc_signal = create_rw_signal(false);

    let counterparty = create_rw_signal("".to_string());

    let next_step = create_rw_signal(1);
    let show_steps = create_rw_signal(true);
    let has_filled = create_rw_signal(false);

    let trade_quotes = RwSignal::new(Vec::<SpotQuote>::default());

    let on_add_quote = move || {
        let expiry_date_time = get_expiry(expiry_in_min.get());
        let group_id = uuid::Uuid::new_v4().to_string();

        let jabra_quote = SpotQuote {
            group_id: group_id.clone(),
            pair: currency_pair.get(),
            amount: notional_amount.get(),
            price: price.get(),
            quote_expiry: expiry_date_time.clone(),
            counterparty: counterparty.get(),
            quote_status: "active".to_string(),
            gtc: gtc_signal.get(),
        };
        // log::info!("New Quote: {:?}", new_quote);
        trade_quotes.update(|v| v.push(jabra_quote));
    };

    let on_remove_qoute = move |group_id: String| {
        trade_quotes.update(|v: &mut Vec<SpotQuote>| {
            v.retain(|x| x.group_id != group_id);
        });
    };

    let on_clear_quote = move || {
        trade_quotes.set(Vec::<SpotQuote>::default());
    };

    let show_generated_quotes = move || trade_quotes.get().len() > 0;

    let reset_form = move || {
        currency_pair.set(CurrencyPair::default());
        notional_amount.set(0.0);
        price.set(1.0);
        expiry_in_min.set(0);
        counterparty.set("".to_string());
        next_step.set(1);
        has_filled.set(false);
    };

    create_effect(move |_| {
        log::info!("Step counter: {}", next_step.get());
        log::info!("Expiry in Min: {}", expiry_in_min.get())
    });

    view! {
        <div class = "flex flex-row gap-2 mt-2 sm:flex:wrap md:flex-nowrap">
            <div class = "flex-1 sm:basis-full lg:basis-1/5 lg:grow-0 rounded-md px-4 py-1 bg-base-300">
                <div class = "flex flex-col flex-initial">
                    <label class = "font-light text-sm">Pair</label>
                    <select class = "select-sm text-xs text-success block w-full rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="currency_pair"
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<u16>().unwrap_or_default();
                            let pair = config.get().get_currency_pair_by_id(val).unwrap_or_default();
                            currency_pair.set(pair.clone());
                            currency.set(pair.base.clone());

                            if has_filled.get() == false {
                                next_step.set(2)
                            } else {
                                next_step.update(|value| *value += 0)
                            }
                        }
                    >
                        <option prop:selected = move || currency_pair.get() == CurrencyPair::default() disabled>Select Pair</option>
                    {
                        move || {
                            // log::info!("Config Resource loaded");
                            currency.set(default_pair().base);
                            config.get().data.into_iter().map(|i| {
                                let i_clone = i.clone();
                                view!{
                                    <option prop:selected = move || currency_pair.get().name == i_clone.name value={i.id.to_string()}>{i.name}</option>
                                }
                            }).collect_view()
                        }
                    }
                    </select>

                    {
                        move || {
                            if next_step.get() < 2 {
                                view! {
                                    <div class = "grid gap-1 mt-1">
                                        <div class="skeleton w-16 h-4 shadow-md rounded-sm"></div>
                                        <div class="skeleton w-full h-8 shadow-md rounded"></div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class = "font-light text-sm">Amount (Notional)</label>
                                    <input class = "input-sm text-xs text-success block w-full rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" type="number" name="deposit_amount" prop:value = notional_amount prop:min = move || currency.get().order_size() prop:step = move || currency.get().tick_size()
                                        on:change = move |e| {
                                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                                            let precise_val = format_with_specs(val, currency.get().tick_size(), currency.get().order_size(), RoundType::Floor, false);
                                            notional_amount.set(precise_val);

                                            if has_filled.get() == false && next_step.get() <= 3 {
                                                next_step.set(3)
                                            } else {
                                                next_step.update(|value| *value += 0)
                                            }
                                        }
                                    />
                                }.into_view()
                            }

                        }
                    }

                    {
                        move || {
                            if next_step.get() < 3 {
                                view! {
                                    <div class = "grid gap-1 mt-1">
                                        <div class="skeleton w-16 h-4 shadow-md rounded-sm"></div>
                                        <div class="skeleton w-full h-8 shadow-md rounded"></div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class = "font-light text-sm">Price</label>
                                    <input class = "input-sm text-xs text-success block w-full rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" type="number" name="price" prop:value = price prop:min = move || currency.get().order_size() prop:step = move || currency.get().tick_size()
                                        on:change = move |e| {
                                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                                            let precise_val = format_with_specs(val, currency.get().tick_size(), currency.get().order_size(), RoundType::Floor, false);
                                            price.set(precise_val);

                                            if has_filled.get() == false && next_step.get() <= 4 {
                                                next_step.set(4)
                                            } else {
                                                next_step.update(|value| *value += 0)
                                            }
                                        }
                                    />
                                }.into_view()
                            }

                        }
                    }

                    {
                        move || {
                            if next_step.get() < 4 {
                                view! {
                                    <div class = "grid gap-1 mt-1">
                                        <div class="skeleton w-16 h-4 shadow-md rounded-sm"></div>
                                        <div class="skeleton w-full h-8 shadow-md rounded"></div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class = "font-light text-sm">Quote Expiry</label>
                                    <select class = "select-sm text-xs text-success block w-full rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="expiry_in_min"
                                        on:change = move |event| {
                                            let val: u16 = event_target_value(&event).parse().unwrap_or_default();
                                            let gtc = val == 1;

                                            expiry_in_min.set(val);
                                            gtc_signal.set(gtc);

                                            if has_filled.get() == false {
                                                next_step.set(5)
                                            } else {
                                                next_step.update(|value| *value += 0)
                                            }
                                        }
                                    >
                                        <option prop:selected = move || expiry_in_min.get() == 0 disabled>Select Quote Expiry</option>
                                        <option prop:selected = move || expiry_in_min.get() == 1 value = "1" >Good Till Canceled</option>
                                        <option prop:selected = move || expiry_in_min.get() == 10 value = "10" >10 mins.</option>
                                        <option prop:selected = move || expiry_in_min.get() == 20 value = "20" >20 mins.</option>
                                        <option prop:selected = move || expiry_in_min.get() == 30 value = "30" >30 mins.</option>
                                        <option prop:selected = move || expiry_in_min.get() == 40 value = "40" >40 mins.</option>
                                        <option prop:selected = move || expiry_in_min.get() == 50 value = "50" >50 mins.</option>
                                        <option prop:selected = move || expiry_in_min.get() == 60 value = "60" >60 mins.</option>
                                    </select>
                                }.into_view()
                            }

                        }
                    }


                    {
                        move || {
                            if next_step.get() < 5 {
                                view! {
                                    <div class = "grid gap-1 mt-1">
                                        <div class="skeleton w-16 h-4 shadow-md rounded-sm"></div>
                                        <div class="skeleton w-full h-8 shadow-md rounded"></div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <label class = "font-light text-sm">Counterparty</label>
                                    <select class = "select-sm text-xs text-success block w-full rounded hover:shadow-sm hover:shadow-success bg-base-100 shadow-md" name="counterparty"
                                        on:change = move |e| {
                                            let val: String = event_target_value(&e);
                                            counterparty.set(val);
                                            has_filled.set(true)
                                        }
                                    >
                                        <option prop:selected = move || counterparty.get() == "".to_string() disabled>Select Counterparty</option>
                                        <option prop:selected = move || counterparty.get() == "Bitbox".to_string() value="Bitbox".to_string()>Bitbox</option>
                                        <option prop:selected = move || counterparty.get() == "Valley Tree LLC".to_string() value="Valley Tree LLC".to_string()>Valley Tree LLC</option>
                                        <option prop:selected = move || counterparty.get() == "ZettaPOW I".to_string() value="ZettaPOW I".to_string()>ZettaPOW I</option>
                                        <option prop:selected = move || counterparty.get() == "Grayson Inc.".to_string() value="Grayson Inc.".to_string()>Grayson Inc.</option>
                                        <option prop:selected = move || counterparty.get() == "Meir".to_string() value="Meir".to_string()>Meir</option>
                                        <option prop:selected = move || counterparty.get() == "Ely".to_string() value="Ely".to_string()>Ely</option>
                                        <option prop:selected = move || counterparty.get() == "Deribit".to_string() value="Deribit".to_string()>Deribit</option>
                                    </select>
                                }.into_view()
                            }

                        }
                    }
                    <div class = "grid justify-items-end my-2">
                        <button prop:disabled = move || has_filled.get() == false class = "btn btn-sm btn-success" on:click = move |_| {on_add_quote(); reset_form();}>ADD</button>
                    </div>
                </div>
            </div>
                // {
                //     move || {
                //         if show_generated_quotes() == false {
                //             view! {
                //                 <div class = "skeleton flex-1 sm:basis-full lg:basis-auto border border-gray-800 rounded-md bg-base-300">
                //                 </div>
                //             }.into_view()
                //         } else {
                //             view! {
                //                 <div class = "flex-1 sm:basis-full lg:basis-auto border border-gray-800 rounded-md bg-base-300">
                //                     <div class = "flex flex-col">
                //                         <div class = "pb-7">
                //                             <table class = "table table-zebra table-xs">
                //                                 <thead class = "text-base text-success font-extralight bg-base-200">
                //                                     <tr class = "text-center border-b border-b-gray-800">
                //                                         <th colspan = "11" class = "text-success">GENERATED QUOTES</th>
                //                                     </tr>
                //                                     <tr class = "border-y border-y-gray-800">
                //                                         <th>PAIR</th>
                //                                         <th>AMOUNT</th>
                //                                         <th>PRICE</th>
                //                                         <th>QUOTE EXPIRY</th>
                //                                         <th>COUNTERPARTY</th>
                //                                     </tr>
                //                                 </thead>
                //                                 <tbody>
                //                                 {
                //                                     move || {
                //                                         trade_quotes.get().into_iter().map(|quote| {
                //                                         view! {
                //                                             <tr class = "hover">
                //                                                 <td>{quote.pair.name}</td>
                //                                                 <td>{quote.amount}</td>
                //                                                 <td>{quote.price}</td>
                //                                                 // <td>{format_to_specs(quote.price, currency.get().tick_size(), currency.get().order_size())}</td>
                //                                                 <td>{calculate_time_difference(None, quote.quote_expiry.clone(), quote.gtc)}</td>
                //                                                 <td>{quote.counterparty}</td>
                //                                                 <td>
                //                                                     <button class = "btn btn-square btn-xs btn-warning" on:click = move |_| on_remove_qoute(quote.group_id.clone())>
                //                                                         <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                //                                                             <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM6.75 9.25a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5z" clip-rule="evenodd" />
                //                                                         </svg>
                //                                                     </button>
                //                                                 </td>
                //                                             </tr>
                //                                         }
                //                                         }).collect_view()
                //                                     }
                //                                 }
                //                                 </tbody>
                //                             </table>
                //                         </div>
                //                         <div class = "flex flex-row-reverse items-center mb-2 border-t border-t-gray-800 p-4">
                //                             <div>
                //                                 <button class = "btn btn-sm btn-info" on:click = move |_| on_clear_quote()>
                //                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                //                                         <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd" />
                //                                     </svg>
                //                                     CLEAR
                //                                 </button>
                //                             </div>
                //                             <div>
                //                                 <button class = "btn btn-sm btn-success mr-4">SUBMIT</button>
                //                             </div>
                //                         </div>
                //                     </div>
                //                 </div>
                //             }.into_view()
                //         }
                //     }
                // }
        </div>

        // STEPS

        <div>
            <div class = "overflow-auto rounded-md mt-2 bg-base-300">
                <div class = ("hidden", move || show_steps.get() == false) class = "grid grid-flow-col justify-stretch">
                    <ul class="steps">
                        <li class="step step-neutral" class=("step-primary", move || next_step.get() >= 1)>Pair</li>
                        <li class="step step-neutral" class=("step-primary", move || next_step.get() >= 2)>Amount</li>
                        <li class="step step-neutral" class=("step-primary", move || next_step.get() >= 3)>Price</li>
                        <li class="step step-neutral" class=("step-primary", move || next_step.get() >= 4)>Quote Expiry</li>
                        <li class="step step-neutral" class=("step-primary", move || next_step.get() >= 5)>Counterparty</li>
                    </ul>
                </div>
            </div>
        </div>

        // QUOTES
        // <Quotes/>

    }
}

// #[allow(non_snake_case)]
// #[component]
// pub fn Quotes() -> impl IntoView {
//     let quotes_resource = create_resource(|| (), move |_| get_quotes());

//     let quote_table = RwSignal::new(Vec::<ExtractedSpotQuote>::default());
//     let selected_page = RwSignal::new(QuotesTab::Active);
//     let select_page = move |page: QuotesTab| {
//         selected_page.set(page);
//     };

//     let show_approve_quote_alert = create_rw_signal(false);
//     let approve_quote_response = create_rw_signal(ApproveTradeQuoteResponse::default());

//     let approve_trade_quotes: Action<(String, Vec<String>), ()> =
//         create_action(move |(status, group_ids): &(String, Vec<String>)| {
//             let stat = if status.clone() == "approved" {
//                 "approval".to_string()
//             } else {
//                 "rejection".to_string()
//             };
//             let request = ApproveTradeQuoteRequest::new(group_ids.clone(), status.clone());
//             log::info!("request-atc: {:?}", request.deserialize());
//             async move {
//                 let result = approve_trade_quote(request).await;
//                 match result {
//                     Ok(res) => {
//                         if res {
//                             show_approve_quote_alert.set(true);
//                             approve_quote_response.update(|v| {
//                                 v.success = true;
//                                 v.message = format!("Trade quote {} is successful", stat)
//                             });
//                         } else {
//                             show_approve_quote_alert.set(true);
//                             approve_quote_response.update(|v| {
//                                 v.success = false;
//                                 v.message = "Failed request, Please try again!.".to_string()
//                             });
//                         }
//                     }
//                     Err(e) => {
//                         log::info!("error>: {:?}", e);
//                         show_approve_quote_alert.set(true);
//                         approve_quote_response.update(|v| {
//                             v.success = false;
//                             v.message =
//                                 "Your session has ended. Please relog your account.".to_string()
//                         });
//                     }
//                 }
//             }
//         });

//     view! {
//         <div class = "text-xl font-semibold text-success py-5">
//             <span>[Quotes]</span>
//         </div>
//         <div class = "flex justify-between">
//             <div class = "flex flex-0 justify-start gap-4">
//                 <div class = "flex-0">
//                     <button class = {move || if selected_page.get() == QuotesTab::Active {"btn btn-sm btn-ghost capitalize font-semibold bg-base-100 rounded border border-success text-success"} else {"btn btn-sm btn-ghost capitalize font-normal bg-base-100 rounded"}} on:click = move |_| select_page(QuotesTab::Active)>Active</button>
//                 </div>
//                 <div class = "flex-0">
//                     <button class = {move || if selected_page.get() == QuotesTab::Approved {"btn btn-sm btn-ghost capitalize font-semibold bg-base-100 rounded border border-success text-success"} else {"btn btn-sm btn-ghost capitalize font-normal bg-base-100 rounded"}} on:click = move |_| select_page(QuotesTab::Approved)>Approved</button>
//                 </div>
//                 <div class = "flex-0">
//                     <button class = {move || if selected_page.get() == QuotesTab::Rejected {"btn btn-sm btn-ghost capitalize font-semibold bg-base-100 rounded border border-success text-success"} else {"btn btn-sm btn-ghost capitalize font-normal bg-base-100 rounded"}} on:click = move |_| select_page(QuotesTab::Rejected)>Rejected</button>
//                 </div>
//             </div>
//         </div>

//         <Suspense
//             fallback = move || view! {
//                     <div class = "items-center mt-5">
//                         <div class = " flex justify-center">
//                         <span class="loading loading-bars loading-sm text-success"></span>
//                         </div>
//                     </div>
//                 }
//         >
//         {
//             move || {
//                 if let Some(data) = quotes_resource.and_then(|e| {e.clone()}) {
//                     match data {
//                         Ok(t) => {
//                             let quotes_list = create_rw_signal(t.data.clone());

//                             match selected_page.get() {
//                                 QuotesTab::Active => {
//                                     quote_table.set(t.extract("active".to_string()));
//                                     view! {
//                                         <ActiveQuotes
//                                             quote_data = quote_table
//                                             show_approve_quote_alert = show_approve_quote_alert
//                                             approve_quote_response = approve_quote_response
//                                             approve_trade_quotes = approve_trade_quotes
//                                             quotes_list = quotes_list
//                                             quotes_resource = quotes_resource
//                                          />
//                                     }
//                                 },
//                                 QuotesTab::Approved => {
//                                     quote_table.set(t.extract("approved".to_string()));
//                                     view! {
//                                         <ApprovedQuotes
//                                             quote_data = quote_table
//                                             show_approve_quote_alert = show_approve_quote_alert
//                                             approve_quote_response = approve_quote_response
//                                             approve_trade_quotes = approve_trade_quotes
//                                             quotes_list = quotes_list
//                                             quotes_resource = quotes_resource
//                                          />
//                                     }
//                                 },
//                                 QuotesTab::Rejected => {
//                                     quote_table.set(t.extract("rejected".to_string()));
//                                     view! {
//                                         <RejectedQuotes
//                                             quote_data = quote_table
//                                             show_approve_quote_alert = show_approve_quote_alert
//                                             approve_quote_response = approve_quote_response
//                                             approve_trade_quotes = approve_trade_quotes
//                                             quotes_list = quotes_list
//                                             quotes_resource = quotes_resource
//                                         />
//                                     }
//                                 }
//                             }
//                         }
//                         Err(_) => view! {
//                             <div class = "py-5">
//                                 <span class = "font-extralight">Please Login</span>
//                             </div>
//                         }.into_view(),
//                     }
//                 } else {
//                     view! {
//                         <div class = "py-5">
//                             <span class = "font-extralight">No Quotes Record Found</span>
//                         </div>
//                     }.into_view()
//                 }
//             }
//         }
//         </Suspense>
//     }
// }

// #[allow(non_snake_case)]
// #[component]
// pub fn ActiveQuotes(
//     quote_data: RwSignal<Vec<ExtractedSpotQuote>>,
//     show_approve_quote_alert: RwSignal<bool>,
//     approve_quote_response: RwSignal<ApproveTradeQuoteResponse>,
//     approve_trade_quotes: Action<(String, Vec<String>), ()>,
//     quotes_list: RwSignal<Vec<SpotQuote>>,
//     quotes_resource: Resource<(), Result<SpotQuoteHistory, ServerFnError>>,
// ) -> impl IntoView {
//     move || {
//         if quote_data.get().len() > 0 {
//             view! {
//                 <div class = "rounded-lg">
//                     <DataTable
//                         data = quote_data
//                         show_approve_quote_alert = show_approve_quote_alert
//                         approve_quote_response = approve_quote_response
//                         approve_trade_quotes = approve_trade_quotes
//                         quotes_list = quotes_list
//                         quotes_resource = quotes_resource
//                     />
//                 </div>
//             }
//         } else {
//             view! {
//                 <div class = "py-5">
//                     <span class = "font-extralight">No active quote record found</span>
//                 </div>
//             }
//         }
//     }
// }

// #[allow(non_snake_case)]
// #[component]
// pub fn ApprovedQuotes(
//     quote_data: RwSignal<Vec<ExtractedSpotQuote>>,
//     show_approve_quote_alert: RwSignal<bool>,
//     approve_quote_response: RwSignal<ApproveTradeQuoteResponse>,
//     approve_trade_quotes: Action<(String, Vec<String>), ()>,
//     quotes_list: RwSignal<Vec<SpotQuote>>,
//     quotes_resource: Resource<(), Result<SpotQuoteHistory, ServerFnError>>,
// ) -> impl IntoView {
//     move || {
//         if quote_data.get().len() > 0 {
//             view! {
//                 <div class = "rounded-lg">
//                     <DataTable
//                         data = quote_data
//                         show_approve_quote_alert = show_approve_quote_alert
//                         approve_quote_response = approve_quote_response
//                         approve_trade_quotes = approve_trade_quotes
//                         quotes_list = quotes_list
//                         quotes_resource = quotes_resource
//                     />
//                 </div>
//             }
//         } else {
//             view! {
//                 <div class = "py-5">
//                     <span class = "font-extralight">No approved quote record found</span>
//                 </div>
//             }
//         }
//     }
// }

// #[allow(non_snake_case)]
// #[component]
// pub fn RejectedQuotes(
//     quote_data: RwSignal<Vec<ExtractedSpotQuote>>,
//     show_approve_quote_alert: RwSignal<bool>,
//     approve_quote_response: RwSignal<ApproveTradeQuoteResponse>,
//     approve_trade_quotes: Action<(String, Vec<String>), ()>,
//     quotes_list: RwSignal<Vec<SpotQuote>>,
//     quotes_resource: Resource<(), Result<SpotQuoteHistory, ServerFnError>>,
// ) -> impl IntoView {
//     move || {
//         if quote_data.get().len() > 0 {
//             view! {
//                 <div class = "rounded-lg">
//                     <DataTable
//                         data = quote_data
//                         show_approve_quote_alert = show_approve_quote_alert
//                         approve_quote_response = approve_quote_response
//                         approve_trade_quotes = approve_trade_quotes
//                         quotes_list = quotes_list
//                         quotes_resource = quotes_resource
//                     />
//                 </div>
//             }
//         } else {
//             view! {
//                 <div class = "py-5">
//                     <span class = "font-extralight">No rejected quote record found</span>
//                 </div>
//             }
//         }
//     }
// }

// #[allow(non_snake_case)]
// #[component]
// pub fn DataTable(
//     data: RwSignal<Vec<ExtractedSpotQuote>>,
//     show_approve_quote_alert: RwSignal<bool>,
//     approve_quote_response: RwSignal<ApproveTradeQuoteResponse>,
//     approve_trade_quotes: Action<(String, Vec<String>), ()>,
//     quotes_list: RwSignal<Vec<SpotQuote>>,
//     quotes_resource: Resource<(), Result<SpotQuoteHistory, ServerFnError>>,
// ) -> impl IntoView {
//     let row_splice = RwSignal::new(10);
//     let selected_header = RwSignal::new(SpotSort::GroupId);
//     let sort_asc = RwSignal::new(true);
//     let table_size = move || data.get().len();
//     let page_slice = move || {
//         if table_size() % row_splice.get() != 0 {
//             table_size() / row_splice.get() + 1
//         } else {
//             table_size() / row_splice.get()
//         }
//     };
//     let selected_page = RwSignal::new(1);
//     let selected_data = move || {
//         let mut end_index = selected_page() * row_splice.get();
//         let start_index = end_index - row_splice.get();
//         log::info!("start_index: {}, end_index: {}", start_index, end_index);
//         if end_index > table_size() {
//             end_index = table_size();
//         }
//         sort(
//             data.get()[start_index..end_index].to_vec(),
//             sort_asc.get(),
//             selected_header.get(),
//         )
//     };

//     // Signals for modal
//     let (show_success_modal, set_show_success_modal) = create_signal(false);
//     let (show_error_modal, set_show_error_modal) = create_signal(false);
//     let (show_confirm_modal_approve, set_show_confirm_modal_approve) = create_signal(false);
//     let (show_confirm_modal_reject, set_show_confirm_modal_reject) = create_signal(false);
//     let (show_confirm_modal_approve_all, set_show_confirm_modal_approve_all) = create_signal(false);
//     let (show_confirm_modal_reject_all, set_show_confirm_modal_reject_all) = create_signal(false);

//     // APPROVE/REJECT QUOTE
//     let dispatch_approve_quote = move |status: String, gi: String| {
//         approve_trade_quotes.dispatch((status.clone(), vec![gi.clone()]));
//     };

//     // APPROVE/REJECT ALL ACTIVE QUOTES
//     let approve_reject_all = move |status: String| {
//         let gi = quotes_list()
//             .into_iter()
//             .map(|qo| qo.group_id)
//             .collect::<Vec<String>>();
//         // log::info!("group_ids: {:?}", group_ids);
//         approve_trade_quotes.dispatch((status, gi.clone()));

//         //remove ids from trade_quotes_list
//         quotes_list.update(|v| {
//             v.retain(|x| !gi.contains(&x.group_id));
//         });
//     };

//     // AFTER CLOSING SUCCESS MODAL, CALL THIS FUNCTION TO REFETCH THE RESOURCE AND SET EVERYTHING TO DEFAULT
//     let refetch_resource = move || {
//         approve_quote_response.set(ApproveTradeQuoteResponse::default());
//         show_approve_quote_alert.set(false);
//         set_show_success_modal.set(false);
//         set_show_error_modal.set(false);
//         quotes_resource.refetch();
//     };

//     // DOWNLOAD ACTIVE QUOTES TABLE
//     // let download_active_uri_resource = create_resource(|| (), move |_| {
//     //     async move {
//     //         download_quotes_table(quotes_list.get_untracked()[0].counterparty_id.id.to_string(), "active".to_string()).await
//     //     }
//     // });

//     // TO CHECK IF THE ACTION IS PENDING, IF IT ISN'T THEN SET THE MODAL TO FALSE AND RESET ACTION VALUE TO NONE
//     let is_pending = approve_trade_quotes.pending();

//     create_effect(move |_| {
//         let action_value = approve_trade_quotes.value();

//         if let Some(_action) = action_value() {
//             set_show_confirm_modal_approve.set(false);
//             set_show_confirm_modal_reject.set(false);
//             action_value.set(None);
//         }
//     });

//     view! {
//         <div>
//             <div class = "flex justify-end gap-1 m-2">
//                 <button class = "btn btn-xs btn-warning" on:click = move |_| set_show_confirm_modal_reject_all(true)>REJECT ALL</button>
//                 <button class = "btn btn-xs btn-success" on:click = move |_| set_show_confirm_modal_approve_all(true)>APPROVE ALL</button>
//             </div>
//             <table class = "table table-sm table-zebra">
//                 <thead>
//                     <tr class = "text-sm uppercase bg-base-300 text-success">
//                         <th class = "cursor-pointer" on:click = move |_| {selected_header.set(SpotSort::Pair); sort_asc.update(|s| *s = !*s)}>
//                             <div class = "flex justify-between">
//                                 <span class = "flex-0">Pair</span>
//                                 <span  class = "flex-0">
//                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
//                                     <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
//                                     </svg>
//                                 </span>
//                             </div>
//                         </th>
//                         <th class = "cursor-pointer" on:click = move |_| {selected_header.set(SpotSort::Amount); sort_asc.update(|s| *s = !*s)}>
//                             <div class = "flex justify-between">
//                                 <span class = "flex-0">Amount</span>
//                                 <span  class = "flex-0">
//                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
//                                     <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
//                                     </svg>
//                                 </span>
//                             </div>
//                         </th>
//                         <th class = "cursor-pointer" on:click = move |_| {selected_header.set(SpotSort::Price); sort_asc.update(|s| *s = !*s)}>
//                             <div class = "flex justify-between">
//                                 <span class = "flex-0">Price</span>
//                                 <span  class = "flex-0">
//                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
//                                     <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
//                                     </svg>
//                                 </span>
//                             </div>
//                         </th>
//                         <th class = "cursor-pointer" on:click = move |_| {selected_header.set(SpotSort::QuoteExpiry); sort_asc.update(|s| *s = !*s)}>
//                             <div class = "flex justify-between">
//                                 <span class = "flex-0">Quote Expiry</span>
//                                 <span  class = "flex-0">
//                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
//                                     <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
//                                     </svg>
//                                 </span>
//                             </div>
//                         </th>
//                         <th class = "cursor-pointer" on:click = move |_| {selected_header.set(SpotSort::Counterparty); sort_asc.update(|s| *s = !*s)}>
//                             <div class = "flex justify-between">
//                                 <span class = "flex-0">Counterparty</span>
//                                 <span  class = "flex-0">
//                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
//                                     <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
//                                     </svg>
//                                 </span>
//                             </div>
//                         </th>
//                         <th>
//                             <span>Action</span>
//                         </th>
//                     </tr>
//                 </thead>
//                 <tbody>
//                     {
//                         move || {
//                             selected_data().into_iter().map(|d| {
//                                 let group_id_signal = create_rw_signal(d.group_id.clone());
//                                 view! {
//                                     <tr>
//                                         <td>{d.pair}</td>
//                                         <td>{d.amount}</td>
//                                         <td>{d.price}</td>
//                                         <td>{calculate_time_difference(None, d.quote_expiry.clone(), d.gtc)}</td>
//                                         <td>{d.counterparty}</td>
//                                         <td>
//                                             <div class = "flex justify-start gap-1 m-2">
//                                                 <button class = "btn btn-xs btn-warning" on:click = move |_| set_show_confirm_modal_reject.set(true)>REJECT</button>
//                                                 <button class = "btn btn-xs btn-success" on:click = move |_| set_show_confirm_modal_approve.set(true)>APPROVE</button>
//                                             </div>
//                                         </td>
//                                     </tr>

//                                     {
//                                         move || {

//                                             view! {
//                                                 <Show when=move || show_confirm_modal_approve.get() fallback=|| ()>
//                                                     <div class="modal-cust-top blur-bg">
//                                                         <div class="modal-box">
//                                                             <h3 class="font-bold text-2xl">APPROVE?</h3>
//                                                             <p class="py-4">Are you sure you want to approve?</p>
//                                                             <div class="modal-action">
//                                                                 <button class="btn btn-error btn-sm rounded" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_confirm_modal_approve.set(false)>CANCEL</button>
//                                                                 {
//                                                                     match is_pending() {
//                                                                         true => view! {
//                                                                             <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
//                                                                         }.into_any(),
//                                                                         false => view! {
//                                                                             <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = move |_| dispatch_approve_quote("approved".to_string(), group_id_signal.get())>CONFIRM</button>
//                                                                         }.into_any(),
//                                                                     }
//                                                                 }

//                                                             </div>
//                                                         </div>
//                                                     </div>
//                                                 </Show>
//                                             }
//                                         }
//                                     }

//                                     {
//                                         move || {

//                                             view! {
//                                                 <Show when=move || show_confirm_modal_reject.get() fallback=|| ()>
//                                                     <div class="modal-cust-top blur-bg">
//                                                         <div class="modal-box">
//                                                             <h3 class="font-bold text-2xl">REJECT?</h3>
//                                                             <p class="py-4">Are you sure you want to reject?</p>
//                                                             <div class="modal-action">
//                                                                 <button class="btn btn-error btn-sm rounded" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_confirm_modal_reject.set(false)>CANCEL</button>
//                                                                 {
//                                                                     match is_pending() {
//                                                                         true => view! {
//                                                                             <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
//                                                                         }.into_any(),
//                                                                         false => view! {
//                                                                             <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = move |_| dispatch_approve_quote("rejected".to_string(), group_id_signal.get())>CONFIRM</button>
//                                                                         }.into_any(),
//                                                                     }
//                                                                 }

//                                                             </div>
//                                                         </div>
//                                                     </div>
//                                                 </Show>
//                                             }
//                                         }
//                                     }


//                                 }
//                             }).collect_view()
//                         }
//                     }
//                 </tbody>
//                 <tfoot>
//                 <tr>
//                     <td colspan = "15">
//                         <div class = "flex justify-between">
//                             <Transition fallback = move || view! {<span class="loading loading-bars loading-sm"></span>}>
//                                 {
//                                     move || {
//                                         // download_active_uri_resource.and_then(|url|{
//                                         //     let uri = url.clone();
//                                             view! {
//                                                 <a class = "btn btn-sm btn-ghost bg-base-100 capitalize font-normal" /*href = uri download*/>
//                                                     <span>
//                                                         <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
//                                                             <path fill-rule="evenodd" d="M4.5 2A1.5 1.5 0 003 3.5v13A1.5 1.5 0 004.5 18h11a1.5 1.5 0 001.5-1.5V7.621a1.5 1.5 0 00-.44-1.06l-4.12-4.122A1.5 1.5 0 0011.378 2H4.5zm4.75 6.75a.75.75 0 011.5 0v2.546l.943-1.048a.75.75 0 011.114 1.004l-2.25 2.5a.75.75 0 01-1.114 0l-2.25-2.5a.75.75 0 111.114-1.004l.943 1.048V8.75z" clip-rule="evenodd" />
//                                                         </svg>
//                                                     </span>
//                                                 Download PDF
//                                                 </a>
//                                             }
//                                         // }).collect_view()
//                                     }
//                                 }
//                             </Transition>
//                             <div class="join flex justify-center">
//                                 {
//                                     move || {
//                                         (1..page_slice()+1).into_iter().map(|i| {
//                                             view! {
//                                                 <button class = {move || if selected_page.get() == i {"join-item btn btn-xs bg-base-content bg-opacity-10 m-0.5"} else {"join-item btn btn-xs m-0.5"}} on:click = move |_| selected_page.set(i) >{i}</button>
//                                             }
//                                         }).collect_view()
//                                     }
//                                 }
//                             </div>

//                             <div></div>

//                         </div>
//                     </td>
//                 </tr>
//                 </tfoot>
//             </table>
//         </div>


//         {
//             move || {

//                 view! {
//                     <Show when=move || show_confirm_modal_reject_all.get() fallback=|| ()>
//                         <div class="modal-cust-top blur-bg">
//                             <div class="modal-box">
//                                 <h3 class="font-bold text-2xl">REJECT ALL?</h3>
//                                 <p class="py-4">Are you sure you want to reject all?</p>
//                                 <div class="modal-action">
//                                     <button class="btn btn-error btn-sm rounded" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_confirm_modal_reject_all(false)>Cancel</button>
//                                     {
//                                         match is_pending() {
//                                             true => view! {
//                                                 <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
//                                             }.into_any(),
//                                             false => view! {
//                                                 <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = move |_| approve_reject_all("rejected".to_string())>Confirm</button>
//                                             }.into_any(),
//                                         }
//                                     }
//                                 </div>
//                             </div>
//                         </div>
//                     </Show>
//                 }
//             }
//         }

//         {
//             move || {

//                 view! {
//                     <Show when=move || show_confirm_modal_approve_all.get() fallback=|| ()>
//                         <div class="modal-cust-top blur-bg">
//                             <div class="modal-box">
//                                 <h3 class="font-bold text-2xl">APPROVE ALL?</h3>
//                                 <p class="py-4">Are you sure you want to approve all?</p>
//                                 <div class="modal-action">
//                                     <button class="btn btn-error btn-sm rounded" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_confirm_modal_approve_all(false)>Cancel</button>
//                                     {
//                                         match is_pending() {
//                                             true => view! {
//                                                 <button class="btn btn-success btn-sm rounded" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
//                                             }.into_any(),
//                                             false => view! {
//                                                 <button class="btn btn-success btn-sm rounded" title="Confirm" on:click = move |_| approve_reject_all("approved".to_string())>Confirm</button>
//                                             }.into_any(),
//                                         }
//                                     }
//                                 </div>
//                             </div>
//                         </div>
//                     </Show>
//                 }
//             }
//         }

//         // SHOW ERROR MODAL / SUCCESS MODAL AFTER APPROVE / REJECT

//         {
//             move || match show_approve_quote_alert() {
//                 true => if !approve_quote_response().success  {
//                     set_show_error_modal.set(true);
//                     view! {
//                         <ErrorModal
//                             read_signal = show_error_modal
//                             write_signal = set_show_error_modal
//                             message = approve_quote_response().message
//                         />
//                     }.into_view()
//                 } else {
//                     set_show_success_modal.set(true);
//                     view! {
//                         <SuccessModalRefetch
//                             read_signal = show_success_modal
//                             message = approve_quote_response().message
//                             function = refetch_resource
//                         />
//                     }.into_view()
//                 }
//                 false => view! {<div></div>}.into_view(),
//             }
//         }

//     }
// }

// #[server]
// pub async fn get_quotes() -> Result<SpotQuoteHistory, ServerFnError> {
    // use crate::common::trade_model::Trade;
    // let cookie = crate::util::cookie::get_cookie_value("jabra-web-client").await;
    // let jwt_cookie = crate::util::cookie::JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    // // let access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjFhMzAzOWU0LTE4ZWItNGRjZC05ZTgzLWE1OTBmMzdhZWZlMiIsInJvbGUiOiI0MmI3YmJkOC05MTJhLTRhZTMtOGNmNy03Y2VkODU3NzNjOGYiLCJhcHBfYWNjZXNzIjp0cnVlLCJhZG1pbl9hY2Nlc3MiOnRydWUsImlhdCI6MTY5OTYyNDkxNSwiZXhwIjoxNjk5NzExMzE1LCJpc3MiOiJkaXJlY3R1cyJ9.sraZ3N-BkHb1NCgUPel3Iolq-rW8IhXGmX7V6m2fKJY";
    // let bearer_token = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    // let path = format!("{}/items/trade?filter[counterparty_id][id][_eq]={}&filter[expiry_timestamp][_gte]=$NOW&fields={}", url, jwt_cookie.counterparty_id, Trade::get_query());
    // let mut headers = http::HeaderMap::new();
    // headers.insert(
    //     "Authorization",
    //     http::HeaderValue::from_str(&bearer_token).unwrap(),
    // );

    // let response = crate::util::http_wrapper::call_and_parse::<crate::common::common_model::BlankRequest,crate::common::trade_model::TradeHistory>(
    //     Option::None,
    //     path,
    //     headers,
    //     crate::util::http_wrapper::HttpMethod::GET,
    // )
    // .await;
    // match response {
    //     Ok(res) => Ok(res),
    //     Err(e) => {
    //         log::error!("error: {:?}", e);
    //         Err(ServerFnError::ServerError(e.to_string()))
    //     }
    // }
//     Ok(SpotQuoteHistory::default())
// }

// Approve Quotes
// #[server(ApproveTradeQuotes, "/api", "Cbor")]
// pub async fn approve_trade_quote(
//     _request: ApproveTradeQuoteRequest,
// ) -> Result<bool, ServerFnError> {
    // log::info!("request: {:?}", request);
    // use crate::util::http_wrapper::{call, HttpMethod};

    // let cookie = crate::util::cookie::get_cookie_value("jabra-web-client").await;
    // let jwt_cookie = crate::util::cookie::JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    // let bearer_token = format!("Bearer {}", jwt_cookie.access_token);

    // log::debug!("request: {:?}", request.deserialize());
    // let url = std::env::var("DIRECTUSURL").unwrap();
    // let path = format!("{}/items/quotes_option", url);

    // let mut headers = http::HeaderMap::new();
    // headers.insert(
    //     "Authorization",
    //     http::HeaderValue::from_str(&bearer_token).unwrap(),
    // );

    // let response =
    //     call::<ApproveTradeQuoteRequest>(Some(request), path, headers, HttpMethod::PATCH).await;
    // match response {
    //     Ok(res) => Ok(res),
    //     Err(e) => {
    //         log::info!("error-: {:?}", e);
    //         Err(ServerFnError::ServerError(e.to_string()))
    //     }
    // }
//     Ok(true)
// }
