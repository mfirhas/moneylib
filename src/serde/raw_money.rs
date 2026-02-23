use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{BaseMoney, Currency, Decimal, RawMoney};

// ---------------------------------------------------------------------------
// Default: Serialize/Deserialize as precise number
// ---------------------------------------------------------------------------

impl<C: Currency + Clone> Serialize for RawMoney<C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let n = serde_json::Number::from_str(&self.amount().to_string())
            .map_err(|_| ::serde::ser::Error::custom("cannot convert Decimal to JSON Number"))?;
        n.serialize(serializer)
    }
}

struct RawMoneyVisitor<C>(PhantomData<C>);

impl<'de, C: Currency + Clone> de::Visitor<'de> for RawMoneyVisitor<C> {
    type Value = RawMoney<C>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a number")
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

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Decimal::from_str(v)
            .map(|d| RawMoney::<C>::from_decimal(d))
            .map_err(|_| de::Error::custom(format!("invalid decimal: {}", v)))
    }

    // Handles serde_json's arbitrary_precision number format
    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let key: String = map
            .next_key()?
            .ok_or_else(|| de::Error::custom("expected number token, got empty map"))?;
        if key == "$serde_json::private::Number" {
            let value: String = map.next_value()?;
            let d = Decimal::from_str(&value)
                .map_err(|_| de::Error::custom(format!("invalid decimal: {}", value)))?;
            Ok(RawMoney::<C>::from_decimal(d))
        } else {
            Err(de::Error::custom(format!("unexpected key in map: {}", key)))
        }
    }
}

