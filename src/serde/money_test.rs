use crate::{BaseMoney, Money, money_macros::dec};
use crate::{EUR, JPY, USD};

// ---------------------------------------------------------------------------
// Default (number) serialize/deserialize
// ---------------------------------------------------------------------------

#[test]
fn test_default_serialize_as_number() {
    let money = Money::<USD>::from_decimal(dec!(1234.56));
    let json = serde_json::to_string(&money).unwrap();
    assert_eq!(json, "1234.56");
}

#[test]
fn test_default_serialize_integer() {
    let money = Money::<USD>::from_decimal(dec!(1234));
    let json = serde_json::to_string(&money).unwrap();
    assert_eq!(json, "1234.0");
}

#[test]
fn test_default_serialize_negative() {
    let money = Money::<USD>::from_decimal(dec!(-1234.56));
    let json = serde_json::to_string(&money).unwrap();
    assert_eq!(json, "-1234.56");
}

#[test]
fn test_default_deserialize_from_float() {
    let money: Money<USD> = serde_json::from_str("1234.56").unwrap();
    assert_eq!(money.amount(), dec!(1234.56));
    assert_eq!(money.code(), "USD");
}

#[test]
fn test_default_deserialize_from_integer() {
    let money: Money<USD> = serde_json::from_str("1234").unwrap();
    assert_eq!(money.amount(), dec!(1234));
}

#[test]
fn test_default_deserialize_negative() {
    let money: Money<USD> = serde_json::from_str("-1234.56").unwrap();
    assert_eq!(money.amount(), dec!(-1234.56));
}

#[test]
fn test_default_roundtrip() {
    let original = Money::<USD>::from_decimal(dec!(100.50));
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Money<USD> = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_default_serialize_jpy() {
    let money = Money::<JPY>::from_decimal(dec!(1234));
    let json = serde_json::to_string(&money).unwrap();
    assert_eq!(json, "1234.0");
}

#[test]
fn test_default_option_none() {
    let money: Option<Money<USD>> = None;
    let json = serde_json::to_string(&money).unwrap();
    assert_eq!(json, "null");
}

#[test]
fn test_default_option_some() {
    let money: Option<Money<USD>> = Some(Money::<USD>::from_decimal(dec!(100.50)));
    let json = serde_json::to_string(&money).unwrap();
    assert_eq!(json, "100.5");
}

#[test]
fn test_default_option_deserialize_none() {
    let money: Option<Money<USD>> = serde_json::from_str("null").unwrap();
    assert!(money.is_none());
}

#[test]
fn test_default_option_deserialize_some() {
    let money: Option<Money<USD>> = serde_json::from_str("100.50").unwrap();
    assert_eq!(money.unwrap().amount(), dec!(100.50));
}

// ---------------------------------------------------------------------------
// comma_str serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentComma {
    #[serde(with = "crate::serde::money::comma_str")]
    amount: Money<USD>,
}

#[test]
fn test_comma_str_serialize() {
    let p = PaymentComma {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56"}"#);
}

#[test]
fn test_comma_str_serialize_no_thousands() {
    let p = PaymentComma {
        amount: Money::<USD>::from_decimal(dec!(100.50)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 100.50"}"#);
}

#[test]
fn test_comma_str_serialize_negative() {
    let p = PaymentComma {
        amount: Money::<USD>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD -1,234.56"}"#);
}

#[test]
fn test_comma_str_deserialize() {
    let p: PaymentComma = serde_json::from_str(r#"{"amount":"USD 1,234.56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_comma_str_roundtrip() {
    let original = PaymentComma {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
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
    #[serde(with = "crate::serde::money::option_comma_str")]
    amount: Option<Money<USD>>,
}

#[test]
fn test_option_comma_str_serialize_some() {
    let p = PaymentOptComma {
        amount: Some(Money::<USD>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56"}"#);
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
        serde_json::from_str(r#"{"amount":"USD 1,234.56"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
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
    #[serde(with = "crate::serde::money::dot_str")]
    amount: Money<EUR>,
}

#[test]
fn test_dot_str_serialize() {
    let p = PaymentDot {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56"}"#);
}

#[test]
fn test_dot_str_serialize_no_thousands() {
    let p = PaymentDot {
        amount: Money::<EUR>::from_decimal(dec!(100.50)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 100,50"}"#);
}

#[test]
fn test_dot_str_serialize_negative() {
    let p = PaymentDot {
        amount: Money::<EUR>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR -1.234,56"}"#);
}

#[test]
fn test_dot_str_deserialize() {
    let p: PaymentDot = serde_json::from_str(r#"{"amount":"EUR 1.234,56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "EUR");
}

#[test]
fn test_dot_str_roundtrip() {
    let original = PaymentDot {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
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
    #[serde(with = "crate::serde::money::option_dot_str")]
    amount: Option<Money<EUR>>,
}

#[test]
fn test_option_dot_str_serialize_some() {
    let p = PaymentOptDot {
        amount: Some(Money::<EUR>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56"}"#);
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
        serde_json::from_str(r#"{"amount":"EUR 1.234,56"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
}

#[test]
fn test_option_dot_str_deserialize_none() {
    let p: PaymentOptDot = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}
