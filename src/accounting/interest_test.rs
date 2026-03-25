use crate::BaseMoney;
use crate::Decimal;
use crate::accounting::InterestOps;
use crate::accounting::interest::RateDays;
use crate::macros::{dec, money};

// ---- Fixed interest: daily rate ----
//
// Convention: the rate is adjusted to the payment period.
// - daily rate × daily period  → effective daily rate   = r / 100
// - daily rate × monthly period → effective monthly rate = r × days_in_month / 100
// - daily rate × yearly period  → effective yearly rate  = r × days_in_year / 100

#[test]
fn test_fixed_daily_days() {
    // P=5000, r=5% daily, 100 days: effective daily rate = r/100 = 0.05
    // returns = P × (r/100) × t = 5000 × 0.05 × 100 = 25000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(100);
    let returns = interest.returns().unwrap();
    let total = interest.future_value().unwrap();

    assert_eq!(returns.amount(), dec!(25_000));
    assert_eq!(total.amount(), dec!(30_000));
}

#[test]
fn test_fixed_daily_months() {
    // P=5000, r=5% daily, 12 months from 2026-01-01:
    // Rate adjusted to period: effective monthly rate = r × days_in_month / 100
    // Jan(31)+Feb(28)+Mar(31)+Apr(30)+May(31)+Jun(30)+Jul(31)+Aug(31)+Sep(30)+Oct(31)+Nov(30)+Dec(31) = 365
    // returns = P × Σ(r × days_in_month / 100) = 5000 × 5 × 365 / 100 = 91250
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);
    let returns = interest.returns().unwrap();
    let total = interest.future_value().unwrap();

    assert_eq!(returns.amount(), dec!(91250.00));
    assert_eq!(total.amount(), dec!(96250.00));
}

#[test]
fn test_fixed_daily_years() {
    // P=5000, r=5% daily, 2 years from 2026 (both non-leap, 365 days each):
    // Rate adjusted to period: effective yearly rate = r × days_in_year / 100
    // Year 2026: 5 × 365 / 100 = 18.25; Year 2027: same
    // returns = 5000 × 18.25 × 2 = 182500
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);
    let returns = interest.returns().unwrap();
    let total = interest.future_value().unwrap();

    assert_eq!(returns.amount(), dec!(182500.00));
    assert_eq!(total.amount(), dec!(187500.00));
}

// Rate30360 convention eliminates date-sensitivity: 360 days/year, 30 days/month.

#[test]
fn test_fixed_daily_days_rate30360() {
    // P=5000, r=5% daily, Rate30360, 100 days: effective daily rate = r/100 = 0.05
    // returns = P × (r/100) × t = 5000 × 0.05 × 100 = 25000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(100);

    assert_eq!(interest.returns().unwrap().amount(), dec!(25_000));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(30_000));
}

#[test]
fn test_fixed_daily_months_rate30360() {
    // P=5000, r=5% daily, Rate30360, 12 months from 2026-01-01:
    // Rate adjusted to period: effective monthly rate = r × 30 / 100 = 1.5
    // returns = P × (5 × 30 / 100) × 12 = 5000 × 1.5 × 12 = 90000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(90000));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(95000));
}

#[test]
fn test_fixed_daily_years_rate30360() {
    // P=5000, r=5% daily, Rate30360, 2 years from 2026-01-01:
    // Rate adjusted to period: effective yearly rate = r × 360 / 100 = 18
    // returns = P × 18 × 2 = 180000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(180000));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(185000));
}

// ---- Fixed interest: monthly rate ----
//
// Convention: the rate is adjusted to the payment period.
// - monthly rate × daily period  → effective daily rate   = r / days_in_month / 100
// - monthly rate × monthly period → effective monthly rate = r / 100
// - monthly rate × yearly period  → effective yearly rate  = r × 12 / 100

#[test]
fn test_fixed_monthly_days() {
    // P=5000, r=5% monthly, 100 days from 2026-01-01 (RateActualActual):
    // Rate adjusted to period: effective daily rate = r / days_in_month / 100
    // Jan(31): 5000 × 5/31/100 × 31 = 250; Feb(28): 5000 × 5/28/100 × 28 = 250
    // Mar(31): 5000 × 5/31/100 × 31 = 250; Apr(10): 5000 × 5/30/100 × 10 = 83.33
    // returns = 250 + 250 + 250 + 83.33 = 833.33
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .days(100);
    let returns = interest.returns().unwrap();
    let total = interest.future_value().unwrap();

    assert_eq!(returns.amount(), dec!(833.33));
    assert_eq!(total.amount(), dec!(5833.33));
}

#[test]
fn test_fixed_monthly_days_rate30360() {
    // P=5000, r=5% monthly, Rate30360, 100 days from 2026-01-01:
    // Rate adjusted to period: effective daily rate = r / 30 / 100 = 5/3000
    // returns = P × (5/3000) × 100 = 833.33
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(100);

    assert_eq!(interest.returns().unwrap().amount(), dec!(833.33));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5833.33));
}

#[test]
fn test_fixed_monthly_months() {
    // P=5000, r=5% monthly, 12 months: effective monthly rate = r/100 = 0.05
    // returns = P × (r/100) × t = 5000 × 0.05 × 12 = 3000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);
    let returns = interest.returns().unwrap();
    let total = interest.future_value().unwrap();

    assert_eq!(returns.amount(), dec!(3000));
    assert_eq!(total.amount(), dec!(8000));
}

#[test]
fn test_fixed_monthly_years() {
    // P=5000, r=5% monthly, 2 years:
    // Rate adjusted to period: effective yearly rate = r × 12 / 100 = 0.60
    // returns = P × 0.60 × 2 = 6000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);
    let returns = interest.returns().unwrap();
    let total = interest.future_value().unwrap();

    assert_eq!(returns.amount(), dec!(6000));
    assert_eq!(total.amount(), dec!(11000));
}

// ---- Fixed interest: yearly rate ----
//
// Convention: the rate is adjusted to the payment period.
// - yearly rate × daily period  → effective daily rate   = r / days_in_year / 100
// - yearly rate × monthly period → effective monthly rate = r / 12 / 100
// - yearly rate × yearly period  → effective yearly rate  = r / 100

#[test]
fn test_fixed_yearly_years() {
    // P=5000, r=5% yearly, 2 years: effective yearly rate = r/100 = 0.05
    // returns = P × (r/100) × t = 5000 × 0.05 × 2 = 500
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(500.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5500.00));
}

#[test]
fn test_fixed_yearly_months() {
    // P=5000, r=5% yearly, 12 months:
    // Rate adjusted to period: effective monthly rate = r / 12 / 100
    // returns = P × (r/12/100) × 12 = P × r/100 = 5000 × 0.05 = 250
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(250.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5250.00));
}

#[test]
fn test_fixed_yearly_days_rate30360() {
    // P=5000, r=5% yearly, Rate30360, 360 days from 2026-01-01:
    // Rate adjusted to period: effective daily rate = r / 360 / 100 = 5/36000
    // returns = P × (5/36000) × 360 = 5000 × 0.05 = 250
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(360);

    assert_eq!(interest.returns().unwrap().amount(), dec!(250.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5250.00));
}

// ---- Compounding interest: daily rate ----

#[test]
fn test_compound_daily_days() {
    // P=5000, r=5% daily, 10 days from 2026-01-01:
    // each day: current_interest = current_principal × (r/100), principal compounds
    // returns = 3144.47 (rounded to USD 2 decimals)
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(10);

    assert_eq!(interest.returns().unwrap().amount(), dec!(3144.47));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(8144.47));
}

#[test]
fn test_compound_daily_months_rate30360() {
    // P=5000, r=5% daily, Rate30360, 12 months from 2026-01-01:
    // Rate adjusted to period: effective monthly rate = r × 30 / 100 = 1.5 (150%)
    // Each month compounds: current_interest = current_principal × 1.5
    // returns = 298018223.88 (large due to extreme rate scaled up to monthly)
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(298018223.88));
    assert_eq!(
        interest.future_value().unwrap().amount(),
        dec!(298023223.88)
    );
}

