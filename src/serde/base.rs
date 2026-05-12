//! Shared serde building blocks for `Money<C>` and `RawMoney<C>`.
//!
//! This module provides generic serialize/deserialize helpers parameterised over
//! any `M` that implements [`BaseMoney<C>`] (plus formatter / parser traits as
//! appropriate).  The concrete `money` and `raw_money` modules are thin wrappers
//! that delegate to these helpers.

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use ::serde::{Deserializer, Serialize, Serializer, de};

use crate::{BaseMoney, Currency, Decimal, MoneyParser};

// ---------------------------------------------------------------------------
// Default: Serialize/Deserialize as precise number
// ---------------------------------------------------------------------------

/// Serialize any `BaseMoney<C>` implementation as a JSON precise number.
pub fn serialize_as_number<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
where
    C: Currency,
    M: BaseMoney<C>,
    S: Serializer,
{
    let n = serde_json::Number::from_str(&value.amount().to_string())
        .map_err(|_| ::serde::ser::Error::custom("cannot convert Decimal to JSON Number"))?;
    n.serialize(serializer)
}

/// Visitor used for the default (number) deserialization of any `BaseMoney<C>`.
pub struct BaseMoneyVisitor<M, C>(pub PhantomData<(M, C)>);

impl<'de, C, M> de::Visitor<'de> for BaseMoneyVisitor<M, C>
where
    C: Currency,
    M: BaseMoney<C> + MoneyParser<C>,
{
    type Value = M;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a number")
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        self.visit_str(&v.to_string())
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        M::new(v).map_err(de::Error::custom)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        M::new(i128::from(v)).map_err(de::Error::custom)
    }

    fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
        M::new(v).map_err(de::Error::custom)
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
        i128::try_from(v)
            .map_err(|_| {
                de::Error::custom(format!(
                    "value too large for {}",
                    std::any::type_name::<M>()
                ))
            })
            .and_then(|n| M::new(n).map_err(de::Error::custom))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        M::from_str(v).map_err(|_| de::Error::custom(format!("invalid decimal: {}", v)))
    }

    // Handles serde_json's arbitrary_precision number format
    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        const ARBITRARY_NUMBER_KEY: &str = "$serde_json::private::Number";

        if let Ok(Some(key)) = map.next_key::<String>()
            && key == ARBITRARY_NUMBER_KEY
        {
            let value: String = map.next_value()?;
            let d = Decimal::from_str(&value)
                .map_err(|_| de::Error::custom(format!("invalid decimal: {}", value)))?;
            Ok(M::from_decimal(d))
        } else {
            Err(de::Error::custom("unexpected key"))
        }
    }
}

/// Deserialize any `BaseMoney<C>` + `MoneyParser<C>` implementation from a JSON number.
pub fn deserialize_as_number<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
where
    C: Currency,
    M: BaseMoney<C> + MoneyParser<C>,
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BaseMoneyVisitor::<M, C>(PhantomData))
}

// ---------------------------------------------------------------------------
// comma_str_code: serialize/deserialize as "USD 1,234.56" using format_code()
// ---------------------------------------------------------------------------

