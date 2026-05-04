use crate::BaseMoney;
use crate::Decimal;
use crate::accounting::dividend::{Dividend, DividendOps};
use crate::macros::{dec, money};

type UsdDividend = Dividend<crate::Money<crate::iso::USD>, crate::iso::USD>;

// ============================================================================
// ============================= Dividend Yield ===============================
// ============================================================================

#[test]
fn test_dividend_yield_annual() {
    // Share price = $100, DPS = $2.50/year (1 period/year).
    // Yield = (2.50 × 1) / 100 × 100 = 2.50%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.50)).unwrap();
    assert_eq!(div.dividend_yield().unwrap(), dec!(2.50));
}

#[test]
fn test_dividend_yield_quarterly() {
    // Share price = $100, DPS = $0.50/quarter (4 periods/year).
    // Annual DPS = 0.50 × 4 = 2.00
    // Yield = 2.00 / 100 × 100 = 2.00%
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(0.50))
        .unwrap()
        .periods_per_year(4);
    assert_eq!(div.dividend_yield().unwrap(), dec!(2.00));
}

#[test]
fn test_dividend_yield_semi_annual() {
    // Share price = $80, DPS = $2.00/semi-annual (2 periods/year).
    // Annual DPS = 2.00 × 2 = 4.00
    // Yield = 4.00 / 80 × 100 = 5.00%
    let share_price = money!(USD, 80);
    let div = share_price
        .dividend(dec!(2.00))
        .unwrap()
        .periods_per_year(2);
    assert_eq!(div.dividend_yield().unwrap(), dec!(5.00));
}

#[test]
fn test_dividend_yield_monthly() {
    // Share price = $50, DPS = $0.25/month (12 periods/year).
    // Annual DPS = 0.25 × 12 = 3.00
    // Yield = 3.00 / 50 × 100 = 6.00%
    let share_price = money!(USD, 50);
    let div = share_price
        .dividend(dec!(0.25))
        .unwrap()
        .periods_per_year(12);
    assert_eq!(div.dividend_yield().unwrap(), dec!(6.00));
}

#[test]
fn test_dividend_yield_high_price() {
    // Share price = $1000, DPS = $5.00/year.
    // Yield = 5.00 / 1000 × 100 = 0.50%
    let share_price = money!(USD, 1000);
    let div = share_price.dividend(dec!(5.00)).unwrap();
    assert_eq!(div.dividend_yield().unwrap(), dec!(0.50));
}

#[test]
fn test_dividend_yield_zero_share_price() {
    // Share price = $0 → division by zero → None.
    let share_price = money!(USD, 0);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert!(div.dividend_yield().is_none());
}

// ============================================================================
// ============================ Dividend Income ===============================
// ============================================================================

// ---- Simple (no reinvestment) ----

#[test]
fn test_income_simple_annual_1year() {
    // Share price = $100, DPS = $2.50/year, 1 share, 1 period/year, 1 year.
    // Income = 2.50 × 1 × 1 × 1 = 2.50
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.50)).unwrap();
    assert_eq!(div.income().unwrap().amount(), dec!(2.50));
}

#[test]
fn test_income_simple_annual_3years() {
    // Share price = $100, DPS = $2.50/year, 1 share, 1 period/year, 3 years.
    // Income = 2.50 × 1 × 1 × 3 = 7.50
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.50)).unwrap().years(3);
    assert_eq!(div.income().unwrap().amount(), dec!(7.50));
}

#[test]
fn test_income_simple_quarterly_2years() {
    // Share price = $50, DPS = $0.50/quarter, 10 shares, 4 periods/year, 2 years.
    // Income = 0.50 × 10 × 4 × 2 = 40.00
    let share_price = money!(USD, 50);
    let div = share_price
        .dividend(dec!(0.50))
        .unwrap()
        .shares(10)
        .unwrap()
        .periods_per_year(4)
        .years(2);
    assert_eq!(div.income().unwrap().amount(), dec!(40.00));
}

#[test]
fn test_income_simple_monthly_1year() {
    // Share price = $100, DPS = $1.00/month, 5 shares, 12 periods/year, 1 year.
    // Income = 1.00 × 5 × 12 × 1 = 60.00
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(1.00))
        .unwrap()
        .shares(5)
        .unwrap()
        .periods_per_year(12);
    assert_eq!(div.income().unwrap().amount(), dec!(60.00));
}

#[test]
fn test_income_simple_zero_dps() {
    // DPS = $0 → income = 0.
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(0)
        .unwrap()
        .shares(100)
        .unwrap()
        .years(5);
    assert_eq!(div.income().unwrap().amount(), dec!(0));
}

// ---- With reinvestment (DRIP) ----