#[test]
fn test_compound_daily_years_rate30360() {
    // P=5000, r=5% daily, Rate30360, 2 years from 2026-01-01:
    // Rate adjusted to period: effective yearly rate = r × 360 / 100 = 18 (1800%)
    // Year 1: 5000 × 18 = 90000, CP = 95000
    // Year 2: 95000 × 18 = 1710000; returns = 1800000
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(1800000));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1805000));
}

// ---- Compounding interest: monthly rate ----

#[test]
fn test_compound_monthly_days_rate30360() {
    // P=5000, r=5% monthly, Rate30360, 10 days from 2026-01-01:
    // Rate adjusted to period: effective daily rate = r / 30 / 100 ≈ 0.1667%
    // Each day compounds: returns = 83.96
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(10);

    assert_eq!(interest.returns().unwrap().amount(), dec!(83.96));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5083.96));
}

#[test]
fn test_compound_monthly_months() {
    // P=5000, r=5% monthly, 12 months from 2026-01-01:
    // effective monthly rate = r/100 = 0.05; each month compounds
    // returns = 3979.28
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(3979.28));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(8979.28));
}

#[test]
fn test_compound_monthly_years() {
    // P=5000, r=5% monthly, 2 years from 2026-01-01:
    // Rate adjusted to period: effective yearly rate = r × 12 / 100 = 0.60 (60%)
    // Year 1: 5000 × 0.60 = 3000, CP = 8000
    // Year 2: 8000 × 0.60 = 4800; returns = 7800
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(7800.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(12800.00));
}

// ---- Compounding interest: yearly rate ----

#[test]
fn test_compound_yearly_days_rate30360() {
    // P=5000, r=5% yearly, Rate30360, 10 days from 2026-01-01:
    // Rate adjusted to period: effective daily rate = r / 360 / 100 ≈ 0.01389%
    // Each day compounds: returns = 6.95
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(10);

    assert_eq!(interest.returns().unwrap().amount(), dec!(6.95));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5006.95));
}

#[test]
fn test_compound_yearly_months() {
    // P=5000, r=5% yearly, 12 months from 2026-01-01:
    // Rate adjusted to period: effective monthly rate = r / 12 / 100
    // Each month compounds: returns = 255.81
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(255.81));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5255.81));
}

#[test]
fn test_compound_yearly_years() {
    // P=5000, r=5% yearly, 2 years from 2026-01-01:
    // effective yearly rate = r/100 = 0.05; each year compounds
    // Year 1: 5000 × 0.05 = 250, CP = 5250
    // Year 2: 5250 × 0.05 = 262.50; returns = 512.50
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(512.50));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5512.50));
}

// ---- Edge cases ----

#[test]
fn test_zero_rate_fixed() {
    // A 0% interest rate should yield zero returns
    let money = money!(USD, 5000);

    let i = money
        .interest_fixed(0)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(100);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.future_value().unwrap().amount(), dec!(5000));

    let i = money
        .interest_fixed(0)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.future_value().unwrap().amount(), dec!(5000));

    let i = money
        .interest_fixed(0)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.future_value().unwrap().amount(), dec!(5000));
}

#[test]
fn test_zero_rate_compound() {
    // A 0% compounding rate should also yield zero returns
    let money = money!(USD, 5000);

    let i = money
        .interest_compound(0)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(100);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.future_value().unwrap().amount(), dec!(5000));

    let i = money
        .interest_compound(0)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(12);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.future_value().unwrap().amount(), dec!(5000));

    let i = money
        .interest_compound(0)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.future_value().unwrap().amount(), dec!(5000));
}

#[test]
fn test_rate_days_rate30360_vs_actual_actual() {
    // Rate30360 uses a fixed 360 days/year, so:
    //   yearly rate → daily: r / 360 / 100
    // RateActualActual uses 365 days for 2026 (non-leap year), so:
    //   yearly rate → daily: r / 365 / 100
    // The two conventions produce different results for the same inputs.
    let money = money!(USD, 5000);

    let i_30360 = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(360);

    let i_actual = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActualActual)
        .days(360);

    // Rate30360: 5000 × (5/360/100) × 360 = 250.00
    assert_eq!(i_30360.returns().unwrap().amount(), dec!(250.00));
    // RateActualActual: 5000 × (5/365/100) × 360 ≠ 250 (365-day year)
    assert_ne!(
        i_actual.returns().unwrap().amount(),
        i_30360.returns().unwrap().amount()
    );
}

#[test]
fn test_rate_days_rate_actual365() {
    // RateActual365 always uses 365 days/year regardless of leap year.
    // For 2026 (non-leap year): effective daily rate = r / 365 / 100
    // returns = 5000 × (5/365/100) × 365 = 5000 × 0.05 = 250
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActual365)
        .days(365);

    assert_eq!(interest.returns().unwrap().amount(), dec!(250.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5250.00));
}

#[test]
fn test_rate_days_rate_actual360() {
    // RateActual360: actual days/month, fixed 360 days/year.
    // Yearly rate: effective daily rate = r / 360 / 100 (denominator fixed at 360).
    // For 60 days from 2026-01-01 (Jan 31 + Feb 28 = 59, plus 1 day of Mar = 60):
    // returns = 5000 × (5/360/100) × 60 = 5000 × 5 × 60 / 36000 = 41.67
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActual360)
        .days(60);

    assert_eq!(interest.returns().unwrap().amount(), dec!(41.67));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5041.67));
}

#[test]
fn test_year_month_day_setters() {
    // Verify that the year/month/day builder setters are used in date-sensitive calculations.
    // Fixed yearly days from 2026-01-01 vs 2026-02-01 with RateActualActual:
    // both have the same daily rate = r / 365 / 100 (same non-leap year 2026),
    // and the same number of days, so returns must be equal.
    let money = money!(USD, 5000);

    let i_jan = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActualActual)
        .days(30);

    let i_feb = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(2)
        .day(1)
        .rate_days(RateDays::RateActualActual)
        .days(30);

    assert_eq!(
        i_jan.returns().unwrap().amount(),
        i_feb.returns().unwrap().amount()
    );
}

#[test]
fn test_fixed_and_compound_returns_differ() {
    // For any positive rate over multiple periods, compound interest must exceed fixed interest.
    let money = money!(USD, 5000);

    let fixed = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(3);
    let compound = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(3);

    assert!(compound.returns().unwrap().amount() > fixed.returns().unwrap().amount());
    assert!(compound.future_value().unwrap().amount() > fixed.future_value().unwrap().amount());
}

#[test]
fn test_total_equals_principal_plus_returns() {
    // total() must always equal principal + returns() for all combinations.
    let money = money!(USD, 5000);

    let cases: &[_] = &[
        money
            .interest_fixed(5)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .days(100),
        money
            .interest_fixed(5)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(6),
        money
            .interest_fixed(5)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .years(1),
        money
            .interest_compound(5)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .days(5),
        money
            .interest_compound(5)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(6),
        money
            .interest_compound(5)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .years(1),
    ];

    for interest in cases {
        let returns = interest.returns().unwrap().amount();
        let total = interest.future_value().unwrap().amount();
        let expected_total = money.amount() + returns;
        // Allow for ±1 cent rounding difference due to USD rounding applied independently
        let diff = (total - expected_total).abs();
        assert!(
            diff <= dec!(0.01),
            "total ({total}) != principal ({}) + returns ({returns}), diff={diff}",
            money.amount()
        );
    }
}

// ---- Additional RateDays coverage: Rate30365 and Rate30Actual ----

#[test]
fn test_fixed_daily_years_rate30365() {
    // P=5000, r=5% daily, Rate30365, 2 years from 2026-01-01.
    // Rate30365 always uses 365 days/year regardless of leap year.
    // Year 2026 (non-leap): yearly_rate = 5 × 365 / 100 = 18.25
    // Year 2027 (non-leap): yearly_rate = 18.25
    // returns = 5000 × 18.25 × 2 = 182500
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30365)
        .years(2);
    assert_eq!(interest.returns().unwrap().amount(), dec!(182500.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(187500.00));
}

