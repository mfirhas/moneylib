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
