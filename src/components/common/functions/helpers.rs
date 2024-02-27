use crate::components::common::components::{common_attributes::ComponentSize, common_icons::Icon};
use chrono::{
    DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
    Timelike, Utc,
};
use currency_rs::{Currency, CurrencyOpts};
use leptos::*;
use num_format::{Locale, ToFormattedString};

pub fn get_input_size_class(size: ComponentSize, base_class: String) -> String {
    match size {
        ComponentSize::ExtraSmall => format!("{} input-xs", base_class.clone()),
        ComponentSize::Small => format!("{} input-sm", base_class.clone()),
        ComponentSize::Base => format!("{} input-md", base_class.clone()),
        ComponentSize::Large => format!("{} input-lg", base_class.clone()),
    }
}

pub fn join_icon(component: View, icon: String, icon_size: ComponentSize) -> View {
    if icon.is_empty() {
        return component.into_view();
    } else {
        let icon_size = match icon_size {
            ComponentSize::ExtraSmall => "h-3 w-3".to_string(),
            ComponentSize::Small => "h-4 w-4".to_string(),
            ComponentSize::Base => "h-5 w-5".to_string(),
            ComponentSize::Large => "h-6 w-6".to_string(),
        };
        view! {
            <div class="join">
                <div class="rounded-md bg-success join-item flex items-center p-3">
                    <Icon
                        title=icon.clone()
                        size=icon_size
                    />
                </div>
                {component}
            </div>
        }
        .into_view()
    }
}

pub fn change_day(dt: DateTime<Utc>, new_day: String) -> Option<DateTime<Utc>> {
    // Parse the new_day string to extract year, month, and day
    if let Ok(parsed_date) = NaiveDate::parse_from_str(&new_day, "%Y-%m-%d") {
        // Extract the time components from the original DateTime
        let time_components = dt.time();

        // Create a new DateTime with the parsed date and extracted time components
        let new_datetime =
            DateTime::from_naive_utc_and_offset(parsed_date.and_time(time_components), Utc);
        // println!("New datetime: {}", new_datetime);
        Some(new_datetime)
    } else {
        None
    }
}

pub fn extract_date_as_string(dt: DateTime<Utc>) -> String {
    // Get the date part from the DateTime
    let date_part = dt.date_naive();

    // Format the date part into a string
    let formatted_date = format!("{}", date_part);
    println!("Formatted exp: {}", formatted_date);
    formatted_date
}

pub fn calculate_time_difference(
    date_created: Option<String>,
    expiry: String,
    gtc: bool,
) -> String {
    if gtc {
        return "Good Till Canceled".to_string();
    }
    let start = match date_created {
        Some(s) => parse_timestamp(&s),
        None => Some(Utc::now().naive_utc()),
    };
    let end = parse_timestamp(&expiry);

    match (start, end) {
        (Some(s), Some(e)) => {
            let duration = e.signed_duration_since(s);

            if duration.num_seconds() < 0 {
                return "Expired".to_string();
            }

            let hours = duration.num_hours();
            let minutes = (duration.num_minutes() - (hours * 60)).abs();
            let seconds = (duration.num_seconds() - (minutes * 60)).abs();

            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
        _ => "Expired".to_string(),
    }
}

pub fn get_expiry(expiry_in_minutes: u16) -> String {
    // if expiry_in_minutes == 0 {
    //     return "".to_string();
    // }
    let current_time = Utc::now();
    let expiry_time = current_time + Duration::minutes(i64::from(expiry_in_minutes));
    let formatted_time = expiry_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    formatted_time
}

pub fn parse_timestamp(timestamp: &str) -> Option<NaiveDateTime> {
    // Define the expected timestamp formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.3fZ", // First format with milliseconds
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S", // First format
        "%Y-%m-%dT%H:%M",    // Second format without milliseconds
    ];

    // Attempt to parse the timestamp using each format
    for format in &formats {
        if let Ok(parsed_dt) = NaiveDateTime::parse_from_str(timestamp, format) {
            return Some(parsed_dt);
        }
    }

    None // Return None if parsing failed with all formats
}