#[test]
fn test_fixed_daily_years_rate30actual() {
    // P=5000, r=5% daily, Rate30Actual, 1 year from 2026-01-01 (non-leap, 365 actual days).
    // Rate30Actual: 30 days/month, actual days/year → 365 for 2026.
    // yearly_rate = 5 × 365 / 100 = 18.25
    // returns = 5000 × 18.25 = 91250
    // Compare: Rate30360 gives 5000 × 5 × 360 / 100 = 90000 (different denominator).
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30Actual)
        .years(1);
    assert_eq!(interest.returns().unwrap().amount(), dec!(91250.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(96250.00));
}

#[test]
fn test_fixed_monthly_days_rate30actual() {
    // P=5000, r=5% monthly, Rate30Actual, 100 days from 2026-01-01.
    // Rate30Actual uses 30 days/month for daily-rate conversion.
    // effective daily rate = 5 / 30 / 100 = 1/600
    // returns = P × (1/600) × 100 = 5000 × 100 / 600 = 833.33
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30Actual)
        .days(100);
    assert_eq!(interest.returns().unwrap().amount(), dec!(833.33));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5833.33));
}

#[test]
fn test_compound_daily_years_rate30365() {
    // P=5000, r=5% daily, Rate30365, 2 years from 2026-01-01.
    // Rate30365 uses fixed 365 days/year.
    // yearly_rate = 5 × 365 / 100 = 18.25
    // Year 1: interest = 5000 × 18.25 = 91250, CP = 96250
    // Year 2: interest = 96250 × 18.25 = 1756562.50; total_interest = 1847812.50
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30365)
        .years(2);
    assert_eq!(interest.returns().unwrap().amount(), dec!(1847812.50));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1852812.50));
}

#[test]
fn test_compound_daily_years_rate30actual() {
    // P=5000, r=5% daily, Rate30Actual, 1 year from 2026 (365 actual days).
    // yearly_rate = 5 × 365 / 100 = 18.25
    // Year 1: interest = 5000 × 18.25 = 91250; total = 96250
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30Actual)
        .years(1);
    assert_eq!(interest.returns().unwrap().amount(), dec!(91250.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(96250.00));
}

// ---- Tests for inputs that cause returns() or the builder to return None ----
//
// These tests exercise the `?` operators in interest_fixed, interest_compound,
// and Interest::returns(), covering the None-propagation paths.

#[test]
fn test_interest_builder_returns_none_for_nan_rate() {
    // f64::NAN cannot be converted to Decimal, so get_decimal() returns None,
    // causing interest_fixed and interest_compound to return None.
    let money = money!(USD, 5000);
    assert!(money.interest_fixed(f64::NAN).is_none());
    assert!(money.interest_compound(f64::NAN).is_none());
}

// ---- Fixed interest: None from arithmetic overflow in returns() ----
//
// Each test uses a rate large enough to cause Decimal overflow in the
// corresponding multiplication, driving returns() to return None.

