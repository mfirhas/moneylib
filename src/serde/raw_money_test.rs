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
