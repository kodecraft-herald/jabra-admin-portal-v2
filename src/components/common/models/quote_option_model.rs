use super::{
    counterparty_model::CounterParty, currency_model::Currency, currencypair_model::CurrencyPair,
    user_model::User,
};
use crate::components::common::functions::helpers::{format_currency, format_utc_str_to_local_str};
use serde::{Deserialize, Serialize};

/// Struct for the response of a trade quote approval/rejection.
/// Used in alert modals.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteResponse {
    pub success: bool,
    pub message: String,
}

impl Default for ApproveTradeQuoteResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}

/// Struct for trade quote request.

/***************Trade Quotes Structs ***************/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequest {
    pub query: ApproveTradeQuoteRequestQuery,
    pub data: ApproveTradeQuoteRequestData,
}

/// Struct for the query of [`ApproveTradeQuoteRequest`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequestQuery {
    pub filter: ApproveTradeQuoteRequestQueryFilter,
}

/// Struct for the filter of [`ApproveTradeQuoteRequestQuery`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequestQueryFilter {
    pub group_id: FilterGroupId,
}

/// Struct for the group id of [`ApproveTradeQuoteRequestQueryFilter`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FilterGroupId {
    pub _in: Vec<String>,
}

/// Struct for the data of [`ApproveTradeQuoteRequest`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequestData {
    pub quote_status: String,
}

impl ApproveTradeQuoteRequest {
    pub fn new(group_id: Vec<String>, quote_status: String) -> Self {
        Self {
            query: ApproveTradeQuoteRequestQuery {
                filter: ApproveTradeQuoteRequestQueryFilter {
                    group_id: FilterGroupId { _in: group_id },
                },
            },
            data: ApproveTradeQuoteRequestData { quote_status },
        }
    }

    pub fn deserialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Response Struct of TradeHistory Request, return a vector of Trade Struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteOptionHistory {
    pub data: Vec<QuoteOption>,
}

impl QuoteOptionHistory {
    /// Extract Trade Struct into a Vector that can be shown in the data Table.
    pub fn extract(&self) -> Vec<ExtractedQuoteOption> {
        self.data
            .iter()
            .map(|t| ExtractedQuoteOption {
                id: t.id,
                market: t.instrument_name.clone(),
                status: t.quote_status.clone(),
                side: t.side.clone(),
                kind: t.option_kind.clone(),
                trans_type: String::from("Option"),
                size: format_currency(t.amount, t.pair_id.base.display_scale.clone()),
                price: match t.payout_ccy.clone() {
                    Some(p) => {
                        if p == "base" {
                            format_currency(
                                t.px_in_base_ccy.clone(),
                                t.pair_id.base.display_scale.clone(),
                            )
                        } else {
                            format_currency(
                                t.px_in_quote_ccy.clone(),
                                t.pair_id.quote.display_scale.clone(),
                            )
                        }
                    }
                    None => format_currency(
                        t.px_in_quote_ccy.clone(),
                        t.pair_id.quote.display_scale.clone(),
                    ),
                },
                group_id: t.group_id.clone(),
                expiration_date: match t.expiry_timestamp.clone() {
                    Some(d) => Some(format_utc_str_to_local_str(d)),
                    _ => Some(String::from("N/A")),
                },
                date_created: format_utc_str_to_local_str(t.date_created.clone()),
                premium_ccy: match t.payout_ccy.clone() {
                    Some(p) => {
                        if p == "base" {
                            t.pair_id.name.split("/").collect::<Vec<&str>>()[0].to_string()
                        } else {
                            t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string()
                        }
                    }
                    None => t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string(),
                },
            })
            .collect()
    }
}

/// Struct for the data that can be shown in the data table.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ExtractedQuoteOption {
    pub id: u32,
    pub market: String,
    pub status: String,
    pub side: String,
    pub kind: String,
    pub trans_type: String,
    pub size: String,
    pub price: String,
    pub group_id: String,
    pub expiration_date: Option<String>,
    pub date_created: String,
    pub premium_ccy: String,
}