#[cfg(feature = "ssr")]
pub fn enc(plain_text: String) -> String {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    let encryption_key = std::env::var("JABRAKEY").unwrap();
    let magic_crypt = new_magic_crypt!(encryption_key, 256);
    magic_crypt.encrypt_str_to_base64(plain_text)
}
#[cfg(feature = "ssr")]
pub fn dec(encrypted_text: String) -> String {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    let encryption_key = std::env::var("JABRAKEY").unwrap();
    let magic_crypt = new_magic_crypt!(encryption_key, 256);
    magic_crypt
        .decrypt_base64_to_string(encrypted_text)
        .unwrap()
}
pub fn transform_string(input: &str) -> String {
    // Split the input string into parts
    let parts: Vec<&str> = input.split('-').collect();

    // Extract components from the parts
    let symbol = parts[0];
    let date = parts[1];
    let amount = parts[2]
        .parse::<i32>()
        .unwrap()
        .to_formatted_string(&Locale::en);
    let option_type = match parts[3].chars().next().unwrap() {
        'C' => "Call",
        'P' => "Put",
        _ => panic!("Invalid option type"),
    };

    // Format the transformed string
    let transformed_string = format!("{} - {} - ${} - {}", symbol, date, amount, option_type);

    transformed_string
}

pub fn extract_date(date_str: String) -> String {
    // Parse the input date string
    let parsed_date = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ")
        .expect("Invalid date format");
    // Format the date as "Mon Day Year"
    let formatted_date = parsed_date.format("%b %d %Y").to_string();
    formatted_date
}