#[test]
fn test_income_drip_annual_1year() {
    // Share price = $100, DPS = $5.00/year, 10 shares, 1 period/year, 1 year.
    // Period 1: income = 5.00 × 10 = 50, new shares = 50/100 = 0.5, shares = 10.5
    // Total income = 50.00
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .with_reinvest();
    assert_eq!(div.income().unwrap().amount(), dec!(50.00));
}

#[test]
fn test_income_drip_annual_2years() {
    // Share price = $100, DPS = $5.00/year, 10 shares, 1 period/year, 2 years.
    // Period 1: income = 5.00 × 10 = 50, new shares = 50/100 = 0.5, shares = 10.5
    // Period 2: income = 5.00 × 10.5 = 52.5, new shares = 52.5/100 = 0.525, shares = 11.025
    // Total income = 50.00 + 52.50 = 102.50
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(2)
        .with_reinvest();
    assert_eq!(div.income().unwrap().amount(), dec!(102.50));
}

#[test]
fn test_income_drip_quarterly_1year() {
    // Share price = $100, DPS = $1.00/quarter, 100 shares, 4 periods/year, 1 year.
    // Q1: income = 1.00 × 100 = 100, new shares = 100/100 = 1, shares = 101
    // Q2: income = 1.00 × 101 = 101, new shares = 101/100 = 1.01, shares = 102.01
    // Q3: income = 1.00 × 102.01 = 102.01, new shares = 102.01/100 = 1.0201, shares = 103.0301
    // Q4: income = 1.00 × 103.0301 = 103.0301
    // Total income = 100 + 101 + 102.01 + 103.0301 = 406.0401 → rounded to 406.04
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(1.00))
        .unwrap()
        .shares(100)
        .unwrap()
        .periods_per_year(4)
        .with_reinvest();
    assert_eq!(div.income().unwrap().amount(), dec!(406.04));
}

#[test]
fn test_income_drip_exceeds_simple() {
    // DRIP income should always exceed simple income for positive DPS over multiple periods.
    let share_price = money!(USD, 100);
    let simple = share_price
        .dividend(dec!(2.00))
        .unwrap()
        .shares(50)
        .unwrap()
        .periods_per_year(4)
        .years(3);
    let drip = share_price
        .dividend(dec!(2.00))
        .unwrap()
        .shares(50)
        .unwrap()
        .periods_per_year(4)
        .years(3)
        .with_reinvest();
    assert!(drip.income().unwrap().amount() > simple.income().unwrap().amount());
}

// ============================================================================
// ============================= Payout Ratio =================================
// ============================================================================

#[test]
fn test_payout_ratio_basic() {
    // DPS = $2.00/year, EPS = $8.00.
    // Payout = (2.00 × 1) / 8.00 × 100 = 25.00%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert_eq!(div.payout_ratio(dec!(8.00)).unwrap(), dec!(25.00));
}

#[test]
fn test_payout_ratio_quarterly() {
    // DPS = $0.50/quarter (4 periods/year), EPS = $4.00.
    // Annual DPS = 0.50 × 4 = 2.00
    // Payout = 2.00 / 4.00 × 100 = 50.00%
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(0.50))
        .unwrap()
        .periods_per_year(4);
    assert_eq!(div.payout_ratio(dec!(4.00)).unwrap(), dec!(50.00));
}

#[test]
fn test_payout_ratio_100_percent() {
    // DPS = $5.00/year, EPS = $5.00.
    // Payout = 5.00 / 5.00 × 100 = 100.00%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(5.00)).unwrap();
    assert_eq!(div.payout_ratio(dec!(5.00)).unwrap(), dec!(100.00));
}

#[test]
fn test_payout_ratio_zero_eps() {
    // EPS = 0 → division by zero → None.
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert!(div.payout_ratio(0).is_none());
}

// ============================================================================
// ============================ Retention Ratio ===============================
// ============================================================================

#[test]
fn test_retention_ratio_basic() {
    // DPS = $2.00/year, EPS = $8.00.
    // Payout = 25%, Retention = 75%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert_eq!(div.retention_ratio(dec!(8.00)).unwrap(), dec!(75.00));
}

#[test]
fn test_retention_ratio_zero_payout() {
    // DPS = $0 → Payout = 0%, Retention = 100%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(0).unwrap();
    assert_eq!(div.retention_ratio(dec!(8.00)).unwrap(), dec!(100.00));
}

#[test]
fn test_retention_ratio_full_payout() {
    // DPS = EPS → Payout = 100%, Retention = 0%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(5.00)).unwrap();
    assert_eq!(div.retention_ratio(dec!(5.00)).unwrap(), dec!(0));
}

