// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use crate::domain::bond::Bond;
use crate::math::constant::FACE_VALUE;

/// Annual Coupon Payment
pub fn annual_coupon(bond: &Bond) -> f64 {
    bond.coupon_rate * FACE_VALUE
}

/// Calculates Current Yield.
///
/// Formula:
/// CurrentYield = AnnualCoupon / CleanPrice
pub fn current_yield(bond: &Bond) -> f64 {
    annual_coupon(bond) / bond.clean_price
}

/// Calculates Simple Yield (approximate YTM).
///
/// Formula:
/// ( C + (FaceValue - Price) / n ) / ( (FaceValue + Price) / 2 )
pub fn simple_yield(bond: &Bond, years_to_maturity: f64) -> f64 {
    let coupon = annual_coupon(bond);
    let capital_gain_per_year = (FACE_VALUE - bond.clean_price) / years_to_maturity;
    let average_price = (FACE_VALUE + bond.clean_price) / 2.0;
    (coupon + capital_gain_per_year) / average_price
}
