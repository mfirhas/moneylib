use crate::iso::{EUR, IDR, JPY, USD};

use crate::macros::dec;
use crate::{BaseMoney, BaseOps, Money};

/// Test adding 2 moneys with same currencies
#[test]
fn test_add_same_currencies() {
    let money1 = Money::<USD>::new(dec!(100.00)).unwrap();
    let money2 = Money::<USD>::new(dec!(50.00)).unwrap();

    let result = money1 + money2;
    assert_eq!(result.amount(), dec!(150.00));
}

/// Test adding moneys with different currencies, won't even compile
#[test]
fn test_add_different_currencies_wont_compile() {
    // let money1 = Money::<USD>::new(dec!(100.50)).unwrap();
    // use crate::EUR;
    // let money2 = Money::<EUR>::new(dec!(50.25)).unwrap();

    // // won't even compile
    // let diff = money1 != money2;
    // let c = money1 == money2;
    // let result = money1 + money2;
    // assert_eq!(result.amount(), dec!(150.75));
}

#[test]
fn test_multiple_arithmetics() {
    let money1 = Money::<IDR>::new(dec!(1000.00)).unwrap();
    let money2 = Money::<IDR>::new(dec!(5000.00)).unwrap();
    let money3 = Money::<IDR>::from_decimal(dec!(123_000_000));

    let ret = (money1 * money2) + money3;
    assert_eq!(ret.amount(), dec!(128_000_000));
}

#[test]
fn test_arithmetics_with_decimals() {
    let money1 = Money::<EUR>::from_decimal(dec!(123234));
    let money2 = Money::<EUR>::from_decimal(dec!(1230));
    let amount = dec!(1230);
    let amount2 = dec!(40000000);

    let a = money1 - amount;
    let b = money2 + a;
    let c = amount2 - b;
    let d = a * c + b / amount - dec!(2);
    assert_eq!(c.amount(), dec!(39876766));
    assert_eq!(d.amount(), dec!(4865124959162.19));
}

#[test]
fn test_operator_ordering_equality() {
    let money1 = Money::<EUR>::from_decimal(dec!(123234));
    let money2 = Money::<EUR>::from_decimal(dec!(1230));
    let money3 = Money::<EUR>::from_decimal(dec!(1230));

    let check = money1 == money2;
    assert!(!check);
    let check = money1 != money2;
    assert!(check);
    let check = money2 == money3;
    assert!(check);

    // // wont even compile
    // let money4 = Money::<IDR>::from_decimal(dec!(400000));
    // let check = money1 == money4;
    // let check = money2 > money4;

    let check = money1 > money2;
    assert!(check);

    let check = money1 < money2;
    assert!(!check);

    let check = money1 >= money2;
    assert!(check);

    let check = money1 <= money2;
    assert!(!check);
}

// ==================== split ====================

#[test]
fn test_split_with_remainder() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal.amount(), dec!(33.33));
    assert_eq!(remainder.amount(), dec!(0.01));
}

#[test]
fn test_split_no_remainder() {
    let money = Money::<USD>::new(dec!(500)).unwrap();
    let (equal, remainder) = money.split(4).unwrap();
    assert_eq!(equal.amount(), dec!(125.00));
    assert!(remainder.is_zero());
}

#[test]
fn test_split_zero_n_returns_none() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    assert!(money.split(0).is_none());
}

#[test]
fn test_split_into_one() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    let (equal, remainder) = money.split(1).unwrap();
    assert_eq!(equal.amount(), dec!(100.00));
    assert!(remainder.is_zero());
}

#[test]
fn test_split_jpy_no_minor_unit() {
    let money = Money::<JPY>::new(dec!(10)).unwrap();
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal.amount(), dec!(3));
    assert_eq!(remainder.amount(), dec!(1));
}

#[test]
fn test_split_negative_amount() {
    let money = Money::<USD>::new(dec!(-100)).unwrap();
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal.amount(), dec!(-33.33));
    assert_eq!(remainder.amount(), dec!(-0.01));
}

// ==================== split_dist ====================

#[test]
fn test_split_dist_with_remainder() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    let parts = money.split_dist(3).unwrap();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].amount(), dec!(33.34));
    assert_eq!(parts[1].amount(), dec!(33.33));
    assert_eq!(parts[2].amount(), dec!(33.33));
    // Verify sum equals original
    let sum: crate::Decimal = parts.iter().map(|p| p.amount()).sum();
    assert_eq!(sum, dec!(100.00));
}

