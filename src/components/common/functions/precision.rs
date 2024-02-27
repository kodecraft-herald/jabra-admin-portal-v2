use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


pub enum RoundType {
    Default,
    Ceiling,
    Floor
}

pub fn format_with_specs(amount: f64, tick_size: f64, order_size: f64, round_type: RoundType, allow_negative: bool) -> f64 {
    if !allow_negative {
        if amount < order_size {
            return order_size;
        }
    }
    // Get decimal precisions
    let amount_precision = get_precision(amount);
    let standard_precision = get_precision(tick_size);

    let converted_amount = convert_to_decimal(amount);
    let converted_min_amount = convert_to_decimal(tick_size);

    let divisible = Decimal::from_str("0.0").unwrap();

    // Check if amount meets the standard specs
    if (amount >= tick_size)
        && (amount_precision <= standard_precision)
        && (converted_amount % converted_min_amount == divisible)
    {
        // Return original amount if true
        println!("Return original amount");
        amount
    } else {
        // Perform Calculation if false
        let value: f64 = amount / tick_size;
        let rounded_value = match round_type {
            RoundType::Default => ((value).round()) * tick_size,
            RoundType::Ceiling => ((value).ceil()) * tick_size,
            RoundType::Floor => ((value).floor()) * tick_size,
        };
        let rounded_value = format!("{:.standard_precision$}", rounded_value)
            .parse::<f64>()
            .unwrap();

        rounded_value
    }
}

pub fn get_precision(decimal: f64) -> usize {
    let decimal = decimal.to_string().split(".").last().unwrap().len();
    decimal
}

pub fn convert_to_decimal(decimal: f64) -> Decimal {
    Decimal::from_str(decimal.to_string().as_str()).unwrap()
}

// pub fn format_to_specs(amount: f64, tick_size: f64, order_size: f64) -> f64 {
//     if amount < order_size {
//         return order_size;
//     }
//     // Get decimal precisions
//     let amount_precision = get_precision(amount);
//     let standard_precision = get_precision(tick_size);

//     let converted_amount = convert_to_decimal(amount);
//     let converted_min_amount = convert_to_decimal(tick_size);

//     let divisible = Decimal::from_str("0.0").unwrap();

//     // Check if amount meets the standard specs
//     if (amount >= tick_size)
//         && (amount_precision <= standard_precision)
//         && (converted_amount % converted_min_amount == divisible)
//     {
//         // Return original amount if true
//         println!("Return original amount");
//         amount
//     } else {
//         // Perform Calculation if false
//         let value: f64 = amount / tick_size;
//         let rounded_value = ((value).floor()) * tick_size;
//         let rounded_value = format!("{:.standard_precision$}", rounded_value)
//             .parse::<f64>()
//             .unwrap();
//         // println!("Return floored amount");

//         rounded_value
//     }
// }

// pub fn format_to_specs_neg(amount: f64, tick_size: f64, _order_size: f64) -> f64 {
//     // if amount < order_size {
//     //     return order_size;
//     // }
//     // Get decimal precisions
//     let amount_precision = get_precision(amount);
//     let standard_precision = get_precision(tick_size);

//     let converted_amount = convert_to_decimal(amount);
//     let converted_min_amount = convert_to_decimal(tick_size);

//     let divisible = Decimal::from_str("0.0").unwrap();

//     // Check if amount meets the standard specs
//     if (amount >= tick_size)
//         && (amount_precision <= standard_precision)
//         && (converted_amount % converted_min_amount == divisible)
//     {
//         // Return original amount if true
//         println!("Return original amount");
//         amount
//     } else {
//         // Perform Calculation if false
//         let value: f64 = amount / tick_size;
//         let rounded_value = ((value).floor()) * tick_size;
//         let rounded_value = format!("{:.standard_precision$}", rounded_value)
//             .parse::<f64>()
//             .unwrap();
//         // println!("Return floored amount");

//         rounded_value
//     }
// }

// Same as format_to_specs but instead of flooring the amount, it rounds it.

