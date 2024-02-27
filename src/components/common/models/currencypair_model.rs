use serde::{Deserialize, Serialize};

use super::currency_model::Currency;

/// This struct is used to get the currency pair details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrencyPair {
    pub id: u16,
    pub name: String,
    pub is_active: bool,
    pub base: Currency,
    pub quote: Currency,
}
impl CurrencyPair {
    pub fn get_query(key: &str) -> String {
        let base_currency = Currency::get_query(format!("{}.base", key).as_str());
        let quote_currency = Currency::get_query(format!("{}.quote", key).as_str());
        format!(
            "{}.id, {}.name, {}.is_active, {}, {}",
            key, key, key, base_currency, quote_currency
        )
    }
}

mod test {
    #[test]
    fn test_get_query() {
        use super::CurrencyPair;
        let query = CurrencyPair::get_query("pair_id");
        assert_eq!(query, "pair_id.id, pair_id.name, pair_id.is_active");
    }
}
