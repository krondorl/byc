// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

/// Calculates remaining years to maturity.
///
/// Hint:
/// - Parse maturity_date (ISO-8601)
/// - Compare with current date
pub fn years_to_maturity_at(
    maturity_date: &str,
    valuation_date: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<f64> {
    let maturity = chrono::DateTime::parse_from_rfc3339(maturity_date)?.with_timezone(&chrono::Utc);

    let millis_per_year = 1000.0 * 60.0 * 60.0 * 24.0 * 365.25;
    let diff = (maturity - valuation_date).num_milliseconds() as f64;

    Ok(diff / millis_per_year)
}
