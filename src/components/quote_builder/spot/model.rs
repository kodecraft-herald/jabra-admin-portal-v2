use serde::{Deserialize, Serialize};

use crate::components::common::models::common_models::{
    Currency, CurrencyPair, OptionInstrumentSpecification,
};

/// Struct for Spot Quote Data.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotQuote {
    pub group_id: String,
    pub pair: CurrencyPair,
    pub amount: f64,
    pub price: f64,
    pub quote_expiry: String,
    pub counterparty: String,
    pub quote_status: String,
    pub gtc: bool,
}

impl SpotQuote {
    pub fn new(
        group_id: String,
        pair: CurrencyPair,
        amount: f64,
        price: f64,
        quote_expiry: String,
        counterparty: String,
        quote_status: String,
        gtc: bool,
    ) -> Self {
        Self {
            group_id,
            pair,
            amount,
            price,
            quote_expiry,
            counterparty,
            quote_status,
            gtc,
        }
    }
}

impl Default for SpotQuote {
    fn default() -> Self {
        Self {
            group_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            pair: CurrencyPair {
                id: 1,
                name: "BTC/USD".to_string(),
                base: Currency {
                    id: 1,
                    ticker: "BTC".to_string(),
                    name: "BTC".to_string(),
                    instrument_option: OptionInstrumentSpecification {
                        ccy_id: 1,
                        contract_multiplier: 1,
                        min_price_increment: 0.01,
                        min_contract_increment: 0.01,
                    },
                },
                quote: Currency {
                    id: 2,
                    ticker: "USD".to_string(),
                    name: "USD".to_string(),
                    instrument_option: OptionInstrumentSpecification {
                        ccy_id: 2,
                        contract_multiplier: 1,
                        min_price_increment: 0.01,
                        min_contract_increment: 0.01,
                    },
                },
            },
            amount: 3.33,
            price: 30326.21,
            quote_expiry: "2023-12-12".to_string(),
            counterparty: "Bitbox".to_string(),
            quote_status: "active".to_string(),
            gtc: false,
        }
    }
}

/// Struct for Spot Quote History.
/// Has a vector of [`SpotQuote`].

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotQuoteHistory {
    pub data: Vec<SpotQuote>,
}

impl Default for SpotQuoteHistory {
    fn default() -> Self {
        Self {
            data: vec![SpotQuote::default(), SpotQuote::default()],
        }
    }
}

impl SpotQuoteHistory {
    ///Extract Trade Struct into a Vector that can be shown in the data Table
    pub fn extract(&self, status: String) -> Vec<ExtractedSpotQuote> {
        self.data
            .iter()
            .filter(|t| status == "active" || t.quote_status == status.clone())
            .map(|t| ExtractedSpotQuote {
                group_id: t.group_id.clone(),
                pair: t.pair.name.clone(),
                amount: t.amount,
                price: t.price,
                quote_expiry: t.quote_expiry.clone(),
                counterparty: t.counterparty.clone(),
                gtc: t.gtc,
            })
            .collect()
    }
}

/// Struct for the data that can be shown in the data table.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractedSpotQuote {
    pub group_id: String,
    pub pair: String,
    pub amount: f64,
    pub price: f64,
    pub quote_expiry: String,
    pub counterparty: String,
    pub gtc: bool,
}

/// Enum for sorting the data table.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SpotSort {
    GroupId,
    Pair,
    Amount,
    Price,
    QuoteExpiry,
    Counterparty,
}

/// Function to sort the data table.

pub fn sort(
    mut data_table: Vec<ExtractedSpotQuote>,
    sort_type: bool,
    sort_by: SpotSort,
) -> Vec<ExtractedSpotQuote> {
    match sort_by {
        SpotSort::GroupId => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.group_id.cmp(&b.group_id));
            }
            false => {
                data_table.sort_by(|a, b| b.group_id.cmp(&a.group_id));
            }
        },
        SpotSort::Pair => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.pair.cmp(&b.pair));
            }
            false => {
                data_table.sort_by(|a, b| b.pair.cmp(&a.pair));
            }
        },
        SpotSort::Amount => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap());
            }
        },
        SpotSort::Price => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
        },
        SpotSort::QuoteExpiry => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.quote_expiry.cmp(&b.quote_expiry));
            }
            false => {
                data_table.sort_by(|a, b| b.quote_expiry.cmp(&a.quote_expiry));
            }
        },
        SpotSort::Counterparty => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.counterparty.cmp(&b.counterparty));
            }
            false => {
                data_table.sort_by(|a, b| b.counterparty.cmp(&a.counterparty));
            }
        },
    }
    data_table
}
