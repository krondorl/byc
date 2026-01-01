// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Bond {
    isin: String,
    ticker: String,
    coupon_rate: f64,
    maturity_date: String,
    clean_price: f64,
}

#[derive(Debug, Serialize)]
struct EnrichedBond {
    isin: String,
    ticker: String,
    coupon_rate: f64,
    maturity_date: String,
    clean_price: f64,
    current_yield: f64,
    simple_yield: f64,
}

fn main() {
    println!("byc");

    // parse CLI args
    // load inputs
    // call pure math
    // print results
}
