use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

/// This is a common model that is used multiple times in this project.
/// It is used to send a blank request to the server to get a response.

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct BlankRequest;

/// Struct for the currency pair config response.
/// Has a vector of [`CurrencyPair`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct UnifiedCurrencyPairConfigurationResponse {
    pub data: Vec<CurrencyPair>,
}

impl UnifiedCurrencyPairConfigurationResponse {
    /// Function that gets the currency pair by passing in the id.
    pub fn get_currency_pair_by_id(&self, id: u16) -> Option<CurrencyPair> {
        self.data.iter().find(|x| x.id == id).map(|x| x.clone())
    }
}

/// Represents the default response from the API.
/// T must be serializable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultResponse<T>
where
    T: serde::Serialize,
{
    /// Represents the success status of the request.
    pub success: bool,
    /// Represents the message from the API, this can be optional
    pub message: Option<String>,
    /// Represents the data from the API, this can be optional and the type should be Serializable
    pub data: Option<T>,
}

/// Represents the query object for Directus API Request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// The filter of the query
    pub filter: Filter,
    /// The fields to be included in the query
    pub fields: Vec<String>,
}


/// Represents the Directus file for file imports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectusFile {
    /// Folder name, should be configured on the directus as public.
    pub folder: String,
    /// File id, UUID for file generated. This will be included in the hyperlink to get the file.
    pub id: String,
}

/// Represents a filter condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Represents the condition for the filter.
    #[serde(flatten)]
    pub condition: Value,
}

/// Represents filters for query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    /// Use for `AND` condition.
    pub _and: Vec<FilterCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBuilder {
    filters: Vec<(String, Value)>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, field: &str, value: Value) {
        self.filters.push((field.to_string(), value));
    }

    pub fn build(&self) -> Value {
        let mut filter_json = json!({});
        for (field, value) in &self.filters {
            let field_json = json!({ field: value });
            filter_json.as_object_mut().unwrap().extend(field_json.as_object().unwrap().clone());
        }
        filter_json
    }
}

/// Struct for the currency pair.
/// Has a base (e.g `BTC`) and a quote (e.g `USD`).

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CurrencyPair {
    pub id: u16,
    pub name: String,
    pub base: Currency,
    pub quote: Currency,
}

impl CurrencyPair {
    /// Function that gets the base ticker and quote ticker.
    /// Then formats it.
    pub fn coinbase_name(&self) -> String {
        format!("{}-{}", self.base.ticker, self.quote.ticker)
    }
    /// Function that gets the currency by ticker.
    pub fn get_currency_by_ticker(&self, ticker: &str) -> Option<Currency> {
        if self.base.ticker == ticker {
            Some(self.base.clone())
        } else if self.quote.ticker == ticker {
            Some(self.quote.clone())
        } else {
            None
        }
    }
    /// Function that gets the currency by id.
    pub fn get_currency_by_id(&self, id: u16) -> Option<Currency> {
        if self.base.id == id {
            Some(self.base.clone())
        } else if self.quote.id == id {
            Some(self.quote.clone())
        } else {
            None
        }
    }
    /// Function that returns the base tick size.
    pub fn base_tick_size(&self) -> f64 {
        self.base.tick_size()
    }
    /// Function that returns the base order size.
    pub fn base_order_size(&self) -> f64 {
        self.base.order_size()
    }
    /// Function that returns the quote tick size.
    pub fn quote_tick_size(&self) -> f64 {
        self.quote.tick_size()
    }
    /// Function that returns the quote order size.
    pub fn quote_order_size(&self) -> f64 {
        self.quote.order_size()
    }
}

/// Struct for the currency.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Currency {
    /// The unique identifier for the currency.
    pub id: u16,
    /// The ticker symbol for the currency.
    pub ticker: String,
    /// The name of the currency.
    pub name: String,
    /// The instrument option for the currency.
    pub instrument_option: OptionInstrumentSpecification,
}

impl Currency {
    /// Function that returns the currency tick size (min_price_increment).
    pub fn tick_size(&self) -> f64 {
        self.instrument_option.min_price_increment
    }
    /// Function that returns the currency order size (min_contract_increment).
    pub fn order_size(&self) -> f64 {
        self.instrument_option.min_contract_increment
    }
}