#[test]
fn test_split_dist_no_remainder() {
    let money = Money::<USD>::new(dec!(500)).unwrap();
    let parts = money.split_dist(4).unwrap();
    assert_eq!(parts.len(), 4);
    assert!(parts.iter().all(|p| p.amount() == dec!(125.00)));
}

#[test]
fn test_split_dist_zero_n_returns_none() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    assert!(money.split_dist(0).is_none());
}

#[test]
fn test_split_dist_into_one() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    let parts = money.split_dist(1).unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].amount(), dec!(100.00));
}

#[test]
fn test_split_dist_jpy() {
    let money = Money::<JPY>::new(dec!(10)).unwrap();
    let parts = money.split_dist(3).unwrap();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].amount(), dec!(4));
    assert_eq!(parts[1].amount(), dec!(3));
    assert_eq!(parts[2].amount(), dec!(3));
    let sum: crate::Decimal = parts.iter().map(|p| p.amount()).sum();
    assert_eq!(sum, dec!(10));
}

// ==================== allocate ====================

#[test]
fn test_allocate_even_split() {
    let money = Money::<USD>::new(dec!(10000.00)).unwrap();
    let shares = money.allocate(&[60, 40]).unwrap();
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].amount(), dec!(6000.00));
    assert_eq!(shares[1].amount(), dec!(4000.00));
}

#[test]
fn test_allocate_five_weights() {
    let money = Money::<USD>::new(dec!(100000.00)).unwrap();
    let depts = money.allocate(&[35, 25, 20, 15, 5]).unwrap();
    assert_eq!(depts.len(), 5);
    assert_eq!(depts[0].amount(), dec!(35000.00));
    assert_eq!(depts[1].amount(), dec!(25000.00));
    assert_eq!(depts[2].amount(), dec!(20000.00));
    assert_eq!(depts[3].amount(), dec!(15000.00));
    assert_eq!(depts[4].amount(), dec!(5000.00));
}

#[test]
fn test_allocate_with_remainder_distribution() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    let parts = money.allocate(&[33, 33, 34]).unwrap();
    assert_eq!(parts.len(), 3);
    // Sum must equal original
    let sum: crate::Decimal = parts.iter().map(|p| p.amount()).sum();
    assert_eq!(sum, dec!(100.00));
}

#[test]
fn test_allocate_percentages_not_100_returns_none() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    assert!(money.allocate(&[60, 30]).is_none()); // sums to 90, not 100
}

#[test]
fn test_allocate_empty_returns_none() {
    let money = Money::<USD>::new(dec!(100)).unwrap();
    let empty: &[i32] = &[];
    assert!(money.allocate(empty).is_none());
}

// ==================== allocate_by_ratios ====================

#[test]
fn test_allocate_by_ratios_equal() {
    let amount = Money::<USD>::new(dec!(400.00)).unwrap();
    let parts = amount.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].amount(), dec!(100.00));
    assert_eq!(parts[1].amount(), dec!(200.00));
    assert_eq!(parts[2].amount(), dec!(100.00));
}

#[test]
fn test_allocate_by_ratios_uneven_remainder() {
    let amount = Money::<USD>::new(dec!(1)).unwrap();
    let parts = amount.allocate_by_ratios(&[1, 1, 1]).unwrap();
    assert_eq!(parts.len(), 3);
    // Remainder 0.01 goes to first part
    assert_eq!(parts[0].amount(), dec!(0.34));
    assert_eq!(parts[1].amount(), dec!(0.33));
    assert_eq!(parts[2].amount(), dec!(0.33));
    let sum: crate::Decimal = parts.iter().map(|p| p.amount()).sum();
    assert_eq!(sum, dec!(1.00));
}

#[test]
fn test_allocate_by_ratios_single() {
    let amount = Money::<USD>::new(dec!(100)).unwrap();
    let parts = amount.allocate_by_ratios(&[1]).unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].amount(), dec!(100.00));
}

#[test]
fn test_allocate_by_ratios_empty_returns_none() {
    let amount = Money::<USD>::new(dec!(100)).unwrap();
    let empty: &[i32] = &[];
    assert!(amount.allocate_by_ratios(empty).is_none());
}

#[test]
fn test_allocate_by_ratios_all_zero_returns_none() {
    let amount = Money::<USD>::new(dec!(100)).unwrap();
    assert!(amount.allocate_by_ratios(&[0, 0, 0]).is_none());
}
