use leptos::*;
use serde::{Deserialize, Serialize};

/// Struct used to represent the base/quote currency.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Currency {
    pub id: u16,
    pub ticker: String,
    pub name: String,
    pub is_active: bool,
    pub display_scale: u8,
    pub sign: Option<String>,
}

impl Currency {
    pub fn get_query(key: &str) -> String {
        format!(
            "{}.id, {}.ticker, {}.name, {}.is_active, {}.display_scale, {}.sign",
            key, key, key, key, key, key
        )
    }
    pub fn get_default_query() -> String {
        format!("id, ticker, name, is_active, display_scale, sign")
    }
}

mod test {
    #[test]
    fn test_get_query() {
        use super::Currency;
        let query = Currency::get_query("base_currency_id");
        assert_eq!(query, "base_currency_id.id, base_currency_id.ticker, base_currency_id.name, base_currency_id.is_active, base_currency_id.display_scale");
    }
}

/// Struct for the Currency Configuration Response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrencyConfigurationResponse {
    pub data: Vec<Currency>,
}

/// Server function to fetch the currencies.

#[server]
pub async fn fetch_currencies() -> Result<CurrencyConfigurationResponse, ServerFnError> {
    use crate::components::common::models::common_models::BlankRequest;
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::{call_and_parse, get_cookie_value, HttpMethod};

    let cookie = get_cookie_value("JabraOPv1_2023").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!(
        "{}/items/supported_ccy?fields={}",
        url,
        Currency::get_default_query()
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, CurrencyConfigurationResponse>(
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