/// Struct for the Option Instrument.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OptionInstrumentSpecification {
    /// The currency id.
    pub ccy_id: u16,
    /// The contract multiplier.
    pub contract_multiplier: u16,
    /// The tick size.
    pub min_price_increment: f64, // Tick Size
    /// The order size.
    pub min_contract_increment: f64, //Order Size
}
impl Default for OptionInstrumentSpecification {
    fn default() -> Self {
        Self {
            ccy_id: 0,
            contract_multiplier: 1,
            min_price_increment: 0.01,
            min_contract_increment: 0.01,
        }
    }
}

/// Struct for the coin base spot price response.
/// Has a [`CoinBaseSpotPriceData`].

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoinBaseSpotPriceResponse {
    pub data: CoinBaseSpotPriceData,
}

impl Default for CoinBaseSpotPriceResponse {
    fn default() -> Self {
        Self {
            data: CoinBaseSpotPriceData::default(),
        }
    }
}

/// Struct for the coin base spot price data.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CoinBaseSpotPriceData {
    /// The amount.
    pub amount: String,
    /// The base currency.
    pub base: String,
    /// The quote currency.
    pub currency: String,
}

/// Struct for the counterparty response.
/// Has a vector of [`CounterParty`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CounterPartyResponse {
    pub data: Vec<CounterParty>,
}

impl Default for CounterPartyResponse {
    fn default() -> Self {
        Self {
            data: vec![CounterParty::default()],
        }
    }
}

impl CounterPartyResponse {
    /// Function that gets the counterparty id by passing in a ticker.
    pub fn get_id_by_ticker(&self, ticker: &str) -> Option<u16> {
        for cp in &self.data {
            if cp.ticker == ticker {
                return Some(cp.id);
            }
        }
        None
    }
    /// Function that gets the default expiry by passing in the id.
    pub fn get_default_expiry_by_id(&self, id: u16) -> Option<String> {
        for cp in &self.data {
            if cp.id == id {
                return Some(cp.default_expiry.clone());
            }
        }
        None
    }
}

/// Struct for the counterparty.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CounterParty {
    /// The ticker symbol for the counterparty.
    pub ticker: String,
    /// The name of the counterparty.
    pub name: String,
    /// The short name of the counterparty, if available.
    pub short_name: Option<String>,
    /// Indicates whether the counterparty is an exchange.
    pub is_exchange: bool,
    /// The unique identifier of the counterparty.
    pub id: u16,
    /// The default expiry of the counterparty.
    pub default_expiry: String,
}

impl Default for CounterParty {
    fn default() -> Self {
        Self {
            ticker: "".to_string(),
            name: "".to_string(),
            short_name: None,
            is_exchange: false,
            id: 0,
            default_expiry: String::from("10:00"),
        }
    }
}

/// Struct for the Estimate IV Request.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EstimateIVRequest {
    /// The option kind.
    pub option_kind: String,
    /// The currency.
    pub currency: String,
    /// The strike amount.
    pub strike: f64,
    /// The time to maturity.
    pub ttm: f64,
}

impl Default for EstimateIVRequest {
    fn default() -> Self {
        Self {
            option_kind: "Call".to_string(),
            currency: "BTC".to_string(),
            strike: 1.0,
            ttm: 6.85,
        }
    }
}

/// Struct for the Estimate IV Response.
/// Has a [`EstimatedIVData`].

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimateIVResponse {
    pub data: EstimatedIVData,
}

impl Default for EstimateIVResponse {
    fn default() -> Self {
        Self {
            data: EstimatedIVData::default(),
        }
    }
}

/// Struct for the Estimated IV Data.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimatedIVData {
    /// The estimated IV.
    pub estimated_iv: f64,
    /// The closest options. Has a vector of [`ClosestOption`].
    pub closest_options: Vec<ClosestOption>,
}

impl Default for EstimatedIVData {
    fn default() -> Self {
        Self {
            estimated_iv: 0.0,
            closest_options: vec![ClosestOption::default()],
        }
    }
}

/// Struct for the Closest Option.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosestOption {
    /// The instrument name.
    pub instrument_name: String,
    /// The mark IV.
    pub mark_iv: f64,
    /// The bid IV.
    pub bid_iv: f64,
    /// The ask IV.
    pub ask_iv: f64,
}

impl ClosestOption {
    pub fn new(instrument_name: String, mark_iv: f64, bid_iv: f64, ask_iv: f64) -> Self {
        Self {
            instrument_name,
            mark_iv,
            bid_iv,
            ask_iv,
        }
    }
}

impl Default for ClosestOption {
    fn default() -> Self {
        Self {
            instrument_name: "".to_string(),
            mark_iv: 0.0,
            bid_iv: 0.0,
            ask_iv: 0.0,
        }
    }
}