#[test]
fn test_retention_ratio_zero_eps() {
    // EPS = 0 → None.
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert!(div.retention_ratio(0).is_none());
}

#[test]
fn test_payout_plus_retention_equals_100() {
    // For any valid EPS, payout + retention = 100%.
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(3.00))
        .unwrap()
        .periods_per_year(4);
    let eps = dec!(20.00);
    let payout = div.payout_ratio(eps).unwrap();
    let retention = div.retention_ratio(eps).unwrap();
    assert_eq!(payout + retention, dec!(100));
}

// ============================================================================
// ========================= Dividend Per Share ===============================
// ============================================================================

#[test]
fn test_dividend_per_share_basic() {
    // Total dividends = $1,000,000, Shares outstanding = 500,000.
    // DPS = 1000000 / 500000 = 2.00
    let dps = UsdDividend::dividend_per_share(dec!(1000000), dec!(500000)).unwrap();
    assert_eq!(dps, dec!(2));
}

#[test]
fn test_dividend_per_share_fractional() {
    // Total dividends = $750, Shares outstanding = 1000.
    // DPS = 750 / 1000 = 0.75
    let dps = UsdDividend::dividend_per_share(dec!(750), dec!(1000)).unwrap();
    assert_eq!(dps, dec!(0.75));
}

#[test]
fn test_dividend_per_share_zero_shares() {
    // Shares = 0 → division by zero → None.
    let dps = UsdDividend::dividend_per_share(dec!(1000), dec!(0));
    assert!(dps.is_none());
}

// ============================================================================
// ============================= Tax on Dividends =============================
// ============================================================================

#[test]
fn test_income_with_tax_simple() {
    // Share price = $100, DPS = $5.00/year, 10 shares, 1 year, 20% tax.
    // Gross income per period = 5.00 × 10 = 50.00
    // Tax = 50.00 × 20/100 = 10.00
    // Net income per period = 50.00 - 10.00 = 40.00
    // Total = 40.00 × 1 = 40.00
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .with_tax(20)
        .unwrap();
    assert_eq!(div.income().unwrap().amount(), dec!(40.00));
}

#[test]
fn test_income_with_tax_quarterly_2years() {
    // Share price = $50, DPS = $1.00/quarter, 20 shares, 4 periods/year, 2 years, 15% tax.
    // Gross income per period = 1.00 × 20 = 20.00
    // Tax = 20.00 × 15/100 = 3.00
    // Net income per period = 20.00 - 3.00 = 17.00
    // Total = 17.00 × 8 = 136.00
    let share_price = money!(USD, 50);
    let div = share_price
        .dividend(dec!(1.00))
        .unwrap()
        .shares(20)
        .unwrap()
        .periods_per_year(4)
        .years(2)
        .with_tax(15)
        .unwrap();
    assert_eq!(div.income().unwrap().amount(), dec!(136.00));
}

#[test]
fn test_income_with_tax_drip() {
    // Share price = $100, DPS = $5.00/year, 10 shares, 2 years, 20% tax, DRIP.
    // Period 1: gross = 50.00, tax = 10.00, net = 40.00, new shares = 40/100 = 0.4, shares = 10.4
    // Period 2: gross = 5.00 × 10.4 = 52.00, tax = 10.40, net = 41.60
    // Total net = 40.00 + 41.60 = 81.60
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(2)
        .with_tax(20)
        .unwrap()
        .with_reinvest();
    assert_eq!(div.income().unwrap().amount(), dec!(81.60));
}

#[test]
fn test_income_before_and_after_tax() {
    // Tax should reduce income.
    let share_price = money!(USD, 100);
    let no_tax = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(3);
    let with_tax = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(3)
        .with_tax(25)
        .unwrap();
    assert!(with_tax.income().unwrap().amount() < no_tax.income().unwrap().amount());
}

// ============================================================================
// ========================= Builder/Edge Cases ===============================
// ============================================================================

#[test]
fn test_dividend_builder_nan_rate() {
    // f64::NAN cannot convert to Decimal → None.
    let share_price = money!(USD, 100);
    assert!(share_price.dividend(f64::NAN).is_none());
}

#[test]
fn test_dividend_builder_nan_shares() {
    // f64::NAN for shares → None.
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert!(div.shares(f64::NAN).is_none());
}

#[test]
fn test_dividend_builder_nan_tax() {
    // f64::NAN for tax → None.
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.00)).unwrap();
    assert!(div.with_tax(f64::NAN).is_none());
}

#[test]
fn test_dividend_yield_none_on_overflow() {
    // Decimal::MAX as DPS: MAX × 1 / 100 × 100 should trigger overflow.
    let share_price = money!(USD, 1);
    let div = share_price.dividend(Decimal::MAX).unwrap();
    // dividend_yield = (MAX × 1) / 1 × 100 = MAX × 100 overflows
    assert!(div.dividend_yield().is_none());
}

