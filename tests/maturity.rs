// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use byc::time::maturity::years_to_maturity_at;

fn days_from_years(years: f64) -> i64 {
    (years * 365.25 * 24.0 * 60.0 * 60.0 * 1000.0) as i64
}

#[test]
fn maturity_in_the_past() {
    let now = chrono::Utc::now();
    let one_year_ago = now - chrono::Duration::days(365);

    let result = years_to_maturity_at(&one_year_ago.to_rfc3339(), now).unwrap();

    assert!(result < 0.0);
}

#[test]
fn maturity_in_one_year() {
    let now = chrono::Utc::now();
    let maturity = now + chrono::Duration::milliseconds(days_from_years(1.0));

    let result = years_to_maturity_at(&maturity.to_rfc3339(), now).unwrap();

    assert!((result - 1.0).abs() < 1e-9);
}

#[test]
fn maturity_far_future() {
    let now = chrono::Utc::now();
    let maturity = now + chrono::Duration::milliseconds(days_from_years(10.0));

    let result = years_to_maturity_at(&maturity.to_rfc3339(), now).unwrap();

    assert!((result - 10.0).abs() < 1e-9);
}
