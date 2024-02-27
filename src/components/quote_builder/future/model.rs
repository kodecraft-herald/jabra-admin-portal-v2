use serde::{Deserialize, Serialize};

use crate::components::common::models::common_models::{Currency, CurrencyPair, OptionInstrumentSpecification};

/// Struct for Futures Quote Data.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuturesQuote {
    pub group_id: String,
    pub rfq_type: String,
    pub expiry: String,
    pub pair: CurrencyPair,
    pub amount: f64,
    pub price: f64,
    pub quote_expiry: String,
    pub counterparty: String,
    pub quote_status: String,
    pub gtc: bool,
}

impl FuturesQuote {
    pub fn new(
        group_id: String,
        rfq_type: String,
        expiry: String,
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
            rfq_type,
            expiry,
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

impl Default for FuturesQuote {
    fn default() -> Self {
        Self {
            group_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            rfq_type: "Future".to_string(),
            expiry: "2023-12-12".to_string(),
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

/// Struct for Futures Quote History.
/// Has a vector of [`FuturesQuote`].

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesQuoteHistory {
    pub data: Vec<FuturesQuote>,
}

impl Default for FuturesQuoteHistory {
    fn default() -> Self {
        Self {
            data: vec![FuturesQuote::default(), FuturesQuote::default()],
        }
    }
}

impl FuturesQuoteHistory {
    ///Extract Trade Struct into a Vector that can be shown in the data Table
    pub fn extract(&self, status: String) -> Vec<ExtractedFuturesQuote> {
        self.data
            .iter()
            .filter(|t| status == "active" || t.quote_status == status.clone())
            .map(|t| ExtractedFuturesQuote {
                group_id: t.group_id.clone(),
                rfq_type: t.rfq_type.clone(),
                expiry: t.expiry.clone(),
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
pub struct ExtractedFuturesQuote {
    pub group_id: String,
    pub rfq_type: String,
    pub expiry: String,
    pub pair: String,
    pub amount: f64,
    pub price: f64,
    pub quote_expiry: String,
    pub counterparty: String,
    pub gtc: bool,
}

/// Enum for sorting the data table.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QuoteSort {
    GroupId,
    Type,
    Expiry,
    Pair,
    Amount,
    Price,
    QuoteExpiry,
    Counterparty,
}

/// Function to sort the data table.

pub fn sort(
    mut data_table: Vec<ExtractedFuturesQuote>,
    sort_type: bool,
    sort_by: QuoteSort,
) -> Vec<ExtractedFuturesQuote> {
    match sort_by {
        QuoteSort::GroupId => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.group_id.cmp(&b.group_id));
            }
            false => {
                data_table.sort_by(|a, b| b.group_id.cmp(&a.group_id));
            }
        },
        QuoteSort::Type => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.rfq_type.cmp(&b.rfq_type));
            }
            false => {
                data_table.sort_by(|a, b| b.rfq_type.cmp(&a.rfq_type));
            }
        },
        QuoteSort::Expiry => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.expiry.cmp(&b.expiry));
            }
            false => {
                data_table.sort_by(|a, b| b.expiry.cmp(&a.expiry));
            }
        },
        QuoteSort::Pair => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.pair.cmp(&b.pair));
            }
            false => {
                data_table.sort_by(|a, b| b.pair.cmp(&a.pair));
            }
        },
        QuoteSort::Amount => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap());
            }
        },
        QuoteSort::Price => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
        },
        QuoteSort::QuoteExpiry => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.quote_expiry.cmp(&b.quote_expiry));
            }
            false => {
                data_table.sort_by(|a, b| b.quote_expiry.cmp(&a.quote_expiry));
            }
        },
        QuoteSort::Counterparty => match sort_type {
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