/// Struct for Quote Option data.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteOption {
    pub id: u32,
    pub date_created: String,
    pub instrument_name: String,
    pub side: String,
    pub group_id: String,
    pub ttm: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub payout_ccy: Option<String>,
    pub strike: f64,
    pub amount: f64,
    pub option_kind: String,
    pub spot: f64,
    pub r1: f64,
    pub r2: f64,
    pub iv: f64,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub gtc: bool,
    pub quote_status: String,
    pub expiry_timestamp: Option<String>,
    pub ccy_id: Currency,
    pub pair_id: CurrencyPair,
    pub counterparty_id: CounterParty,
    pub party_a: CounterParty,
    pub party_b: CounterParty,
    pub user_created: User,
}

impl QuoteOption {
    pub fn get_query() -> String {
        format!(
            "id, date_created, expiry_timestamp, instrument_name, side, group_id, ttm, px_in_base_ccy, px_in_quote_ccy, payout_ccy, strike, amount, option_kind, spot, r1, r2, iv, delta, gamma, theta, gtc, quote_status, expiry_timestamp, {}, {}, {}, {}, {}, {}",
            // Currency::get_query("base_currency_id"),
            // Currency::get_query("quote_currency_id"),
            Currency::get_query("ccy_id"),
            CurrencyPair::get_query("pair_id"),
            CounterParty::get_query("counterparty_id"),
            CounterParty::get_query("party_a"),
            CounterParty::get_query("party_b"),
            User::get_query("user_created")
        )
    }
}

/// Function for Sorting the data table.
pub fn sort(
    mut data_table: Vec<ExtractedQuoteOption>,
    sort_type: bool,
    sort_by: String,
) -> Vec<ExtractedQuoteOption> {
    match sort_by.to_uppercase().as_str() {
        "ID" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.id.cmp(&b.id));
            }
            false => {
                data_table.sort_by(|a, b| b.id.cmp(&a.id));
            }
        },
        "MARKET" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.market.cmp(&b.market));
            }
            false => {
                data_table.sort_by(|a, b| b.market.cmp(&a.market));
            }
        },
        "STATUS" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.status.cmp(&b.status));
            }
            false => {
                data_table.sort_by(|a, b| b.status.cmp(&a.status));
            }
        },
        "PRICE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
        },
        "SIDE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.side.cmp(&b.side));
            }
            false => {
                data_table.sort_by(|a, b| b.side.cmp(&a.side));
            }
        },
        "KIND" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.kind.cmp(&b.kind));
            }
            false => {
                data_table.sort_by(|a, b| b.kind.cmp(&a.kind));
            }
        },
        "SIZE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.size.partial_cmp(&b.size).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
            }
        },
        "EXPIRATION DATE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.expiration_date.cmp(&b.expiration_date));
            }
            false => {
                data_table.sort_by(|a, b| b.expiration_date.cmp(&a.expiration_date));
            }
        },
        "DATE CREATED" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.date_created.cmp(&b.date_created));
            }
            false => {
                data_table.sort_by(|a, b| b.date_created.cmp(&a.date_created));
            }
        },
        "TYPE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.trans_type.cmp(&b.trans_type));
            }
            false => {
                data_table.sort_by(|a, b| b.trans_type.cmp(&a.trans_type));
            }
        },
        _ => (),
    }
    data_table
}

mod test {
    #[test]
    fn test_get_query() {
        use super::QuoteOption;
        let query = QuoteOption::get_query();
        // assert_eq!(query, "id, date_created, venue_instrument_name, side, group_id, ttm, px_in_base_ccy, px_in_quote_ccy, strike, amount, option_kind, spot, r1, r2, iv, base_currency_id.id, base_currency_id.ticker, base_currency_id.name, base_currency_id.is_active, quote_currency_id.id, quote_currency_id.ticker, quote_currency_id.name, quote_currency_id.is_active, ccy_id.id, ccy_id.ticker, ccy_id.name, ccy_id.is_active, pair_id.id, pair_id.name, pair_id.is_active, counterparty_id.id, counterparty_id.name, counterparty_id.is_active, party_a.id, party_a.name, party_a.is_active, party_b.id, party_b.name, party_b.is_active");
        println!("{}", query)
    }
}