pub mod comma_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: MoneyFormatter<C>,
        S: Serializer,
    {
        serializer.serialize_str(&value.format_with_separator("c na", ",", "."))
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'CCC 1,234.56'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            M::from_str_code_with(v, ",", ".").map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_comma_str_code: optional variant of comma_str_code
// ---------------------------------------------------------------------------

pub mod option_comma_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        match value {
            Some(m) => serializer.serialize_some(m.format_code().as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'CCC 1,234.56' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::comma_str_code::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// comma_str_symbol: serialize/deserialize as "$1,234.56" using format_symbol()
// ---------------------------------------------------------------------------

pub mod comma_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: MoneyFormatter<C>,
        S: Serializer,
    {
        serializer.serialize_str(&value.format_with_separator("nsa", ",", "."))
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '$1,234.56'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            M::from_str_symbol_with(v, ",", ".").map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_comma_str_symbol: optional variant of comma_str_symbol
// ---------------------------------------------------------------------------

pub mod option_comma_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        match value {
            Some(m) => serializer.serialize_some(m.format_symbol().as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '$1,234.56' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::comma_str_symbol::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// dot_str_code: serialize/deserialize as "EUR 1.234,56" using format_code()
// ---------------------------------------------------------------------------

pub mod dot_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: MoneyFormatter<C>,
        S: Serializer,
    {
        serializer.serialize_str(&value.format_with_separator("c na", ".", ","))
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'CCC 1.234,56'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            M::from_str_code_with(v, ".", ",").map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_dot_str_code: optional variant of dot_str_code
// ---------------------------------------------------------------------------

pub mod option_dot_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        match value {
            Some(m) => serializer.serialize_some(m.format_code().as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'CCC 1.234,56' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::dot_str_code::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// dot_str_symbol: serialize/deserialize as "€1.234,56" using format_symbol()
// ---------------------------------------------------------------------------

pub mod dot_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: MoneyFormatter<C>,
        S: Serializer,
    {
        serializer.serialize_str(&value.format_with_separator("nsa", ".", ","))
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '€1.234,56'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            M::from_str_symbol_with(v, ".", ",").map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_dot_str_symbol: optional variant of dot_str_symbol
// ---------------------------------------------------------------------------

pub mod option_dot_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        match value {
            Some(m) => serializer.serialize_some(m.format_symbol().as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '€1.234,56' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::dot_str_symbol::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// str_code: serialize/deserialize using currency locale separators (code)
// ---------------------------------------------------------------------------

pub mod str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        serializer.serialize_str(&value.format_code())
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'CCC amount' with locale separators")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            M::from_str_code(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_str_code: optional variant of str_code
// ---------------------------------------------------------------------------

pub mod option_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        match value {
            Some(m) => serializer.serialize_some(m.format_code().as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'CCC amount' with locale separators, or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::str_code::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// str_symbol: serialize/deserialize using currency locale separators (symbol)
// ---------------------------------------------------------------------------

pub mod str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        serializer.serialize_str(&value.format_symbol())
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'S<amount>' with locale separators")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            M::from_str_symbol(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_str_symbol: optional variant of str_symbol
// ---------------------------------------------------------------------------

pub mod option_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyFormatter, MoneyParser};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C> + MoneyFormatter<C>,
        S: Serializer,
    {
        match value {
            Some(m) => serializer.serialize_some(m.format_symbol().as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: MoneyParser<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'S<amount>' with locale separators, or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::str_symbol::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: MoneyParser<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// minor: serialize/deserialize as minor amount (integer)
// ---------------------------------------------------------------------------

pub mod minor {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, MoneyError};

    pub fn serialize<C, M, S>(value: &M, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C>,
        S: Serializer,
    {
        let minor = value
            .minor_amount()
            .ok_or(::serde::ser::Error::custom(MoneyError::OverflowError))?;
        serializer.serialize_i128(minor)
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: BaseMoney<C>,
    {
        type Value = M;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an integer representing the minor amount")
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            M::from_minor(i128::from(v)).map_err(de::Error::custom)
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            M::from_minor(i128::from(v)).map_err(de::Error::custom)
        }

        fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
            M::from_minor(v).map_err(de::Error::custom)
        }

        fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
            i128::try_from(v)
                .map_err(|_| de::Error::custom("value too large for minor amount"))
                .and_then(|n| M::from_minor(n).map_err(de::Error::custom))
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        C: Currency,
        M: BaseMoney<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor::<M, C>(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_minor: optional variant of minor
// ---------------------------------------------------------------------------

pub mod option_minor {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency};

    pub fn serialize<C, M, S>(value: &Option<M>, serializer: S) -> Result<S::Ok, S::Error>
    where
        C: Currency,
        M: BaseMoney<C>,
        S: Serializer,
    {
        match value {
            Some(m) => super::minor::serialize::<C, M, S>(m, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub struct Visitor<M, C>(pub PhantomData<(M, C)>);

    impl<'de, C, M> de::Visitor<'de> for Visitor<M, C>
    where
        C: Currency,
        M: BaseMoney<C>,
    {
        type Value = Option<M>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an integer representing the minor amount, or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::minor::deserialize::<C, M, D>(d).map(Some)
        }
    }

    pub fn deserialize<'de, C, M, D>(deserializer: D) -> Result<Option<M>, D::Error>
    where
        C: Currency,
        M: BaseMoney<C>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(Visitor::<M, C>(PhantomData))
    }
}