pub fn format_date(date_str: String) -> String {
    let formats = [
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S.%3fZ",
        "%Y-%m-%d %H:%M:%S.%6f+00",
        "%Y-%m-%d %H:%M:%S+00",
    ];

    for f in &formats {
        if let Ok(d) = NaiveDateTime::parse_from_str(date_str.as_str(), f) {
            return d.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
    String::from("")
}

pub fn format_utc_str_to_local_str(utc_date_str: String) -> String {
    let utc_str_to_naive_dt =
        NaiveDateTime::parse_from_str(utc_date_str.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ");

    // log::info!("utc_date_str : {:?}", utc_date_str);

    // log::info!("utc_str_to_naive_dt : {:?}", utc_str_to_naive_dt);

    if let Ok(d) = utc_str_to_naive_dt {
        let utc = d.and_utc();
        let utc_local = utc.with_timezone(&Local);
        let formatted_local = utc_local.format("%Y-%m-%d %H:%M:%S").to_string();

        return formatted_local;
    }

    String::from("")
}

pub fn format_currency(number: f64, scale: u8) -> String {
    format!("{:.*}", usize::from(scale), number)
}
pub fn format_money(value: String, separator: &str, precision: u8) -> String {
    let opts: CurrencyOpts = CurrencyOpts::new()
        .set_separator(separator)
        .set_symbol("")
        .set_decimal(".");
    let opts = opts.set_precision(precision as i64);
    let currency = Currency::new_string(value.as_str(), Some(opts)).unwrap();
    currency.format()
}

pub fn format_currency_with_scale(number: f64, scale: u8, separator: &str) -> String {
    let value = format_currency(number, scale);
    format_money(value, separator, scale)
}

pub fn format_date_from_utc(date_time: DateTime<Utc>) -> String {
    let formatted_time = date_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    formatted_time
}
pub fn format_date_utc(date_time: DateTime<Utc>) -> String {
    let formatted_time = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
    formatted_time
}

pub fn parse_string_to_utc(input: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    // let date_time = DateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S");
    let naive_date_time = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S");

    match naive_date_time {
        Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
        Err(e) => Err(e),
    }
}

pub fn format_date_for_trades(date_time: DateTime<Utc>) -> String {
    let formatted_time = date_time.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
    formatted_time
}

pub fn format_date_offset(date_time: DateTime<FixedOffset>) -> String {
    let formatted_time = date_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    formatted_time
}

pub fn format_date_offset_for_trades(date_time: DateTime<FixedOffset>) -> String {
    let formatted_time = date_time.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
    formatted_time
}

pub fn get_expiration_datetime_utc(
    start_date: Option<DateTime<Utc>>,
    expiration_in_day: f64,
) -> DateTime<Utc> {
    // Calculate the total seconds (1 day = 86400 seconds)
    let seconds_in_1_day: f64 = 86400.0;
    let total_seconds: i64 = (seconds_in_1_day * expiration_in_day) as i64;

    // Create a duration of the calculated seconds
    let expiration_duration: Duration = Duration::seconds(total_seconds);

    // Add the duration to the current date
    match start_date {
        Some(dt) => dt + expiration_duration,
        None => Utc::now() + expiration_duration,
    }
    // let expiration_date: DateTime<Utc> = start_date + expiration_duration;

    // expiration_date
}

pub fn get_trade_expiration_datetime(
    start_date: Option<NaiveDateTime>,
    expiration_in_day: f64,
) -> NaiveDateTime {
    // Calculate the total seconds (1 day = 86400 seconds)
    let seconds_in_1_day: f64 = 86400.0;
    let total_seconds: i64 = (seconds_in_1_day * expiration_in_day) as i64;

    // Create a duration of the calculated seconds
    let expiration_duration: Duration = Duration::seconds(total_seconds);

    // Add the duration to the current date
    match start_date {
        Some(dt) => dt + expiration_duration,
        None => Local::now().naive_local() + expiration_duration,
    }
}

pub fn parse_str_to_local_datetime(input: &str) -> NaiveDateTime {
    let formats = [
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S", // First format
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S.%3fZ", // Second format without seconds
    ];
    for format in &formats {
        if let Ok(parsed_dt) = NaiveDateTime::parse_from_str(input, format) {
            return parsed_dt;
        }
    }
    Local::now().naive_local()
    // String::from("")
    // let date_time = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S");
    // match date_time {
    //     Ok(dt) => dt,
    //     _ => Local::now().naive_local(),
    // }
}
pub fn parse_str_to_utc_datetime_str(input: &str) -> String {
    // let date_time = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S");
    // match date_time {
    //     Ok(dt) => {
    //         let utc = Utc.from_utc_datetime(&dt);
    //         let formatted_datetime = utc.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    //         formatted_datetime
    //     }
    //     _ => "".to_string(),
    // }
    let formats = [
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S", // First format
        "%Y-%m-%dT%H:%M",    // Second format without seconds
    ];

    // Attempt to parse the timestamp using each format
    for format in &formats {
        if let Ok(parsed_dt) = NaiveDateTime::parse_from_str(input, format) {
            let utc: NaiveDateTime = Local.from_local_datetime(&parsed_dt).unwrap().naive_utc();
            // let utc = Utc.from_utc_datetime(&parsed_dt);
            let formatted_datetime = utc.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            return formatted_datetime;
        }
    }

    String::from("")
}

pub fn parse_local_datetime_to_str(input: NaiveDateTime) -> String {
    let formatted_time = input.format("%Y-%m-%dT%H:%M:%S").to_string();
    formatted_time
}

// pub fn convert_datetime(expiry: &str, time: &str) -> String {
//     println!("Expiry: {:?}", expiry);
//     // Parse the input expiry string into a DateTime object
//     let expiry_datetime = NaiveDateTime::parse_from_str(expiry, "%Y-%m-%d %H:%M:%S")
//         .expect("Invalid expiry date format");

//     // Convert the expiry date to UTC
//     let expiry_utc = Local.from_local_datetime(&expiry_datetime).unwrap().naive_utc();
//     println!("Expiry UTC: {:?}", expiry_utc);
//     // Extract the date part from the converted expiry date
//     let expiry_date = expiry_utc.date();

//     // Parse the input time string into a NaiveTime object
//     let parsed_time = NaiveTime::parse_from_str(time, "%H:%M").expect("Invalid time format");

//     // Combine the extracted date with the parsed time
//     let combined_datetime = expiry_date
//         .and_time(parsed_time)
//         .checked_add_signed(Duration::hours(0)) // Adjust the offset if needed
//         .expect("Invalid time format");

//     let combined_local = combined_datetime.and_local_timezone(Local);

//     // Format the final result as a string
//     let formatted_result = combined_local.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
//     println!("Formatted exp: {}", formatted_result);
//     formatted_result
// }

pub fn get_traders_preffered_expiry(expiry: &str, time: &str) -> String {
    //Parse str to naive datetime
    let expiry_datetime = parse_timestamp(expiry).expect("Invalid expiry date format");
    //Get the date
    let expiry_date = expiry_datetime.date();
    //Parse time to naive time
    let parsed_time = NaiveTime::parse_from_str(time, "%H:%M:%S").expect("Invalid time format");
    let combined_datetime = expiry_date
        .and_time(parsed_time)
        .checked_add_signed(Duration::hours(0)) // Adjust the offset if needed
        .expect("Invalid time format");
    let formatted_result = combined_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    // println!("Formatted exp: {}", formatted_result);
    formatted_result
}

pub fn convert_utc_to_edt(dt: DateTime<Utc>) -> DateTime<FixedOffset> {
    let edt_offset = FixedOffset::east_opt(-4 * 3600); // EDT offset is UTC-4
    dt.with_timezone(&edt_offset.unwrap())
}

pub fn format_edt_date_time(dt: DateTime<FixedOffset>) -> String {
    let formatted_datetime = dt.format("%Y-%m-%d %H:%M:%S EST").to_string();
    println!("New EDT: {}", formatted_datetime);
    formatted_datetime
}

pub fn update_edt_time(dt: DateTime<FixedOffset>, new_time: &str) -> DateTime<FixedOffset> {
    // Parse the new_time string
    let new_time_parts: Vec<&str> = new_time.split(':').collect();
    if new_time_parts.len() != 3 {
        panic!("Invalid time format");
    }

    let new_hour: u32 = new_time_parts[0].parse::<u32>().expect("Invalid hour");
    let new_minute: u32 = new_time_parts[1].parse::<u32>().expect("Invalid minute");
    let new_second: u32 = new_time_parts[2].parse::<u32>().expect("Invalid second");

    // Create a new DateTime with the updated time
    let result = dt
        .with_hour(new_hour)
        .expect("Invalid hour")
        .with_minute(new_minute)
        .expect("Invalid minute")
        .with_second(new_second)
        .expect("Invalid second");
    result
}

pub fn format_iso_date_reduce(date: String) -> String {
    // Parse the input string as a DateTime object
    let parsed_datetime = DateTime::parse_from_rfc3339(&date);
    match parsed_datetime {
        Ok(dt) => {
            // Format the DateTime object as a string
            let formatted_datetime = dt.format("%Y-%m-%d %H:%M:%S").to_string();
            // println!("{}", formatted_datetime);
            formatted_datetime
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            "".to_string()
        }
    }
}

pub fn create_trade_expiry_in_utc(datetime: String, time: String) -> String {
    let expiry_datetime = parse_timestamp(&datetime);
    match expiry_datetime {
        Some(ex) => {
            let expiry_date = ex.date();
            let parsed_time =
                NaiveTime::parse_from_str(&time, "%H:%M:%S").expect("Invalid time format");
            let combined_datetime = expiry_date
                .and_time(parsed_time)
                .checked_add_signed(Duration::hours(0)) // Adjust the offset if needed
                .expect("Invalid time format");
            // let formatted_result = combined_datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
            // // println!("Formatted exp: {}", formatted_result);
            // formatted_result
            // Convert combined_datetime to UTC
            let combined_utc_datetime = TimeZone::from_utc_datetime(&Utc, &combined_datetime);

            let formatted_result = combined_utc_datetime
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string();
            // println!("Formatted exp: {}", formatted_result);
            formatted_result
        }
        _ => {
            println!("Formatted exp: {}", datetime);
            datetime
        }
    }
}

pub fn get_trade_expiry_date(datetime: String) -> String {
    let expiry_datetime = parse_timestamp(&datetime);
    match expiry_datetime {
        Some(ex) => {
            let expiry_date = ex.date();
            expiry_date.to_string()
        }
        _ => datetime,
    }
}

pub fn is_date_greater_than_24_hrs(date: String) -> bool {
    // Parse the input string as a DateTime object
    let date = date + ".000Z";
    // log::info!("DT: {:?}", date);
    let parsed_datetime = DateTime::parse_from_rfc3339(&date);
    // log::info!("Parsed DT: {:?}", parsed_datetime);
    match parsed_datetime {
        Ok(dt) => {
            // Get the current UTC time
            let current_time = Utc::now();

            // Calculate the difference in hours
            let time_difference = current_time.signed_duration_since(dt);

            //    log::info!("Time Difference (hours): {:?}", time_difference.num_hours());

            // Check if the difference is greater than 24 hours
            if time_difference.num_hours() > 24 {
                // log::info!("Greater than 24 hours");
                return true;
            } else {
                // log::info!("Less than 24 hours");
                return false;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            false
        }
    }
}

///Get conditional Loss
pub fn get_conditional_loss(
    option_kind: String,
    stop_loss_level: f64,
    ccy1_amount: f64,
    ccy2_premium: f64,
    strike: f64,
    counterparty_name: String,
    currency: String,
) -> String {
    if option_kind == "Call".to_string() {
        format! {
            "If the value of the {:.2} CE exceeds ${:.2} for {:.2} {} notional, JABRA TRADING LLC will execute a market order and terminate the contract early. {} will owe the difference between the closeout price and ${:.2} to JABRA TRADING LLC.",
            strike, stop_loss_level, ccy1_amount, currency, counterparty_name, ccy2_premium
        }
    } else {
        format! {
            "If the value of the {:.2} PE exceeds ${:.2} for {:.2} {} notional, JABRA TRADING LLC will execute a market order and terminate the contract early. {} will owe the difference between the closeout price and ${:.2} to JABRA TRADING LLC.",
            strike, stop_loss_level, ccy1_amount, currency, counterparty_name, ccy2_premium
        }
    }
}

//Simple encryption and decryption
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn encrypt(plain_text: String) -> String {
    let magic_crypt = new_magic_crypt!("U2VjcmV0Q29kZTEyMzQ", 256);
    magic_crypt.encrypt_str_to_base64(plain_text)
}

pub fn decrypt(encrypted_text: String) -> String {
    let magic_crypt = new_magic_crypt!("U2VjcmV0Q29kZTEyMzQ", 256);
    magic_crypt
        .decrypt_base64_to_string(encrypted_text)
        .unwrap()
}

pub fn format_number_en(number_str: String, precision: usize) -> String {
    let test_number = number_str.parse::<f64>();
    if test_number.is_err() {
        return number_str;
    }
    let parts = number_str.split('.').collect::<Vec<&str>>();
    if parts.len() < 2 {
        let f = parts[0].to_string().parse::<i64>().unwrap();
        let formatted_f = f.to_formatted_string(&Locale::en);
        let decimal_places = format!("{:.*}", precision, "000000000");
        let concat = format!("{}.{}", formatted_f, decimal_places);
        concat
    } else {
        let f = parts[0].to_string().parse::<i64>().unwrap();
        let formatted_f = f.to_formatted_string(&Locale::en);
        let decimal_places = format!("{:.*}", precision, parts[1].to_string());
        let concat = format!("{}.{}", formatted_f, decimal_places);
        concat
    }
}

pub fn convert_utc_to_local(timestamp: &str) -> String {
    let date_time = parse_timestamp(timestamp);
    match date_time {
        Some(dt) => {
            // let utc = Utc.offset_from_utc_datetime(&dt);
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%Y-%m-%d %H:%M:%S").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}

pub fn generate_instrument_name(
    base_ccy: String,
    ttm: f64,
    strike: f64,
    option_kind: String,
) -> String {
    let seconds_in_1_day: f64 = 86400.0;
    let total_seconds: i64 = (seconds_in_1_day * ttm) as i64;
    let expiration_duration: Duration = Duration::seconds(total_seconds);
    let expiration_date: DateTime<Utc> = Utc::now() + expiration_duration;
    let formatted_date = expiration_date.format("%e%b%y").to_string();
    let truncated_strike = strike as i64;

    let option_kind_val = if !option_kind.is_empty() {
        option_kind.chars().next().unwrap().to_string()
    } else {
        "".to_string()
    };
    let instrument_name = format!(
        "{}-{}-{}-{}",
        base_ccy,
        formatted_date.trim(),
        truncated_strike,
        option_kind_val
    )
    .to_uppercase();
    instrument_name
}
pub fn generate_instrument_name_v2(
    base_ccy: String,
    expiry_datetime: String,
    strike: f64,
    option_kind: String,
) -> String {
    // let seconds_in_1_day: f64 = 86400.0;
    // let total_seconds: i64 = (seconds_in_1_day * ttm) as i64;
    // let expiration_duration: Duration = Duration::seconds(total_seconds);
    let expiration_date = parse_str_to_local_datetime(expiry_datetime.as_str()).and_utc();
    let formatted_date = expiration_date.format("%e%b%y").to_string();
    let truncated_strike = strike as i64;

    let option_kind_val = if !option_kind.is_empty() {
        option_kind.chars().next().unwrap().to_string()
    } else {
        "".to_string()
    };
    let instrument_name = format!(
        "{}-{}-{}-{}",
        base_ccy,
        formatted_date.trim(),
        truncated_strike,
        option_kind_val
    )
    .to_uppercase();
    instrument_name
}

///Accepts Date time in UTC and convert to local time to get date_time
pub fn get_datetime_in_local_time(utc_date_time: String) -> String {
    let utc_date_time = parse_timestamp(&utc_date_time);
    match utc_date_time {
        Some(dt) => {
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%Y-%m-%d %H:%M:%S").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}
///Accepts Date time in UTC and convert to local time to get date
pub fn get_date_in_local_time(utc_date_time: String) -> String {
    let utc_date_time = parse_timestamp(&utc_date_time);
    match utc_date_time {
        Some(dt) => {
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%Y-%m-%d").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}
///Accepts Date time in UTC and convert to local time to get time
pub fn get_time_in_local_time(utc_date_time: String) -> String {
    let utc_date_time = parse_timestamp(&utc_date_time);
    match utc_date_time {
        Some(dt) => {
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%H:%M:%S").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}
