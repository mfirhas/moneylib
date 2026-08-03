use crate::obj_money::DynMoney;
use crate::obj_money::*;
use crate::{BaseMoney, dec};

#[test]
fn currency_try_new() {
    let currency = ObjCurrency::try_new("USD", "$", "¢", "US Dollar", 2).unwrap();

    let money: ObjMoney = ObjMoney::new(currency, dec!(123.45));

    assert_eq!(money.code(), "USD");
    assert_eq!(money.symbol(), "$");
    assert_eq!(money.minor_unit_symbol(), "¢");
    assert_eq!(money.name(), "US Dollar");
    assert_eq!(money.minor_unit(), 2);
    assert_eq!(money.amount(), dec!(123.45));
}

#[test]
fn currency_try_new_empty_minor_symbol() {
    let currency = ObjCurrency::try_new("JPY", "¥", "", "Japanese Yen", 0).unwrap();

    let obj_money = ObjMoney::<false>::new(currency, dec!(123.98));

    assert_eq!(obj_money.minor_unit(), 0);
    assert_eq!(obj_money.amount(), dec!(124));
}

#[test]
fn currency_try_new_invalid_code() {
    assert!(ObjCurrency::try_new("", "$", "¢", "Dollar", 2).is_err());
}

#[test]
fn currency_try_new_invalid_symbol() {
    assert!(ObjCurrency::try_new("USD", "", "¢", "Dollar", 2).is_err());
}

#[test]
fn currency_try_new_invalid_name() {
    assert!(ObjCurrency::try_new("USD", "$", "¢", "", 2).is_err());
}

#[test]
fn obj_money_try_new() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(123.45)).unwrap();

    assert_eq!(money.code(), "USD");
    assert_eq!(money.amount(), dec!(123.45));
    assert_eq!(money.symbol(), "$");
    assert_eq!(money.name(), "United States dollar");
    assert_eq!(money.minor_unit(), 2);
}

#[test]
fn obj_money_try_new_unknown_currency() {
    assert!(ObjMoney::<false>::try_new("ZZZ", dec!(1)).is_err());
}

#[test]
fn obj_money_try_new_invalid_code() {
    assert!(ObjMoney::<false>::try_new("", dec!(1)).is_err());
}

#[test]
fn set_amount() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1))
        .unwrap()
        .set_amount(dec!(99.99));

    assert_eq!(money.amount(), dec!(99.99));
    assert_eq!(money.code(), "USD");
}

#[test]
fn amount_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(123.45)).unwrap();

    assert_eq!(money.amount(), dec!(123.45));
}

#[test]
fn minor_amount() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(123.45)).unwrap();

    assert_eq!(money.minor_amount(), Some(12345));
}

#[test]
fn code_getter() {
    let money: ObjMoney = ObjMoney::try_new("EUR", dec!(1)).unwrap();

    assert_eq!(money.code(), "EUR");
}

#[test]
fn symbol_getter() {
    let money: ObjMoney = ObjMoney::try_new("EUR", dec!(1)).unwrap();

    assert_eq!(money.symbol(), "€");
}

#[test]
fn minor_unit_symbol_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();

    assert_eq!(money.minor_unit_symbol(), "¢");
}

#[test]
fn name_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();

    assert_eq!(money.name(), "United States dollar");
}

#[test]
fn minor_unit_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();

    assert_eq!(money.minor_unit(), 2);
}

#[test]
fn register_currency_test() {
    register_currency("XYZ", "@", "", "Test Currency", 3).unwrap();

    let money: ObjMoney = ObjMoney::try_new("XYZ", dec!(1.234)).unwrap();

    assert_eq!(money.code(), "XYZ");
    assert_eq!(money.symbol(), "@");
    assert_eq!(money.name(), "Test Currency");
    assert_eq!(money.minor_unit(), 3);

    let ret = register_currency("XYZ", "@", "", "Test Currency", 3);
    assert!(ret.is_err());
}

#[test]
fn dyn_money_obj_money() {
    let money = ObjMoney::try_new("USD", dec!(42)).unwrap();

    let dyn_money: &dyn DynMoney = &money;

    assert_eq!(dyn_money.amount(), dec!(42));
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.symbol(), "$");
    assert_eq!(dyn_money.minor_unit(), 2);
}

#[test]
fn dyn_money_money() {
    let money = crate::Money::<crate::iso::USD>::from_decimal(dec!(42));

    let dyn_money: &dyn DynMoney = &money;

    assert_eq!(dyn_money.amount(), dec!(42));
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.symbol(), "$");
    assert_eq!(dyn_money.minor_unit(), 2);
}

#[cfg(feature = "raw_money")]
#[test]
fn dyn_money_raw_money() {
    let money = crate::RawMoney::<crate::iso::USD>::from_decimal(dec!(42));

    let dyn_money: &dyn DynMoney = &money;

    assert_eq!(dyn_money.amount(), dec!(42));
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.symbol(), "$");
    assert_eq!(dyn_money.minor_unit(), 2);
}
