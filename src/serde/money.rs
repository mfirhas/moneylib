use std::fmt;
use std::marker::PhantomData;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use rust_decimal::prelude::ToPrimitive;

use crate::{BaseMoney, Currency, Money};

// ---------------------------------------------------------------------------
// Default: Serialize/Deserialize as number (f64)
// ---------------------------------------------------------------------------

impl<C: Currency + Clone> Serialize for Money<C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(
            self.amount()
                .to_f64()
                .ok_or_else(|| ::serde::ser::Error::custom("Decimal cannot be converted to f64"))?,
        )
    }
}

struct MoneyVisitor<C>(PhantomData<C>);

impl<'de, C: Currency + Clone> de::Visitor<'de> for MoneyVisitor<C> {
    type Value = Money<C>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a number")
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

// ---------------------------------------------------------------------------
// comma_str: serialize/deserialize as "USD 1,234.56"
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a string with comma thousands separator.
///
/// Format: `"CCC 1,234.56"`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::comma_str")]
/// amount: Money<USD>,
/// ```
pub mod comma_str {
    use std::fmt;
    use std::marker::PhantomData;
    use std::str::FromStr;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, Money};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let amount_str = format_decimal_abs(value.amount(), ",", ".", C::MINOR_UNIT);
        let neg = if value.is_negative() { "-" } else { "" };
        serializer.serialize_str(&format!("{} {}{}", C::CODE, neg, amount_str))
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Money<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'USD 1,234.56'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            Money::<C>::from_str(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_comma_str: serialize/deserialize as "USD 1,234.56" or null
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<Money<C>>` as a string with comma thousands separator.
///
/// Format: `"CCC 1,234.56"` or `null`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_comma_str")]
/// amount: Option<Money<USD>>,
/// ```
pub mod option_comma_str {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, Money};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<Money<C>>,
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
        type Value = Option<Money<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'USD 1,234.56' or null")
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
    ) -> Result<Option<Money<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// dot_str: serialize/deserialize as "EUR 1.234,56"
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a string with dot thousands separator.
///
/// Format: `"CCC 1.234,56"`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::dot_str")]
/// amount: Money<EUR>,
/// ```
pub mod dot_str {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, Money};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let amount_str = format_decimal_abs(value.amount(), ".", ",", C::MINOR_UNIT);
        let neg = if value.is_negative() { "-" } else { "" };
        serializer.serialize_str(&format!("{} {}{}", C::CODE, neg, amount_str))
    }

    struct Visitor<C>(PhantomData<C>);

    impl<'de, C: Currency + Clone> de::Visitor<'de> for Visitor<C> {
        type Value = Money<C>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'EUR 1.234,56'")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            Money::<C>::from_str_dot_thousands(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, C: Currency + Clone, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

// ---------------------------------------------------------------------------
// option_dot_str: serialize/deserialize as "EUR 1.234,56" or null
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<Money<C>>` as a string with dot thousands separator.
///
/// Format: `"CCC 1.234,56"` or `null`
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_dot_str")]
/// amount: Option<Money<EUR>>,
/// ```
pub mod option_dot_str {
    use std::fmt;
    use std::marker::PhantomData;

    use ::serde::{Deserializer, Serializer, de};

    use crate::fmt::format_decimal_abs;
    use crate::{BaseMoney, Currency, Money};

    pub fn serialize<C: Currency + Clone, S: Serializer>(
        value: &Option<Money<C>>,
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
        type Value = Option<Money<C>>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a string like 'EUR 1.234,56' or null")
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
    ) -> Result<Option<Money<C>>, D::Error> {
        deserializer.deserialize_option(Visitor(PhantomData))
    }
}
