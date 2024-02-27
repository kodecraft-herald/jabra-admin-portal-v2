use crate::components::common::functions::wrapper;

use super::{
    counterparty_model::CounterParty, currency_model::Currency, currencypair_model::CurrencyPair,
    user_model::User,
};
use leptos::*;
use serde::{Deserialize, Serialize};

/// This enum represents the menu tabs in the quotes page.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuotesTab {
    Active,
    Approved,
    Rejected,
}

/// This struct is used to get the quote option details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuoteOption {
    pub id: u32,
    pub date_created: String,
    pub quote_id: String,
    pub amount: f64,
    pub option_kind: String,
    pub r1: f64,
    pub r2: f64,
    pub offstrike_percentage: f64,
    pub strike: f64,
    pub iv: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub side: String,
    pub quote_expiry: String,
    pub modified_date: String,
    pub quote_status: String,
    pub instrument_name: String,
    pub spot: f64,
    pub ttm: f64,
    pub gtc: bool,
    pub group_id: String,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub payout_ccy: Option<String>,
    pub user_created: User,
    pub pair_id: CurrencyPair,
    pub ccy_id: Currency,
    pub counterparty_id: CounterParty,
    pub party_a: Option<CounterParty>,
    pub party_b: Option<CounterParty>,
}

impl QuoteOption {
    pub fn get_query() -> String {
        format!(
            "id, date_created, quote_id, amount, option_kind, r1, r2, offstrike_percentage, strike, iv, px_in_base_ccy, px_in_quote_ccy, side, quote_expiry, modified_date, quote_status, delta, instrument_name, spot, ttm, gtc, group_id, gamma, theta, payout_ccy, gtc, {}, {}, {}, {}, {}, {}",
            User::get_query("user_created"),
            CurrencyPair::get_query("pair_id"),
            Currency::get_query("ccy_id"),
            CounterParty::get_query("counterparty_id"),
            CounterParty::get_query("party_a"),
            CounterParty::get_query("party_b"),
        )
    }
}

/// This is the response struct for the [`get_quotes_option`] server function.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteOptionResponse {
    pub data: Vec<QuoteOption>,
}

/// This struct is used to handle all quotes option status change requests.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesOptionForStatusChange {
    pub id: u32,
    pub quote_status: String,
}

impl QuotesOptionForStatusChange {
    pub fn new(id: u32, quote_status: String) -> Self {
        Self { id, quote_status }
    }
}

/// This struct is used to handle all quotes option modification requests.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesOptionsForModification {
    pub id: u32,
    pub amount: f64,
    pub counterparty_id: u16,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub quote_expiry: String,
    pub payout_ccy: Option<String>,
    pub party_a: u16,
    pub party_b: u16,
    pub gtc: bool,
}
impl QuotesOptionsForModification {
    pub fn new(
        id: u32,
        amount: f64,
        counterparty_id: u16,
        px_in_base_ccy: f64,
        px_in_quote_ccy: f64,
        quote_expiry: String,
        payout_ccy: Option<String>,
        party_a: u16,
        party_b: u16,
        gtc: bool,
    ) -> Self {
        Self {
            id,
            amount,
            counterparty_id,
            px_in_base_ccy,
            px_in_quote_ccy,
            quote_expiry,
            payout_ccy,
            party_a,
            party_b,
            gtc,
        }
    }
}

/// This struct is used for the response when a quote is modified.
/// It is used to display the success or failure message modal.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyQuoteResponse {
    pub success: bool,
    pub message: String,
}

impl Default for ModifyQuoteResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}

/// This is a server function that gets the quotes option based on the quote status.
/// The input string variations are `active`, `approved`, `rejected`, and `expired`.