#[test]
fn test_fixed_yearly_years_returns_none_on_overflow() {
    // yearly_rate = MAX / 100 ≈ 7.92e26
    // principal × yearly_rate = 5000 × 7.92e26 ≈ 3.96e30 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_yearly_months_returns_none_on_overflow() {
    // monthly_rate = MAX / 12 / 100 ≈ 6.6e25
    // principal × monthly_rate = 5000 × 6.6e25 ≈ 3.3e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .months(12)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_yearly_days_returns_none_on_overflow() {
    // daily_rate = MAX / 365 / 100 ≈ 2.17e24 (division never overflows)
    // With principal = 40000: 40000 × 2.17e24 ≈ 8.68e28 > Decimal::MAX → None
    // A larger principal is required here because daily_rate is reduced by /365/100.
    let money = money!(USD, 40000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .days(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_monthly_years_returns_none_on_overflow() {
    // Rate = 1e27: get_yearly_rate(Monthly(1e27)) = 1e27 × 12 / 100 = 1.2e26 (succeeds,
    // since 1e27 × 12 = 1.2e28 < Decimal::MAX).
    // principal × yearly_rate = 5000 × 1.2e26 = 6e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(dec!(1000000000000000000000000000))
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_monthly_months_returns_none_on_overflow() {
    // monthly_rate = MAX / 100 ≈ 7.92e26
    // principal × monthly_rate = 5000 × 7.92e26 ≈ 3.96e30 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(12)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_monthly_days_returns_none_on_overflow() {
    // get_daily_rate(Monthly(MAX)) = MAX / days_in_month / 100 ≈ 2.56e25 (succeeds; division).
    // principal × daily_rate = 5000 × 2.56e25 ≈ 1.28e29 > Decimal::MAX → checked_mul overflows → None.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .days(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_daily_years_returns_none_on_overflow() {
    // Rate = 1e25: get_yearly_rate(Daily(1e25)) = 1e25 × 365 / 100 = 3.65e25 (succeeds,
    // since 1e25 × 365 = 3.65e27 < Decimal::MAX).
    // principal × yearly_rate = 5000 × 3.65e25 = 1.825e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(dec!(10000000000000000000000000))
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_daily_months_returns_none_on_overflow() {
    // Rate = 1e26: get_monthly_rate(Daily(1e26)) = 1e26 × 31 / 100 = 3.1e25 (succeeds,
    // since 1e26 × 31 = 3.1e27 < Decimal::MAX).
    // principal × monthly_rate = 5000 × 3.1e25 = 1.55e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(dec!(100000000000000000000000000))
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .months(12)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_daily_days_returns_none_on_overflow() {
    // daily_rate = MAX / 100 ≈ 7.92e26
    // principal × daily_rate = 5000 × 7.92e26 ≈ 3.96e30 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .days(1)
            .returns()
            .is_none()
    );
}

// ---- Compounding interest: None from arithmetic overflow in returns() ----

#[test]
fn test_compound_yearly_years_returns_none_on_overflow() {
    // Same logic as fixed: yearly_rate = MAX/100, 5000 × MAX/100 overflows.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_yearly_months_returns_none_on_overflow() {
    // monthly_rate = MAX/12/100 ≈ 6.6e25; 5000 × 6.6e25 ≈ 3.3e29 > MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .months(12)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_yearly_days_returns_none_on_overflow() {
    // daily_rate = MAX/365/100 ≈ 2.17e24; with principal=40000:
    // 40000 × 2.17e24 ≈ 8.68e28 > Decimal::MAX → None
    let money = money!(USD, 40000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .days(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_monthly_years_returns_none_on_overflow() {
    // Rate = 1e27: get_yearly_rate(Monthly(1e27)) = 1e27 × 12 / 100 = 1.2e26 (succeeds).
    // 5000 × 1.2e26 = 6e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(dec!(1000000000000000000000000000))
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_monthly_months_returns_none_on_overflow() {
    // monthly_rate = MAX/100 ≈ 7.92e26; 5000 × 7.92e26 overflows → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(12)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_monthly_days_returns_none_on_overflow() {
    // get_daily_rate(Monthly(MAX)) = MAX / days_in_month / 100 ≈ 2.56e25 (succeeds; division).
    // current_principal × daily_rate = 5000 × 2.56e25 ≈ 1.28e29 > Decimal::MAX → checked_mul overflows → None.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .days(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_daily_years_returns_none_on_overflow() {
    // Rate = 1e25: get_yearly_rate(Daily(1e25)) = 1e25 × 365 / 100 = 3.65e25 (succeeds).
    // 5000 × 3.65e25 = 1.825e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(dec!(10000000000000000000000000))
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_daily_months_returns_none_on_overflow() {
    // Rate = 1e26: get_monthly_rate(Daily(1e26)) = 1e26 × 31 / 100 = 3.1e25 (succeeds).
    // 5000 × 3.1e25 = 1.55e29 > Decimal::MAX → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(dec!(100000000000000000000000000))
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .months(12)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_daily_days_returns_none_on_overflow() {
    // daily_rate = MAX/100 ≈ 7.92e26; 5000 × 7.92e26 overflows → None
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .days(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_daily_years_checked_add_overflow() {
    // Rate = 1e24: get_yearly_rate(Daily(1e24)) = 1e24 × 365 / 100 = 3.65e24 (succeeds,
    // since 1e24 × 365 = 3.65e26 < Decimal::MAX).
    // Per-year interest = 5000 × 3.65e24 = 1.825e28 < Decimal::MAX → checked_mul succeeds.
    // After 5 years the cumulative sum 5 × 1.825e28 = 9.125e28 > Decimal::MAX → checked_add overflows.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(dec!(1000000000000000000000000))
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .rate_days(RateDays::Rate30365)
            .years(5)
            .returns()
            .is_none()
    );
}

#[test]
fn test_fixed_daily_months_get_monthly_rate_overflow() {
    // Rate = Decimal::MAX (daily): get_monthly_rate(Daily(MAX)) = MAX × 31 / 100.
    // MAX × 31 overflows inside get_monthly_rate itself, so it returns None.
    // The ? on get_monthly_rate propagates None out of the loop.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .months(1)
            .returns()
            .is_none()
    );
}

#[test]
fn test_compound_daily_months_get_monthly_rate_overflow() {
    // Same as the fixed case: get_monthly_rate(Daily(MAX)) overflows inside the function,
    // and the ? propagates None from the compounding loop.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .months(1)
            .returns()
            .is_none()
    );
}

// ---- Present value tests ----
//
// Round-trip invariant: present_value(future_value(P)) == P when using the same
// rate/period/date parameters.  Every match arm in present_value() is exercised
// by one or more of the tests below.

// ---- Fixed present value: round-trip tests ----

#[test]
fn test_pv_fixed_yearly_years() {
    // P=5000, r=5% yearly, 2 years.
    // yearly_rate = 5/100 = 0.05; actual_r = 0.05 + 0.05 = 0.10; divisor = 1.10
    // FV = 5500; PV = 5500 / 1.10 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5500.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_yearly_months() {
    // P=5000, r=60% yearly, 2 months.
    // monthly_rate = 60/12/100 = 0.05 per month (exact); actual_r = 0.10; divisor = 1.10
    // FV = 5500; PV = 5500 / 1.10 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(60)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .months(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5500.00));
    let pv = fv
        .interest_fixed(60)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .months(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_yearly_days() {
    // P=5000, r=5% yearly, Rate30360, 360 days.
    // daily_rate = 5/360/100; actual_r = 360 × (5/36000) = 0.05; divisor = 1.05
    // FV = 5250; PV = 5250 / 1.05 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(360)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5250.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(360)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_monthly_years() {
    // P=5000, r=5% monthly, 2 years.
    // yearly_rate = 5×12/100 = 0.6 per year; actual_r = 1.2; divisor = 2.2
    // FV = 5000 + 5000 × 1.2 = 11000; PV = 11000 / 2.2 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(11000.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_monthly_days() {
    // P=5000, r=5% monthly, Rate30360, 30 days.
    // get_daily_rate(Monthly(5)) = 5 / 30 / 100 = 1/600 per day
    // actual_r = 30 × (1/600) = 0.05; divisor = 1.05
    // FV = 5000 × 1.05 = 5250; PV = 5250 / 1.05 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(30)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5250.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(30)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_daily_years() {
    // P=5000, r=5% daily, Rate30360, 1 year.
    // yearly_rate = 5×360/100 = 18; actual_r = 18; divisor = 19
    // FV = 5000 + 5000×18 = 95000; PV = 95000 / 19 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(1)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(95000.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(1)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_daily_days() {
    // P=5000, r=5% daily, 10 days.
    // daily_rate = 5/100 = 0.05; actual_r = 0.5; divisor = 1.5
    // FV = 5000 + 5000×0.5 = 7500; PV = 7500 / 1.5 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(10)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(7500.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(10)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_daily_months() {
    // P=5000, r=5% daily, Rate30360, 12 months.
    // monthly_rate = 5×30/100 = 1.5 per month; actual_r = 18; divisor = 19
    // FV = 5000 + 5000×18 = 95000; PV = 95000 / 19 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(12)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(95000.00));
    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(12)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

// ---- Compound present value: round-trip tests ----

#[test]
fn test_pv_compound_yearly_years() {
    // P=5000, r=5% yearly, 2 years.
    // divisor = (1.05)^2 = 1.1025; FV = 5512.50; PV = 5512.50 / 1.1025 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5512.50));
    let pv = fv
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_yearly_months() {
    // P=5000, r=60% yearly, 2 months.
    // monthly_rate = 60/12/100 = 0.05 per month (exact)
    // divisor = (1.05)^2 = 1.1025; FV = 5512.50; PV = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(60)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .months(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5512.50));
    let pv = fv
        .interest_compound(60)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .months(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_yearly_days() {
    // P=5000, r=720% yearly, Rate30360, 1 day.
    // daily_rate = 720/360/100 = 0.02; divisor = 1.02
    // FV = 5000 × 1.02 = 5100; PV = 5100 / 1.02 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(720)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(1)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5100.00));
    let pv = fv
        .interest_compound(720)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(1)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_monthly_years() {
    // P=5000, r=5% monthly, 2 years.
    // yearly_rate = 5×12/100 = 0.6; divisor = (1.6)^2 = 2.56
    // FV = 12800; PV = 12800 / 2.56 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(12800.00));
    let pv = fv
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .years(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_monthly_days() {
    // P=5000, r=300% monthly, Rate30360, 1 day.
    // present_value compound (Monthly, Days) uses get_daily_rate per day:
    //   daily_rate = 300/30/100 = 0.1; d = 1.1; divisor = 1.1
    //   FV = 5000 × 1.1 = 5500; PV = 5500 / 1.1 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(300)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(1)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5500.00));
    let pv = fv
        .interest_compound(300)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(1)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_daily_years() {
    // P=5000, r=5% daily, Rate30360, 1 year.
    // yearly_rate = 5×360/100 = 18; divisor = 1+18 = 19
    // FV = 5000 × 19 = 95000; PV = 95000 / 19 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(1)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(95000.00));
    let pv = fv
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(1)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_daily_days() {
    // P=5000, r=5% daily, 2 days.
    // daily_rate = 0.05; divisor = (1.05)^2 = 1.1025
    // FV = 5512.50; PV = 5512.50 / 1.1025 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(5512.50));
    let pv = fv
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_daily_months() {
    // P=5000, r=5% daily, Rate30360, 1 month.
    // monthly_rate = 5×30/100 = 1.5; divisor = 1+1.5 = 2.5
    // FV = 5000 × (1+1.5) = 12500; PV = 12500 / 2.5 = 5000
    let money = money!(USD, 5000);
    let fv = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(1)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(12500.00));
    let pv = fv
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(1)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

// ---- Present value: None-propagation tests ----
//
// Each test exercises a specific `?` operator inside present_value().
// Inputs are chosen to trigger the failure point with the minimum
// setup required.

// --- Fixed: Yearly/Years ---
#[test]
fn test_pv_fixed_yearly_years_none_on_year_overflow() {
    // year = u32::MAX: after computing the first year's rate, current_year.checked_add(1)
    // returns None, propagating None out of the loop.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .yearly()
            .year(u32::MAX)
            .month(1)
            .day(1)
            .years(1)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Yearly/Months ---
#[test]
fn test_pv_fixed_yearly_months_none_on_zero_months() {
    // months=0: get_years_months returns None for num_of_months==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .months(0)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Yearly/Days ---
#[test]
fn test_pv_fixed_yearly_days_none_on_invalid_day() {
    // day=0: get_years_months_days returns None for start_day==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(0)
            .days(1)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Monthly/Years ---
#[test]
fn test_pv_fixed_monthly_years_none_on_year_overflow() {
    // year = u32::MAX: current_year.checked_add(1) overflows after the first iteration.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .monthly()
            .year(u32::MAX)
            .month(1)
            .day(1)
            .years(1)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Monthly/Months ---
#[test]
fn test_pv_fixed_monthly_months_none_on_zero_months() {
    // months=0: get_years_months returns None for num_of_months==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(0)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Monthly/Days ---
#[test]
fn test_pv_fixed_monthly_days_none_on_invalid_day() {
    // day=0: get_years_months_days returns None for start_day==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(0)
            .days(1)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Daily/Years ---
#[test]
fn test_pv_fixed_daily_years_none_on_rate_overflow() {
    // Rate = Decimal::MAX: get_yearly_rate(Daily) = MAX × 365 overflows → None.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Daily/Months ---
#[test]
fn test_pv_fixed_daily_months_none_on_rate_overflow() {
    // Rate = Decimal::MAX: get_monthly_rate(Daily) = MAX × days_in_month overflows → None.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .months(1)
            .present_value()
            .is_none()
    );
}

// --- Fixed: Daily/Days ---
#[test]
fn test_pv_fixed_daily_days_none_on_invalid_day() {
    // day=0: get_years_months_days returns None for start_day==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_fixed(5)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(0)
            .days(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Yearly/Years ---
#[test]
fn test_pv_compound_yearly_years_none_on_year_overflow() {
    // year = u32::MAX: current_year.checked_add(1) overflows after the first iteration.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .yearly()
            .year(u32::MAX)
            .month(1)
            .day(1)
            .years(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Yearly/Months ---
#[test]
fn test_pv_compound_yearly_months_none_on_zero_months() {
    // months=0: get_years_months returns None for num_of_months==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .months(0)
            .present_value()
            .is_none()
    );
}

// --- Compound: Yearly/Days ---
#[test]
fn test_pv_compound_yearly_days_none_on_invalid_day() {
    // day=0: get_years_months_days returns None for start_day==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(0)
            .days(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Monthly/Years ---
#[test]
fn test_pv_compound_monthly_years_none_on_year_overflow() {
    // year = u32::MAX: current_year.checked_add(1) overflows after the first iteration.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .monthly()
            .year(u32::MAX)
            .month(1)
            .day(1)
            .years(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Monthly/Months ---
#[test]
fn test_pv_compound_monthly_months_none_on_zero_months() {
    // months=0: get_years_months returns None for num_of_months==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(0)
            .present_value()
            .is_none()
    );
}

// --- Compound: Monthly/Days ---
#[test]
fn test_pv_compound_monthly_days_none_on_invalid_day() {
    // day=0: get_years_months_days returns None for start_day==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(0)
            .days(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Daily/Years ---
#[test]
fn test_pv_compound_daily_years_none_on_rate_overflow() {
    // Rate = Decimal::MAX: get_yearly_rate(Daily) = MAX × 365 overflows → None.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .years(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Daily/Months ---
#[test]
fn test_pv_compound_daily_months_none_on_rate_overflow() {
    // Rate = Decimal::MAX: get_monthly_rate(Daily) = MAX × days_in_month overflows → None.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(1)
            .months(1)
            .present_value()
            .is_none()
    );
}

// --- Compound: Daily/Days ---
#[test]
fn test_pv_compound_daily_days_none_on_invalid_day() {
    // day=0: get_years_months_days returns None for start_day==0.
    let money = money!(USD, 5000);
    assert!(
        money
            .interest_compound(5)
            .unwrap()
            .daily()
            .year(2026)
            .month(1)
            .day(0)
            .days(1)
            .present_value()
            .is_none()
    );
}

#[test]
fn test_present() {
    let money = money!(USD, 1000);
    println!("Original: {money}");
    let fv = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .months(2)
        .future_value()
        .unwrap();

    println!("fv: {fv}");

    let pv = fv
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .months(2)
        .present_value()
        .unwrap();
    println!("pv: {pv}");
    assert_eq!(money, pv);

    let money = money!(USD, 1000);
    println!("Original: {money}");
    let fv = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .months(2)
        .future_value()
        .unwrap();

    println!("fv: {fv}");

    let pv = fv
        .interest_compound(5)
        .unwrap()
        .monthly()
        .months(2)
        .present_value()
        .unwrap();
    println!("pv: {pv}");
    assert_eq!(money, pv);
}

// ---- PMT (amortized payment) tests ----
//
// PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
// where r is the period rate and n is the total number of periods.

#[test]
fn test_pmt_months_yearly_rate() {
    // 30-year mortgage: $300,000 at 4% annual, Rate30360 (default)
    // monthly rate r = 4/12/100 = 0.003333...
    // c = (1 + r)^360, PMT = P × r × c / (c − 1) ≈ 1432.25
    let total = money!(USD, 300000.00);
    let pmt = total
        .interest_fixed(4)
        .unwrap()
        .yearly()
        .months(360)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(1432.25));
}

#[test]
fn test_pmt_months_short_yearly_rate() {
    // 3-month loan: $1,000 at 12% annual
    // monthly rate r = 12/12/100 = 0.01
    // c = (1.01)^3, PMT = 1000 × 0.01 × c / (c − 1) ≈ 340.02
    let total = money!(USD, 1000);
    let pmt = total
        .interest_fixed(12)
        .unwrap()
        .yearly()
        .months(3)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(340.02));
}

#[test]
fn test_pmt_years_yearly_rate() {
    // 5-year loan: $10,000 at 6% annual
    // yearly rate r = 6/100 = 0.06
    // c = (1.06)^5, PMT = 10000 × 0.06 × c / (c − 1) ≈ 2373.96
    let total = money!(USD, 10000);
    let pmt = total
        .interest_fixed(6)
        .unwrap()
        .yearly()
        .years(5)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(2373.96));
}

#[test]
fn test_pmt_years_short_yearly_rate() {
    // 3-year loan: $5,000 at 5% annual
    // yearly rate r = 5/100 = 0.05
    // c = (1.05)^3, PMT = 5000 × 0.05 × c / (c − 1) ≈ 1836.04
    let total = money!(USD, 5000);
    let pmt = total
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .years(3)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(1836.04));
}

#[test]
fn test_pmt_days_yearly_rate() {
    // 30-day loan: $1,000 at 12% annual, Rate30360
    // daily rate r = 12/360/100 = 0.000333...
    // c = (1 + r)^30, PMT ≈ 33.51
    let total = money!(USD, 1000);
    let pmt = total
        .interest_fixed(12)
        .unwrap()
        .yearly()
        .days(30)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(33.50));
}

#[test]
fn test_pmt_days_short_yearly_rate() {
    // 90-day loan: $1,000 at 3.6% annual, Rate30360
    // daily rate r = 3.6/360/100 = 0.0001
    // c = (1.0001)^90, PMT ≈ 11.16
    let total = money!(USD, 1000);
    let pmt = total
        .interest_fixed(dec!(3.6))
        .unwrap()
        .yearly()
        .days(90)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(11.16));
}

#[test]
fn test_pmt_months_returns_none_on_overflow() {
    // With Decimal::MAX as yearly rate, r = MAX/12/100 ≈ 6.6e25.
    // After 2 months c = (1 + r)^2 ≈ (6.6e25)^2 > Decimal::MAX → checked_mul overflows → None.
    let total = money!(USD, 1000);
    assert!(
        total
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .months(2)
            .year(2026)
            .month(1)
            .day(1)
            .payment()
            .is_none()
    );
}

#[test]
fn test_pmt_years_returns_none_on_overflow() {
    // With Decimal::MAX as yearly rate, r = MAX/100.
    // After 2 years c = (1 + r)^2 > Decimal::MAX → checked_mul overflows → None.
    let total = money!(USD, 1000);
    assert!(
        total
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .years(2)
            .year(2026)
            .month(1)
            .day(1)
            .payment()
            .is_none()
    );
}

#[test]
fn test_pmt_days_returns_none_on_overflow() {
    // With Decimal::MAX as yearly rate, r = MAX/360/100.
    // After 2 days c = (1 + r)^2 > Decimal::MAX → checked_mul overflows → None.
    let total = money!(USD, 1000);
    assert!(
        total
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .days(2)
            .year(2026)
            .month(1)
            .day(1)
            .payment()
            .is_none()
    );
}

#[test]
fn test_pmt_returns_none_on_zero_rate() {
    // With rate 0: r = 0, c = 1, c-1 = 0 → checked_div returns None (division by zero).
    let total = money!(USD, 1000);
    assert!(
        total
            .interest_fixed(0)
            .unwrap()
            .yearly()
            .months(12)
            .year(2026)
            .month(1)
            .day(1)
            .payment()
            .is_none()
    );
}

#[test]
fn test_pmt_quarters_yearly_rate() {
    // 4-quarter loan (1 year): $10,000 at 6% annual
    // quarterly rate r = 6/4/100 = 0.015
    // c = (1.015)^4, PMT = 10000 × 0.015 × c / (c − 1) ≈ 2594.45
    let total = money!(USD, 10000);
    let pmt = total
        .interest_fixed(6)
        .unwrap()
        .yearly()
        .quarters(4)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(2594.45));
}

#[test]
fn test_pmt_quarters_short_yearly_rate() {
    // 8-quarter loan (2 years): $5,000 at 12% annual
    // quarterly rate r = 12/4/100 = 0.03
    // c = (1.03)^8, PMT = 5000 × 0.03 × c / (c − 1) ≈ 712.28
    let total = money!(USD, 5000);
    let pmt = total
        .interest_fixed(12)
        .unwrap()
        .yearly()
        .quarters(8)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(712.28));
}

#[test]
fn test_pmt_semi_annuals_yearly_rate() {
    // 4 semi-annual periods (2 years): $10,000 at 6% annual
    // semi-annual rate r = 6/2/100 = 0.03
    // c = (1.03)^4, PMT = 10000 × 0.03 × c / (c − 1) ≈ 2690.27
    let total = money!(USD, 10000);
    let pmt = total
        .interest_fixed(6)
        .unwrap()
        .yearly()
        .semi_annuals(4)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(2690.27));
}

#[test]
fn test_pmt_semi_annuals_short_yearly_rate() {
    // 2 semi-annual periods (1 year): $5,000 at 8% annual
    // semi-annual rate r = 8/2/100 = 0.04
    // c = (1.04)^2, PMT = 5000 × 0.04 × c / (c − 1) ≈ 2650.98
    let total = money!(USD, 5000);
    let pmt = total
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .semi_annuals(2)
        .year(2026)
        .month(1)
        .day(1)
        .payment()
        .unwrap();
    assert_eq!(pmt.amount(), dec!(2650.98));
}

#[test]
fn test_pmt_quarters_returns_none_on_overflow() {
    // With Decimal::MAX as yearly rate, the quarterly rate overflows after 2 quarters → None.
    let total = money!(USD, 1000);
    assert!(
        total
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .quarters(2)
            .year(2026)
            .month(1)
            .day(1)
            .payment()
            .is_none()
    );
}

#[test]
fn test_pmt_semi_annuals_returns_none_on_overflow() {
    // With Decimal::MAX as yearly rate, the semi-annual rate overflows after 2 periods → None.
    let total = money!(USD, 1000);
    assert!(
        total
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .semi_annuals(2)
            .year(2026)
            .month(1)
            .day(1)
            .payment()
            .is_none()
    );
}

// ---- Fixed interest: quarterly period ----

#[test]
fn test_fixed_quarterly_returns_yearly_rate() {
    // P=1000, r=8% yearly, 4 quarters from 2026-01-01.
    // quarterly_rate = 8/4/100 = 0.02 (constant across all quarters for yearly rate).
    // returns = 4 × 1000 × 0.02 = 80.00
    let money = money!(USD, 1000);
    let interest = money
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4);

    assert_eq!(interest.returns().unwrap().amount(), dec!(80.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1080.00));
}

#[test]
fn test_fixed_quarterly_returns_monthly_rate() {
    // P=1000, r=1% monthly, 4 quarters from 2026-01-01.
    // get_quarterly_rate(Monthly(1)) = 1 × 3 / 100 = 0.03 per quarter.
    // returns = 4 × 1000 × 0.03 = 120.00
    let money = money!(USD, 1000);
    let interest = money
        .interest_fixed(1)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4);

    assert_eq!(interest.returns().unwrap().amount(), dec!(120.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1120.00));
}

#[test]
fn test_fixed_quarterly_returns_daily_rate() {
    // P=1000, r=1% daily, Rate30360, 4 quarters from 2026-01-01.
    // get_quarterly_rate(Daily(1)) with Rate30360 = 1 × (3×30) / 100 = 0.9 per quarter.
    // returns = 4 × 1000 × 0.9 = 3600.00
    let money = money!(USD, 1000);
    let interest = money
        .interest_fixed(1)
        .unwrap()
        .daily()
        .rate_days(RateDays::Rate30360)
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4);

    assert_eq!(interest.returns().unwrap().amount(), dec!(3600.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(4600.00));
}

#[test]
fn test_fixed_quarterly_returns_none_on_overflow() {
    // Decimal::MAX as rate: P × (MAX/400) overflows checked_mul → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .quarters(2)
            .returns()
            .is_none()
    );
}

// ---- Fixed interest: semi-annual period ----

#[test]
fn test_fixed_semi_annual_returns_yearly_rate() {
    // P=1000, r=8% yearly, 2 semi-annuals from 2026-01-01.
    // semi_annual_rate = 8/2/100 = 0.04 per period.
    // returns = 2 × 1000 × 0.04 = 80.00
    let money = money!(USD, 1000);
    let interest = money
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(80.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1080.00));
}

#[test]
fn test_fixed_semi_annual_returns_monthly_rate() {
    // P=1000, r=1% monthly, 2 semi-annuals from 2026-01-01.
    // get_semi_annualy_rate(Monthly(1)) = 1 × 6 / 100 = 0.06 per period.
    // returns = 2 × 1000 × 0.06 = 120.00
    let money = money!(USD, 1000);
    let interest = money
        .interest_fixed(1)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(120.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1120.00));
}

#[test]
fn test_fixed_semi_annual_returns_daily_rate() {
    // P=100, r=1% daily, Rate30360, 2 semi-annuals from 2026-01-01.
    // get_semi_annualy_rate(Daily(1)) with Rate30360 = 1 × (6×30) / 100 = 1.8 per period.
    // returns = 2 × 100 × 1.8 = 360.00
    let money = money!(USD, 100);
    let interest = money
        .interest_fixed(1)
        .unwrap()
        .daily()
        .rate_days(RateDays::Rate30360)
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(360.00));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(460.00));
}

#[test]
fn test_fixed_semi_annual_returns_none_on_overflow() {
    // Decimal::MAX as rate: P × (MAX/200) overflows checked_mul → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .semi_annuals(2)
            .returns()
            .is_none()
    );
}

// ---- Compound interest: quarterly period ----

#[test]
fn test_compound_quarterly_returns_yearly_rate() {
    // P=1000, r=8% yearly, 4 quarters from 2026-01-01.
    // quarterly_rate = 0.02.
    // Q1: 1000×0.02=20, total=20, principal=1020
    // Q2: 1020×0.02=20.4, total=40.4, principal=1040.4
    // Q3: 1040.4×0.02=20.808, total=61.208, principal=1061.208
    // Q4: 1061.208×0.02=21.22416, total=82.43216 → rounded to 82.43 (USD 2 dp)
    let money = money!(USD, 1000);
    let interest = money
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4);

    assert_eq!(interest.returns().unwrap().amount(), dec!(82.43));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1082.43));
}

#[test]
fn test_compound_quarterly_returns_monthly_rate() {
    // P=1000, r=1% monthly, 4 quarters from 2026-01-01.
    // quarterly_rate = 1×3/100 = 0.03.
    // Q1: 1000×0.03=30, total=30, principal=1030
    // Q2: 1030×0.03=30.9, total=60.9, principal=1060.9
    // Q3: 1060.9×0.03=31.827, total=92.727, principal=1092.727
    // Q4: 1092.727×0.03=32.78181, total=125.50881 → rounded to 125.51 (USD 2 dp)
    let money = money!(USD, 1000);
    let interest = money
        .interest_compound(1)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4);

    assert_eq!(interest.returns().unwrap().amount(), dec!(125.51));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1125.51));
}

#[test]
fn test_compound_quarterly_returns_daily_rate() {
    // P=10, r=1% daily, Rate30360, 4 quarters from 2026-01-01.
    // quarterly_rate = 1×90/100 = 0.9.
    // Q1: 10×0.9=9, total=9, principal=19
    // Q2: 19×0.9=17.1, total=26.1, principal=36.1
    // Q3: 36.1×0.9=32.49, total=58.59, principal=68.59
    // Q4: 68.59×0.9=61.731, total=120.321 → rounded to 120.32 (USD 2 dp)
    let money = money!(USD, 10);
    let interest = money
        .interest_compound(1)
        .unwrap()
        .daily()
        .rate_days(RateDays::Rate30360)
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4);

    assert_eq!(interest.returns().unwrap().amount(), dec!(120.32));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(130.32));
}

#[test]
fn test_compound_quarterly_returns_none_on_overflow() {
    // Decimal::MAX as rate overflows compound multiplication → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .quarters(2)
            .returns()
            .is_none()
    );
}

// ---- Compound interest: semi-annual period ----

#[test]
fn test_compound_semi_annual_returns_yearly_rate() {
    // P=1000, r=8% yearly, 2 semi-annuals from 2026-01-01.
    // semi_annual_rate = 0.04.
    // S1: 1000×0.04=40, total=40, principal=1040
    // S2: 1040×0.04=41.6, total=81.6
    let money = money!(USD, 1000);
    let interest = money
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(81.6));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1081.6));
}

#[test]
fn test_compound_semi_annual_returns_monthly_rate() {
    // P=1000, r=1% monthly, 2 semi-annuals from 2026-01-01.
    // semi_annual_rate = 1×6/100 = 0.06.
    // S1: 1000×0.06=60, total=60, principal=1060
    // S2: 1060×0.06=63.6, total=123.6
    let money = money!(USD, 1000);
    let interest = money
        .interest_compound(1)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(123.6));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1123.6));
}

#[test]
fn test_compound_semi_annual_returns_daily_rate() {
    // P=100, r=1% daily, Rate30360, 2 semi-annuals from 2026-01-01.
    // semi_annual_rate = 1×180/100 = 1.8.
    // S1: 100×1.8=180, total=180, principal=280
    // S2: 280×1.8=504, total=684
    let money = money!(USD, 100);
    let interest = money
        .interest_compound(1)
        .unwrap()
        .daily()
        .rate_days(RateDays::Rate30360)
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(684));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(784));
}

#[test]
fn test_compound_semi_annual_returns_none_on_overflow() {
    // Decimal::MAX as rate overflows compound multiplication → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .semi_annuals(2)
            .returns()
            .is_none()
    );
}

// ---- Present value: fixed quarterly (round-trip) ----

#[test]
fn test_pv_fixed_quarterly_yearly_rate() {
    // P=1000, r=8% yearly, 4 quarters: FV=1080.00.
    // PV: actual_r = 4×0.02 = 0.08; divisor = 1.08; PV = 1080/1.08 = 1000.
    let money = money!(USD, 1000);
    let fv = money
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(1080.00));
    let pv = fv
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_quarterly_none_on_overflow() {
    // Daily rate Decimal::MAX: get_quarterly_rate computes MAX × 90 which overflows → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .daily()
            .rate_days(RateDays::Rate30360)
            .year(2026)
            .month(1)
            .day(1)
            .quarters(2)
            .present_value()
            .is_none()
    );
}

