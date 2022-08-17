use std::error::Error;

use serde_json::Value;

/// Converts Korean Won to US Dollars
/// # Examples
/// ```
/// use kd::korean::utils;
/// let usd = utils::krw_to_usd(10_000_000_000.0);
pub fn krw_to_usd(won: f64) -> Result<f64, Box<dyn Error>> {
    let forex_rate = get_krw_to_usd_forex_rate()?;
    Ok(won * forex_rate)
}

pub fn get_krw_to_usd_forex_rate() -> Result<f64, Box<dyn Error>> {
    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")?;
    // Get JSON response text
    let url = format!("https://www.alphavantage.co/query?function=CURRENCY_EXCHANGE_RATE&from_currency=KRW&to_currency=USD&apikey={}", api_key);
    let body = reqwest::blocking::get(url)?.text()?;

    // Parse JSON and get rate as string
    let json: Value = serde_json::from_str(&body)?;
    let rate = match json.get("Realtime Currency Exchange Rate") {
        Some(value) => match value.get("5. Exchange Rate") {
            Some(value) => match value.as_str() {
                Some(value) => value,
                None => return Err("none".into()),
            },
            None => return Err("Nope".into()),
        },
        None => {
            println!("response: {body}");
            return Err("Response not found".into())
    },
    };

    // Return as float
    match rate.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => return Err("Not a number".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::get_krw_to_usd_forex_rate;

    #[test]
    fn should_get_rate() {
        let rate = get_krw_to_usd_forex_rate().expect("should be a number");
        dbg!(rate);
    }
}