#[server(GetQuotesOption)]
pub async fn get_quotes_option(
    quote_status: String,
) -> Result<std::collections::HashMap<String, Vec<QuoteOption>>, ServerFnError> {
    use super::common_models::BlankRequest;
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::get_cookie_value;
    use crate::components::common::functions::wrapper::{call_and_parse, HttpMethod};
    use std::collections::HashMap;
    let cookie = get_cookie_value("JabraOPv1_2023").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("DIRECTUSURL").unwrap();
    // let path = format!("{}/items/quotes_option?filter[quote_status][_eq]={}&filter[modified_date][_between]=[{}, {}]&fields={}", url, quote_status, QuoteOption::get_query());
    let path = format!(
        "{}/items/quotes_option?filter[quote_status][_eq]={}&fields={}",
        url,
        quote_status,
        QuoteOption::get_query()
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetQuoteOptionResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;

    match response {
        Ok(res) => {
            let mut trade_quotes_map: HashMap<String, Vec<QuoteOption>> = HashMap::new();
            let mut admin_trade_quotes: Vec<QuoteOption> = Vec::<QuoteOption>::default();
            for trade_quote in res.data {
                if trade_quote.counterparty_id.ticker != "JABRA" {
                    let key = format!(
                        "{}~{}",
                        trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                    );
                    trade_quotes_map
                        .entry(key)
                        .or_insert(vec![])
                        .push(trade_quote);
                } else {
                    admin_trade_quotes.push(trade_quote);
                }
            }

            let mut admin_trade_quotes_map = HashMap::<String, Vec<QuoteOption>>::new();
            //Iterate Over HashMap
            for (_, value) in trade_quotes_map.iter_mut() {
                //Iterate over Vector in Traders Quotes
                for trade_quote in value.iter_mut() {
                    //Iterate over Vector in Admin Quotes
                    for tq in admin_trade_quotes.iter() {
                        if trade_quote.group_id == tq.group_id {
                            let key = format!(
                                "{}~{}",
                                trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                            );
                            admin_trade_quotes_map
                                .entry(key)
                                .or_insert(vec![])
                                .push(tq.clone());
                        }
                    }
                }
            }

            for (key, value) in admin_trade_quotes_map.iter_mut() {
                if trade_quotes_map.contains_key(key) {
                    let v = trade_quotes_map.get_mut(key).unwrap();
                    v.append(value);
                } else {
                    trade_quotes_map.insert(key.clone(), value.clone());
                }
            }
            // log::info!("trade_quotes_map: {:?}", trade_quotes_map);
            Ok(trade_quotes_map)
        }
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// This is a server function that gets the quotes option based on the quote status and date range.
/// The format for the date is `%Y-%m-%dT%H:%M:%S%.3fZ`.

#[server(GetQuotesOptionUnder24Hrs)]
pub async fn get_quotes_option_under_24_hrs(
    quote_status: String,
    start_date: String,
    end_date: String,
) -> Result<std::collections::HashMap<String, Vec<QuoteOption>>, ServerFnError> {
    use super::common_models::BlankRequest;
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::get_cookie_value;
    use crate::components::common::functions::wrapper::{call_and_parse, HttpMethod};
    use std::collections::HashMap;
    let cookie = get_cookie_value("JabraOPv1_2023").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/items/quotes_option?filter[quote_status][_eq]={}&filter[modified_date][_between]=[{}, {}]&fields={}", url, quote_status, start_date, end_date, QuoteOption::get_query());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetQuoteOptionResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;

    match response {
        Ok(res) => {
            let mut trade_quotes_map: HashMap<String, Vec<QuoteOption>> = HashMap::new();
            let mut admin_trade_quotes: Vec<QuoteOption> = Vec::<QuoteOption>::default();
            for trade_quote in res.data {
                if trade_quote.counterparty_id.ticker != "JABRA" {
                    let key = format!(
                        "{}~{}",
                        trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                    );
                    trade_quotes_map
                        .entry(key)
                        .or_insert(vec![])
                        .push(trade_quote);
                } else {
                    admin_trade_quotes.push(trade_quote);
                }
            }

            let mut admin_trade_quotes_map = HashMap::<String, Vec<QuoteOption>>::new();
            //Iterate Over HashMap
            for (_, value) in trade_quotes_map.iter_mut() {
                //Iterate over Vector in Traders Quotes
                for trade_quote in value.iter_mut() {
                    //Iterate over Vector in Admin Quotes
                    for tq in admin_trade_quotes.iter() {
                        if trade_quote.group_id == tq.group_id {
                            let key = format!(
                                "{}~{}",
                                trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                            );
                            admin_trade_quotes_map
                                .entry(key)
                                .or_insert(vec![])
                                .push(tq.clone());
                        }
                    }
                }
            }

            for (key, value) in admin_trade_quotes_map.iter_mut() {
                if trade_quotes_map.contains_key(key) {
                    let v = trade_quotes_map.get_mut(key).unwrap();
                    v.append(value);
                } else {
                    trade_quotes_map.insert(key.clone(), value.clone());
                }
            }
            // log::info!("trade_quotes_map: {:?}", trade_quotes_map);
            Ok(trade_quotes_map)
        }
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// This is a server function that approves or rejects a quote option.
/// ## Examples
///
/// ```rust
/// approve_reject_quotes_option(vec![QuotesOptionForStatusChange::new(1, "approved".to_string())]) {
///     Ok(true)
/// };
/// ```
///
/// ```rust
/// approve_reject_quotes_option(vec![QuotesOptionForStatusChange::new(2, "rejected".to_string())]) {
///     Ok(true)
/// };
/// ```

#[server(ApproveRejectQuotesOption, "/api", "Cbor")]
pub async fn approve_reject_quotes_option(
    request: Vec<QuotesOptionForStatusChange>,
) -> Result<bool, ServerFnError> {
    log::info!("request: {:?}", request);
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{call, get_cookie_value, HttpMethod};

    let cookie = get_cookie_value("JabraOPv1_2023").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // Check if token expires, this checking will be available only to actions and server action
    // Other resources will still work due to 10 minutes buffer time
    if jwt_cookie.is_expired() {
        let refresh =
            wrapper::refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
        match refresh {
            Ok(r) => {
                bearer = format!("Bearer {}", r.access_token);
                wrapper::set_jabra_cookie(Some(r), "JabraOPv1_2023".to_string()).await;
            }
            Err(e) => {
                log::error!("error-token: {:?}", e);
                // directus_wrapper::set_jabra_cookie(None, "JabraOPv1_2023".to_string()).await;
                return Err(ServerFnError::ServerError(e.to_string()));
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

    let response =
        call::<Vec<QuotesOptionForStatusChange>>(Some(request), path, headers, HttpMethod::PATCH)
            .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error-: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
    // Err(ServerFnError::ServerError("Transaction failed".to_string()))
    // log::info!("request: {:?}", request);
    // Ok(false)
}

/// This is a server function that modifies a quote option.
/// It accepts a vector of [QuotesOptionsForModification] as input.

#[server(EditQuotesOption, "/api", "Cbor")]
pub async fn edit_quotes_option(
    request: Vec<QuotesOptionsForModification>,
) -> Result<bool, ServerFnError> {
    log::info!("request: {:?}", request);
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{call, get_cookie_value, HttpMethod};

    let cookie = get_cookie_value("JabraOPv1_2023").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // Check if token expires, this checking will be available only to actions and server action
    // Other resources will still work due to 10 minutes buffer time
    if jwt_cookie.is_expired() {
        let refresh =
            wrapper::refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
        match refresh {
            Ok(r) => {
                bearer = format!("Bearer {}", r.access_token);
                wrapper::set_jabra_cookie(Some(r), "JabraOPv1_2023".to_string()).await;
            }
            Err(e) => {
                log::error!("error-token: {:?}", e);
                // directus_wrapper::set_jabra_cookie(None, "JabraOPv1_2023".to_string()).await;
                return Err(ServerFnError::ServerError(e.to_string()));
            }
        }
    }
    // log::debug!("request: {:?}", request.deserialize());
    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/items/quotes_option", url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response =
        call::<Vec<QuotesOptionsForModification>>(Some(request), path, headers, HttpMethod::PATCH)
            .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error-: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
    // // Err(ServerFnError::ServerError("Transaction failed".to_string()))
    // log::info!("request: {:?}", request);
    // Ok(true)
}

mod tests {
    #[test]
    fn test_get_query() {
        use super::QuoteOption;
        let query = QuoteOption::get_query();
        println!("{}", query);
    }
}