// ---- Present value: fixed semi-annual (round-trip) ----

#[test]
fn test_pv_fixed_semi_annual_yearly_rate() {
    // P=1000, r=8% yearly, 2 semi-annuals: FV=1080.00.
    // PV: actual_r = 2×0.04 = 0.08; divisor = 1.08; PV = 1080/1.08 = 1000.
    let money = money!(USD, 1000);
    let fv = money
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(1080.00));
    let pv = fv
        .interest_fixed(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_fixed_semi_annual_none_on_overflow() {
    // Daily rate Decimal::MAX: get_semi_annualy_rate computes MAX × 180 which overflows → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_fixed(Decimal::MAX)
            .unwrap()
            .daily()
            .rate_days(RateDays::Rate30360)
            .year(2026)
            .month(1)
            .day(1)
            .semi_annuals(2)
            .present_value()
            .is_none()
    );
}

// ---- Present value: compound quarterly (round-trip) ----

#[test]
fn test_pv_compound_quarterly_yearly_rate() {
    // P=1000, r=8% yearly, 4 quarters: FV=1082.43 (rounds to 2 dp).
    // PV: divisor = (1.02)^4 = 1.08243216; PV = 1082.43/1.08243216 ≈ 999.9997 → rounds to 1000.00.
    let money = money!(USD, 1000);
    let fv = money
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(1082.43));
    let pv = fv
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(4)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_quarterly_none_on_overflow() {
    // Decimal::MAX causes overflow → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .quarters(2)
            .present_value()
            .is_none()
    );
}

