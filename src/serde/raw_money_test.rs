use crate::iso::{CAD, CHF, EUR, GBP, IDR, JPY, USD};
use crate::{BaseMoney, RawMoney, macros::dec};

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

// visit_f64 now delegates to visit_str via v.to_string(), so values that have
// exact decimal representations are parsed precisely rather than going through
// Decimal::from_f64 which can produce binary-representation artifacts.
#[test]
fn test_default_deserialize_visit_f64_precision() {
    // RawMoney has no fixed decimal places so precision is preserved as-is
    let money: RawMoney<USD> = serde_yaml::from_str("1.10").unwrap();
    assert_eq!(money.amount(), dec!(1.10));

    let money: RawMoney<USD> = serde_yaml::from_str("99.99").unwrap();
    assert_eq!(money.amount(), dec!(99.99));

    // negative
    let money: RawMoney<USD> = serde_yaml::from_str("-0.01").unwrap();
    assert_eq!(money.amount(), dec!(-0.01));
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

// ---------------------------------------------------------------------------
// minor serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentMinor {
    #[serde(with = "crate::serde::raw_money::minor")]
    amount: RawMoney<USD>,
}

#[test]
fn test_minor_serialize_usd() {
    let p = PaymentMinor {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":123456}"#);
}

#[test]
fn test_minor_serialize_rounds_before_minor() {
    // Extra precision is rounded to currency's minor unit before computing minor amount
    let p = PaymentMinor {
        amount: RawMoney::<USD>::from_decimal(dec!(123.238533)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":12324}"#);
}

#[test]
fn test_minor_serialize_negative() {
    let p = PaymentMinor {
        amount: RawMoney::<USD>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":-123456}"#);
}

#[test]
fn test_minor_serialize_jpy() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct PaymentMinorJPY {
        #[serde(with = "crate::serde::raw_money::minor")]
        amount: RawMoney<JPY>,
    }
    let p = PaymentMinorJPY {
        amount: RawMoney::<JPY>::from_decimal(dec!(1234)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":1234}"#);
}

#[test]
fn test_minor_deserialize() {
    let p: PaymentMinor = serde_json::from_str(r#"{"amount":123456}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_minor_deserialize_negative() {
    let p: PaymentMinor = serde_json::from_str(r#"{"amount":-123456}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56));
}

#[test]
fn test_minor_roundtrip() {
    let original = PaymentMinor {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentMinor = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// option_minor serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptMinor {
    #[serde(with = "crate::serde::raw_money::option_minor")]
    amount: Option<RawMoney<USD>>,
}

#[test]
fn test_option_minor_serialize_some() {
    let p = PaymentOptMinor {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":123456}"#);
}

#[test]
fn test_option_minor_serialize_none() {
    let p = PaymentOptMinor { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_minor_deserialize_some() {
    let p: PaymentOptMinor = serde_json::from_str(r#"{"amount":123456}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
}

#[test]
fn test_option_minor_deserialize_none() {
    let p: PaymentOptMinor = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

#[test]
fn test_option_minor_roundtrip() {
    let original = PaymentOptMinor {
        amount: Some(RawMoney::<USD>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptMinor = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

// ---------------------------------------------------------------------------
// visit_unit: option visitors accept unit (None) from unit-based deserializers
// ---------------------------------------------------------------------------

#[test]
fn test_option_comma_str_code_visit_unit() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::UnitDeserializer<E> = ().into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_code::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_comma_str_symbol_visit_unit() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::UnitDeserializer<E> = ().into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_symbol::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_dot_str_code_visit_unit() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::UnitDeserializer<E> = ().into_deserializer();
    let result = crate::serde::raw_money::option_dot_str_code::deserialize::<EUR, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_dot_str_symbol_visit_unit() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::UnitDeserializer<E> = ().into_deserializer();
    let result = crate::serde::raw_money::option_dot_str_symbol::deserialize::<EUR, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_minor_visit_unit() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::UnitDeserializer<E> = ().into_deserializer();
    let result = crate::serde::raw_money::option_minor::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

// ---------------------------------------------------------------------------
// minor: visit_i128 and visit_u128 (including overflow)
// ---------------------------------------------------------------------------

#[test]
fn test_minor_deserialize_i128() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    // Value smaller than i64::MIN → dispatches to visit_i128
    let val: i128 = i64::MIN as i128 - 1;
    let d: serde::de::value::I128Deserializer<E> = val.into_deserializer();
    let result = crate::serde::raw_money::minor::deserialize::<USD, _>(d);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().code(), "USD");
}

#[test]
fn test_minor_deserialize_u128() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    // Value greater than u64::MAX → dispatches to visit_u128 (success path)
    let val: u128 = u64::MAX as u128 + 1;
    let d: serde::de::value::U128Deserializer<E> = val.into_deserializer();
    let result = crate::serde::raw_money::minor::deserialize::<USD, _>(d);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().code(), "USD");
}

#[test]
fn test_minor_deserialize_u128_overflow() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    // u128::MAX exceeds i128::MAX → error path in visit_u128
    let d: serde::de::value::U128Deserializer<E> = u128::MAX.into_deserializer();
    let result = crate::serde::raw_money::minor::deserialize::<USD, _>(d);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("too large"));
}

// ---------------------------------------------------------------------------
// option_minor: expecting (triggered by unexpected type via BoolDeserializer)
// ---------------------------------------------------------------------------

#[test]
fn test_option_minor_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    // BoolDeserializer::deserialize_option forwards to deserialize_any → visit_bool
    // option_minor::Visitor has no visit_bool → default impl calls expecting()
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_minor::deserialize::<USD, _>(d);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("integer") || msg.contains("null") || msg.contains("minor"),
        "unexpected error message: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// JPY: comma_str_code::Visitor::expecting triggered by wrong input type
// ---------------------------------------------------------------------------

#[test]
fn test_comma_str_code_jpy_deserialize_wrong_type() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<JPY>,
    }
    // Passing an integer where a string is expected triggers Visitor::expecting for JPY
    let result: Result<W, _> = serde_json::from_str(r#"{"amount":1234}"#);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Default deserializer: EUR, IDR, CAD with serde_json (via visit_map)
// ---------------------------------------------------------------------------

#[test]
fn test_default_deserialize_eur_json() {
    let raw: RawMoney<EUR> = serde_json::from_str("1234.56789").unwrap();
    assert_eq!(raw.code(), "EUR");
}

#[test]
fn test_default_deserialize_idr_json() {
    let raw: RawMoney<IDR> = serde_json::from_str("5000").unwrap();
    assert_eq!(raw.code(), "IDR");
}

#[test]
fn test_default_deserialize_cad_json() {
    let raw: RawMoney<CAD> = serde_json::from_str("99.99123").unwrap();
    assert_eq!(raw.code(), "CAD");
}

// ---------------------------------------------------------------------------
// Multi-currency struct: covers visit_map<MapAccess<StrRead>> for EUR/IDR/CAD
// ---------------------------------------------------------------------------

#[test]
fn test_default_deserialize_multi_currency_struct_json() {
    #[derive(::serde::Deserialize)]
    struct Multi {
        eur: RawMoney<EUR>,
        idr: RawMoney<IDR>,
        cad: RawMoney<CAD>,
    }
    let m: Multi = serde_json::from_str(r#"{"eur":100.56789,"idr":5000,"cad":99.99123}"#).unwrap();
    assert_eq!(m.eur.code(), "EUR");
    assert_eq!(m.idr.code(), "IDR");
    assert_eq!(m.cad.code(), "CAD");
}

// ---------------------------------------------------------------------------
// visit_map error path: YAML mapping triggers "unexpected key"
// ---------------------------------------------------------------------------

#[test]
fn test_default_deserialize_yaml_mapping_error() {
    // A YAML mapping fed to the default deserializer calls visit_map with
    // serde_yaml's MapAccess; our visitor returns "unexpected key"
    let result = serde_yaml::from_str::<RawMoney<USD>>(r#"unexpected_key: value"#);
    assert!(result.is_err());
}

#[test]
fn test_default_deserialize_eur_yaml_mapping_error() {
    let result = serde_yaml::from_str::<RawMoney<EUR>>(r#"unexpected_key: value"#);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Serialize via serde_json::to_value (exercises NumberStrEmitter path)
// ---------------------------------------------------------------------------

#[test]
fn test_default_serialize_to_value_usd() {
    let raw = RawMoney::<USD>::from_decimal(dec!(1234.56789));
    let val = serde_json::to_value(&raw).unwrap();
    assert!(val.is_number());
}

#[test]
fn test_default_serialize_to_value_eur() {
    let raw = RawMoney::<EUR>::from_decimal(dec!(99.99));
    let val = serde_json::to_value(&raw).unwrap();
    assert!(val.is_number());
}

#[test]
fn test_default_serialize_to_value_jpy() {
    let raw = RawMoney::<JPY>::from_decimal(dec!(1234));
    let val = serde_json::to_value(&raw).unwrap();
    assert!(val.is_number());
}

#[test]
fn test_comma_str_code_serialize_to_value_usd() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let val = serde_json::to_value(&w).unwrap();
    assert_eq!(val["amount"], "USD 1,234.56789");
}

#[test]
fn test_comma_str_code_serialize_to_value_jpy() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::comma_str_code")]
        amount: RawMoney<JPY>,
    }
    let w = W {
        amount: RawMoney::<JPY>::from_decimal(dec!(1234)),
    };
    let val = serde_json::to_value(&w).unwrap();
    assert_eq!(val["amount"], "JPY 1,234");
}

#[test]
fn test_dot_str_symbol_serialize_to_value_eur() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::dot_str_symbol")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let val = serde_json::to_value(&w).unwrap();
    assert_eq!(val["amount"], "€1.234,56789");
}

// ---------------------------------------------------------------------------
// Default deserializer: EUR, IDR, CAD with serde_yaml (visit_f64, visit_i64)
// ---------------------------------------------------------------------------

#[test]
fn test_default_deserialize_eur_yaml() {
    let raw: RawMoney<EUR> = serde_yaml::from_str("1234.56789").unwrap();
    assert_eq!(raw.code(), "EUR");
    let raw2: RawMoney<EUR> = serde_yaml::from_str("-500").unwrap();
    assert_eq!(raw2.code(), "EUR");
}

#[test]
fn test_default_deserialize_idr_yaml() {
    let raw: RawMoney<IDR> = serde_yaml::from_str("50000.5").unwrap();
    assert_eq!(raw.code(), "IDR");
    let raw2: RawMoney<IDR> = serde_yaml::from_str("-1000").unwrap();
    assert_eq!(raw2.code(), "IDR");
}

#[test]
fn test_default_deserialize_cad_yaml() {
    let raw: RawMoney<CAD> = serde_yaml::from_str("99.99123").unwrap();
    assert_eq!(raw.code(), "CAD");
    let raw2: RawMoney<CAD> = serde_yaml::from_str("-50").unwrap();
    assert_eq!(raw2.code(), "CAD");
}

// ---------------------------------------------------------------------------
// visit_map<MapAccess<StrRead>>: JSON object input (both branches)
// ---------------------------------------------------------------------------

#[test]
fn test_default_deserialize_json_object_number_key_usd() {
    // JSON object with serde_json's private number key → success path of visit_map<MapAccess<StrRead>>
    let raw: RawMoney<USD> =
        serde_json::from_str(r#"{"$serde_json::private::Number":"1234.56789"}"#).unwrap();
    assert_eq!(raw.code(), "USD");
    assert_eq!(raw.amount(), dec!(1234.56789));
}

#[test]
fn test_default_deserialize_json_object_number_key_eur() {
    let raw: RawMoney<EUR> =
        serde_json::from_str(r#"{"$serde_json::private::Number":"99.99"}"#).unwrap();
    assert_eq!(raw.code(), "EUR");
}

#[test]
fn test_default_deserialize_json_object_number_key_idr() {
    let raw: RawMoney<IDR> =
        serde_json::from_str(r#"{"$serde_json::private::Number":"5000"}"#).unwrap();
    assert_eq!(raw.code(), "IDR");
}

#[test]
fn test_default_deserialize_json_object_number_key_cad() {
    let raw: RawMoney<CAD> =
        serde_json::from_str(r#"{"$serde_json::private::Number":"99.99123"}"#).unwrap();
    assert_eq!(raw.code(), "CAD");
}

#[test]
fn test_default_deserialize_json_object_wrong_key() {
    // JSON object with wrong key → else branch ("unexpected key") of visit_map<MapAccess<StrRead>>
    let result = serde_json::from_str::<RawMoney<USD>>(r#"{"wrong_key":"value"}"#);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unexpected key"));
}

#[test]
fn test_default_deserialize_json_object_invalid_decimal() {
    // JSON object with valid key but non-decimal value → map_err closure in visit_map
    let result =
        serde_json::from_str::<RawMoney<USD>>(r#"{"$serde_json::private::Number":"not_a_number"}"#);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid decimal"));
}

// ---------------------------------------------------------------------------
// visit_unit: option visitors with serde_yaml::Error via UnitDeserializer
// ---------------------------------------------------------------------------

#[test]
fn test_option_comma_str_code_visit_unit_yaml_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_code::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_comma_str_symbol_visit_unit_yaml_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_symbol::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_dot_str_code_visit_unit_yaml_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_dot_str_code::deserialize::<EUR, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_dot_str_symbol_visit_unit_yaml_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_dot_str_symbol::deserialize::<EUR, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_minor_visit_unit_yaml_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_minor::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_comma_str_code_visit_unit_json_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_json::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_code::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

#[test]
fn test_option_minor_visit_unit_json_error() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_json::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_minor::deserialize::<USD, _>(d);
    assert!(result.unwrap().is_none());
}

// ---------------------------------------------------------------------------
// str_code: serialize/deserialize using currency locale separators (code)
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentLocaleCode {
    #[serde(with = "crate::serde::raw_money::str_code")]
    amount: RawMoney<CHF>,
}

#[test]
fn test_str_code_serialize_chf() {
    // CHF: thousands='\'' decimal='.'
    let p = PaymentLocaleCode {
        amount: RawMoney::<CHF>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"CHF 1'234.56789"}"#);
}

#[test]
fn test_str_code_serialize_chf_negative() {
    let p = PaymentLocaleCode {
        amount: RawMoney::<CHF>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"CHF -1'234.56789"}"#);
}

#[test]
fn test_str_code_deserialize_chf() {
    let p: PaymentLocaleCode = serde_json::from_str(r#"{"amount":"CHF 1'234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "CHF");
}

#[test]
fn test_str_code_deserialize_chf_negative() {
    let p: PaymentLocaleCode = serde_json::from_str(r#"{"amount":"CHF -1'234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56789));
}

#[test]
fn test_str_code_roundtrip_chf() {
    let original = PaymentLocaleCode {
        amount: RawMoney::<CHF>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentLocaleCode = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_str_code_serialize_usd_locale() {
    // USD locale is same as comma_str_code
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::str_code")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&w).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56789"}"#);
}

#[test]
fn test_str_code_serialize_eur_locale() {
    // EUR locale is same as dot_str_code
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::str_code")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&w).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56789"}"#);
}

#[test]
fn test_str_code_deserialize_chf_wrong_currency() {
    let result: Result<PaymentLocaleCode, _> =
        serde_json::from_str(r#"{"amount":"EUR 1.234,56789"}"#);
    assert!(result.is_err());
}

#[test]
fn test_str_code_deserialize_chf_malformed() {
    let result: Result<PaymentLocaleCode, _> = serde_json::from_str(r#"{"amount":"not_valid"}"#);
    assert!(result.is_err());
}

#[test]
fn test_str_code_serialize_zero_chf() {
    let p = PaymentLocaleCode {
        amount: RawMoney::<CHF>::from_decimal(dec!(0)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"CHF 0.00"}"#);
}

// ---------------------------------------------------------------------------
// option_str_code: optional variant of str_code
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptLocaleCode {
    #[serde(with = "crate::serde::raw_money::option_str_code")]
    amount: Option<RawMoney<CHF>>,
}

#[test]
fn test_option_str_code_serialize_some_chf() {
    let p = PaymentOptLocaleCode {
        amount: Some(RawMoney::<CHF>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"CHF 1'234.56789"}"#);
}

#[test]
fn test_option_str_code_serialize_none() {
    let p = PaymentOptLocaleCode { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_str_code_deserialize_some_chf() {
    let p: PaymentOptLocaleCode = serde_json::from_str(r#"{"amount":"CHF 1'234.56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_str_code_deserialize_none() {
    let p: PaymentOptLocaleCode = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

#[test]
fn test_option_str_code_roundtrip() {
    let original = PaymentOptLocaleCode {
        amount: Some(RawMoney::<CHF>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptLocaleCode = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_option_str_code_visit_unit() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_str_code::deserialize::<CHF, _>(d);
    assert!(result.unwrap().is_none());
}

// ---------------------------------------------------------------------------
// str_symbol: serialize/deserialize using currency locale separators (symbol)
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentLocaleSymbol {
    #[serde(with = "crate::serde::raw_money::str_symbol")]
    amount: RawMoney<CHF>,
}

#[test]
fn test_str_symbol_serialize_chf() {
    // CHF symbol: ₣, thousands='\'', decimal='.'
    let p = PaymentLocaleSymbol {
        amount: RawMoney::<CHF>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"₣1'234.56789"}"#);
}

#[test]
fn test_str_symbol_serialize_chf_negative() {
    let p = PaymentLocaleSymbol {
        amount: RawMoney::<CHF>::from_decimal(dec!(-1234.56789)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"-₣1'234.56789"}"#);
}

#[test]
fn test_str_symbol_deserialize_chf() {
    let p: PaymentLocaleSymbol = serde_json::from_str(r#"{"amount":"₣1'234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56789));
    assert_eq!(p.amount.code(), "CHF");
}

#[test]
fn test_str_symbol_deserialize_chf_negative() {
    let p: PaymentLocaleSymbol = serde_json::from_str(r#"{"amount":"-₣1'234.56789"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56789));
}

#[test]
fn test_str_symbol_roundtrip_chf() {
    let original = PaymentLocaleSymbol {
        amount: RawMoney::<CHF>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentLocaleSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_str_symbol_serialize_usd_locale() {
    // USD locale is same as comma_str_symbol
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::str_symbol")]
        amount: RawMoney<USD>,
    }
    let w = W {
        amount: RawMoney::<USD>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&w).unwrap();
    assert_eq!(json, r#"{"amount":"$1,234.56789"}"#);
}

#[test]
fn test_str_symbol_serialize_eur_locale() {
    // EUR locale is same as dot_str_symbol
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::str_symbol")]
        amount: RawMoney<EUR>,
    }
    let w = W {
        amount: RawMoney::<EUR>::from_decimal(dec!(1234.56789)),
    };
    let json = serde_json::to_string(&w).unwrap();
    assert_eq!(json, r#"{"amount":"€1.234,56789"}"#);
}

#[test]
fn test_str_symbol_deserialize_chf_wrong_symbol() {
    let result: Result<PaymentLocaleSymbol, _> =
        serde_json::from_str(r#"{"amount":"€1.234,56789"}"#);
    assert!(result.is_err());
}

#[test]
fn test_str_symbol_serialize_zero_chf() {
    let p = PaymentLocaleSymbol {
        amount: RawMoney::<CHF>::from_decimal(dec!(0)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"₣0.00"}"#);
}

// ---------------------------------------------------------------------------
// option_str_symbol: optional variant of str_symbol
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentOptLocaleSymbol {
    #[serde(with = "crate::serde::raw_money::option_str_symbol")]
    amount: Option<RawMoney<CHF>>,
}

#[test]
fn test_option_str_symbol_serialize_some_chf() {
    let p = PaymentOptLocaleSymbol {
        amount: Some(RawMoney::<CHF>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"₣1'234.56789"}"#);
}

#[test]
fn test_option_str_symbol_serialize_none() {
    let p = PaymentOptLocaleSymbol { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_str_symbol_deserialize_some_chf() {
    let p: PaymentOptLocaleSymbol = serde_json::from_str(r#"{"amount":"₣1'234.56789"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56789));
}

#[test]
fn test_option_str_symbol_deserialize_none() {
    let p: PaymentOptLocaleSymbol = serde_json::from_str(r#"{"amount":null}"#).unwrap();
    assert!(p.amount.is_none());
}

#[test]
fn test_option_str_symbol_roundtrip() {
    let original = PaymentOptLocaleSymbol {
        amount: Some(RawMoney::<CHF>::from_decimal(dec!(1234.56789))),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PaymentOptLocaleSymbol = serde_json::from_str(&json).unwrap();
    assert_eq!(original.amount, deserialized.amount);
}

#[test]
fn test_option_str_symbol_visit_unit() {
    use serde::de::IntoDeserializer;
    let d: serde::de::value::UnitDeserializer<serde_yaml::Error> = ().into_deserializer();
    let result = crate::serde::raw_money::option_str_symbol::deserialize::<CHF, _>(d);
    assert!(result.unwrap().is_none());
}

// ---------------------------------------------------------------------------
// expecting: triggered by BoolDeserializer (no visit_bool → default calls expecting)
// ---------------------------------------------------------------------------

#[test]
fn test_option_comma_str_code_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_code::deserialize::<USD, _>(d);
    assert!(result.is_err());
}

#[test]
fn test_option_comma_str_symbol_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_comma_str_symbol::deserialize::<USD, _>(d);
    assert!(result.is_err());
}

#[test]
fn test_option_dot_str_code_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_dot_str_code::deserialize::<EUR, _>(d);
    assert!(result.is_err());
}

#[test]
fn test_option_dot_str_symbol_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_dot_str_symbol::deserialize::<EUR, _>(d);
    assert!(result.is_err());
}

#[test]
fn test_option_str_code_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_str_code::deserialize::<CHF, _>(d);
    assert!(result.is_err());
}

#[test]
fn test_option_str_symbol_expecting() {
    use serde::de::IntoDeserializer;
    type E = serde::de::value::Error;
    let d: serde::de::value::BoolDeserializer<E> = true.into_deserializer();
    let result = crate::serde::raw_money::option_str_symbol::deserialize::<CHF, _>(d);
    assert!(result.is_err());
}

#[test]
fn test_str_code_expecting() {
    // Passing an integer where a string is expected triggers Visitor::expecting for str_code
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::str_code")]
        amount: RawMoney<CHF>,
    }
    let result: Result<W, _> = serde_json::from_str(r#"{"amount":123}"#);
    assert!(result.is_err());
}

#[test]
fn test_str_symbol_expecting() {
    // Passing an integer where a string is expected triggers Visitor::expecting for str_symbol
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::str_symbol")]
        amount: RawMoney<CHF>,
    }
    let result: Result<W, _> = serde_json::from_str(r#"{"amount":123}"#);
    assert!(result.is_err());
}

#[test]
fn test_minor_expecting() {
    // Passing a string where an integer is expected triggers Visitor::expecting for minor
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::raw_money::minor")]
        amount: RawMoney<USD>,
    }
    let result: Result<W, _> = serde_json::from_str(r#"{"amount":"not-a-number"}"#);
    assert!(result.is_err());
}
