use leptos::*;
use serde::{Deserialize, Serialize};

/// This struct is used to get the details of a counterparty.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CounterParty {
    pub id: u16,
    pub ticker: String,
    pub name: String,
    pub short_name: Option<String>,
    pub is_exchange: bool,
}

impl CounterParty {
    pub fn get_query(key: &str) -> String {
        format!(
            "{}.id, {}.ticker, {}.name, {}.short_name, {}.is_exchange",
            key, key, key, key, key
        )
    }
}

/// This struct is the response of the [`get_counter_parties`] server function.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCounterPartiesResponse {
    pub data: Vec<CounterParty>,
}

impl GetCounterPartiesResponse {
    pub fn get_counterparty_by_name(&self, name: &str) -> Option<&CounterParty> {
        self.data.iter().find(|cp| cp.name == name)
    }
}

/// Server function to get the counterparties.

#[server(GetCounterParties)]
pub async fn get_counter_parties() -> Result<GetCounterPartiesResponse, ServerFnError> {
    use crate::components::common::functions::wrapper::JabraCookie;
    use crate::components::common::functions::wrapper::get_cookie_value;
    use crate::components::common::functions::wrapper::{call_and_parse, HttpMethod};

    let cookie = get_cookie_value("JabraOPv1_2023").await;
    let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = std::env::var("DIRECTUSURL").unwrap();
    let path = format!("{}/items/counterparty", url);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<super::common_models::BlankRequest, GetCounterPartiesResponse>(
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

mod tests {
    #[test]
    fn test_get_query() {
        use super::CounterParty;
        let query = CounterParty::get_query("party_a");
        assert_eq!(
            query,
            "party_a.id, party_a.ticker, party_a.name, party_a.short_name, party_a.is_exchange"
        );
    }
}