#[test]
fn test_income_none_on_period_overflow() {
    // periods_per_year × years overflows u32.
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(1.00))
        .unwrap()
        .periods_per_year(u32::MAX)
        .years(2);
    assert!(div.income().is_none());
}

#[test]
fn test_income_drip_none_on_share_price_zero() {
    // DRIP with share_price=0 → division by zero in reinvestment → None.
    let share_price = money!(USD, 0);
    let div = share_price
        .dividend(dec!(1.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(2)
        .with_reinvest();
    assert!(div.income().is_none());
}

#[test]
fn test_default_values() {
    // Default: 1 share, 1 period/year, 1 year, no reinvestment, no tax.
    // Share price = $100, DPS = $3.00.
    // Income = 3.00 × 1 × 1 × 1 = 3.00
    // Yield = 3.00 / 100 × 100 = 3.00%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(3.00)).unwrap();
    assert_eq!(div.income().unwrap().amount(), dec!(3.00));
    assert_eq!(div.dividend_yield().unwrap(), dec!(3.00));
}

#[test]
fn test_fractional_shares() {
    // 2.5 shares, DPS = $4.00/year, 1 year.
    // Income = 4.00 × 2.5 × 1 = 10.00
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(4.00))
        .unwrap()
        .shares(dec!(2.5))
        .unwrap();
    assert_eq!(div.income().unwrap().amount(), dec!(10.00));
}

#[test]
fn test_zero_years() {
    // 0 years → 0 total periods → income = 0.
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(0);
    assert_eq!(div.income().unwrap().amount(), dec!(0));
}

#[test]
fn test_zero_periods() {
    // 0 periods/year → 0 total periods → income = 0.
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .periods_per_year(0);
    assert_eq!(div.income().unwrap().amount(), dec!(0));
}

#[test]
fn test_income_drip_zero_years() {
    // DRIP with 0 years → 0 total periods → income = 0.
    let share_price = money!(USD, 100);
    let div = share_price
        .dividend(dec!(5.00))
        .unwrap()
        .shares(10)
        .unwrap()
        .years(0)
        .with_reinvest();
    assert_eq!(div.income().unwrap().amount(), dec!(0));
}

// ============================================================================
// ========================= Real-world scenarios =============================
// ============================================================================

#[test]
fn test_real_world_blue_chip_stock() {
    // A blue-chip stock: share price $150, quarterly dividend $1.25.
    // 200 shares, held for 5 years, 15% tax.
    // Annual DPS = 1.25 × 4 = 5.00
    // Yield = 5.00 / 150 × 100 = 3.333...%
    let share_price = money!(USD, 150);
    let div = share_price
        .dividend(dec!(1.25))
        .unwrap()
        .shares(200)
        .unwrap()
        .periods_per_year(4)
        .years(5)
        .with_tax(15)
        .unwrap();

    // Gross per period = 1.25 × 200 = 250
    // Tax = 250 × 15/100 = 37.50
    // Net per period = 212.50
    // Total = 212.50 × 20 = 4250.00
    assert_eq!(div.income().unwrap().amount(), dec!(4250.00));
}

#[test]
fn test_real_world_drip_long_term() {
    // DRIP scenario: share price $50, monthly DPS $0.20, 100 shares, 5 years.
    // Without reinvestment: 0.20 × 100 × 12 × 5 = 1200.00
    // With reinvestment: should exceed 1200.00
    let share_price = money!(USD, 50);
    let simple = share_price
        .dividend(dec!(0.20))
        .unwrap()
        .shares(100)
        .unwrap()
        .periods_per_year(12)
        .years(5);
    let drip = share_price
        .dividend(dec!(0.20))
        .unwrap()
        .shares(100)
        .unwrap()
        .periods_per_year(12)
        .years(5)
        .with_reinvest();
    assert_eq!(simple.income().unwrap().amount(), dec!(1200.00));
    assert!(drip.income().unwrap().amount() > dec!(1200.00));
}

#[test]
fn test_payout_and_retention_real_world() {
    // Company EPS = $6.50, annual DPS = $2.60.
    // Payout = 2.60 / 6.50 × 100 = 40.00%
    // Retention = 60.00%
    let share_price = money!(USD, 100);
    let div = share_price.dividend(dec!(2.60)).unwrap();
    assert_eq!(div.payout_ratio(dec!(6.50)).unwrap(), dec!(40.00));
    assert_eq!(div.retention_ratio(dec!(6.50)).unwrap(), dec!(60.00));
}
