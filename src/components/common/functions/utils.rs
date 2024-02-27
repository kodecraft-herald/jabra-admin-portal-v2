use leptos::*;

use crate::components::common::{
    functions::wrapper,
    models::common_models::{
        BlankRequest, CoinBaseSpotPriceResponse, CounterPartyResponse, CurrencyPair,
        EstimateIVRequest, EstimateIVResponse, Quote, QuoteOptionRequest, QuoteOptionResponse,
        UnifiedCurrencyPairConfigurationResponse,
    },
};

/// Server function that gets the unified configuration for the currency pair.

#[server]
pub async fn fetch_unified_configuration() -> Result<
    crate::components::common::models::common_models::UnifiedCurrencyPairConfigurationResponse,
    ServerFnError,
> {
    use crate::components::wrapper::get_cookie_value;
    use crate::components::wrapper::JabraCookie;
    use crate::components::wrapper::{call_and_parse, HttpMethod};

    let cookie = get_cookie_value("jabra-admin-portal-v2").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/items/supported_pair?filter[is_active][_eq]=true&sort=id&fields=id,name,is_active,base.id,base.ticker,base.name,base.instrument_option.ccy_id,base.instrument_option.contract_multiplier,base.instrument_option.min_price_increment,base.instrument_option.min_contract_increment,quote.id,quote.ticker,quote.name,quote.instrument_option.ccy_id,quote.instrument_option.contract_multiplier,quote.instrument_option.min_price_increment,quote.instrument_option.min_contract_increment", url);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, UnifiedCurrencyPairConfigurationResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function that gets the coin base spot price.
/// Accepts a [`CurrencyPair`].

#[server]
pub async fn coin_base_spot(
    currency_pair: CurrencyPair,
) -> Result<CoinBaseSpotPriceResponse, ServerFnError> {
    use crate::components::common::functions::wrapper::{call_and_parse, HttpMethod};
    let url = std::env::var("COINBASE_V2").unwrap();
    let coinbase_name = currency_pair.coinbase_name();
    if coinbase_name == "-" {
        return Ok(CoinBaseSpotPriceResponse::default());
    }
    let path = format!("{}/prices/{}/spot", url, coinbase_name);

    let response = call_and_parse::<BlankRequest, CoinBaseSpotPriceResponse>(
        Option::None,
        path,
        reqwest::header::HeaderMap::default(),
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function that gets counterparties.

#[server]
pub async fn sb_counter_parties() -> Result<CounterPartyResponse, ServerFnError> {
    use crate::components::common::functions::wrapper::get_cookie_value;
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{call_and_parse, HttpMethod};

    let cookie = get_cookie_value("jabra-admin-portal-v2").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/items/counterparty", url);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, CounterPartyResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error6: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function that gets the estimated IVs.
/// Accepts a [`EstimateIVRequest`].

#[server]
pub async fn sb_fetch_estimate_iv(
    request: EstimateIVRequest,
) -> Result<EstimateIVResponse, ServerFnError> {
    use crate::components::common::functions::wrapper::get_cookie_value;
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{call_and_parse, HttpMethod};
    // use gloo_timers::future::TimeoutFuture;
    // TimeoutFuture::new(1000).await; //Try delaying
    log::info!("Estimate IVz request: {:?}", request);
    if request.strike == 0.0 {
        return Ok(EstimateIVResponse::default());
    }
    let cookie = get_cookie_value("jabra-admin-portal-v2").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("JABRAAPIGATEWAY").unwrap();
    let path = format!("{}/option_pricer/estimate_iv", url);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<EstimateIVRequest, EstimateIVResponse>(
        Some(request),
        path,
        headers,
        HttpMethod::POST,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error4: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function that posts the quote option.
/// Accepts a [`QuoteOptionRequest`].

#[server]
pub async fn sb_post_qoute_option(
    request: QuoteOptionRequest,
) -> Result<QuoteOptionResponse, ServerFnError> {
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{
        call_and_parse, get_cookie_value, HttpMethod,
    };

    let cookie = get_cookie_value("jabra-admin-portal-v2").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // Check if token expires, this checking will be available only to actions and server action
    // Other resources will still work due to 10 minutes buffer time
    if jwt_cookie.is_expired() {
        let refresh = wrapper::refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
        match refresh {
            Ok(r) => {
                bearer = format!("Bearer {}", r.access_token);
                wrapper::set_jabra_cookie(Some(r), "jabra-admin-portal-v2".to_string()).await;
            }
            Err(e) => {
                // directus_wrapper::set_jabra_cookie(cx, None, "JabraOPv1_2023".to_string()).await;
                return Err(ServerFnError::new(e.to_string()));
            }
        }
    }

    let url = std::env::var("JABRAAPIGATEWAY").unwrap();
    let path = format!("{}/option_pricer/quote_option", url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<QuoteOptionRequest, QuoteOptionResponse>(
        Some(request),
        path,
        headers,
        HttpMethod::POST,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function that adds a quote.
/// Accepts a vector of [`Quote`].

#[server(AddQuote, "/api")]
pub async fn add_quote(request: Vec<Quote>) -> Result<bool, ServerFnError> {
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{call, get_cookie_value, HttpMethod};

    let cookie = get_cookie_value("jabra-admin-portal-v2").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // Check if token expires, this checking will be available only to actions and server action
    // Other resources will still work due to 10 minutes buffer time
    if jwt_cookie.is_expired() {
        let refresh = wrapper::refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
        match refresh {
            Ok(r) => {
                bearer = format!("Bearer {}", r.access_token);
                wrapper::set_jabra_cookie(Some(r), "jabra-admin-portal-v2".to_string()).await;
            }
            Err(e) => {
                return Err(ServerFnError::new(e.to_string()));
            }
        }
    }

    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/items/quotes_option", url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );
    // log::info!("request: {:?}", request);
    let response = call::<Vec<crate::components::common::models::common_models::Quote>>(
        Some(request),
        path,
        headers,
        HttpMethod::POST,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
    // Err(ServerFnError::ServerError("Transaction failed".to_string()))
    // log::info!("request: {:?}", request);
    // Ok(false)
}
