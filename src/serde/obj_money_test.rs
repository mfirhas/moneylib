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

// failed cases

#[test]
fn deserialize_missing_code() {
    let json = r#"{"amount":123.45}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_missing_amount() {
    let json = r#"{"code":"USD"}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_invalid_code() {
    let json = r#"{"code":"INVALID","amount":123.45}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_invalid_amount_type() {
    let json = r#"{"code":"USD","amount":"123.45"}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_invalid_amount_value() {
    let json = r#"{"code":"USD","amount":1e999999}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_str_code_comma_invalid_currency() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(with = "crate::serde::obj_money::str_code_comma")]
        money: ObjMoney,
    }

    let json = r#"{"money":"ABC 1,234.56"}"#;

    assert!(serde_json::from_str::<Wrapper>(json).is_err());
}

#[test]
fn deserialize_str_code_comma_invalid_format() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(with = "crate::serde::obj_money::str_code_comma")]
        money: ObjMoney,
    }

    let json = r#"{"money":"not money"}"#;

    assert!(serde_json::from_str::<Wrapper>(json).is_err());
}

#[test]
fn deserialize_str_code_comma_not_string() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(with = "crate::serde::obj_money::str_code_comma")]
        money: ObjMoney,
    }

    let json = r#"{"money":123}"#;

    assert!(serde_json::from_str::<Wrapper>(json).is_err());
}

#[test]
fn deserialize_str_code_dot_invalid_currency() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(with = "crate::serde::obj_money::str_code_dot")]
        money: ObjMoney,
    }

    let json = r#"{"money":"ABC 1.234,56"}"#;

    assert!(serde_json::from_str::<Wrapper>(json).is_err());
}

#[test]
fn deserialize_str_code_dot_invalid_format() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(with = "crate::serde::obj_money::str_code_dot")]
        money: ObjMoney,
    }

    let json = r#"{"money":"not money"}"#;

    assert!(serde_json::from_str::<Wrapper>(json).is_err());
}

#[test]
fn deserialize_str_code_dot_not_string() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(with = "crate::serde::obj_money::str_code_dot")]
        money: ObjMoney,
    }

    let json = r#"{"money":123}"#;

    assert!(serde_json::from_str::<Wrapper>(json).is_err());
}

#[test]
fn deserialize_invalid_json_number() {
    // Overflowed JSON number; serde_json::Number cannot represent it.
    let json = r#"{"code":"USD","amount":1e999999}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_amount_as_string() {
    // Amount must be a JSON number, not a string.
    let json = r#"{"code":"USD","amount":"123.45"}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_amount_as_bool() {
    let json = r#"{"code":"USD","amount":true}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_amount_as_null() {
    let json = r#"{"code":"USD","amount":null}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_amount_as_array() {
    let json = r#"{"code":"USD","amount":[123.45]}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}

#[test]
fn deserialize_amount_as_object() {
    let json = r#"{"code":"USD","amount":{"value":123.45}}"#;

    assert!(serde_json::from_str::<ObjMoney>(json).is_err());
}
