use crate::BaseMoney;
use crate::accounting::InterestOps;
use crate::accounting::interest::RateDays;
use crate::macros::{dec, money};

// ---- Fixed interest: daily rate ----

#[test]
fn test_fixed_daily_days() {
    // P=5000, r=5% daily, 100 days: returns = P * r * t = 5000 * 0.05 * 100 = 25000
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().daily().days(100);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(25_000));
    assert_eq!(total.amount(), dec!(30_000));
}

#[test]
fn test_fixed_daily_months() {
    // P=5000, r=5% daily, 12 months starting 2026-03-16: each month = P * r * days_in_month
    // Mar–Feb spans 365 days total: returns = 5000 * 0.05 * 365 = 91250
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().daily().months(12);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(91250.00));
    assert_eq!(total.amount(), dec!(96250.00));
}

#[test]
fn test_fixed_daily_years() {
    // P=5000, r=5% daily, 2 years starting 2026 (both non-leap, 365 days each):
    // returns = 5000 * 0.05 * 365 * 2 = 182500
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().daily().years(2);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(182500.00));
    assert_eq!(total.amount(), dec!(187500.00));
}

// Rate30360 convention eliminates date-sensitivity: 360 days/year, 30 days/month.

#[test]
fn test_fixed_daily_days_rate30360() {
    // P=5000, r=5% daily, Rate30360, 100 days: returns = P * r * t = 5000 * 0.05 * 100 = 25000
    // (daily rate is r/100, independent of rate_days convention)
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(100);

    assert_eq!(interest.returns().unwrap().amount(), dec!(25_000));
    assert_eq!(interest.total().unwrap().amount(), dec!(30_000));
}

#[test]
fn test_fixed_daily_months_rate30360() {
    // P=5000, r=5% daily, Rate30360, 12 months from 2024-01:
    // each month = P * r * 30 / 100 = 5000 * 5 * 30 / 100 = 7500 per month * 12 = 90000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(90000));
    assert_eq!(interest.total().unwrap().amount(), dec!(95000));
}

#[test]
fn test_fixed_daily_years_rate30360() {
    // P=5000, r=5% daily, Rate30360, 2 years: each year = P * r * 360 / 100 = 90000
    // returns = 90000 * 2 = 180000
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .daily()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(180000));
    assert_eq!(interest.total().unwrap().amount(), dec!(185000));
}

// ---- Fixed interest: monthly rate ----

#[test]
fn test_fixed_monthly_days() {
    // P=5000, r=5% monthly, 100 days starting 2026-03-16 (RateActualActual):
    // each day = P * r / days_in_month / 100; total ≈ 820.70 (date-sensitive)
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().monthly().days(100);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(820.70));
    assert_eq!(total.amount(), dec!(5820.70));
}

#[test]
fn test_fixed_monthly_days_rate30360() {
    // P=5000, r=5% monthly, Rate30360, 100 days from 2024-01-01:
    // each day = P * r / 30 / 100 = 5000 * 5 / 3000 = 8.333...
    // 100 days: 8.333... * 100 = 833.33
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .monthly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(100);

    assert_eq!(interest.returns().unwrap().amount(), dec!(833.33));
    assert_eq!(interest.total().unwrap().amount(), dec!(5833.33));
}

#[test]
fn test_fixed_monthly_months() {
    // P=5000, r=5% monthly, 12 months: returns = P * r * t = 5000 * 0.05 * 12 = 3000
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().monthly().months(12);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(3000));
    assert_eq!(total.amount(), dec!(8000));
}

#[test]
fn test_fixed_monthly_years() {
    // P=5000, r=5% monthly, 2 years: yearly_rate = r * 12 / 100 = 60%
    // returns = P * 0.60 * 2 = 6000
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().monthly().years(2);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(6000));
    assert_eq!(total.amount(), dec!(11000));
}

// ---- Fixed interest: yearly rate ----

#[test]
fn test_fixed_yearly_years() {
    // P=5000, r=5% yearly, 2 years: returns = P * r * t = 5000 * 0.05 * 2 = 500
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().yearly().years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(500.00));
    assert_eq!(interest.total().unwrap().amount(), dec!(5500.00));
}

#[test]
fn test_fixed_yearly_months() {
    // P=5000, r=5% yearly, 12 months: monthly_rate = r/12/100
    // returns = P * (r/12/100) * 12 = P * r/100 = 5000 * 0.05 = 250
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().yearly().months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(250.00));
    assert_eq!(interest.total().unwrap().amount(), dec!(5250.00));
}

#[test]
fn test_fixed_yearly_days_rate30360() {
    // P=5000, r=5% yearly, Rate30360, 360 days from 2024-01-01:
    // daily_rate = r / 360 / 100; returns = P * (5/36000) * 360 = 5000 * 0.05 = 250
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(360);

    assert_eq!(interest.returns().unwrap().amount(), dec!(250.00));
    assert_eq!(interest.total().unwrap().amount(), dec!(5250.00));
}

