use crate::BaseMoney;
use crate::accounting::InterestOps;
use crate::macros::{dec, money};

#[test]
fn test_fixed_daily_days() {
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().daily().days(100);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(25_000));
    assert_eq!(total.amount(), dec!(30_000));
}

#[test]
fn test_fixed_daily_months() {
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().daily().months(12);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(91250.00));
    assert_eq!(total.amount(), dec!(96250.00));
}

#[test]
fn test_fixed_daily_years() {
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().daily().years(2);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(182500.00));
    assert_eq!(total.amount(), dec!(187500.00));
}

#[test]
fn test_fixed_monthly_days() {
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().monthly().days(100);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(820.70));
    assert_eq!(total.amount(), dec!(5820.70));
}

#[test]
fn test_fixed_monthly_months() {
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().monthly().months(12);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(3000));
    assert_eq!(total.amount(), dec!(8000));
}

#[test]
fn test_fixed_monthly_years() {
    let money = money!(USD, 5000);
    let interest = money.interest_fixed(5).unwrap().monthly().years(2);
    let returns = interest.returns().unwrap();
    let total = interest.total().unwrap();

    assert_eq!(returns.amount(), dec!(6000));
    assert_eq!(total.amount(), dec!(11000));
}
