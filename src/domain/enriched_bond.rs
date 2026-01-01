// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EnrichedBond {
    pub isin: String,
    pub ticker: String,
    pub coupon_rate: f64,
    pub maturity_date: String,
    pub clean_price: f64,
    pub current_yield: f64,
    pub simple_yield: f64,
}
