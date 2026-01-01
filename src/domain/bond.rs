use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bond {
    pub isin: String,
    pub ticker: String,
    pub coupon_rate: f64,
    pub maturity_date: String,
    clean_price: f64,
}