// ---- Present value: compound semi-annual (round-trip) ----

#[test]
fn test_pv_compound_semi_annual_yearly_rate() {
    // P=1000, r=8% yearly, 2 semi-annuals: FV=1081.6.
    // PV: divisor = (1.04)^2 = 1.0816; PV = 1081.6/1.0816 = 1000.
    let money = money!(USD, 1000);
    let fv = money
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2)
        .future_value()
        .unwrap();
    assert_eq!(fv.amount(), dec!(1081.6));
    let pv = fv
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(2)
        .present_value()
        .unwrap();
    assert_eq!(pv, money);
}

#[test]
fn test_pv_compound_semi_annual_none_on_overflow() {
    // Decimal::MAX causes overflow → None.
    let money = money!(USD, 1000);
    assert!(
        money
            .interest_compound(Decimal::MAX)
            .unwrap()
            .yearly()
            .year(2026)
            .month(1)
            .day(1)
            .semi_annuals(2)
            .present_value()
            .is_none()
    );
}

// ============================================================================
// ============================= Contributions =================================
// ============================================================================
//
// Convention for contribs:
// - contribs.len() <= period - 1 (contribution starts from the second period).
// - Positive value  → add capital.
// - Negative value  → withdraw capital.
// - Zero value      → no-op (same result as no contribution).
//
// future_value() = principal + sum(contribs) + returns()

