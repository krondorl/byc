// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use byc::domain::bond::Bond;
use byc::math::yielder::{annual_coupon, current_yield, simple_yield};

const EPS: f64 = 1e-6;

/// Shared bond fixture
fn bond(coupon_rate: f64, clean_price: f64) -> Bond {
    Bond {
        isin: "TEST".into(),
        ticker: "TEST".into(),
        coupon_rate,
        maturity_date: "2030-01-01".into(),
        clean_price,
    }
}

mod annual_coupon_tests {
    use super::*;

    #[test]
    fn standard_coupon() {
        let bond = bond(0.05, 100.0);
        let result = annual_coupon(&bond);
        assert!((result - 5.0).abs() < EPS);
    }

    #[test]
    fn zero_coupon() {
        let bond = bond(0.0, 100.0);
        let result = annual_coupon(&bond);
        assert!((result - 0.0).abs() < EPS);
    }

    #[test]
    fn high_coupon() {
        let bond = bond(0.10, 100.0);
        let result = annual_coupon(&bond);
        assert!((result - 10.0).abs() < EPS);
    }
}

mod current_yield_tests {
    use super::*;

    #[test]
    fn par_price() {
        let bond = bond(0.05, 100.0);
        let result = current_yield(&bond);
        assert!((result - 0.05).abs() < EPS);
    }

    #[test]
    fn discount_price() {
        let bond = bond(0.05, 95.0);
        let result = current_yield(&bond);
        assert!((result - 0.0526315789).abs() < EPS);
    }

    #[test]
    fn premium_price() {
        let bond = bond(0.05, 105.0);
        let result = current_yield(&bond);
        assert!((result - 0.0476190476).abs() < EPS);
    }
}

mod simple_yield_tests {
    use super::*;

    #[test]
    fn at_par() {
        let bond = bond(0.05, 100.0);
        let result = simple_yield(&bond, 5.0);
        assert!((result - 0.05).abs() < EPS);
    }

    #[test]
    fn discount_bond() {
        let bond = bond(0.05, 95.0);
        let result = simple_yield(&bond, 5.0);
        assert!((result - 0.0615384615).abs() < EPS);
    }

    #[test]
    fn premium_bond() {
        let bond = bond(0.05, 105.0);
        let result = simple_yield(&bond, 5.0);
        assert!((result - 0.03902439024).abs() < EPS);
    }
}
