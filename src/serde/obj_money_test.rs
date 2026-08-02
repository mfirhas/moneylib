use crate::obj_money::ObjMoney;
use crate::*;
use ::serde::{Deserialize, Serialize};
use rust_decimal_macros::dec;

#[test]
fn serialize_obj_money() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1234.56)).unwrap();

    let json = serde_json::to_string(&money).unwrap();

    assert_eq!(json, r#"{"code":"USD","amount":1234.56}"#);
}

#[test]
fn deserialize_obj_money() {
    let json = r#"{"code":"USD","amount":1234.56}"#;

    let money: ObjMoney = serde_json::from_str(json).unwrap();

    assert_eq!(money.code(), "USD");
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn roundtrip_obj_money() {
    let money: ObjMoney = ObjMoney::try_new("EUR", dec!(9876543.21)).unwrap();

    let json = serde_json::to_string(&money).unwrap();
    let deserialized: ObjMoney = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.code(), money.code());
    assert_eq!(deserialized.amount(), money.amount());
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CodeCommaWrapper {
    #[serde(with = "crate::serde::obj_money::str_code_comma")]
    money: ObjMoney,
}

#[test]
fn serialize_str_code_comma() {
    let wrapper = CodeCommaWrapper {
        money: ObjMoney::try_new("USD", dec!(1234567.89)).unwrap(),
    };

    let json = serde_json::to_string(&wrapper).unwrap();

    assert_eq!(json, r#"{"money":"USD 1,234,567.89"}"#);
}

#[test]
fn deserialize_str_code_comma() {
    let json = r#"{"money":"USD 1,234,567.89"}"#;

    let wrapper: CodeCommaWrapper = serde_json::from_str(json).unwrap();

    assert_eq!(wrapper.money.code(), "USD");
    assert_eq!(wrapper.money.amount(), dec!(1234567.89));
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CodeDotWrapper {
    #[serde(with = "crate::serde::obj_money::str_code_dot")]
    money: ObjMoney,
}

#[test]
fn serialize_str_code_dot() {
    let wrapper = CodeDotWrapper {
        money: ObjMoney::try_new("EUR", dec!(1234567.89)).unwrap(),
    };

    let json = serde_json::to_string(&wrapper).unwrap();

    assert_eq!(json, r#"{"money":"EUR 1.234.567,89"}"#);
}

#[test]
fn deserialize_str_code_dot() {
    let json = r#"{"money":"EUR 1.234.567,89"}"#;

    let wrapper: CodeDotWrapper = serde_json::from_str(json).unwrap();

    assert_eq!(wrapper.money.code(), "EUR");
    assert_eq!(wrapper.money.amount(), dec!(1234567.89));
}

#[test]
fn deserialize_invalid_currency() {
    let json = r#"{"code":"XXX","amount":1}"#;

    let obj = serde_json::from_str::<ObjMoney>(json).unwrap();

    assert_eq!(obj.amount(), dec!(1));
    assert_eq!(obj.code(), crate::iso::XXX::CODE);
}

#[test]
fn deserialize_invalid_amount() {
    let json = r#"{"code":"USD","amount":"abc"}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}