// ---- with_contribs validation ----

#[test]
fn test_with_contribs_too_many_returns_none() {
    // 3 months → max contribs length = 2; passing 3 contribs → None.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 100), money!(USD, 100), money!(USD, 100)];
    assert!(
        money
            .interest_fixed(10)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(3)
            .with_contribs(&contribs)
            .is_none()
    );
}

#[test]
fn test_with_contribs_exactly_max_length_ok() {
    // 3 months → max contribs length = 2; passing exactly 2 contribs → Some.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 100), money!(USD, 100)];
    assert!(
        money
            .interest_fixed(10)
            .unwrap()
            .monthly()
            .year(2026)
            .month(1)
            .day(1)
            .months(3)
            .with_contribs(&contribs)
            .is_some()
    );
}

#[test]
fn test_with_contribs_empty_slice_same_as_no_contribs() {
    // Empty contribs slice → same result as calling no with_contribs.
    // P=1000, r=10%/month, 3 months: returns = 300, FV = 1300.
    let money = money!(USD, 1000);
    let contribs: [crate::Money<crate::iso::USD>; 0] = [];
    let interest_with_empty = money
        .interest_fixed(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3)
        .with_contribs(&contribs)
        .unwrap();
    let interest_no_contribs = money
        .interest_fixed(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3);
    assert_eq!(
        interest_with_empty.returns().unwrap().amount(),
        interest_no_contribs.returns().unwrap().amount()
    );
    assert_eq!(
        interest_with_empty.future_value().unwrap().amount(),
        interest_no_contribs.future_value().unwrap().amount()
    );
}

// ---- Fixed rate with contributions ----

#[test]
fn test_fixed_monthly_rate_months_with_positive_contrib() {
    // P=1000, r=10%/month, 3 months, contribs=[100].
    // Month 1: interest = 1000 × 0.10 = 100, principal → 1100 (after contrib)
    // Month 2: interest = 1100 × 0.10 = 110
    // Month 3: interest = 1100 × 0.10 = 110
    // returns = 320, contribs_sum = 100, FV = 1420.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 100)];
    let interest = money
        .interest_fixed(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(320));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1420));
}

#[test]
fn test_fixed_yearly_rate_years_with_multiple_contribs() {
    // P=1000, r=10%/year, 3 years, contribs=[200, 300].
    // Year 1: interest = 1000 × 0.10 = 100, principal → 1200 (after 200 contrib)
    // Year 2: interest = 1200 × 0.10 = 120, principal → 1500 (after 300 contrib)
    // Year 3: interest = 1500 × 0.10 = 150
    // returns = 370, contribs_sum = 500, FV = 1870.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 200), money!(USD, 300)];
    let interest = money
        .interest_fixed(10)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(370));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1870));
}