pub fn format_to_specs_round(amount: f64, tick_size: f64, _order_size: f64) -> f64 {
    // if amount < order_size {
    //     return order_size;
    // }
    // Get decimal precisions
    let amount_precision = get_precision(amount);
    let standard_precision = get_precision(tick_size);

    let converted_amount = convert_to_decimal(amount);
    let converted_min_amount = convert_to_decimal(tick_size);

    let divisible = Decimal::from_str("0.0").unwrap();

    // Check if amount meets the standard specs
    if (amount >= tick_size)
        && (amount_precision <= standard_precision)
        && (converted_amount % converted_min_amount == divisible)
    {
        // Return original amount if true
        println!("Return original amount");
        amount
    } else {
        // Perform Calculation if false
        let value: f64 = amount / tick_size;
        let rounded_value = ((value).round()) * tick_size;
        let rounded_value = format!("{:.standard_precision$}", rounded_value)
            .parse::<f64>()
            .unwrap();
        // println!("Return floored amount");
       
        rounded_value
    }
}

//Instrument Specs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstrumentSpecsResponse {
    pub data: Vec<InstrumentSpecs>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentSpecs {
    pub instrument_type: String,
    pub contract_multiplier: f64,
    pub min_price_increment: f64,
    pub min_contract_increment: f64,
    pub ccy_id: InstrumentSpecTickerName,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentSpecTickerName {
    pub ticker: String,
}

impl InstrumentSpecsResponse {
    pub fn find_by_ticker_and_instrument_type(
        &self,
        ticker: &str,
        instrument_type: &str,
    ) -> Option<&InstrumentSpecs> {
        for specs in &self.data {
            if specs.ccy_id.ticker == ticker && specs.instrument_type == instrument_type {
                return Some(specs);
            }
        }
        None
    }

    pub fn get_limit_and_precision_by_currency(
        &self,
        ticker: &str,
        instrument_type: &str,
    ) -> LimitAndPrecision {
        let specs = self.find_by_ticker_and_instrument_type(ticker, instrument_type);
        match specs {
            Some(s) => {
                let limit = s.min_price_increment;
                let precision = s.min_contract_increment;
                LimitAndPrecision {
                    tick_size: limit,
                    order_size: precision,
                }
            }
            None => LimitAndPrecision::default(),
        }
    }
    pub fn get_limit_and_precision_by_currency_with_default(
        &self,
        ticker: &str,
        instrument_type: &str,
        tick_size: f64,
        order_size: f64,
    ) -> LimitAndPrecision {
        let specs = self.find_by_ticker_and_instrument_type(ticker, instrument_type);
        match specs {
            Some(s) => {
                let limit = s.min_price_increment;
                let precision = s.min_contract_increment;
                LimitAndPrecision {
                    tick_size: limit,
                    order_size: precision,
                }
            }
            None => LimitAndPrecision::new(tick_size, order_size),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitAndPrecision {
    pub tick_size: f64,  //decimal precision, price increment
    pub order_size: f64, //contract increment / minimum order size
}

impl LimitAndPrecision {
    pub fn new(tick_size: f64, order_size: f64) -> Self {
        Self {
            tick_size,
            order_size,
        }
    }
}

impl Default for LimitAndPrecision {
    fn default() -> Self {
        Self {
            tick_size: 0.0001,
            order_size: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let amount = 0.00001;
        let min_amount = 0.1;
        let order_size = 0.1;
        let result = format_with_specs(amount, min_amount, order_size, RoundType::Floor, false);
        assert_eq!(result, 0.1);
    }
    #[test]
    fn test2() {
        let amount = 1.3;
        let min_amount = 0.2;
        let order_size = 0.1;
        let result = format_with_specs(amount, min_amount, order_size, RoundType::Floor, false);
        assert_eq!(result, 1.2);
    }
    #[test]
    fn test3() {
        let amount = 13.5;
        let min_amount = 0.1;
        let order_size = 0.1;
        let result = format_with_specs(amount, min_amount, order_size, RoundType::Floor, false);
        assert_eq!(result, 13.5);
    }
    #[test]
    fn test4() {
        let amount = 1234.123456789;
        let min_amount = 0.0001;
        let order_size = 0.1;
        let result = format_with_specs(amount, min_amount, order_size, RoundType::Floor, false);
        assert_eq!(result, 1234.1234);
    }
    #[test]
    fn test5() {
        let amount = 5.7;
        let min_amount = 1.0;
        let order_size = 0.1;
        let result = format_with_specs(amount, min_amount, order_size, RoundType::Floor, false);
        assert_eq!(result, 5.0);
    }
}