// ---- Compounding interest: daily rate ----

#[test]
fn test_compound_daily_days() {
    // P=5000, r=5% daily, 10 days from 2024-01-01:
    // each day: current_interest = current_principal * 0.05, principal compounds
    // returns = 3144.47 (rounded to USD 2 decimals)
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2024)
        .month(1)
        .day(1)
        .days(10);

    assert_eq!(interest.returns().unwrap().amount(), dec!(3144.47));
    assert_eq!(interest.total().unwrap().amount(), dec!(8144.47));
}

#[test]
fn test_compound_daily_months_rate30360() {
    // P=5000, r=5% daily, Rate30360, 12 months from 2024-01:
    // monthly_rate = r * 30 / 100 = 150% per month; compounded monthly
    // returns = 298018223.88 (large due to extreme daily rate compounding monthly)
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(298018223.88));
    assert_eq!(interest.total().unwrap().amount(), dec!(298023223.88));
}

#[test]
fn test_compound_daily_years_rate30360() {
    // P=5000, r=5% daily, Rate30360, 2 years from 2024:
    // yearly_rate = r * 360 / 100 = 1800% per year; compounded annually
    // Year 1: 5000 * 18 = 90000, CP = 95000
    // Year 2: 95000 * 18 = 1710000, CP = 1805000; returns = 1800000
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .daily()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(1800000));
    assert_eq!(interest.total().unwrap().amount(), dec!(1805000));
}

// ---- Compounding interest: monthly rate ----

#[test]
fn test_compound_monthly_days_rate30360() {
    // P=5000, r=5% monthly, Rate30360, 10 days from 2024-01-01:
    // daily_rate = r / 30 / 100 = 0.1667%; each day compounds
    // returns = 83.96
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(10);

    assert_eq!(interest.returns().unwrap().amount(), dec!(83.96));
    assert_eq!(interest.total().unwrap().amount(), dec!(5083.96));
}

#[test]
fn test_compound_monthly_months() {
    // P=5000, r=5% monthly, 12 months: each month = current_principal * 0.05, compounds
    // returns = 3979.28
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2024)
        .month(1)
        .day(1)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(3979.28));
    assert_eq!(interest.total().unwrap().amount(), dec!(8979.28));
}

#[test]
fn test_compound_monthly_years() {
    // P=5000, r=5% monthly, 2 years: yearly_rate = r * 12 / 100 = 60% per year, compounds
    // Year 1: 5000 * 0.60 = 3000, CP = 8000
    // Year 2: 8000 * 0.60 = 4800, CP = 12800; returns = 7800
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .monthly()
        .year(2024)
        .month(1)
        .day(1)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(7800.00));
    assert_eq!(interest.total().unwrap().amount(), dec!(12800.00));
}

// ---- Compounding interest: yearly rate ----

#[test]
fn test_compound_yearly_days_rate30360() {
    // P=5000, r=5% yearly, Rate30360, 10 days from 2024-01-01:
    // daily_rate = r / 360 / 100 ≈ 0.01389%; each day compounds
    // returns = 6.95
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(10);

    assert_eq!(interest.returns().unwrap().amount(), dec!(6.95));
    assert_eq!(interest.total().unwrap().amount(), dec!(5006.95));
}

#[test]
fn test_compound_yearly_months() {
    // P=5000, r=5% yearly, 12 months: each month = current_principal * (r/12/100), compounds
    // returns = 255.81
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .months(12);

    assert_eq!(interest.returns().unwrap().amount(), dec!(255.81));
    assert_eq!(interest.total().unwrap().amount(), dec!(5255.81));
}

#[test]
fn test_compound_yearly_years() {
    // P=5000, r=5% yearly, 2 years: compounds annually at 5%
    // Year 1: 5000 * 0.05 = 250, CP = 5250
    // Year 2: 5250 * 0.05 = 262.50; returns = 512.50
    let money = money!(USD, 5000);
    let interest = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .years(2);

    assert_eq!(interest.returns().unwrap().amount(), dec!(512.50));
    assert_eq!(interest.total().unwrap().amount(), dec!(5512.50));
}

// ---- Edge cases ----

#[test]
fn test_zero_rate_fixed() {
    // A 0% interest rate should yield zero returns
    let money = money!(USD, 5000);

    let i = money.interest_fixed(0).unwrap().daily().days(100);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.total().unwrap().amount(), dec!(5000));

    let i = money.interest_fixed(0).unwrap().monthly().months(12);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.total().unwrap().amount(), dec!(5000));

    let i = money.interest_fixed(0).unwrap().yearly().years(2);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.total().unwrap().amount(), dec!(5000));
}

