use leptos::*;
use web_sys::Event;

use crate::components::common::{functions::{helpers::{create_trade_expiry_in_utc, generate_instrument_name_v2, get_expiry, get_trade_expiration_datetime, get_trade_expiry_date, parse_local_datetime_to_str}, precision::{convert_to_decimal, format_with_specs, RoundType}, utils::{add_quote, coin_base_spot, fetch_unified_configuration, sb_counter_parties, sb_fetch_estimate_iv, sb_post_qoute_option}}, models::common_models::{AddQuoteResponse, ClosestOption, CoinBaseSpotPriceResponse, CounterPartyResponse, Currency, CurrencyPair, EstimateIVRequest, EstimateIVResponse, Quote, QuoteOptionRequest, QuoteOptionResponse, UnifiedCurrencyPairConfigurationResponse}};


const MINIMUM_PREMIUM_IN_USD: f64 = 10.0;

#[allow(non_snake_case)]
#[component]
pub fn OptionsGeneratorActivity() -> impl IntoView {
    let currency_pair = RwSignal::new(CurrencyPair::default());
    let config_resource = create_blocking_resource(|| (), move |_| fetch_unified_configuration());
    let spot_resource = create_blocking_resource(currency_pair, move |c| coin_base_spot(c));
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
                        <OptionsBuilderSpecs config = configuration spot_resource = spot_resource currency_pair = currency_pair/>
                    }.into_view()
                })
            }
        </Suspense>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn OptionsBuilderSpecs(
    config: RwSignal<UnifiedCurrencyPairConfigurationResponse>,
    spot_resource: Resource<CurrencyPair, Result<CoinBaseSpotPriceResponse, ServerFnError>>,
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

    let counter_parties = RwSignal::new(CounterPartyResponse::default());
    let currency = RwSignal::new(Currency::default());
    let deposit_amount = RwSignal::new(1.0);
    let option_kind = RwSignal::new(String::from("Call"));
    let spot = RwSignal::new(0.0);
    let ttm = RwSignal::new(6.85);
    let r2 = RwSignal::new(0.042);
    let off_strike = RwSignal::new(5.0);
    let strike = RwSignal::new(0.0);
    let auto_calculate_strike = RwSignal::new(true);
    let cubic_spline_iv = RwSignal::new(0.0);
    let closest_option = RwSignal::new(ClosestOption::default());
    let closest_instrument_name = move || closest_option.get().instrument_name;
    let iv = RwSignal::new(0.0001);
    let side = RwSignal::new(String::from("Buy"));
    let qoute_option_response = RwSignal::new(QuoteOptionResponse::default());
    let deal_btn_disabled = RwSignal::new(true);
    let trade_quotes = RwSignal::new(Vec::<Quote>::default());
    let counterparty = RwSignal::new("".to_string());
    let expiry_in_min = RwSignal::new(0 as u16);
    let show_trade_quote_alert = create_rw_signal(false);
    let add_quote_response = create_rw_signal(AddQuoteResponse::default());
    let (can_fetch_iv, set_can_fetch_iv) = create_signal(false);
    let trade_expiry = create_memo(move |_| {
        let exp = get_trade_expiration_datetime(None, ttm.get());
        let formatted_trade_expiry = parse_local_datetime_to_str(exp);
        formatted_trade_expiry
    });
    // let expiry_timestamp = RwSignal::new(trade_expiry.get_untracked());
    // Signals for modal
    let (show_confirm_modal, set_show_confirm_modal) = create_signal(false);
    let (show_success_modal, set_show_success_modal) = create_signal(true);
    let (show_error_modal, set_show_error_modal) = create_signal(true);

    let add_quote_action = create_action(move |trade_quote_request: &Vec<Quote>| {
        let req = trade_quote_request.clone();
        async move {
            let result = add_quote(req).await;
            match result {
                Ok(res) => {
                    if res {
                        show_trade_quote_alert.set(true);
                        add_quote_response.update(|v| {
                            v.success = true;
                            v.message = "Quote submitted successfully.".to_string()
                        });
                    } else {
                        show_trade_quote_alert.set(true);
                        add_quote_response.update(|v| {
                            v.success = false;
                            v.message = "Failed request, Please try again!.".to_string()
                        });
                    }
                }
                Err(e) => {
                    log::error!("error: {:?}", e);
                    show_trade_quote_alert.set(true);
                    add_quote_response.update(|v| {
                        v.success = false;
                        v.message = "Your session has ended. Please relog your account.".to_string()
                    });
                }
            }
        }
    });

    let counterparty_resource = Resource::once(move || sb_counter_parties());
    let calculated_strike = create_memo(move |_| {
        if auto_calculate_strike.get() {
            let off_strike = if option_kind.get() == "Put" {
                off_strike.get() * -1.0
            } else {
                off_strike.get()
            };
            let cs = ((off_strike / 100.00) * spot.get()) + spot.get();
            format_with_specs(
                cs,
                currency_pair.get().quote.tick_size(),
                currency_pair.get().quote.order_size(),
                RoundType::Floor,
                false
            )
        } else {
            strike.get()
        }
    });

    let calculated_base_currency_amount = create_memo(move |_| {
        if currency.get().id == currency_pair.get().base.id {
            deposit_amount.get()
        } else {
            deposit_amount.get() / spot.get()
        }
    });

    let estimate_iv_request = create_memo(move |_| EstimateIVRequest {
        option_kind: option_kind.get(),
        currency: currency_pair.get_untracked().base.ticker, // untracked currency pair to prevent double sending of request
        ttm: ttm.get(),
        strike: calculated_strike.get(),
    });
    create_effect(move |_| {
        log::info!("Estimate IV Request: {:?}", estimate_iv_request.get());
    });

    let estimate_iv_resource = create_resource_with_initial_value(
        estimate_iv_request,
        move |e| sb_fetch_estimate_iv(e),
        Some(Ok(EstimateIVResponse::default())),
    );

    let qoute_option_action: Action<QuoteOptionRequest, ()> =
        create_action(move |qoute_option_request: &QuoteOptionRequest| {
            let req: QuoteOptionRequest = qoute_option_request.clone();
            log::info!("Called Quote Option {:?}", req);
            async move {
                let result = sb_post_qoute_option(req).await;
                match result {
                    Ok(response) => {
                        log::info!("response: {:?}", response);
                        qoute_option_response.set(response);
                        deal_btn_disabled.set(false);
                    }
                    Err(e) => {
                        qoute_option_response.set(QuoteOptionResponse::default());
                        deal_btn_disabled.set(true);
                        log::error!("error: {:?}", e);
                    }
                }
            }
        });

    //Calculate PX values, using action, amount should always be the calculated base currency amount
    let dispatch_quote_option_action = move || {
        qoute_option_action.dispatch(QuoteOptionRequest {
            option_kind: option_kind.get(),
            amount: calculated_base_currency_amount.get(),
            strike: calculated_strike.get(),
            ttm: ttm.get(),
            spot: Some(spot.get()),
            r2: Some(r2.get()),
            r1: Some(0.0),
            iv: Some(iv.get()),
            side: side.get(),
        })
    };

    let calculated_px_base = move || {
        format_with_specs(
            qoute_option_response().data.px_in_base_ccy,
            currency_pair.get().base_tick_size(),
            currency_pair.get().base_tick_size(),
            RoundType::Floor,
            true
        )
    };
    let calculated_px_quote = move || {
        format_with_specs(
            qoute_option_response().data.px_in_quote_ccy,
            currency_pair.get().quote_tick_size(),
            currency_pair.get().quote_order_size(),
            RoundType::Floor,
            true
        )
    };

    let quote_premium_min = move |side: String| {
        match side.as_str() {
            "Buy" => MINIMUM_PREMIUM_IN_USD * -1.0,
            "Sell" => MINIMUM_PREMIUM_IN_USD,
            _ => 0.0,
        }
    };

    let base_premium_min = move |side: String| {
        let min = (convert_to_decimal(MINIMUM_PREMIUM_IN_USD) / convert_to_decimal(spot.get())).to_string().parse::<f64>().unwrap();
        let formatted_min = format_with_specs(
            min,
            currency_pair.get().base_tick_size(),
            currency_pair.get().base_tick_size(),
            RoundType::Default,
            false
        );
        log::info!("Min: {}", min);
        log::info!("Formatted Min: {}", formatted_min);
        match side.as_str() {
            "Buy" => formatted_min * -1.0,
            "Sell" => formatted_min,
            _ => 0.0,
        }
    };

    let delta = move || {
        let value = format!("{:.2}", qoute_option_response().data.greeks.delta);
        //Prevent from getting -0.00 value
        if value == "-0.00000" {
            "0.00000".to_string()
        } else {
            value
        }
    };
    let gamma = move || {
        let value = format!("{:.5}", qoute_option_response().data.greeks.gamma);
        //Prevent from getting -0.00 value
        if value == "-0.00000" {
            "0.00000".to_string()
        } else {
            value
        }
    };
    let theta = move || {
        let value = format!("{:.5}", qoute_option_response().data.greeks.theta);
        //Prevent from getting -0.00 value
        if value == "-0.00" {
            "0.00".to_string()
        } else {
            value
        }
    };

    //Trade Qoute
    let on_add_qoute = move || {
        let counterparty_id = counterparty.get().parse::<u16>().unwrap_or_default();
        let pair_id = currency_pair.get().id;
        let ccy_id = currency.get().id;
        let calculated_offstrike = if option_kind.get() == "Call" {
            off_strike.get() / 100.00
        } else {
            -off_strike.get() / 100.00
        };
        let party_a = counter_parties()
            .get_id_by_ticker("JABRA")
            .unwrap_or_default();
        let party_b = counterparty_id;
        let precise_mbca = format_with_specs(
            calculated_base_currency_amount(),
            currency_pair().base_tick_size(),
            currency_pair().base_order_size(),
            RoundType::Floor,
            false
        );
        let calculated_deposit_amount = if side.get() == "Buy" {
            precise_mbca
        } else {
            -precise_mbca
        };
        let trade_expiry_utc = trade_expiry.get();
        let expiry_date_time = get_expiry(expiry_in_min.get());
        let default_trade_time = counter_parties
            .get()
            .get_default_expiry_by_id(counterparty.get().parse::<u16>().unwrap_or_default())
            .unwrap_or(String::from("10:00:00"));
        let trade_expiry =
            create_trade_expiry_in_utc(trade_expiry_utc.clone(), default_trade_time.clone());
        let group_id = uuid::Uuid::new_v4().to_string();
        let jabra_quote = Quote {
            temp_id: uuid::Uuid::new_v4().to_string(),
            counterparty_id: counter_parties()
                .get_id_by_ticker("JABRA")
                .unwrap_or_default(),
            pair_id,
            ccy_id,
            amount: calculated_deposit_amount,
            option_kind: option_kind.get(),
            ttm: ttm.get(),
            r2: r2.get(),
            r1: 0.0,
            offstrike_percentage: calculated_offstrike,
            spot: spot.get(),
            // strike: strike(),
            strike: calculated_strike(),
            iv: iv.get(),
            px_in_base_ccy: calculated_px_base(),
            px_in_quote_ccy: calculated_px_quote(),
            side: side.get(),
            quote_status: "active".to_string(),
            quote_origin: "JabraAdminGUI".to_string(),
            instrument_name: generate_instrument_name_v2(
                currency_pair.get().base.ticker,
                trade_expiry_utc.clone(),
                calculated_strike(),
                option_kind.get(),
            ),
            quote_expiry: expiry_date_time.clone(),
            gtc: true,
            group_id: group_id.clone(),
            party_a,
            party_b,
            delta: delta().parse::<f64>().unwrap_or_default(),
            gamma: gamma().parse::<f64>().unwrap_or_default(),
            theta: theta().parse::<f64>().unwrap_or_default(),
            payout_ccy: String::from("base"),
            // expiry_timestamp: trade_expiry_utc.clone(),
            expiry_timestamp: trade_expiry.clone(),
        };
        // log::info!("New Quote: {:?}", new_quote);
        trade_quotes.update(|v| v.push(jabra_quote));

        let flip_side = if side.get() == "Buy" { "Sell" } else { "Buy" };
        //This quote is for CounterParty
        let trader_quote = Quote {
            temp_id: uuid::Uuid::new_v4().to_string(),
            counterparty_id: counterparty_id,
            pair_id: pair_id,
            ccy_id: ccy_id,
            amount: calculated_deposit_amount * -1.0, //flip amount
            option_kind: option_kind.get(),
            ttm: ttm.get(),
            r2: r2.get(),
            r1: 0.0,
            offstrike_percentage: calculated_offstrike,
            spot: spot.get(),
            // strike: strike(),
            strike: calculated_strike(),
            iv: iv.get(),
            px_in_base_ccy: calculated_px_base() * -1.0,
            px_in_quote_ccy: calculated_px_quote() * -1.0,
            side: flip_side.to_string(),
            quote_status: "active".to_string(),
            quote_origin: "JabraAdminGUI".to_string(),
            instrument_name: generate_instrument_name_v2(
                currency_pair.get().base.ticker,
                trade_expiry_utc.clone(),
                // strike(),
                calculated_strike(),
                option_kind.get(),
            ),
            quote_expiry: expiry_date_time.clone(),
            gtc: true,
            group_id: group_id.clone(),
            party_a: party_b, //flip Counter Party
            party_b: party_a,
            delta: -delta().parse::<f64>().unwrap_or_default(),
            gamma: -gamma().parse::<f64>().unwrap_or_default(),
            theta: -theta().parse::<f64>().unwrap_or_default(),
            payout_ccy: String::from("base"),
            // expiry_timestamp: trade_expiry_utc.clone(),
            expiry_timestamp: trade_expiry.clone(),
        };

        trade_quotes.update(|v| v.push(trader_quote));
        qoute_option_response.set(QuoteOptionResponse::default());
        deal_btn_disabled.set(true);
        spot_resource.refetch();
        log::info!("Trade Quotes: {:?}", trade_quotes());
    };

    let trade_quote_dispatch = move || {
        add_quote_action.dispatch(trade_quotes.get());

        //Reset Trade Quotes
        trade_quotes.set(Vec::<Quote>::default());
    };

    let on_remove_qoute = move |group_id: String| {
        trade_quotes.update(|v| {
            v.retain(|x| x.group_id != group_id);
        });
    };

    //Derived Signals
    let on_clear_quote = move || {
        trade_quotes.set(Vec::<Quote>::default());
    };
    let quote_total_base_currency_price = move || {
        let jabra_id = counter_parties()
            .get_id_by_ticker("JABRA")
            .unwrap_or_default();
        let bt = trade_quotes
            .get()
            .iter()
            .filter(|x| x.counterparty_id == jabra_id)
            .fold(0.0, |acc, x| acc + x.px_in_base_ccy);
        format_with_specs(
            bt,
            currency_pair().base_tick_size(),
            currency_pair().base_tick_size(),
            RoundType::Floor,
            true
        )
    };
    let quote_total_quote_currency_price = move || {
        let jabra_id = counter_parties()
            .get_id_by_ticker("JABRA")
            .unwrap_or_default();
        let qt = trade_quotes
            .get()
            .iter()
            .filter(|x| x.counterparty_id == jabra_id)
            .fold(0.0, |acc, x| acc + x.px_in_quote_ccy);
        format_with_specs(
            qt,
            currency_pair().quote_tick_size(),
            currency_pair().quote_tick_size(),
            RoundType::Floor,
            true
        )
    };

    //Every Trade Quote counterparty_id will be updated duting change in this
    let on_change_counterparty = move |event: Event| {
        let val: String = event_target_value(&event);
        let default_trade_time = counter_parties
            .get()
            .get_default_expiry_by_id(val.parse::<u16>().unwrap_or_default())
            .unwrap_or(String::from("10:00:00"));
        // let default_trade_expiry = get_traders_preffered_expiry(expiry_timestamp.get().as_str(), default_trade_time.as_str());
        let use_val = val.clone().parse::<u16>().unwrap_or_default();
        counterparty.set(val);
        //Set all the counter party id of the trade quotes
        let mut cloned_trade_quotes = trade_quotes.get().clone();
        let jabra_id = counter_parties()
            .get_id_by_ticker("JABRA")
            .unwrap_or_default();
        for cloned_trade_quote in &mut cloned_trade_quotes {
            let saved_trade_expiry = cloned_trade_quote.expiry_timestamp.clone();
            let trade_expiry =
                create_trade_expiry_in_utc(saved_trade_expiry, default_trade_time.clone());
            if cloned_trade_quote.counterparty_id != jabra_id {
                cloned_trade_quote.counterparty_id = use_val;
            }
            if cloned_trade_quote.party_a != jabra_id {
                cloned_trade_quote.party_a = use_val;
            }
            if cloned_trade_quote.party_b != jabra_id {
                cloned_trade_quote.party_b = use_val;
            }
            cloned_trade_quote.expiry_timestamp = trade_expiry;
        }
        trade_quotes.set(cloned_trade_quotes);
        log::info!("Trade Quotes After change CP: {:?}", trade_quotes());
        // expiry_timestamp.set(default_trade_expiry);
    };

    let on_changed_expiry = move |event: Event| {
        let val: u16 = event_target_value(&event).parse().unwrap_or_default();
        // let use_val = val.clone();
        expiry_in_min.set(val);
        // let mem = memo_in_min_expiry.get();
        let gtc = expiry_in_min.get() == 0;

        let date_time = get_expiry(val);
        let mut cloned_trade_quotes = trade_quotes.get().clone();
        for cloned_trade_quote in &mut cloned_trade_quotes {
            cloned_trade_quote.quote_expiry = date_time.clone();
            cloned_trade_quote.gtc = gtc;
        }
        trade_quotes.set(cloned_trade_quotes);
    };

    let on_changed_payout_ccy = move |group_id: String, payout_ccy: String| {
        let mut cloned_trade_quotes = trade_quotes.get().clone();
        for cloned_trade_quote in &mut cloned_trade_quotes {
            if cloned_trade_quote.group_id == group_id {
                cloned_trade_quote.payout_ccy = payout_ccy.clone();
            }
        }
        trade_quotes.set(cloned_trade_quotes);
    };

    // let on_change_trade_expiry = move |expiry: String| {
    //     expiry_timestamp.set(expiry.clone());
    //     let mut cloned_trade_quotes = trade_quotes.get().clone();
    //     for cloned_trade_quote in &mut cloned_trade_quotes {
    //         cloned_trade_quote.expiry_timestamp = parse_str_to_utc_datetime_str(expiry.as_str());
    //     }
    //     trade_quotes.set(cloned_trade_quotes);
    // };

    //Show Flag for Quote Builder
    let show_builder = move || trade_quotes.get().len() > 0;

    view! {
        <div class = "flex flex-row gap-2 mt-2 sm:flex:wrap px924:flex-nowrap">
            <div class = "flex-1 grow-0 border border-gray-800 rounded-md px-4 py-1 bg-base-300">
                <div class = "flex flex-col">
                    <label class = "block font-light text-sm">Select Pair</label>
                    <select class = "select-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" name="currency_pair"
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<u16>().unwrap_or_default();
                            let pair = config.get().get_currency_pair_by_id(val).unwrap_or_default();
                            currency_pair.set(pair.clone());
                            currency.set(pair.base.clone());
                        }
                    >
                    {
                        move || {
                            // log::info!("Config Resource loaded");
                            currency_pair.set(default_pair());
                            currency.set(default_pair().base);
                            config.get().data.into_iter().map(|i| {
                                view!{
                                    <option value={i.id.to_string()}>{i.name}</option>
                                }
                            }).collect_view()
                        }
                    }
                    </select>

                    <label class = "block font-light text-sm">Select Currency</label>
                    <select class = "select-sm text-xs  text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" name="currency"
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<u16>().unwrap_or_default();
                            let curr = currency_pair().get_currency_by_id(val).unwrap_or_default();
                            currency.set(curr);
                        }
                    >
                        {
                            move || {
                                // log::info!("Currency Pair loaded");
                                view! {
                                    <>
                                        <option value={currency_pair().base.id.to_string()}>{currency_pair().base.ticker}</option>
                                        <option value={currency_pair().quote.id.to_string()}>{currency_pair().quote.ticker}</option>
                                    </>
                                }.into_view()
                            }
                        }
                    </select>

                    <label class = "block font-light text-sm">Input Amount</label>
                    <input class = "input-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" type="number" name="deposit_amount" prop:value = deposit_amount prop:min = move || currency.get().order_size() prop:step = move || currency.get().tick_size()
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            let precise_val = format_with_specs(val, currency.get().tick_size(), currency.get().order_size(), RoundType::Floor, false);
                            deposit_amount.set(precise_val);
                        }
                    />

                    <label class = "block font-light text-sm">Call or Put</label>
                    <select class = "select-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" name="option_kind"
                        on:change = move |e| {
                            let val = event_target_value(&e);
                            option_kind.set(val);
                        }
                    >
                        <option value="Call">Call</option>
                        <option value="Put">Put</option>
                    </select>

                    <label class = "block font-light text-sm">Time to Maturity (days)</label>
                    <input class = "input-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" type="number" name="ttm" prop:value = ttm prop:min = "0.01" prop:step = "0.01"
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            ttm.set(val);
                        }
                    />

                    <label class = "block font-light text-sm">Risk Free Interest Rate</label>
                    <input class = "input-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" type="number" name="r2" value = r2 prop:min = "0.001" prop:step = "0.001"
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            r2.set(val);
                        }
                    />

                    <label class = "block font-light text-sm">Spot Price</label>
                    <input class = "input-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" type="number" name="spot" prop:value = spot prop:min = move || currency_pair.get().quote_order_size() prop:step = move || currency_pair.get().quote_tick_size()
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            let precise_val = format_with_specs(val, currency_pair.get().quote_tick_size(), currency_pair.get().quote_order_size(), RoundType::Floor, false);
                            spot.set(precise_val);
                        }
                    />
                    <Transition>
                        {
                            move || {
                                spot_resource.and_then(|s| {
                                    // log::info!("Spot Resource loaded");
                                    let val = s.data.amount.parse::<f64>().unwrap_or_default();
                                    let precise_val = format_with_specs(val, currency_pair.get().quote.tick_size(), currency_pair.get().quote.order_size(), RoundType::Floor, false);
                                    spot.set(precise_val);
                                    set_can_fetch_iv(true);
                                })
                            }
                        }
                    </Transition>
                    <div class = "flex flex-grow justify-between my-2  font-extralight">
                        <label class = "block  font-light text-sm">Percentage Off Strike</label>
                        <span class="indicator-item badge badge-success badge-sm">{move || off_strike()}</span>
                    </div>

                    <input class = "range range-success range-xs" type="range" name="off_strike" prop:step = "1.0" prop:value = off_strike prop:min = 0.0 prop:max = 15.0
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            off_strike.set(val);
                        }
                    />
                    <div class = "flex flex-grow justify-between font-light text-sm">
                        <span>"1.0"</span>
                        <span>"15.0"</span>
                    </div>

                    <label class = "block font-light text-sm">Calculated Strike</label>
                    <input class = "input-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 mb-2 border-gray-800 shadow-md" type="number" name="strike" prop:value = calculated_strike prop:disabled = auto_calculate_strike prop:min = move || currency.get().order_size() prop:step = move || currency.get().tick_size()
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            let precise_val = format_with_specs(val, currency_pair.get().quote.tick_size(), currency_pair.get().quote.order_size(), RoundType::Floor, false);
                            strike.set(precise_val);
                        }
                    />

                    <div class = "flex flex-grow">
                        <input class="checkbox checkbox-success checkbox-xs mr-2" type="checkbox" name="auto_calculate_strike" prop:checked = auto_calculate_strike
                            on:click = move |_| {
                                auto_calculate_strike.update(|v| *v = !*v);
                            }
                        />
                        <span class = "font-light text-sm">Auto Calculate Strike</span>
                    </div>
                </div>
            </div>
            <div class = "flex-1 flex-col cb-layout border-gray-800 rounded-md bg-base-300">
                    <div class="flex stats shadow justify-center bg-inherit">
                        <div class="flex flex-wrap">
                            <div class="stat">
                                <div class="stat-figure text-success">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                                </div>
                                <div class="stat-title">Cubic Spline IV</div>
                                <div class="stat-value text-success text-2xl">{cubic_spline_iv}</div>
                            </div>
                        </div>
                    </div>
                    <div>
                        <table class = "table table-zebra table-xs overflow-auto">
                            <thead class = "text-sm text-success border-y bg-base-200 border-gray-800">
                                <th>INSTRUMENT NAME</th>
                                <th>MARK IV</th>
                                <th>BID IV</th>
                                <th>ASK IV</th>
                            </thead>
                            <tbody>
                            <Show when = move || can_fetch_iv() >
                                <Suspense
                                fallback = move || view! {
                                    <div class = "skeleton flex w-96 h-52 border border-gray-800 rounded-md bg-base-300 items-center justify-center">
                                        <span class="loading loading-bars loading-sm text-success"></span>
                                    </div>
                                }
                                >
                                    {
                                        move || {
                                            qoute_option_response.set(QuoteOptionResponse::default());
                                            estimate_iv_resource.and_then(|i| {
                                                cubic_spline_iv.set(i.data.estimated_iv.clone());
                                                let cs: ClosestOption = i.data.closest_options.first().unwrap().clone();
                                                closest_option.set(cs);
                                                i.clone().data.closest_options.into_iter().map(|option| {
                                                    let mut first_row_style = "hover";
                                                    let mut highest = true;
                                                    if option.instrument_name == closest_instrument_name() {
                                                        first_row_style = "hover text-info font-bold";
                                                        highest = false;
                                                    }
                                                    view!{
                                                        <tr class = {first_row_style}>
                                                            <td>
                                                            <Show
                                                                when = move || !highest
                                                                fallback = move || view! {<span></span>}>
                                                                <span class = "inline-block mr-4">
                                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941" />
                                                                    </svg>
                                                                </span>
                                                            </Show>
                                                            {option.instrument_name.clone()}
                                                            </td>
                                                            <td>{option.mark_iv}</td>
                                                            <td>{option.bid_iv}</td>
                                                            <td>{option.ask_iv}</td>
                                                        </tr>
                                                    }
                                                }).collect_view()
                                            })
                                        }
                                    }
                                </Suspense>
                            </Show>
                            </tbody>
                        </table>
                    </div>
                    <div class = "flex flex-row justify-end gap-2 py-2 border-y border-y-gray-800 mx-1">
                        <div class = "join flex-1 basis-1/5 lg:grow-0 mr-4">
                            <button class = "join-item btn btn-outline pointer-events-none btn-sm bg-base-100 border-gray-800">ENTER IV</button>
                            <input class = "join-item input-sm text-xs text-success rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" required name = "iv" type="number" prop:value = iv prop:min = move || currency_pair.get().base_order_size() prop:step = move || currency_pair.get().base_tick_size()
                                on:change = move |e| {
                                    let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                                    iv.set(val);
                                }
                            />
                        </div>
                        <select class = "flex-1 basis-1/6 lg:grow-0 select-sm block rounded hover:shadow-sm hover:shadow-success bg-base-100 text-xs text-success border-gray-800 shadow-md" name="side"
                            on:change = move |e| {
                                let val = event_target_value(&e);
                                side.set(val);
                            }
                        >
                            <option value="Buy">Buy</option>
                            <option value="Sell">Sell</option>
                        </select>
                        <button class="flex-1 basis-1/6 lg:grow-0 btn btn-success btn-sm rounded" on:click = move |_| dispatch_quote_option_action()>CALCULATE PRICE</button>
                    </div>

                    <div class = "flex flex-col border-gray-800 rounded-md bg-base-300">
                        <div class = "my-3 text-center">
                            <span class = "text-base font-semibold text-success">OPTION PRICER</span>
                        </div>

                        <form on:submit=move |ev| {ev.prevent_default(); on_add_qoute()}>
                            <div class="flex justify-center">
                                <div class="flex flex-col gap-2 p-4"> 
                                    <div>
                                        <div class="stat-title text-xs xl:text-sm">{move || format!("Price ({})", currency_pair.get().base.ticker)}</div>
                                        <div class="stat-value">
                                            {
                                                move || match side.get().to_uppercase().as_str() {
                                                    "BUY" => view! {
                                                        <input class = "width-inherit text-md input-sm rounded bg-base-100 text-success border-gray-800 shadow-md hover:shadow-sm hover:shadow-info" type = "number" prop:value = move || calculated_px_base()
                                                            prop:max = move || base_premium_min(side.get()) prop:step = move || currency_pair.get().base.tick_size()
                                                            on:change = move |event| {
                                                                let val: f64 = event_target_value(&event).parse().unwrap();
                                                                qoute_option_response.update(|v| v.data.px_in_base_ccy = val);
                                                            }
                                                        />
                                                    }.into_view(),
                                                    "SELL" => view! {
                                                        <input class = "width-inherit text-md input-sm rounded bg-base-100 text-success border-gray-800 shadow-md hover:shadow-sm hover:shadow-info" type = "number" prop:value = move || calculated_px_base()
                                                            prop:min = move || base_premium_min(side.get()) prop:step = move || currency_pair.get().base.tick_size()
                                                            on:change = move |event| {
                                                                let val: f64 = event_target_value(&event).parse().unwrap();
                                                                qoute_option_response.update(|v| v.data.px_in_base_ccy = val);
                                                            }
                                                        />
                                                    }.into_view(),
                                                    _ => view! {
                                                        <div></div>
                                                    }.into_view()
                                                }
                                            }
                                        </div>
                                    </div>
                                    <div>
                                        <div class="stat-title text-xs xl:text-sm">{move || format!("Price ({})", currency_pair.get().quote.ticker)}</div>
                                        <div class="stat-value">
                                            {
                                                move || match side.get().to_uppercase().as_str() {
                                                    "BUY" => view! {
                                                        <input class = "text-md input-sm rounded bg-base-100 text-success border-gray-800 shadow-md hover:shadow-sm hover:shadow-info" required type = "number" prop:value = move || calculated_px_quote()
                                                            prop:max = move || quote_premium_min(side.get()) prop:step = move || currency_pair.get().quote.tick_size()
                                                            on:change = move |event| {
                                                                let val: f64 = event_target_value(&event).parse().unwrap();
                                                                qoute_option_response.update(|v| v.data.px_in_quote_ccy = val);
                                                            }
                                                        />
                                                    }.into_view(),
                                                    "SELL" => view! {
                                                        <input class = "text-md input-sm rounded bg-base-100 text-success border-gray-800 shadow-md hover:shadow-sm hover:shadow-info" required type = "number" prop:value = move || calculated_px_quote()
                                                            prop:min = move || quote_premium_min(side.get()) prop:step = move || currency_pair.get().quote.tick_size()
                                                            on:change = move |event| {
                                                                let val: f64 = event_target_value(&event).parse().unwrap();
                                                                qoute_option_response.update(|v| v.data.px_in_quote_ccy = val);
                                                            }
                                                        />
                                                    }.into_view(),
                                                    _ => view! {
                                                        <div></div>
                                                    }.into_view()
                                                }
                                            }
                                        </div>
                                    </div>
                                    // Button For Mobile View
                                    <div class="flex justify-start py-4">
                                        <button type="submit" class="btn btn-success btn-sm rounded" prop:disabled=move || deal_btn_disabled.get() required>
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm.75-11.25a.75.75 0 00-1.5 0v2.5h-2.5a.75.75 0 000 1.5h2.5v2.5a.75.75 0 001.5 0v-2.5h2.5a.75.75 0 000-1.5h-2.5v-2.5z" clip-rule="evenodd" />
                                            </svg>
                                            Add Quote
                                        </button>
                                    </div>
                                </div>
                                
                                <div class="flex flex-col gap-2 p-4">
                                    <div>
                                        <div class="stat-title text-xs xl:text-sm pb-2">Delta</div>
                                        <input class = "text-md input-sm rounded bg-base-100 border-gray-800 shadow-md" disabled type = "text" prop:value = move || delta()/>
                                    </div>
                                    <div>
                                        <div class="stat-title text-xs xl:text-sm pb-2">Gamma</div>
                                        <input class = "text-md input-sm rounded bg-base-100 border-gray-800 shadow-md" disabled type = "text" prop:value = move || gamma()/>
                                    </div>
                                    <div>
                                        <div class="stat-title text-xs xl:text-sm pb-2">Theta</div>
                                        <input class = "text-md input-sm rounded bg-base-100 border-gray-800 shadow-md" disabled type = "text" prop:value = move || theta()/>
                                    </div>
                                </div>
                            </div>
                        </form>
                    </div>
            </div>
        </div>

        // <div>
        //     <div class = "overflow-auto border border-gray-800 rounded-md mt-2 bg-base-300" prop:hidden = {move || !show_builder()}>
        //         <div class = "pb-7">
        //             <table class = "table table-zebra table-xs">
        //                 <thead class = "text-base text-success font-extralight bg-base-200">
        //                     <tr class = "text-center border-b border-b-gray-800">
        //                         <th colspan = "12" class = "text-success">GENERATED QUOTES</th>
        //                     </tr>
        //                     <tr class = "border-y border-y-gray-800">
        //                         <th>INSTRUMENT</th>
        //                         <th>AMOUNT</th>
        //                         <th>SIDE</th>
        //                         <th>SPOT</th>
        //                         <th>STRIKE</th>
        //                         <th>"% / STRIKE"</th>
        //                         <th>IV</th>
        //                         <th>CCY1 AMOUNT</th>
        //                         <th>CCY2 AMOUNT</th>
        //                         <th>TRADE EXPIRY</th>
        //                         <th>PREMIUM</th>
        //                         <th></th>
        //                     </tr>
        //                 </thead>
        //                 <tbody>
        //                     {
        //                         move || {
        //                             trade_quotes.get().into_iter().map(|quote| {
        //                                 let jabra_id = counter_parties().get_id_by_ticker("JABRA").unwrap_or_default();
        //                                 if quote.counterparty_id == jabra_id {
        //                                     let is_pos_class_b = if quote.px_in_base_ccy >= 0.0 {"text-success"} else {"text-error"};
        //                                     let is_pos_value_b = if quote.px_in_base_ccy >= 0.0 {format!("+{}",quote.px_in_base_ccy)} else {quote.px_in_base_ccy.to_string()};
        //                                     let is_pos_class_q = if quote.px_in_quote_ccy >= 0.0 {"text-success"} else {"text-error"};
        //                                     let is_pos_value_q = if quote.px_in_quote_ccy >= 0.0 {format!("+{}",quote.px_in_quote_ccy)} else {quote.px_in_quote_ccy.to_string()};
        //                                     let gp = quote.group_id.clone();
        //                                     Some(view! {
        //                                         <tr class = "hover">
        //                                             <td>{quote.option_kind}</td>
        //                                             <td>{quote.amount}</td>
        //                                             <td>{quote.side}</td>
        //                                             <td>{quote.spot}</td>
        //                                             <td>{quote.strike}</td>
        //                                             <td>{quote.offstrike_percentage}</td>
        //                                             <td>{quote.iv}</td>
        //                                             <td class = {is_pos_class_b}>{is_pos_value_b}</td>
        //                                             <td class = {is_pos_class_q}>{is_pos_value_q}</td>
        //                                             <td>{get_trade_expiry_date(quote.expiry_timestamp)}</td>
        //                                             <td>
        //                                             <select class = "select-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" name="premium"
        //                                                     // prop:value = quote.payout_ccy
        //                                                     on:change = move |event| on_changed_payout_ccy(gp.clone(), event_target_value(&event))
        //                                                 >
        //                                                 {
        //                                                     if quote.payout_ccy == "base" {
        //                                                         view! {
        //                                                             <option value = "base">{config.get().get_currency_pair_by_id(quote.pair_id).unwrap().base.ticker}</option>
        //                                                             <option value = "quote">{config.get().get_currency_pair_by_id(quote.pair_id).unwrap().quote.ticker}</option>
        //                                                         }.into_view()
        //                                                     }else{
        //                                                         view! {
        //                                                             <option value = "quote">{config.get().get_currency_pair_by_id(quote.pair_id).unwrap().quote.ticker}</option>
        //                                                             <option value = "base">{config.get().get_currency_pair_by_id(quote.pair_id).unwrap().base.ticker}</option>
        //                                                         }.into_view()
        //                                                     }
        //                                                 }
        //                                                 </select>
        //                                             </td>
        //                                             // <td>{quote.px_in_base_ccy}</td>
        //                                             // <td>{quote.px_in_quote_ccy}</td>
        //                                             <td>
        //                                                 <button class = "btn btn-square btn-xs btn-warning" on:click = move |_| on_remove_qoute(quote.group_id.clone())>
        //                                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
        //                                                         <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM6.75 9.25a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5z" clip-rule="evenodd" />
        //                                                     </svg>
        //                                                 </button>
        //                                             </td>
        //                                         </tr>
        //                                     })
        //                                 } else {
        //                                     None
        //                                 }
        //                             }).collect_view()
        //                         }
        //                     }
        //                 </tbody>
        //             </table>
        //         </div>
        //     <div class="flex justify-evenly border-t border-t-gray-800">

        //         <div class="flex-initial w-1/6 stat bg-base-100 border-r border-gray-800 bg-inherit">
        //             <div class="stat-title text-sm">Total Base Cost</div>
        //             {
        //                 move || {
        //                     let is_pos = if quote_total_base_currency_price() >= 0.0 {"stat-value text-lg text-success"} else {"stat-value text-lg text-error"};
        //                     let is_pos_value = if quote_total_base_currency_price() >= 0.0 {format!("+{}",quote_total_base_currency_price())} else {quote_total_base_currency_price().to_string()};

        //                     view!{
        //                         <div class= {is_pos}>{is_pos_value}</div>
        //                     }
        //                 }
        //             }
        //         </div>

        //         <div class="flex-initial w-1/6 stat bg-base-100 border-r border-gray-800 bg-inherit">
        //             <div class="stat-title text-sm">Total Quote Cost</div>
        //             {
        //                 move || {
        //                     let is_pos = if quote_total_quote_currency_price() >= 0.0 {"stat-value text-lg text-success"} else {"stat-value text-lg text-error"};
        //                     let is_pos_value = if quote_total_quote_currency_price() >= 0.0 {format!("+{}",quote_total_quote_currency_price())} else {quote_total_quote_currency_price().to_string()};

        //                     view!{
        //                         <div class= {is_pos}>{is_pos_value}</div>
        //                     }
        //                 }
        //             }
        //         </div>
        //         <form on:submit=|ev| ev.prevent_default() class = "flex flex-auto gap-2 m-3">
        //                 // <div class = "grid grid-cols-4 gap-3 justify-center pt-2">
        //             <div class = "flex-initial w-1/4">
        //                 <label class = "block font-light text-sm">Counter Party</label>
        //                 <select class = "select-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" name="counterparty" required
        //                     on:change = move |event| on_change_counterparty(event)
        //                     >
        //                     <option value = "" prop:selected=true prop:disabled=true>Select Counter Party</option>
        //                     <Transition fallback = move || view!{<div><span class="loading loading-bars loading-sm"></span></div>}>
        //                     {
        //                         move || {
        //                             counterparty_resource.and_then(|cp| {
        //                             log::info!("Calling Counter Party");
        //                             counter_parties.set(cp.clone());

        //                                 //Loop counter parties
        //                                 cp.data.iter().map(|party| {

        //                                 let jabra_id = cp.get_id_by_ticker("JABRA").unwrap_or_default();
        //                                 if party.id != jabra_id {
        //                                     Some(view! {
        //                                         <option value={&party.id.to_string()}>{&party.name}</option>
        //                                     })
        //                                 } else {
        //                                     None
        //                                 }
        //                                 }).collect_view()
        //                         })
        //                         }
        //                     }
        //                     </Transition>
        //                 </select>
        //             </div>
        //             <div class = "flex-initial w-1/4">
        //                 <label class = "block font-light text-sm">Quote Expiry</label>
        //                 <select class = "flex-1 select-sm text-xs text-success block rounded hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md" name="expiry_in_min"
        //                     on:change = move |event| on_changed_expiry(event)
        //                     >
        //                 //  <option value = "0" prop:selected=true prop:disabled=true>Expiry (in Min.)</option>
        //                     <option value = "0" >Good Till Canceled</option>
        //                     <option value = "10" >10 mins.</option>
        //                     <option value = "20" >20 mins.</option>
        //                     <option value = "30" >30 mins.</option>
        //                     <option value = "40" >40 mins.</option>
        //                     <option value = "50" >50 mins.</option>
        //                     <option value = "60" >60 mins.</option>
        //                 </select>
        //             </div>
        //                     <button class = "flex-auto btn btn-sm btn-success mt-5" on:click = move |_| if counterparty.get() != "" {set_show_confirm_modal.set(true)}>SUBMIT</button>
        //                     <button type = "button" class = "flex-auto btn btn-sm btn-info mt-5" on:click = move |_| on_clear_quote()>
        //                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
        //                         <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd" />
        //                     </svg>
        //                     CLEAR
        //                     </button>
        //                 //  </div>
        //             </form>
        //         </div>
        //     </div>
        // </div>

        // {
        //     move || {
        //         view! {
        //             <ConfirmModal
        //                 when = show_confirm_modal.get()
        //                 write_signal = set_show_confirm_modal
        //                 function = trade_quote_dispatch
        //                 action = add_quote_action
        //             />
        //         }
        //     }
        // }

        // {
        //     move || match show_trade_quote_alert() {
        //         true => if !add_quote_response().success  {
        //             view! {
        //                 <ErrorModal
        //                     read_signal = show_error_modal
        //                     write_signal = set_show_error_modal
        //                     message = add_quote_response().message
        //                 />
        //                 }
        //         } else {
        //             view! {
        //                 <SuccessModal
        //                     read_signal = show_success_modal
        //                     write_signal = set_show_success_modal
        //                     message = add_quote_response().message
        //                 />
        //                 }
        //         }.into_view(),
        //         false => view! {<div></div>}.into_view(),
        //     }
        // }
    }
}
