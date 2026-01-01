// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

/// Calculates remaining years to maturity.
///
/// Hint:
/// - Parse maturity_date (ISO-8601)
/// - Compare with current date
pub fn years_to_maturity(maturity_date: &str) -> f64 {
    let maturity = chrono::DateTime::parse_from_rfc3339(maturity_date)
        .expect("Invalid date format")
        .with_timezone(&chrono::Utc);
    let millis_per_day = (1000 * 60 * 60 * 24) as f64;
    let millis_per_year = millis_per_day * 365.25;
    let today = chrono::Utc::now();
    let diff_millis = (maturity - today).num_milliseconds() as f64;
    diff_millis / millis_per_year
}
