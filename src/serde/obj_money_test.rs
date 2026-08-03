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

mod failed_serialize_tests {
    use crate::Decimal;
    use crate::dec;
    use crate::obj_money::ObjMoney;
    use ::serde::Serialize;
    use serde::ser::{Impossible, SerializeStruct, Serializer};
    use std::fmt;

    // ---------- (1): amount -> serde_json::Number conversion ----------
    // Decimal::to_string() always yields valid JSON-number syntax, so the
    // map_err in the real impl can't be triggered via the public API.
    // These prove that holds at the extremes of Decimal's range.

    #[test]
    fn serialize_max_decimal_does_not_hit_number_conversion_error() {
        let money: ObjMoney<true> = ObjMoney::try_new("USD", Decimal::MAX).unwrap();
        let json = serde_json::to_string(&money).unwrap();
        assert!(json.contains(&Decimal::MAX.to_string()));
    }

    #[test]
    fn serialize_min_decimal_does_not_hit_number_conversion_error() {
        let money: ObjMoney<true> = ObjMoney::try_new("USD", Decimal::MIN).unwrap();
        let json = serde_json::to_string(&money).unwrap();
        assert!(json.contains(&Decimal::MIN.to_string()));
    }

    // ---------- (2)-(4): propagation from the underlying Serializer ----------
    // A minimal Serializer that fails at a configurable call, to prove the
    // `?`s in serialize() actually propagate the serializer's error rather
    // than being swallowed or panicking.

    #[derive(Debug, PartialEq)]
    struct TestError(String);

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::error::Error for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T: fmt::Display>(msg: T) -> Self {
            TestError(msg.to_string())
        }
    }

    /// Which call should fail: 1 = serialize_struct, 2 = field "code", 3 = field "amount".
    struct FailingSerializer {
        fail_at: usize,
    }

    struct FailingStruct {
        fail_at: usize,
        call_count: usize,
    }

    impl SerializeStruct for FailingStruct {
        type Ok = ();
        type Error = TestError;

        fn serialize_field<T: ?Sized + serde::Serialize>(
            &mut self,
            key: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            self.call_count += 1;
            if self.call_count == self.fail_at {
                Err(TestError(format!("failed serializing field \"{}\"", key)))
            } else {
                Ok(())
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl Serializer for FailingSerializer {
        type Ok = ();
        type Error = TestError;
        type SerializeSeq = Impossible<(), TestError>;
        type SerializeTuple = Impossible<(), TestError>;
        type SerializeTupleStruct = Impossible<(), TestError>;
        type SerializeTupleVariant = Impossible<(), TestError>;
        type SerializeMap = Impossible<(), TestError>;
        type SerializeStruct = FailingStruct;
        type SerializeStructVariant = Impossible<(), TestError>;

        fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            if self.fail_at == 1 {
                Err(TestError("failed at serialize_struct".into()))
            } else {
                Ok(FailingStruct {
                    fail_at: self.fail_at,
                    call_count: 1,
                })
            }
        }

        // Nothing below this is exercised by ObjMoney::serialize; each stub
        // proves that by panicking if it's ever actually called.
        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            // self.code() goes through serialize_field, not this directly.
            unreachable!()
        }
        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_some<T: ?Sized + serde::Serialize>(
            self,
            _v: &T,
        ) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_unit_variant(
            self,
            _n: &'static str,
            _i: u32,
            _v: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(
            self,
            _n: &'static str,
            _v: &T,
        ) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
            self,
            _n: &'static str,
            _i: u32,
            _v: &'static str,
            _val: &T,
        ) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        }
        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            unreachable!()
        }
        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            unreachable!()
        }
        fn serialize_tuple_struct(
            self,
            _n: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            unreachable!()
        }
        fn serialize_tuple_variant(
            self,
            _n: &'static str,
            _i: u32,
            _v: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            unreachable!()
        }
        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            unreachable!()
        }
        fn serialize_struct_variant(
            self,
            _n: &'static str,
            _i: u32,
            _v: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            unreachable!()
        }
    }

    // ---------- Tests using the failing serializer ----------

    #[test]
    fn serialize_propagates_error_from_serialize_struct() {
        let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();
        let result = money.serialize(FailingSerializer { fail_at: 1 });
        assert_eq!(result, Err(TestError("failed at serialize_struct".into())));
    }

    #[test]
    fn serialize_propagates_error_from_code_field() {
        let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();
        let result = money.serialize(FailingSerializer { fail_at: 2 });
        assert_eq!(
            result,
            Err(TestError("failed serializing field \"code\"".into()))
        );
    }

    #[test]
    fn serialize_propagates_error_from_amount_field() {
        let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();
        let result = money.serialize(FailingSerializer { fail_at: 3 });
        assert_eq!(
            result,
            Err(TestError("failed serializing field \"amount\"".into()))
        );
    }

    #[test]
    fn serialize_succeeds_when_serializer_never_fails() {
        let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();
        let result = money.serialize(FailingSerializer { fail_at: 4 }); // no call reaches index 4
        assert_eq!(result, Ok(()));
    }
}
