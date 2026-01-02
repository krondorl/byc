// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use anyhow::Context;

/// Calculates remaining years to maturity.
///
/// Uses 365 days + 1 leap day every 4 years
/// This ACT/365.25 is widely used for: bonds, swaps, risk models, analytics
pub fn years_to_maturity_at(
    maturity_date: &str,
    valuation_date: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<f64> {
    let maturity = chrono::NaiveDate::parse_from_str(maturity_date, "%Y-%m-%d")
        .with_context(|| format!("invalid maturity_date '{}'", maturity_date))?
        .and_hms_opt(0, 0, 0)
        .context("invalid time")?;

    let maturity =
        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(maturity, chrono::Utc);

    let millis_per_year = 1000.0 * 60.0 * 60.0 * 24.0 * 365.25;
    let diff = (maturity - valuation_date).num_milliseconds() as f64;

    Ok(diff / millis_per_year)
}
