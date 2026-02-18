use crate::{EUR, IDR, USD};

use crate::money_macros::dec;
use crate::{BaseMoney, Money};

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
    // let check = money1 == money3;
    // let check = money2 > money3;

    let check = money1 > money2;
    assert!(check);

    let check = money1 < money2;
    assert!(!check);

    let check = money1 >= money2;
    assert!(check);

    let check = money1 <= money2;
    assert!(!check);
}
