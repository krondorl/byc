# Formulas

## Assumptions

- Annual coupon payments
- Face (par) value = 100
- `coupon_rate` is annual (e.g. 5% = 0.05)
- `clean_price` is price per 100 face value
- No accrued interest
- Simple Yield is an approximation (not exact YTM)

---

## Variables

- \( FaceValue = 100 \)
- \( coupon\_rate \) = annual coupon rate (decimal)
- \( clean\_price \) = market price
- \( n \) = years to maturity

---

## Annual Coupon

Annual coupon payment:

$$
C = {coupon\_rate} \times FaceValue
$$

Since \( FaceValue = 100 \):

$$
C = {coupon\_rate} \times 100
$$

---

## Current Yield

Current Yield measures annual income relative to the price paid.

$$
CurrentYield = \frac{C}{clean\_price}
$$

### Example

Given:

- \( C = 5 \)
- \( clean\_price = 95 \)

$$
CurrentYield = \frac{5}{95} = 0.05263 = 5.263\%
$$

---

## Simple Yield (Approximate Yield to Maturity)

Simple Yield includes:
- Annual coupon income
- Capital gain or loss amortized over time

$$
SimpleYield =
\frac{
C + \frac{FaceValue - clean\_price}{n}
}{
\frac{FaceValue + clean\_price}{2}
}
$$

Where:

- \( n \) = years to maturity

### Example

Given:

- \( C = 5 \)
- \( FaceValue = 100 \)
- \( clean\_price = 95 \)
- \( n = 5 \)

$$
CapitalGainPerYear = \frac{100 - 95}{5} = 1
$$

$$
AveragePrice = \frac{100 + 95}{2} = 97.5
$$

$$
SimpleYield = \frac{5 + 1}{97.5} = 0.0615 = 6.15\%
$$

---
