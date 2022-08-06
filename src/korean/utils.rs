const KRW_PER_USD: f64 = 1_303.74;

/// Converts Korean Won to US Dollars
/// # Examples
/// ```
/// let usd = kd::korean::utils::krw_to_usd(10_000_000_000.0);
/// assert_eq!(usd, 7_670_240.998972188)
pub fn krw_to_usd(won: f64) -> f64 {
    won / KRW_PER_USD
}