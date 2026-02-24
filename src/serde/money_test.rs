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
    assert_eq!(json, "1234");
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
    assert_eq!(json, "1234");
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
    assert_eq!(json, "100.50");
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
// comma_str_code serialize/deserialize
// ---------------------------------------------------------------------------

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct PaymentCommaCode {
    #[serde(with = "crate::serde::money::comma_str_code")]
    amount: Money<USD>,
}

#[test]
fn test_comma_str_code_serialize() {
    let p = PaymentCommaCode {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56"}"#);
}

#[test]
fn test_comma_str_code_serialize_negative() {
    let p = PaymentCommaCode {
        amount: Money::<USD>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD -1,234.56"}"#);
}

#[test]
fn test_comma_str_code_deserialize() {
    let p: PaymentCommaCode = serde_json::from_str(r#"{"amount":"USD 1,234.56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_comma_str_code_roundtrip() {
    let original = PaymentCommaCode {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
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
    #[serde(with = "crate::serde::money::option_comma_str_code")]
    amount: Option<Money<USD>>,
}

#[test]
fn test_option_comma_str_code_serialize_some() {
    let p = PaymentOptCommaCode {
        amount: Some(Money::<USD>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"USD 1,234.56"}"#);
}

#[test]
fn test_option_comma_str_code_serialize_none() {
    let p = PaymentOptCommaCode { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_comma_str_code_deserialize_some() {
    let p: PaymentOptCommaCode = serde_json::from_str(r#"{"amount":"USD 1,234.56"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
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
    #[serde(with = "crate::serde::money::comma_str_symbol")]
    amount: Money<USD>,
}

#[test]
fn test_comma_str_symbol_serialize() {
    let p = PaymentCommaSymbol {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"$1,234.56"}"#);
}

#[test]
fn test_comma_str_symbol_serialize_negative() {
    let p = PaymentCommaSymbol {
        amount: Money::<USD>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"-$1,234.56"}"#);
}

#[test]
fn test_comma_str_symbol_deserialize() {
    let p: PaymentCommaSymbol = serde_json::from_str(r#"{"amount":"$1,234.56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "USD");
}

#[test]
fn test_comma_str_symbol_deserialize_negative() {
    let p: PaymentCommaSymbol = serde_json::from_str(r#"{"amount":"-$1,234.56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56));
}

#[test]
fn test_comma_str_symbol_roundtrip() {
    let original = PaymentCommaSymbol {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
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
    #[serde(with = "crate::serde::money::option_comma_str_symbol")]
    amount: Option<Money<USD>>,
}

#[test]
fn test_option_comma_str_symbol_serialize_some() {
    let p = PaymentOptCommaSymbol {
        amount: Some(Money::<USD>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"$1,234.56"}"#);
}

#[test]
fn test_option_comma_str_symbol_serialize_none() {
    let p = PaymentOptCommaSymbol { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_comma_str_symbol_deserialize_some() {
    let p: PaymentOptCommaSymbol = serde_json::from_str(r#"{"amount":"$1,234.56"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
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
    #[serde(with = "crate::serde::money::dot_str_code")]
    amount: Money<EUR>,
}

#[test]
fn test_dot_str_code_serialize() {
    let p = PaymentDotCode {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56"}"#);
}

#[test]
fn test_dot_str_code_serialize_negative() {
    let p = PaymentDotCode {
        amount: Money::<EUR>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR -1.234,56"}"#);
}

#[test]
fn test_dot_str_code_deserialize() {
    let p: PaymentDotCode = serde_json::from_str(r#"{"amount":"EUR 1.234,56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "EUR");
}

#[test]
fn test_dot_str_code_roundtrip() {
    let original = PaymentDotCode {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
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
    #[serde(with = "crate::serde::money::option_dot_str_code")]
    amount: Option<Money<EUR>>,
}

#[test]
fn test_option_dot_str_code_serialize_some() {
    let p = PaymentOptDotCode {
        amount: Some(Money::<EUR>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"EUR 1.234,56"}"#);
}

#[test]
fn test_option_dot_str_code_serialize_none() {
    let p = PaymentOptDotCode { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_dot_str_code_deserialize_some() {
    let p: PaymentOptDotCode = serde_json::from_str(r#"{"amount":"EUR 1.234,56"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
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
    #[serde(with = "crate::serde::money::dot_str_symbol")]
    amount: Money<EUR>,
}

#[test]
fn test_dot_str_symbol_serialize() {
    let p = PaymentDotSymbol {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"€1.234,56"}"#);
}

#[test]
fn test_dot_str_symbol_serialize_negative() {
    let p = PaymentDotSymbol {
        amount: Money::<EUR>::from_decimal(dec!(-1234.56)),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"-€1.234,56"}"#);
}

#[test]
fn test_dot_str_symbol_deserialize() {
    let p: PaymentDotSymbol = serde_json::from_str(r#"{"amount":"€1.234,56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(1234.56));
    assert_eq!(p.amount.code(), "EUR");
}

#[test]
fn test_dot_str_symbol_deserialize_negative() {
    let p: PaymentDotSymbol = serde_json::from_str(r#"{"amount":"-€1.234,56"}"#).unwrap();
    assert_eq!(p.amount.amount(), dec!(-1234.56));
}

#[test]
fn test_dot_str_symbol_roundtrip() {
    let original = PaymentDotSymbol {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
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
    #[serde(with = "crate::serde::money::option_dot_str_symbol")]
    amount: Option<Money<EUR>>,
}

#[test]
fn test_option_dot_str_symbol_serialize_some() {
    let p = PaymentOptDotSymbol {
        amount: Some(Money::<EUR>::from_decimal(dec!(1234.56))),
    };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":"€1.234,56"}"#);
}

#[test]
fn test_option_dot_str_symbol_serialize_none() {
    let p = PaymentOptDotSymbol { amount: None };
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, r#"{"amount":null}"#);
}

#[test]
fn test_option_dot_str_symbol_deserialize_some() {
    let p: PaymentOptDotSymbol = serde_json::from_str(r#"{"amount":"€1.234,56"}"#).unwrap();
    assert_eq!(p.amount.unwrap().amount(), dec!(1234.56));
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
        #[serde(with = "crate::serde::money::comma_str_code")]
        amount: Money<USD>,
    }
    let w = W {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: USD 1,234.56\n");
}

#[test]
fn test_yaml_comma_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_code")]
        amount: Money<USD>,
    }
    let w: W = serde_yaml::from_str("amount: \"USD 1,234.56\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
    assert_eq!(w.amount.code(), "USD");
}

#[test]
fn test_yaml_comma_str_code_roundtrip() {
    #[derive(::serde::Serialize, ::serde::Deserialize, PartialEq, Debug)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_code")]
        amount: Money<USD>,
    }
    let original = W {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let yaml = serde_yaml::to_string(&original).unwrap();
    let result: W = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(original.amount, result.amount);
}

#[test]
fn test_yaml_comma_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_symbol")]
        amount: Money<USD>,
    }
    let w = W {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: $1,234.56\n");
}

#[test]
fn test_yaml_comma_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_symbol")]
        amount: Money<USD>,
    }
    let w: W = serde_yaml::from_str("amount: \"$1,234.56\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
}

#[test]
fn test_yaml_dot_str_code_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_code")]
        amount: Money<EUR>,
    }
    let w = W {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: EUR 1.234,56\n");
}

#[test]
fn test_yaml_dot_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_code")]
        amount: Money<EUR>,
    }
    let w: W = serde_yaml::from_str("amount: \"EUR 1.234,56\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
    assert_eq!(w.amount.code(), "EUR");
}

#[test]
fn test_yaml_dot_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_symbol")]
        amount: Money<EUR>,
    }
    let w = W {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: €1.234,56\n");
}

#[test]
fn test_yaml_dot_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_symbol")]
        amount: Money<EUR>,
    }
    let w: W = serde_yaml::from_str("amount: \"€1.234,56\"").unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
}

#[test]
fn test_yaml_option_comma_str_code_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::option_comma_str_code")]
        amount: Option<Money<USD>>,
    }
    let w = W {
        amount: Some(Money::<USD>::from_decimal(dec!(1234.56))),
    };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: USD 1,234.56\n");
}

#[test]
fn test_yaml_option_comma_str_code_serialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::option_comma_str_code")]
        amount: Option<Money<USD>>,
    }
    let w = W { amount: None };
    let yaml = serde_yaml::to_string(&w).unwrap();
    assert_eq!(yaml, "amount: null\n");
}

#[test]
fn test_yaml_option_comma_str_code_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::option_comma_str_code")]
        amount: Option<Money<USD>>,
    }
    let w: W = serde_yaml::from_str("amount: \"USD 1,234.56\"").unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56));
}

#[test]
fn test_yaml_option_comma_str_code_deserialize_none() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::option_comma_str_code")]
        amount: Option<Money<USD>>,
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
        #[serde(with = "crate::serde::money::comma_str_code")]
        amount: Money<USD>,
    }
    let w = W {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"USD 1,234.56\"\n");
}

#[test]
fn test_toml_comma_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_code")]
        amount: Money<USD>,
    }
    let w: W = toml::from_str(r#"amount = "USD 1,234.56""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
    assert_eq!(w.amount.code(), "USD");
}

#[test]
fn test_toml_comma_str_code_roundtrip() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_code")]
        amount: Money<USD>,
    }
    let original = W {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let t = toml::to_string(&original).unwrap();
    let result: W = toml::from_str(&t).unwrap();
    assert_eq!(original.amount, result.amount);
}

#[test]
fn test_toml_comma_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_symbol")]
        amount: Money<USD>,
    }
    let w = W {
        amount: Money::<USD>::from_decimal(dec!(1234.56)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"$1,234.56\"\n");
}

#[test]
fn test_toml_comma_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::comma_str_symbol")]
        amount: Money<USD>,
    }
    let w: W = toml::from_str(r#"amount = "$1,234.56""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
}

#[test]
fn test_toml_dot_str_code_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_code")]
        amount: Money<EUR>,
    }
    let w = W {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"EUR 1.234,56\"\n");
}

#[test]
fn test_toml_dot_str_code_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_code")]
        amount: Money<EUR>,
    }
    let w: W = toml::from_str(r#"amount = "EUR 1.234,56""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
    assert_eq!(w.amount.code(), "EUR");
}

#[test]
fn test_toml_dot_str_symbol_serialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_symbol")]
        amount: Money<EUR>,
    }
    let w = W {
        amount: Money::<EUR>::from_decimal(dec!(1234.56)),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"€1.234,56\"\n");
}

#[test]
fn test_toml_dot_str_symbol_deserialize() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::dot_str_symbol")]
        amount: Money<EUR>,
    }
    let w: W = toml::from_str(r#"amount = "€1.234,56""#).unwrap();
    assert_eq!(w.amount.amount(), dec!(1234.56));
}

#[test]
fn test_toml_option_comma_str_code_serialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::option_comma_str_code")]
        amount: Option<Money<USD>>,
    }
    let w = W {
        amount: Some(Money::<USD>::from_decimal(dec!(1234.56))),
    };
    let t = toml::to_string(&w).unwrap();
    assert_eq!(t, "amount = \"USD 1,234.56\"\n");
}

#[test]
fn test_toml_option_comma_str_code_deserialize_some() {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct W {
        #[serde(with = "crate::serde::money::option_comma_str_code")]
        amount: Option<Money<USD>>,
    }
    let w: W = toml::from_str(r#"amount = "USD 1,234.56""#).unwrap();
    assert_eq!(w.amount.unwrap().amount(), dec!(1234.56));
}