#[test]
fn test_fixed_daily_rate_days_with_contrib() {
    // P=1000, r=1%/day, 3 days from 2026-01-01, contribs=[100].
    // Day 1: interest = 1000 × 0.01 = 10, principal → 1100 (after contrib)
    // Day 2: interest = 1100 × 0.01 = 11
    // Day 3: interest = 1100 × 0.01 = 11
    // returns = 32, contribs_sum = 100, FV = 1132.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 100)];
    let interest = money
        .interest_fixed(1)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(32));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1132));
}

#[test]
fn test_fixed_yearly_rate_quarters_with_contrib() {
    // P=1000, r=10%/year → quarterly rate = 2.5%, 3 quarters, contribs=[200].
    // Q1: interest = 1000 × 0.025 = 25, principal → 1200 (after contrib)
    // Q2: interest = 1200 × 0.025 = 30
    // Q3: interest = 1200 × 0.025 = 30
    // returns = 85, contribs_sum = 200, FV = 1285.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 200)];
    let interest = money
        .interest_fixed(10)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(85));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1285));
}

#[test]
fn test_fixed_yearly_rate_semi_annuals_with_contrib() {
    // P=1000, r=10%/year → semi-annual rate = 5%, 3 semi-annuals, contribs=[300].
    // S1: interest = 1000 × 0.05 = 50, principal → 1300 (after contrib)
    // S2: interest = 1300 × 0.05 = 65
    // S3: interest = 1300 × 0.05 = 65
    // returns = 180, contribs_sum = 300, FV = 1480.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 300)];
    let interest = money
        .interest_fixed(10)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(180));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1480));
}

#[test]
fn test_fixed_with_negative_contrib_withdrawal() {
    // P=1000, r=10%/month, 3 months, contribs=[-100] (withdrawal).
    // Month 1: interest = 1000 × 0.10 = 100, principal → 900 (after withdrawal)
    // Month 2: interest = 900 × 0.10 = 90
    // Month 3: interest = 900 × 0.10 = 90
    // returns = 280, contribs_sum = -100, FV = 1180.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, -100)];
    let interest = money
        .interest_fixed(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(280));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1180));
}

#[test]
fn test_fixed_with_zero_contrib_same_as_no_contrib() {
    // P=1000, r=10%/month, 3 months, contribs=[0] → same as no contribution.
    // returns = 300, FV = 1300.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 0)];
    let interest = money
        .interest_fixed(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(300));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1300));
}

// ---- Compounding rate with contributions ----

#[test]
fn test_compound_monthly_rate_months_with_positive_contrib() {
    // P=1000, r=10%/month, 3 months, contribs=[100].
    // Month 1: interest = 1000 × 0.10 = 100, balance = 1100, +100 → 1200
    // Month 2: interest = 1200 × 0.10 = 120, balance = 1320
    // Month 3: interest = 1320 × 0.10 = 132, balance = 1452
    // returns = 352, contribs_sum = 100, FV = 1452.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 100)];
    let interest = money
        .interest_compound(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(352));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1452));
}

#[test]
fn test_compound_yearly_rate_years_with_multiple_contribs() {
    // P=1000, r=10%/year, 3 years, contribs=[200, 300].
    // Year 1: interest = 1000 × 0.10 = 100, balance = 1100, +200 → 1300
    // Year 2: interest = 1300 × 0.10 = 130, balance = 1430, +300 → 1730
    // Year 3: interest = 1730 × 0.10 = 173, balance = 1903
    // returns = 403, contribs_sum = 500, FV = 1903.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 200), money!(USD, 300)];
    let interest = money
        .interest_compound(10)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .years(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(403));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1903));
}

#[test]
fn test_compound_daily_rate_days_with_contrib() {
    // P=1000, r=10%/day, 3 days from 2026-01-01, contribs=[100].
    // Day 1: interest = 1000 × 0.10 = 100, balance = 1100, +100 → 1200
    // Day 2: interest = 1200 × 0.10 = 120, balance = 1320
    // Day 3: interest = 1320 × 0.10 = 132, balance = 1452
    // returns = 352, contribs_sum = 100, FV = 1452.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 100)];
    let interest = money
        .interest_compound(10)
        .unwrap()
        .daily()
        .year(2026)
        .month(1)
        .day(1)
        .days(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(352));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1452));
}

#[test]
fn test_compound_yearly_rate_quarters_with_contrib() {
    // P=1000, r=8%/year → quarterly rate = 2%, 3 quarters, contribs=[200].
    // Q1: interest = 1000 × 0.02 = 20, balance = 1020, +200 → 1220
    // Q2: interest = 1220 × 0.02 = 24.4, balance = 1244.4
    // Q3: interest = 1244.4 × 0.02 = 24.888, balance = 1269.288
    // returns = 69.29 (USD), contribs_sum = 200, FV = 1269.29.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 200)];
    let interest = money
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .quarters(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(69.29));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1269.29));
}

#[test]
fn test_compound_yearly_rate_semi_annuals_with_contrib() {
    // P=1000, r=8%/year → semi-annual rate = 4%, 3 semi-annuals, contribs=[300].
    // S1: interest = 1000 × 0.04 = 40, balance = 1040, +300 → 1340
    // S2: interest = 1340 × 0.04 = 53.6, balance = 1393.6
    // S3: interest = 1393.6 × 0.04 = 55.744, balance = 1449.344
    // returns = 149.34 (USD), contribs_sum = 300, FV = 1449.34.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, 300)];
    let interest = money
        .interest_compound(8)
        .unwrap()
        .yearly()
        .year(2026)
        .month(1)
        .day(1)
        .semi_annuals(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(149.34));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1449.34));
}

#[test]
fn test_compound_with_negative_contrib_withdrawal() {
    // P=1000, r=10%/month, 3 months, contribs=[-100] (withdrawal).
    // Month 1: interest = 1000 × 0.10 = 100, balance = 1100, -100 → 1000
    // Month 2: interest = 1000 × 0.10 = 100, balance = 1100
    // Month 3: interest = 1100 × 0.10 = 110, balance = 1210
    // returns = 310, contribs_sum = -100, FV = 1210.
    let money = money!(USD, 1000);
    let contribs = [money!(USD, -100)];
    let interest = money
        .interest_compound(10)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(3)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(310));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(1210));
}

// ---- Many contributions (100) with mixed add/withdraw/zero ----

#[test]
fn test_fixed_monthly_rate_101months_100_mixed_contribs() {
    // P=1000, r=1%/month, 101 months, 100 contribs cycling [+100, -50, 0].
    // contribs_sum = (34 × 100) + (33 × -50) + (33 × 0) = 3400 - 1650 = 1750
    // Each month: interest = current_principal × 0.01; after interest contrib is applied.
    // Computed returns = 1885.50, FV = principal + contribs_sum + returns = 4635.50.
    let money = money!(USD, 1000);
    let contribs: Vec<crate::Money<crate::iso::USD>> = (0..100usize)
        .map(|i| match i % 3 {
            0 => money!(USD, 100),
            1 => money!(USD, -50),
            _ => money!(USD, 0),
        })
        .collect();
    let interest = money
        .interest_fixed(1)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(101)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(1885.50));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(4635.50));
}

#[test]
fn test_compound_monthly_rate_101months_100_mixed_contribs() {
    // P=1000, r=1%/month, 101 months, 100 contribs cycling [+100, -50, 0].
    // contribs_sum = 1750, balance compounds each month then contrib applied.
    // Computed returns ≈ 2992.76, FV = principal + contribs_sum + returns ≈ 5742.76.
    let money = money!(USD, 1000);
    let contribs: Vec<crate::Money<crate::iso::USD>> = (0..100usize)
        .map(|i| match i % 3 {
            0 => money!(USD, 100),
            1 => money!(USD, -50),
            _ => money!(USD, 0),
        })
        .collect();
    let interest = money
        .interest_compound(1)
        .unwrap()
        .monthly()
        .year(2026)
        .month(1)
        .day(1)
        .months(101)
        .with_contribs(&contribs)
        .unwrap();
    assert_eq!(interest.returns().unwrap().amount(), dec!(2992.76));
    assert_eq!(interest.future_value().unwrap().amount(), dec!(5742.76));
}