impl<'de, C: Currency + Clone> Deserialize<'de> for RawMoney<C> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(RawMoneyVisitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// comma_str: serialize/deserialize as "USD 1,234.56789"
// ---------------------------------------------------------------------------

/// Serialize/deserialize `RawMoney<C>` as a string with comma thousands separator.
///
/// Format: `"CCC 1,234.56789"`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::comma_str")]
/// amount: RawMoney<USD>,
/// ```
pub mod comma_str {
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &RawMoney<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let amount_str = format_decimal_abs(value.amount(), ",", ".", C::MINOR_UNIT);
        let neg = if value.is_negative() { "-" } else { "" };
        serializer.serialize_str(&format!("{} {}{}", C::CODE, neg, amount_str))
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'USD 1,234.56789'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            RawMoney::<C>::from_str(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RawMoney<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_comma_str: serialize/deserialize as "USD 1,234.56789" or null
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<RawMoney<C>>` as a string with comma thousands separator.
///
/// Format: `"CCC 1,234.56789"` or `null`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::option_comma_str")]
/// amount: Option<RawMoney<USD>>,
/// ```
pub mod option_comma_str {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<RawMoney<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(m) => {
                let amount_str = format_decimal_abs(m.amount(), ",", ".", C::MINOR_UNIT);
                let neg = if m.is_negative() { "-" } else { "" };
                serializer.serialize_some(format!("{} {}{}", C::CODE, neg, amount_str).as_str())
            }
            None => serializer.serialize_none(),
        }
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Option<RawMoney<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'USD 1,234.56789' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::comma_str::deserialize(d).map(Some)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RawMoney<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// comma_str_code: serialize/deserialize as "USD 1,234.56789" using format_code()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `RawMoney<C>` as a string with currency code prefix,
/// using the currency's natural thousands/decimal separators.
///
/// Uses [`BaseMoney::format_code`] for serialization (e.g. `"USD 1,234.56789"`).
/// Deserializes via comma thousands separator parser.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::comma_str_code")]
/// amount: RawMoney<USD>,
/// ```
pub mod comma_str_code {
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &RawMoney<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.format_code())
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'USD 1,234.56789'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            RawMoney::<C>::from_str(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RawMoney<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_comma_str_code: optional variant of comma_str_code
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<RawMoney<C>>` using [`comma_str_code`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::option_comma_str_code")]
/// amount: Option<RawMoney<USD>>,
/// ```
pub mod option_comma_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<RawMoney<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(m) => serializer.serialize_some(m.format_code().as_str()),
            None => serializer.serialize_none(),
        }
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Option<RawMoney<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'USD 1,234.56789' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::comma_str_code::deserialize(d).map(Some)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RawMoney<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// comma_str_symbol: serialize/deserialize as "$1,234.56789" using format_symbol()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `RawMoney<C>` as a string with currency symbol prefix,
/// using the currency's natural thousands/decimal separators.
///
/// Uses [`BaseMoney::format_symbol`] for serialization (e.g. `"$1,234.56789"`).
/// Deserializes by stripping the symbol and parsing with comma thousands separator.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::comma_str_symbol")]
/// amount: RawMoney<USD>,
/// ```
pub mod comma_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, Decimal, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &RawMoney<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.format_symbol())
    }

    fn parse<C: Currency + Clone, E: de::Error>(s: &str) -> Result<RawMoney<C>, E> {
        let (neg, rest) = if let Some(r) = s.strip_prefix('-') {
            (true, r)
        } else {
            (false, s)
        };
        let rest = rest
            .strip_prefix(C::SYMBOL)
            .ok_or_else(|| E::custom(format!("expected currency symbol '{}'", C::SYMBOL)))?;
        let amount_str = rest.replace(',', "");
        Decimal::from_str(&amount_str)
            .map(|d| RawMoney::<C>::from_decimal(if neg { -d } else { d }))
            .map_err(|_| E::custom(format!("invalid decimal: {}", amount_str)))
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '$1,234.56789'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            parse::<C, E>(v)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RawMoney<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_comma_str_symbol: optional variant of comma_str_symbol
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<RawMoney<C>>` using [`comma_str_symbol`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::option_comma_str_symbol")]
/// amount: Option<RawMoney<USD>>,
/// ```
pub mod option_comma_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<RawMoney<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(m) => serializer.serialize_some(m.format_symbol().as_str()),
            None => serializer.serialize_none(),
        }
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Option<RawMoney<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '$1,234.56789' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::comma_str_symbol::deserialize(d).map(Some)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RawMoney<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// dot_str: serialize/deserialize as "EUR 1.234,56789"
// ---------------------------------------------------------------------------

/// Serialize/deserialize `RawMoney<C>` as a string with dot thousands separator.
///
/// Format: `"CCC 1.234,56789"`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::dot_str")]
/// amount: RawMoney<EUR>,
/// ```
pub mod dot_str {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &RawMoney<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let amount_str = format_decimal_abs(value.amount(), ".", ",", C::MINOR_UNIT);
        let neg = if value.is_negative() { "-" } else { "" };
        serializer.serialize_str(&format!("{} {}{}", C::CODE, neg, amount_str))
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'EUR 1.234,56789'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            RawMoney::<C>::from_str_dot_thousands(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RawMoney<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_dot_str: serialize/deserialize as "EUR 1.234,56789" or null
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<RawMoney<C>>` as a string with dot thousands separator.
///
/// Format: `"CCC 1.234,56789"` or `null`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::option_dot_str")]
/// amount: Option<RawMoney<EUR>>,
/// ```
pub mod option_dot_str {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<RawMoney<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(m) => {
                let amount_str = format_decimal_abs(m.amount(), ".", ",", C::MINOR_UNIT);
                let neg = if m.is_negative() { "-" } else { "" };
                serializer.serialize_some(format!("{} {}{}", C::CODE, neg, amount_str).as_str())
            }
            None => serializer.serialize_none(),
        }
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Option<RawMoney<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'EUR 1.234,56789' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::dot_str::deserialize(d).map(Some)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RawMoney<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// dot_str_code: serialize/deserialize as "EUR 1.234,56789" using format_code()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `RawMoney<C>` as a string with currency code prefix,
/// using the currency's natural thousands/decimal separators.
///
/// Uses [`BaseMoney::format_code`] for serialization (e.g. `"EUR 1.234,56789"`).
/// Deserializes via dot thousands separator parser.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::dot_str_code")]
/// amount: RawMoney<EUR>,
/// ```
pub mod dot_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &RawMoney<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.format_code())
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'EUR 1.234,56789'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            RawMoney::<C>::from_str_dot_thousands(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RawMoney<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_dot_str_code: optional variant of dot_str_code
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<RawMoney<C>>` using [`dot_str_code`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::option_dot_str_code")]
/// amount: Option<RawMoney<EUR>>,
/// ```
pub mod option_dot_str_code {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<RawMoney<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(m) => serializer.serialize_some(m.format_code().as_str()),
            None => serializer.serialize_none(),
        }
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Option<RawMoney<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'EUR 1.234,56789' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::dot_str_code::deserialize(d).map(Some)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RawMoney<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// dot_str_symbol: serialize/deserialize as "€1.234,56789" using format_symbol()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `RawMoney<C>` as a string with currency symbol prefix,
/// using the currency's natural thousands/decimal separators.
///
/// Uses [`BaseMoney::format_symbol`] for serialization (e.g. `"€1.234,56789"`).
/// Deserializes by stripping the symbol and parsing with dot thousands separator.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::dot_str_symbol")]
/// amount: RawMoney<EUR>,
/// ```
pub mod dot_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, Decimal, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &RawMoney<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.format_symbol())
    }

    fn parse<C: Currency + Clone, E: de::Error>(s: &str) -> Result<RawMoney<C>, E> {
        let (neg, rest) = if let Some(r) = s.strip_prefix('-') {
            (true, r)
        } else {
            (false, s)
        };
        let rest = rest
            .strip_prefix(C::SYMBOL)
            .ok_or_else(|| E::custom(format!("expected currency symbol '{}'", C::SYMBOL)))?;
        // Remove dot thousands separators; replace comma decimal with dot
        let amount_str = rest.replace('.', "").replace(',', ".");
        Decimal::from_str(&amount_str)
            .map(|d| RawMoney::<C>::from_decimal(if neg { -d } else { d }))
            .map_err(|_| E::custom(format!("invalid decimal: {}", amount_str)))
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = RawMoney<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '€1.234,56789'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            parse::<C, E>(v)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RawMoney<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_dot_str_symbol: optional variant of dot_str_symbol
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<RawMoney<C>>` using [`dot_str_symbol`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::raw_money::option_dot_str_symbol")]
/// amount: Option<RawMoney<EUR>>,
/// ```
pub mod option_dot_str_symbol {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::{BaseMoney, Currency, RawMoney};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<RawMoney<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(m) => serializer.serialize_some(m.format_symbol().as_str()),
            None => serializer.serialize_none(),
        }
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Option<RawMoney<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like '€1.234,56789' or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            super::dot_str_symbol::deserialize(d).map(Some)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RawMoney<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}
