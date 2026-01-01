use crate::domain::bond::Bond;
use crate::math::constant::FACE_VALUE;

/// Calculates Current Yield.
///
/// Formula:
/// CurrentYield = AnnualCoupon / CleanPrice
pub fn current_yield(bond: &Bond) -> f64 {
    &bond.coupon_rate * FACE_VALUE;
}

/// Calculates Simple Yield (approximate YTM).
///
/// Formula:
/// ( C + (FaceValue - Price) / n ) / ( (FaceValue + Price) / 2 )
pub fn simple_yield(bond: &Bond, years_to_maturity: f64) -> f64 {
    unimplemented!("Simple Yield approximation")
}