#[test]
fn test_zero_rate_compound() {
    // A 0% compounding rate should also yield zero returns
    let money = money!(USD, 5000);

    let i = money.interest_compound(0).unwrap().daily().days(100);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.total().unwrap().amount(), dec!(5000));

    let i = money.interest_compound(0).unwrap().monthly().months(12);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.total().unwrap().amount(), dec!(5000));

    let i = money.interest_compound(0).unwrap().yearly().years(2);
    assert_eq!(i.returns().unwrap().amount(), dec!(0));
    assert_eq!(i.total().unwrap().amount(), dec!(5000));
}

#[test]
fn test_rate_days_rate30360_vs_actual_actual() {
    // Rate30360 uses 30 days/month and 360 days/year, so:
    //   yearly rate → daily: 5/360/100
    // RateActualActual uses 366 days for 2024 (leap year), so:
    //   yearly rate → daily: 5/366/100
    // The two conventions should produce different results for the same inputs.
    let money = money!(USD, 5000);

    let i_30360 = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::Rate30360)
        .days(360);

    let i_actual = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActualActual)
        .days(360);

    // Rate30360: 5000 * (5/360/100) * 360 = 250.00
    assert_eq!(i_30360.returns().unwrap().amount(), dec!(250.00));
    // RateActualActual: 5000 * (5/366/100) * 360 ≠ 250 (2024 is leap, 366 days)
    assert_ne!(
        i_actual.returns().unwrap().amount(),
        i_30360.returns().unwrap().amount()
    );
}

#[test]
fn test_rate_days_rate_actual365() {
    // RateActual365 always uses 365 days/year regardless of leap year.
    // For 2024 (leap year): fixed yearly days(365) with RateActual365
    // daily_rate = 5/365/100; returns = 5000 * (5/365/100) * 365 = 250
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActual365)
        .days(365);

    assert_eq!(interest.returns().unwrap().amount(), dec!(250.00));
    assert_eq!(interest.total().unwrap().amount(), dec!(5250.00));
}

#[test]
fn test_rate_days_rate_actual360() {
    // RateActual360: actual days/month, 360 days/year.
    // Yearly rate: daily_rate = 5/360/100 (fixed at 360 regardless of actual year length).
    // For 60 days from 2024-01-01 (Jan 31 + Feb 29 = actual days used):
    // returns = 5000 * (5/360/100) * 60 = 5000 * 5 * 60 / 36000 = 41.67
    let money = money!(USD, 5000);
    let interest = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActual360)
        .days(60);

    assert_eq!(interest.returns().unwrap().amount(), dec!(41.67));
    assert_eq!(interest.total().unwrap().amount(), dec!(5041.67));
}

#[test]
fn test_year_month_day_setters() {
    // Verify that the year/month/day builder setters affect date-sensitive calculations.
    // Fixed yearly days from 2024-02-01 vs 2024-01-01 with RateActualActual:
    // both have daily_rate = 5/366/100 (same leap year), but the months visited differ.
    let money = money!(USD, 5000);

    let i_jan = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .rate_days(RateDays::RateActualActual)
        .days(30);

    let i_feb = money
        .interest_fixed(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(2)
        .day(1)
        .rate_days(RateDays::RateActualActual)
        .days(30);

    // Same rate (both in 2024 leap year), same number of days → same result
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
        .year(2024)
        .month(1)
        .day(1)
        .years(3);
    let compound = money
        .interest_compound(5)
        .unwrap()
        .yearly()
        .year(2024)
        .month(1)
        .day(1)
        .years(3);

    assert!(compound.returns().unwrap().amount() > fixed.returns().unwrap().amount());
    assert!(compound.total().unwrap().amount() > fixed.total().unwrap().amount());
}

#[test]
fn test_total_equals_principal_plus_returns() {
    // total() must always equal principal + returns() for all combinations.
    let money = money!(USD, 5000);

    let cases: &[_] = &[
        money.interest_fixed(5).unwrap().daily().days(100),
        money.interest_fixed(5).unwrap().monthly().months(6),
        money.interest_fixed(5).unwrap().yearly().years(1),
        money
            .interest_compound(5)
            .unwrap()
            .daily()
            .year(2024)
            .month(1)
            .day(1)
            .days(5),
        money
            .interest_compound(5)
            .unwrap()
            .monthly()
            .year(2024)
            .month(1)
            .day(1)
            .months(6),
        money
            .interest_compound(5)
            .unwrap()
            .yearly()
            .year(2024)
            .month(1)
            .day(1)
            .years(1),
    ];

    for interest in cases {
        let returns = interest.returns().unwrap().amount();
        let total = interest.total().unwrap().amount();
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
