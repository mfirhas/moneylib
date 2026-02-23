use crate::{BaseMoney, RawMoney, money_macros::dec};
use crate::{EUR, USD};

// ---------------------------------------------------------------------------
// Default (number) serialize/deserialize
// ---------------------------------------------------------------------------

#[test]
fn test_default_serialize_as_number() {
    let raw = RawMoney::<USD>::from_decimal(dec!(1234.56789));
    let json = serde_json::to_string(&raw).unwrap();
    assert_eq!(json, "1234.56789");
}

#[test]
fn test_default_serialize_negative() {
    let raw = RawMoney::<USD>::from_decimal(dec!(-1234.56789));
    let json = serde_json::to_string(&raw).unwrap();
    assert_eq!(json, "-1234.56789");
}

#[test]
fn test_default_deserialize_from_float() {
    let raw: RawMoney<USD> = serde_json::from_str("1234.56").unwrap();
    assert_eq!(raw.code(), "USD");
}

#[test]
fn test_default_deserialize_from_integer() {
    let raw: RawMoney<USD> = serde_json::from_str("1234").unwrap();
    assert_eq!(raw.amount(), dec!(1234));
}

#[test]
fn test_default_option_none() {
    let raw: Option<RawMoney<USD>> = None;
    let json = serde_json::to_string(&raw).unwrap();
    assert_eq!(json, "null");
}

#[test]
fn test_default_option_some() {
    let raw: Option<RawMoney<USD>> = Some(RawMoney::<USD>::from_decimal(dec!(100.567)));
    let json = serde_json::to_string(&raw).unwrap();
    assert_eq!(json, "100.567");
}

#[test]
fn test_default_option_deserialize_none() {
    let raw: Option<RawMoney<USD>> = serde_json::from_str("null").unwrap();
    assert!(raw.is_none());
}

#[test]
fn test_default_option_deserialize_some() {
    let raw: Option<RawMoney<USD>> = serde_json::from_str("100.567").unwrap();
    assert!(raw.is_some());
}

// ---------------------------------------------------------------------------
// comma_str serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentComma {
    #[serde(with = "crate::serde::raw_money::comma_str")]
    amount: RawMoney<USD>,
}

#[test]
fn test_comma_str_serialize() {
    let p = PaymentComma {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56789"}"#);
}

#[test]
fn test_comma_str_serialize_negative() {
    let p = PaymentComma {
        amount: RawMoney::<USD>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD -1,234.56789"}"#);
}

#[test]
fn test_comma_str_deserialize() {
    let p: PaymentComma =
        serde_json::from_str(r#"{"amount":"USD 1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_comma_str_roundtrip() {
    let original = PaymentComma {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentComma = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_comma_str serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptComma {
    #[serde(with = "crate::serde::raw_money::option_comma_str")]
    amount: Option<RawMoney<USD>>,
}

#[test]
fn test_option_comma_str_serialize_some() {
    let p = PaymentOptComma {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56789"}"#);
}

#[test]
fn test_option_comma_str_serialize_none() {
    let p = PaymentOptComma { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_comma_str_deserialize_some() {
    let p: PaymentOptComma =
        serde_json::from_str(r#"{"amount":"USD 1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_comma_str_deserialize_none() {
    let p: PaymentOptComma = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

// ---------------------------------------------------------------------------
// dot_str serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentDot {
    #[serde(with = "crate::serde::raw_money::dot_str")]
    amount: RawMoney<EUR>,
}

#[test]
fn test_dot_str_serialize() {
    let p = PaymentDot {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56789"}"#);
}

#[test]
fn test_dot_str_serialize_negative() {
    let p = PaymentDot {
        amount: RawMoney::<EUR>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR -1.234,56789"}"#);
}

#[test]
fn test_dot_str_deserialize() {
    let p: PaymentDot = serde_json::from_str(r#"{"amount":"EUR 1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "EUR");
}

#[test]
fn test_dot_str_roundtrip() {
    let original = PaymentDot {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentDot = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_dot_str serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptDot {
    #[serde(with = "crate::serde::raw_money::option_dot_str")]
    amount: Option<RawMoney<EUR>>,
}

#[test]
fn test_option_dot_str_serialize_some() {
    let p = PaymentOptDot {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56789"}"#);
}

#[test]
fn test_option_dot_str_serialize_none() {
    let p = PaymentOptDot { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_dot_str_deserialize_some() {
    let p: PaymentOptDot =
        serde_json::from_str(r#"{"amount":"EUR 1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_dot_str_deserialize_none() {
    let p: PaymentOptDot = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}
