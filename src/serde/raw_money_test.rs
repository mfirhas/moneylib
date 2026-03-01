use currencylib::{CAD, IDR};

use crate::{BaseMoney, RawMoney, money_macros::dec};
use crate::{EUR, GBP, JPY, USD};

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

#[test]
fn test_default_deserialize_overflow() {
    let money = serde_json::from_str::<RawMoney<USD>>(u128::MAX.to_string().as_str());
    assert!(money.is_err());
}

// ---------------------------------------------------------------------------
// comma_str_code serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentCommaCode {
    #[serde(with = "crate::serde::raw_money::comma_str_code")]
    amount: RawMoney<USD>,
}

#[test]
fn test_comma_str_code_serialize() {
    let p = PaymentCommaCode {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56789"}"#);
}

#[test]
fn test_comma_str_code_serialize_negative() {
    let p = PaymentCommaCode {
        amount: RawMoney::<USD>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD -1,234.56789"}"#);
}

#[test]
fn test_comma_str_code_deserialize() {
    let p: PaymentCommaCode = serde_json::from_str(r#"{"amount":"USD 1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_comma_str_code_roundtrip() {
    let original = PaymentCommaCode {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentCommaCode = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_comma_str_code serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptCommaCode {
    #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
    amount: Option<RawMoney<USD>>,
}

#[test]
fn test_option_comma_str_code_serialize_some() {
    let p = PaymentOptCommaCode {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56789"}"#);
}

#[test]
fn test_option_comma_str_code_serialize_none() {
    let p = PaymentOptCommaCode { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_comma_str_code_deserialize_some() {
    let p: PaymentOptCommaCode = serde_json::from_str(r#"{"amount":"USD 1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_comma_str_code_deserialize_none() {
    let p: PaymentOptCommaCode = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

// ---------------------------------------------------------------------------
// comma_str_symbol serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentCommaSymbol {
    #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
    amount: RawMoney<USD>,
}

#[test]
fn test_comma_str_symbol_serialize() {
    let p = PaymentCommaSymbol {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"$1,234.56789"}"#);
}

#[test]
fn test_comma_str_symbol_serialize_negative() {
    let p = PaymentCommaSymbol {
        amount: RawMoney::<USD>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"-$1,234.56789"}"#);
}

#[test]
fn test_comma_str_symbol_deserialize() {
    let p: PaymentCommaSymbol = serde_json::from_str(r#"{"amount":"$1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_comma_str_symbol_deserialize_negative() {
    let p: PaymentCommaSymbol = serde_json::from_str(r#"{"amount":"-$1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56789));
}

#[test]
fn test_comma_str_symbol_roundtrip() {
    let original = PaymentCommaSymbol {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentCommaSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_comma_str_symbol serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptCommaSymbol {
    #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
    amount: Option<RawMoney<USD>>,
}

#[test]
fn test_option_comma_str_symbol_serialize_some() {
    let p = PaymentOptCommaSymbol {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"$1,234.56789"}"#);
}

#[test]
fn test_option_comma_str_symbol_serialize_none() {
    let p = PaymentOptCommaSymbol { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_comma_str_symbol_deserialize_some() {
    let p: PaymentOptCommaSymbol = serde_json::from_str(r#"{"amount":"$1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_comma_str_symbol_deserialize_none() {
    let p: PaymentOptCommaSymbol = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

// ---------------------------------------------------------------------------
// dot_str_code serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentDotCode {
    #[serde(with = "crate::serde::raw_money::dot_str_code")]
    amount: RawMoney<EUR>,
}

#[test]
fn test_dot_str_code_serialize() {
    let p = PaymentDotCode {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56789"}"#);
}

#[test]
fn test_dot_str_code_serialize_negative() {
    let p = PaymentDotCode {
        amount: RawMoney::<EUR>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR -1.234,56789"}"#);
}

#[test]
fn test_dot_str_code_deserialize() {
    let p: PaymentDotCode = serde_json::from_str(r#"{"amount":"EUR 1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "EUR");
}

#[test]
fn test_dot_str_code_roundtrip() {
    let original = PaymentDotCode {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentDotCode = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_dot_str_code serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptDotCode {
    #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
    amount: Option<RawMoney<EUR>>,
}

#[test]
fn test_option_dot_str_code_serialize_some() {
    let p = PaymentOptDotCode {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56789"}"#);
}

#[test]
fn test_option_dot_str_code_serialize_none() {
    let p = PaymentOptDotCode { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_dot_str_code_deserialize_some() {
    let p: PaymentOptDotCode = serde_json::from_str(r#"{"amount":"EUR 1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_dot_str_code_deserialize_none() {
    let p: PaymentOptDotCode = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

// ---------------------------------------------------------------------------
// dot_str_symbol serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentDotSymbol {
    #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
    amount: RawMoney<EUR>,
}

#[test]
fn test_dot_str_symbol_serialize() {
    let p = PaymentDotSymbol {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"€1.234,56789"}"#);
}

#[test]
fn test_dot_str_symbol_serialize_negative() {
    let p = PaymentDotSymbol {
        amount: RawMoney::<EUR>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"-€1.234,56789"}"#);
}

#[test]
fn test_dot_str_symbol_deserialize() {
    let p: PaymentDotSymbol = serde_json::from_str(r#"{"amount":"€1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "EUR");
}

#[test]
fn test_dot_str_symbol_deserialize_negative() {
    let p: PaymentDotSymbol = serde_json::from_str(r#"{"amount":"-€1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56789));
}

#[test]
fn test_dot_str_symbol_roundtrip() {
    let original = PaymentDotSymbol {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentDotSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_dot_str_symbol serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptDotSymbol {
    #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
    amount: Option<RawMoney<EUR>>,
}

#[test]
fn test_option_dot_str_symbol_serialize_some() {
    let p = PaymentOptDotSymbol {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"€1.234,56789"}"#);
}

#[test]
fn test_option_dot_str_symbol_serialize_none() {
    let p = PaymentOptDotSymbol { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_dot_str_symbol_deserialize_some() {
    let p: PaymentOptDotSymbol = serde_json::from_str(r#"{"amount":"€1.234,56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_dot_str_symbol_deserialize_none() {
    let p: PaymentOptDotSymbol = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

// ---------------------------------------------------------------------------
// YAML serialize/deserialize (string modes)
// ---------------------------------------------------------------------------

#[test]
fn test_yaml_comma_str_code_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: USD 1,234.56789\n");
}

#[test]
fn test_yaml_comma_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let w: W = serde_yaml::from_str("amount: \"USD 1,234.56789\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
    assert_eq!(w.amount.code(), "USD");
}

#[test]
fn test_yaml_comma_str_code_roundtrip() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let original = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let yaml = serde_yaml::to_string(&original).unwrap();
    let result: W = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(original.amount, result.amount);
}

#[test]
fn test_yaml_comma_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: $1,234.56789\n");
}

#[test]
fn test_yaml_comma_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        amount: RawMoney<USD>,
    }
    let w: W = serde_yaml::from_str("amount: \"$1,234.56789\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
}

#[test]
fn test_yaml_dot_str_code_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: EUR 1.234,56789\n");
}

#[test]
fn test_yaml_dot_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        amount: RawMoney<EUR>,
    }
    let w: W = serde_yaml::from_str("amount: \"EUR 1.234,56789\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
    assert_eq!(w.amount.code(), "EUR");
}

#[test]
fn test_yaml_dot_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: €1.234,56789\n");
}

#[test]
fn test_yaml_dot_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount: RawMoney<EUR>,
    }
    let w: W = serde_yaml::from_str("amount: \"€1.234,56789\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
}

#[test]
fn test_yaml_option_comma_str_code_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    let w = W {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: USD 1,234.56789\n");
}

#[test]
fn test_yaml_option_comma_str_code_serialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    let w = W { amount: None };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: null\n");
}

#[test]
fn test_yaml_option_comma_str_code_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    let w: W = serde_yaml::from_str("amount: \"USD 1,234.56789\"").unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_yaml_option_comma_str_code_deserialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    let w: W = serde_yaml::from_str("amount: null").unwrap();
    assert!(w.amount.is_none());
}

// ---------------------------------------------------------------------------
// TOML serialize/deserialize (string modes)
// ---------------------------------------------------------------------------

#[test]
fn test_toml_comma_str_code_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"USD 1,234.56789\"\n");
}

#[test]
fn test_toml_comma_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let w: W = toml::from_str(r#"amount = "USD 1,234.56789""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
    assert_eq!(w.amount.code(), "USD");
}

#[test]
fn test_toml_comma_str_code_roundtrip() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let original = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let t = toml::to_string(&original).unwrap();
    let result: W = toml::from_str(&t).unwrap();
    assert_eq!(original.amount, result.amount);
}

#[test]
fn test_toml_comma_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"$1,234.56789\"\n");
}

#[test]
fn test_toml_comma_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        amount: RawMoney<USD>,
    }
    let w: W = toml::from_str(r#"amount = "$1,234.56789""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
}

#[test]
fn test_toml_dot_str_code_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"EUR 1.234,56789\"\n");
}

#[test]
fn test_toml_dot_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        amount: RawMoney<EUR>,
    }
    let w: W = toml::from_str(r#"amount = "EUR 1.234,56789""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
    assert_eq!(w.amount.code(), "EUR");
}

#[test]
fn test_toml_dot_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"€1.234,56789\"\n");
}

#[test]
fn test_toml_dot_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount: RawMoney<EUR>,
    }
    let w: W = toml::from_str(r#"amount = "€1.234,56789""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56789));
}

#[test]
fn test_toml_option_comma_str_code_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    let w = W {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"USD 1,234.56789\"\n");
}

#[test]
fn test_toml_option_comma_str_code_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    let w: W = toml::from_str(r#"amount = "USD 1,234.56789""#).unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

// ---------------------------------------------------------------------------
// Edge cases: zero and large amounts (default format)
// ---------------------------------------------------------------------------

#[test]
fn test_default_serialize_zero() {
    let raw = RawMoney::<USD>::from_decimal(dec!(0));
    let json = serde_json::to_string(&raw).unwrap();
    assert_eq!(json, "0");
}

#[test]
fn test_default_deserialize_zero() {
    let raw: RawMoney<USD> = serde_json::from_str("0").unwrap();
    assert_eq!(raw.amount(), dec!(0));
}

#[test]
fn test_default_serialize_large() {
    let raw = RawMoney::<USD>::from_decimal(dec!(1000000.123456));
    let json = serde_json::to_string(&raw).unwrap();
    assert_eq!(json, "1000000.123456");
}

#[test]
fn test_default_deserialize_large() {
    let raw: RawMoney<USD> = serde_json::from_str("1000000.123456").unwrap();
    assert_eq!(raw.amount(), dec!(1000000.123456));
}

#[test]
fn test_default_roundtrip() {
    let original = RawMoney::<USD>::from_decimal(dec!(100.56789));
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: RawMoney<USD> = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

// ---------------------------------------------------------------------------
// dot_str_symbol: negative roundtrip (has its own sign handling)
// ---------------------------------------------------------------------------

#[test]
fn test_dot_str_symbol_roundtrip_negative() {
    let original = PaymentDotSymbol {
        amount: RawMoney::<EUR>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentDotSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// Error/invalid input deserialization
// ---------------------------------------------------------------------------

#[test]
fn test_comma_str_code_deserialize_wrong_currency() {
    let result: Result<PaymentCommaCode, _> =
        serde_json::from_str(r#"{"amount":"EUR 1,234.56789"}"#);
    assert!(result.is_err());
}

#[test]
fn test_comma_str_code_deserialize_malformed() {
    let result: Result<PaymentCommaCode, _> = serde_json::from_str(r#"{"amount":"not_valid"}"#);
    assert!(result.is_err());
}

#[test]
fn test_comma_str_symbol_deserialize_wrong_symbol() {
    let result: Result<PaymentCommaSymbol, _> =
        serde_json::from_str(r#"{"amount":"€1,234.56789"}"#);
    assert!(result.is_err());
}

#[test]
fn test_dot_str_code_deserialize_wrong_currency() {
    let result: Result<PaymentDotCode, _> = serde_json::from_str(r#"{"amount":"USD 1.234,56789"}"#);
    assert!(result.is_err());
}

#[test]
fn test_dot_str_symbol_deserialize_wrong_symbol() {
    let result: Result<PaymentDotSymbol, _> = serde_json::from_str(r#"{"amount":"$1.234,56789"}"#);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Zero amounts in string formats
// ---------------------------------------------------------------------------

#[test]
fn test_comma_str_code_serialize_zero() {
    let p = PaymentCommaCode {
        amount: RawMoney::<USD>::from_decimal(dec!(0)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 0.00"}"#);
}

#[test]
fn test_dot_str_code_serialize_zero() {
    let p = PaymentDotCode {
        amount: RawMoney::<EUR>::from_decimal(dec!(0)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 0,00"}"#);
}

// ---------------------------------------------------------------------------
// JPY string format (no decimal places)
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentJpyCommaCode {
    #[serde(with = "crate::serde::raw_money::comma_str_code")]
    amount: RawMoney<JPY>,
}

#[test]
fn test_comma_str_code_serialize_jpy() {
    let p = PaymentJpyCommaCode {
        amount: RawMoney::<JPY>::from_decimal(dec!(1234)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"JPY 1,234"}"#);
}

#[test]
fn test_comma_str_code_deserialize_jpy() {
    let p: PaymentJpyCommaCode = serde_json::from_str(r#"{"amount":"JPY 1,234"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234));
    assert_eq!(p.amount.code(), "JPY");
}

// ---------------------------------------------------------------------------
// GBP symbol tests
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentGbpCommaSymbol {
    #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
    amount: RawMoney<GBP>,
}

#[test]
fn test_comma_str_symbol_serialize_gbp() {
    let p = PaymentGbpCommaSymbol {
        amount: RawMoney::<GBP>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"£1,234.56789"}"#);
}

#[test]
fn test_comma_str_symbol_deserialize_gbp() {
    let p: PaymentGbpCommaSymbol = serde_json::from_str(r#"{"amount":"£1,234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "GBP");
}

// ---------------------------------------------------------------------------
// Option roundtrip tests
// ---------------------------------------------------------------------------

#[test]
fn test_option_comma_str_code_roundtrip() {
    let original = PaymentOptCommaCode {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptCommaCode = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_option_comma_str_symbol_roundtrip() {
    let original = PaymentOptCommaSymbol {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptCommaSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_option_dot_str_code_roundtrip() {
    let original = PaymentOptDotCode {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptDotCode = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_option_dot_str_symbol_roundtrip() {
    let original = PaymentOptDotSymbol {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptDotSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// YAML: option_comma_str_symbol
// ---------------------------------------------------------------------------

#[test]
fn test_yaml_option_comma_str_symbol_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount: Option<RawMoney<USD>>,
    }
    let w = W {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: $1,234.56789\n");
}

#[test]
fn test_yaml_option_comma_str_symbol_serialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount: Option<RawMoney<USD>>,
    }
    let w = W { amount: None };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: null\n");
}

#[test]
fn test_yaml_option_comma_str_symbol_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount: Option<RawMoney<USD>>,
    }
    let w: W = serde_yaml::from_str("amount: \"$1,234.56789\"").unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_yaml_option_comma_str_symbol_deserialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount: Option<RawMoney<USD>>,
    }
    let w: W = serde_yaml::from_str("amount: null").unwrap();
    assert!(w.amount.is_none());
}

// ---------------------------------------------------------------------------
// YAML: option_dot_str_code
// ---------------------------------------------------------------------------

#[test]
fn test_yaml_option_dot_str_code_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount: Option<RawMoney<EUR>>,
    }
    let w = W {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: EUR 1.234,56789\n");
}

#[test]
fn test_yaml_option_dot_str_code_serialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount: Option<RawMoney<EUR>>,
    }
    let w = W { amount: None };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: null\n");
}

#[test]
fn test_yaml_option_dot_str_code_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount: Option<RawMoney<EUR>>,
    }
    let w: W = serde_yaml::from_str("amount: \"EUR 1.234,56789\"").unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_yaml_option_dot_str_code_deserialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount: Option<RawMoney<EUR>>,
    }
    let w: W = serde_yaml::from_str("amount: null").unwrap();
    assert!(w.amount.is_none());
}

// ---------------------------------------------------------------------------
// YAML: option_dot_str_symbol
// ---------------------------------------------------------------------------

#[test]
fn test_yaml_option_dot_str_symbol_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount: Option<RawMoney<EUR>>,
    }
    let w = W {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: €1.234,56789\n");
}

#[test]
fn test_yaml_option_dot_str_symbol_serialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount: Option<RawMoney<EUR>>,
    }
    let w = W { amount: None };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: null\n");
}

#[test]
fn test_yaml_option_dot_str_symbol_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount: Option<RawMoney<EUR>>,
    }
    let w: W = serde_yaml::from_str("amount: \"€1.234,56789\"").unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_yaml_option_dot_str_symbol_deserialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount: Option<RawMoney<EUR>>,
    }
    let w: W = serde_yaml::from_str("amount: null").unwrap();
    assert!(w.amount.is_none());
}

// ---------------------------------------------------------------------------
// TOML: option_comma_str_code none roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_toml_option_comma_str_code_serialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount: Option<RawMoney<USD>>,
    }
    // TOML doesn't support top-level null; verify via JSON roundtrip
    let original = W { amount: None };
    let json = serde_json::to_string(&original).unwrap();
    let result: W = serde_json::from_str(&json).unwrap();
    assert!(result.amount.is_none());
}

// ---------------------------------------------------------------------------
// TOML: option_comma_str_symbol
// ---------------------------------------------------------------------------

#[test]
fn test_toml_option_comma_str_symbol_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount: Option<RawMoney<USD>>,
    }
    let w = W {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56789))),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"$1,234.56789\"\n");
}

#[test]
fn test_toml_option_comma_str_symbol_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount: Option<RawMoney<USD>>,
    }
    let w: W = toml::from_str(r#"amount = "$1,234.56789""#).unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

// ---------------------------------------------------------------------------
// TOML: option_dot_str_code
// ---------------------------------------------------------------------------

#[test]
fn test_toml_option_dot_str_code_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount: Option<RawMoney<EUR>>,
    }
    let w = W {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"EUR 1.234,56789\"\n");
}

#[test]
fn test_toml_option_dot_str_code_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount: Option<RawMoney<EUR>>,
    }
    let w: W = toml::from_str(r#"amount = "EUR 1.234,56789""#).unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

// ---------------------------------------------------------------------------
// TOML: option_dot_str_symbol
// ---------------------------------------------------------------------------

#[test]
fn test_toml_option_dot_str_symbol_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount: Option<RawMoney<EUR>>,
    }
    let w = W {
        amount: Some(RawMoney::<EUR>::from_decimal(dec!(1234.56789))),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"€1.234,56789\"\n");
}

#[test]
fn test_toml_option_dot_str_symbol_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount: Option<RawMoney<EUR>>,
    }
    let w: W = toml::from_str(r#"amount = "€1.234,56789""#).unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56789));
}

// ---------------------------------------------------------------------------
// RawMoneyVisitor: expecting and visit_f64
// ---------------------------------------------------------------------------

#[test]
fn test_default_deserialize_visit_number_types() {
    // f64
    let money: RawMoney<USD> = serde_yaml::from_str("100.25").unwrap();
    assert_eq!(
        money.amount(),
        RawMoney::<USD>::new(100.25_f64).unwrap().amount()
    );

    // f64 rounded
    let money: RawMoney<USD> = serde_yaml::from_str("100.25899").unwrap();
    assert_eq!(
        money.amount(),
        RawMoney::<USD>::new(100.25899_f64).unwrap().amount()
    );

    // i64
    let money: RawMoney<USD> = serde_yaml::from_str("-123234").unwrap();
    assert_eq!(
        money.amount(),
        RawMoney::<USD>::new(-123234_i64).unwrap().amount()
    );

    // i128
    let money: RawMoney<USD> = serde_yaml::from_str("-9223372036854775809").unwrap();
    assert_eq!(
        money.amount(),
        RawMoney::<USD>::new(-9223372036854775809_i128)
            .unwrap()
            .amount()
    );

    // u128
    let money: RawMoney<USD> = serde_yaml::from_str("92233720368547758100").unwrap();
    assert_eq!(
        money.amount(),
        RawMoney::<USD>::new(92233720368547758100_i128)
            .unwrap()
            .amount()
    );

    // from str
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct A {
        amount: RawMoney<USD>,
    }
    let money = serde_yaml::from_str::<A>(r#"{"amount":"123"}"#).unwrap();
    assert_eq!(money.amount, RawMoney::<USD>::from_decimal(dec!(123)));
}

#[test]
fn test_default_deserialize_visit_f64_negative() {
    let money: RawMoney<USD> = serde_yaml::from_str("-50.5").unwrap();
    assert_eq!(
        money.amount(),
        RawMoney::<USD>::new(-50.5_f64).unwrap().amount()
    );
}

#[test]
fn test_deserialize_expecting_message() {
    let err = serde_json::from_str::<RawMoney<USD>>("true").unwrap_err();
    assert!(
        err.to_string().contains("a number"),
        "error message should contain 'a number', got: {}",
        err
    );

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct A {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let w = serde_json::from_str::<A>(r#"{"amount":123}"#);
    assert!(w.is_err());
    println!("A: {:?}", w.err());

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct B {
        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        amount: RawMoney<EUR>,
    }
    let w = serde_json::from_str::<B>(r#"{"amount":234}"#);
    assert!(w.is_err());
    println!("B: {:?}", w.err());

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct C {
        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        amount: RawMoney<USD>,
    }
    let w = serde_json::from_str::<C>(r#"{"amount":234}"#);
    assert!(w.is_err());
    println!("C: {:?}", w.err());

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct D {
        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount: RawMoney<USD>,
    }
    let w = serde_json::from_str::<D>(r#"{"amount":234}"#);
    assert!(w.is_err());
    println!("D: {:?}", w.err());
}

#[test]
fn test_all() {
    #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
    struct All {
        amount_from_f64: RawMoney<USD>,

        // `default` must be declared if you want to let users omit this field giving it money with zero amount.
        #[serde(default)]
        amount_from_f64_omit: RawMoney<IDR>,

        // `default` must be declared if you want to let users omit this field giving it money with zero amount.
        #[serde(default)]
        amount_from_str_omit: RawMoney<CAD>,

        amount_from_i64: RawMoney<EUR>,

        amount_from_u64: RawMoney<USD>,

        amount_from_i128: RawMoney<USD>,

        amount_from_u128: RawMoney<USD>,

        amount_from_str: RawMoney<USD>,

        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount_from_str_comma_code: RawMoney<USD>,

        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount_from_str_comma_code_some: Option<RawMoney<USD>>,

        #[serde(with = "crate::serde::raw_money::option_comma_str_code")]
        amount_from_str_comma_code_none: Option<RawMoney<USD>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "crate::serde::raw_money::option_comma_str_code", default)]
        amount_from_str_comma_code_omit: Option<RawMoney<USD>>,

        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        amount_from_str_comma_symbol: RawMoney<USD>,

        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount_from_str_comma_symbol_some: Option<RawMoney<USD>>,

        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol")]
        amount_from_str_comma_symbol_none: Option<RawMoney<USD>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "crate::serde::raw_money::option_comma_str_symbol", default)]
        amount_from_str_comma_symbol_omit: Option<RawMoney<USD>>,

        // dot
        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        amount_from_str_dot_code: RawMoney<EUR>,

        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount_from_str_dot_code_some: Option<RawMoney<EUR>>,

        #[serde(with = "crate::serde::raw_money::option_dot_str_code")]
        amount_from_str_dot_code_none: Option<RawMoney<EUR>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "crate::serde::raw_money::option_dot_str_code", default)]
        amount_from_str_dot_code_omit: Option<RawMoney<EUR>>,

        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount_from_str_dot_symbol: RawMoney<EUR>,

        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount_from_str_dot_symbol_some: Option<RawMoney<EUR>>,

        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol")]
        amount_from_str_dot_symbol_none: Option<RawMoney<EUR>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "crate::serde::raw_money::option_dot_str_symbol", default)]
        amount_from_str_dot_symbol_omit: Option<RawMoney<EUR>>,

        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        neg_amount_from_code_comma: RawMoney<USD>,

        #[serde(with = "crate::serde::raw_money::dot_str_code")]
        neg_amount_from_code_dot: RawMoney<EUR>,

        #[serde(with = "crate::serde::raw_money::comma_str_symbol")]
        neg_amount_from_symbol_comma: RawMoney<USD>,

        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        neg_amount_from_symbol_dot: RawMoney<EUR>,
    }

    let json_str = r#"
        {
          "amount_from_f64": 1234.56988,
          "amount_from_i64": -1234,
          "amount_from_u64": 18446744073709551615,
          "amount_from_i128": -1844674407370955161588,
          "amount_from_u128": 34028236692093846346337,
          "amount_from_str": "1234.56",
          "amount_from_str_comma_code": "USD 1,234.56",
          "amount_from_str_comma_code_some": "USD 2,000.00",
          "amount_from_str_comma_code_none": null,
          "amount_from_str_comma_symbol": "$1,234.56",
          "amount_from_str_comma_symbol_some": "$2,345.6799",
          "amount_from_str_comma_symbol_none": null,
          "amount_from_str_dot_code": "EUR 1.234,5634",
          "amount_from_str_dot_code_some": "EUR 2.000,00",
          "amount_from_str_dot_code_none": null,
          "amount_from_str_dot_symbol": "€1.234,56",
          "amount_from_str_dot_symbol_some": "€2.345,67",
          "amount_from_str_dot_symbol_none": null,

          "neg_amount_from_code_comma": "USD -34345.5566",
          "neg_amount_from_code_dot": "EUR -23.000,00",
          "neg_amount_from_symbol_comma": "-$1.123",
          "neg_amount_from_symbol_dot": "-€1,123"
        }
    "#;
    let all = serde_json::from_str::<All>(json_str);
    assert!(all.is_ok());

    let ret = all.unwrap();

    // no rounding: expect exact decimal values parsed from inputs

    // f64 keeps its fractional digits as provided
    assert_eq!(ret.amount_from_f64.amount(), dec!(1234.56988));
    assert_eq!(ret.amount_from_f64_omit.amount(), dec!(0));
    assert_eq!(ret.amount_from_str_omit.amount(), dec!(0));

    assert_eq!(ret.amount_from_i64.amount(), dec!(-1234));
    assert_eq!(ret.amount_from_u64.amount(), dec!(18446744073709551615));

    assert_eq!(ret.amount_from_i128.amount(), dec!(-1844674407370955161588));
    assert_eq!(ret.amount_from_u128.amount(), dec!(34028236692093846346337));

    assert_eq!(ret.amount_from_str.amount(), dec!(1234.56));

    // comma + code
    assert_eq!(ret.amount_from_str_comma_code.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_comma_code_some.is_some());
    assert_eq!(
        ret.amount_from_str_comma_code_some
            .as_ref()
            .unwrap()
            .amount(),
        dec!(2000.00)
    );
    assert!(ret.amount_from_str_comma_code_none.is_none());
    assert!(ret.amount_from_str_comma_code_omit.is_none());

    // comma + symbol
    assert_eq!(ret.amount_from_str_comma_symbol.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_comma_symbol_some.is_some());
    // "$2,345.6799" -> keep all fractional digits (no rounding)
    assert_eq!(
        ret.amount_from_str_comma_symbol_some
            .as_ref()
            .unwrap()
            .amount(),
        dec!(2345.6799)
    );
    assert!(ret.amount_from_str_comma_symbol_none.is_none());
    assert!(ret.amount_from_str_comma_symbol_omit.is_none());

    // dot + code (European formatting)
    // "EUR 1.234,5634" -> 1234.5634 (no rounding)
    assert_eq!(ret.amount_from_str_dot_code.amount(), dec!(1234.5634));
    assert!(ret.amount_from_str_dot_code_some.is_some());
    assert_eq!(
        ret.amount_from_str_dot_code_some.as_ref().unwrap().amount(),
        dec!(2000.00)
    );
    assert!(ret.amount_from_str_dot_code_none.is_none());
    assert!(ret.amount_from_str_dot_code_omit.is_none());

    // dot + symbol
    assert_eq!(ret.amount_from_str_dot_symbol.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_dot_symbol_some.is_some());
    assert_eq!(
        ret.amount_from_str_dot_symbol_some
            .as_ref()
            .unwrap()
            .amount(),
        dec!(2345.67)
    );
    assert!(ret.amount_from_str_dot_symbol_none.is_none());
    assert!(ret.amount_from_str_dot_symbol_omit.is_none());

    assert_eq!(ret.neg_amount_from_code_comma.amount(), dec!(-34345.5566));
    assert_eq!(ret.neg_amount_from_code_dot.amount(), dec!(-23_000.00));
    assert_eq!(ret.neg_amount_from_symbol_comma.amount(), dec!(-1.123));
    assert_eq!(ret.neg_amount_from_symbol_dot.amount(), dec!(-1.123));
}