/// Struct for the Quote Option Request.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteOptionRequest {
    /// The option kind.
    pub option_kind: String,
    /// The amount.
    pub amount: f64,
    /// The strike.
    pub strike: f64,
    /// The time to maturity.
    pub ttm: f64,
    /// The spot, if available.
    pub spot: Option<f64>,
    /// The r2, if available.
    pub r2: Option<f64>,
    /// The r1, if available.
    pub r1: Option<f64>,
    /// The iv, if available.
    pub iv: Option<f64>,
    /// The side (buy or sell).
    pub side: String,
}

impl QuoteOptionRequest {
    pub fn new(
        option_kind: String,
        amount: f64,
        strike: f64,
        ttm: f64,
        spot: Option<f64>,
        r2: Option<f64>,
        r1: Option<f64>,
        iv: Option<f64>,
        side: String,
    ) -> Self {
        Self {
            option_kind,
            amount,
            strike,
            ttm,
            spot,
            r2,
            r1,
            iv,
            side,
        }
    }
}

/// Struct for the Quote Option Response.
/// Has a [`QouteOptionsData`].

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteOptionResponse {
    pub data: QouteOptionsData,
}

impl Default for QuoteOptionResponse {
    fn default() -> Self {
        Self {
            data: QouteOptionsData::default(),
        }
    }
}

/// Struct for the Quote Options Data.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QouteOptionsData {
    /// The base currency.
    pub px_in_base_ccy: f64,
    /// The quote currency.
    pub px_in_quote_ccy: f64,
    /// The greeks. Has a [`Greeks`].
    pub greeks: Greeks,
}

impl Default for QouteOptionsData {
    fn default() -> Self {
        Self {
            px_in_base_ccy: 0.0,
            px_in_quote_ccy: 0.0,
            greeks: Greeks::default(),
        }
    }
}

/// Struct for the Greeks.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Greeks {
    /// The delta.
    pub delta: f64,
    /// The gamma.
    pub gamma: f64,
    /// The theta.
    pub theta: f64,
}

/// Struct for the Quote Request.
/// Has a vector of [`Quote`].

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub data: Vec<Quote>,
}

/// Struct for the Quote Data.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub temp_id: String,
    pub counterparty_id: u16,
    pub pair_id: u16,
    pub ccy_id: u16,
    pub amount: f64,
    pub option_kind: String,
    pub ttm: f64,
    pub r2: f64,
    pub r1: f64,
    pub offstrike_percentage: f64,
    pub spot: f64,
    pub strike: f64,
    pub iv: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub side: String,
    pub quote_status: String,
    pub quote_origin: String,
    pub instrument_name: String,
    pub quote_expiry: String,
    pub gtc: bool,
    /// Generated as uuid.
    pub group_id: String, //uuid
    pub party_a: u16,
    pub party_b: u16,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub payout_ccy: String,
    pub expiry_timestamp: String,
}

impl Quote {
    pub fn new(
        temp_id: String,
        counterparty_id: u16,
        pair_id: u16,
        ccy_id: u16,
        amount: f64,
        option_kind: String,
        ttm: f64,
        r2: f64,
        r1: f64,
        offstrike_percentage: f64,
        spot: f64,
        strike: f64,
        iv: f64,
        px_in_base_ccy: f64,
        px_in_quote_ccy: f64,
        side: String,
        quote_status: String,
        quote_origin: String,
        instrument_name: String,
        quote_expiry: String,
        gtc: bool,
        group_id: String,
        party_a: u16,
        party_b: u16,
        delta: f64,
        gamma: f64,
        theta: f64,
        payout_ccy: String,
        expiry_timestamp: String,
    ) -> Self {
        Self {
            temp_id,
            counterparty_id,
            pair_id,
            ccy_id,
            amount,
            option_kind,
            ttm,
            r2,
            r1,
            offstrike_percentage,
            spot,
            strike,
            iv,
            px_in_base_ccy,
            px_in_quote_ccy,
            side,
            quote_status,
            quote_origin,
            instrument_name,
            quote_expiry,
            gtc,
            group_id,
            party_a,
            party_b,
            delta,
            gamma,
            theta,
            payout_ccy,
            expiry_timestamp,
        }
    }
}

/// Struct for the Add Quote Response.
/// Used in modals.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddQuoteResponse {
    pub success: bool,
    pub message: String,
}

impl Default for AddQuoteResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}
