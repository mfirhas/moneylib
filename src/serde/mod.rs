use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{Currency, Money};

impl<C: Currency + Clone> Serialize for Money<C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

struct MoneyVisitor<C>(PhantomData<C>);

impl<'de, C: Currency + Clone> de::Visitor<'de> for MoneyVisitor<C> {
    type Value = Money<C>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string like 'USD 1,234.56' or a number")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Money::<C>::from_str(v).map_err(de::Error::custom)
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Money::<C>::new(v).map_err(de::Error::custom)
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Money::<C>::new(v).map_err(de::Error::custom)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Money::<C>::new(i128::from(v)).map_err(de::Error::custom)
    }

    fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
        Money::<C>::new(v).map_err(de::Error::custom)
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
        i128::try_from(v)
            .map_err(|_| de::Error::custom("value too large for Money"))
            .and_then(|n| Money::<C>::new(n).map_err(de::Error::custom))
    }
}

impl<'de, C: Currency + Clone> Deserialize<'de> for Money<C> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(MoneyVisitor(PhantomData))
    }
}

#[cfg(feature = "raw_money")]
mod raw_money_serde {
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de};

    use crate::{Currency, RawMoney};

    impl<C: Currency + Clone> Serialize for RawMoney<C> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct RawMoneyVisitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for RawMoneyVisitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string like 'USD 1,234.56789' or a number")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            RawMoney::<C>::from_str(v).map_err(de::Error::custom)
        }

        fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
            RawMoney::<C>::new(v).map_err(de::Error::custom)
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            RawMoney::<C>::new(v).map_err(de::Error::custom)
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            RawMoney::<C>::new(i128::from(v)).map_err(de::Error::custom)
        }

        fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
            RawMoney::<C>::new(v).map_err(de::Error::custom)
        }

        fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
            i128::try_from(v)
                .map_err(|_| de::Error::custom("value too large for RawMoney"))
                .and_then(|n| RawMoney::<C>::new(n).map_err(de::Error::custom))
        }
    }

    impl<'de, C: Currency + Clone> Deserialize<'de> for RawMoney<C> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_any(RawMoneyVisitor(PhantomData))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{BaseMoney, Money, money_macros::dec};
    use crate::{EUR, JPY, USD};

    #[test]
    fn test_money_serialize_as_string() {
        let money = Money::<USD>::from_decimal(dec!(1234.56));
        assert_eq!(money.to_string(), "USD 1,234.56");
    }

    #[test]
    fn test_money_deserialize_from_string() {
        let money = Money::<USD>::from_str("USD 1,234.56").unwrap();
        assert_eq!(money.amount(), dec!(1234.56));
        assert_eq!(money.code(), "USD");
    }

    #[test]
    fn test_money_deserialize_from_string_no_thousands() {
        let money = Money::<EUR>::from_str("EUR 100.50").unwrap();
        assert_eq!(money.amount(), dec!(100.50));
    }

    #[test]
    fn test_money_serialize_jpy() {
        let money = Money::<JPY>::from_decimal(dec!(1234));
        assert_eq!(money.to_string(), "JPY 1,234");
    }

    #[test]
    fn test_money_deserialize_from_f64() {
        let money = Money::<USD>::new(1234.56_f64).unwrap();
        assert_eq!(money.code(), "USD");
    }

    #[test]
    fn test_money_deserialize_from_integer() {
        let money = Money::<USD>::new(1234_i64).unwrap();
        assert_eq!(money.amount(), dec!(1234));
    }

    #[cfg(feature = "raw_money")]
    #[test]
    fn test_raw_money_serialize_as_string() {
        use crate::RawMoney;
        let raw = RawMoney::<USD>::from_decimal(dec!(1234.56789));
        assert_eq!(raw.to_string(), "USD 1,234.56789");
    }

    #[cfg(feature = "raw_money")]
    #[test]
    fn test_raw_money_deserialize_from_string() {
        use crate::RawMoney;
        let raw = RawMoney::<USD>::from_str("USD 1,234.56789").unwrap();
        assert_eq!(raw.amount(), dec!(1234.56789));
    }
}