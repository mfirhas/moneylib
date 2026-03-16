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

#[test]
fn compute_values_for_new_tests() {
    use crate::accounting::interest::RateDays;
    
    let p = money!(USD, 5000);
    
    // Fixed yearly
    let i = p.interest_fixed(5).unwrap().yearly().year(2024).month(1).day(1).years(2);
    println!("Fixed yearly years(2): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_fixed(5).unwrap().yearly().year(2024).month(1).day(1).months(12);
    println!("Fixed yearly months(12): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_fixed(5).unwrap().yearly().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).days(360);
    println!("Fixed yearly days(360) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    // Compound yearly
    let i = p.interest_compound(5).unwrap().yearly().year(2024).month(1).day(1).years(2);
    println!("Compound yearly years(2): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_compound(5).unwrap().yearly().year(2024).month(1).day(1).months(12);
    println!("Compound yearly months(12): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_compound(5).unwrap().yearly().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).days(10);
    println!("Compound yearly days(10) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    // Compound monthly
    let i = p.interest_compound(5).unwrap().monthly().year(2024).month(1).day(1).years(2);
    println!("Compound monthly years(2): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_compound(5).unwrap().monthly().year(2024).month(1).day(1).months(12);
    println!("Compound monthly months(12): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_compound(5).unwrap().monthly().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).days(10);
    println!("Compound monthly days(10) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    // Compound daily
    let i = p.interest_compound(5).unwrap().daily().year(2024).month(1).day(1).days(10);
    println!("Compound daily days(10): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_compound(5).unwrap().daily().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).months(12);
    println!("Compound daily months(12) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_compound(5).unwrap().daily().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).years(2);
    println!("Compound daily years(2) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    // Fixed monthly Rate30360
    let i = p.interest_fixed(5).unwrap().monthly().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).days(100);
    println!("Fixed monthly days(100) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_fixed(5).unwrap().monthly().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).months(12);
    println!("Fixed monthly months(12) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_fixed(5).unwrap().monthly().year(2024).month(1).day(1).years(2);
    println!("Fixed monthly years(2): returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    // Fixed daily Rate30360
    let i = p.interest_fixed(5).unwrap().daily().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).days(100);
    println!("Fixed daily days(100) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_fixed(5).unwrap().daily().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).months(12);
    println!("Fixed daily months(12) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
    
    let i = p.interest_fixed(5).unwrap().daily().year(2024).month(1).day(1).rate_days(RateDays::Rate30360).years(2);
    println!("Fixed daily years(2) Rate30360: returns={}, total={}", i.returns().unwrap().amount(), i.total().unwrap().amount());
}
