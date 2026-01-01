use byc::time::maturity::years_to_maturity;

const EPS: f64 = 1e-3; // looser tolerance due to time passing

#[test]
fn maturity_in_one_year() {
    let one_year_later = (chrono::Utc::now() + chrono::Duration::days(365)).to_rfc3339();

    let result = years_to_maturity(&one_year_later);

    assert!((result - 1.0).abs() < EPS);
}

#[test]
fn maturity_in_the_past() {
    let one_year_ago = (chrono::Utc::now() - chrono::Duration::days(365)).to_rfc3339();

    let result = years_to_maturity(&one_year_ago);

    assert!(result < 0.0);
}

#[test]
fn maturity_far_future() {
    let ten_years_later = (chrono::Utc::now() + chrono::Duration::days(365 * 10)).to_rfc3339();

    let result = years_to_maturity(&ten_years_later);

    assert!((result - 10.0).abs() < 0.01);
}
