use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Currency, Money};

use super::base;

// ---------------------------------------------------------------------------
// Default: Serialize/Deserialize as precise number
// ---------------------------------------------------------------------------

impl<C: Currency> Serialize for Money<C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        base::serialize_as_number::<C, Money<C>, S>(self, serializer)
    }
}

impl<'de, C: Currency> Deserialize<'de> for Money<C> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        base::deserialize_as_number::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// comma_str_code: serialize/deserialize as "USD 1,234.56" using format_code()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a string with currency code prefix,
/// using comma as thousands separator and dot as decimal separator.
///
/// Uses [`crate::BaseMoney::format_code`] for serialization (e.g. `"USD 1,234.56"`).
/// Deserializes via comma thousands separator parser.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::comma_str_code")]
/// amount: Money<USD>,
/// ```
pub mod comma_str_code {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::comma_str_code::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::comma_str_code::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// option_comma_str_code: optional variant of comma_str_code
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<Money<C>>` using [`comma_str_code`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_comma_str_code")]
/// amount: Option<Money<USD>>,
/// ```
pub mod option_comma_str_code {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_comma_str_code::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_comma_str_code::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// comma_str_symbol: serialize/deserialize as "$1,234.56" using format_symbol()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a string with currency symbol prefix,
/// using comma as thousands separator and dot as decimal separator.
///
/// Uses [`crate::BaseMoney::format_symbol`] for serialization (e.g. `"$1,234.56"`).
/// Deserializes by stripping the symbol and parsing with comma thousands separator.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::comma_str_symbol")]
/// amount: Money<USD>,
/// ```
pub mod comma_str_symbol {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::comma_str_symbol::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::comma_str_symbol::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// option_comma_str_symbol: optional variant of comma_str_symbol
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<Money<C>>` using [`comma_str_symbol`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_comma_str_symbol")]
/// amount: Option<Money<USD>>,
/// ```
pub mod option_comma_str_symbol {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_comma_str_symbol::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_comma_str_symbol::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// dot_str_code: serialize/deserialize as "EUR 1.234,56" using format_code()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a string with currency code prefix,
/// using dot as thousands separator and comma as decimal separator.
///
/// Uses [`crate::BaseMoney::format_code`] for serialization (e.g. `"EUR 1.234,56"`).
/// Deserializes via dot thousands separator parser.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::dot_str_code")]
/// amount: Money<EUR>,
/// ```
pub mod dot_str_code {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::dot_str_code::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::dot_str_code::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// option_dot_str_code: optional variant of dot_str_code
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<Money<C>>` using [`dot_str_code`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_dot_str_code")]
/// amount: Option<Money<EUR>>,
/// ```
pub mod option_dot_str_code {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_dot_str_code::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_dot_str_code::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// dot_str_symbol: serialize/deserialize as "€1.234,56" using format_symbol()
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a string with currency symbol prefix,
/// using dot as thousands separator and comma as decimal separator.
///
/// Uses [`crate::BaseMoney::format_symbol`] for serialization (e.g. `"€1.234,56"`).
/// Deserializes by stripping the symbol and parsing with dot thousands separator.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::dot_str_symbol")]
/// amount: Money<EUR>,
/// ```
pub mod dot_str_symbol {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::dot_str_symbol::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::dot_str_symbol::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------
// option_dot_str_symbol: optional variant of dot_str_symbol
// ---------------------------------------------------------------------------

/// Serialize/deserialize `Option<Money<C>>` using [`dot_str_symbol`] format or `null`.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_dot_str_symbol")]
/// amount: Option<Money<EUR>>,
/// ```
pub mod option_dot_str_symbol {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_dot_str_symbol::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_dot_str_symbol::deserialize::<C, Money<C>, D>(deserializer)
    }
}

/// Serialize/deserialize money as string with code formatting like `CCC amount`.
/// The separators used are from currency's locale separator.
///
/// Uses [`crate::BaseMoney::format_code`] for serialization (e.g. `"USD 1,234.56"` or `"CHF 1'234.56"`).
/// Deserializes via [`crate::MoneyParser::from_str_code`].
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::str_code")]
/// amount: Money<USD>,
/// ```
pub mod str_code {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::str_code::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::str_code::deserialize::<C, Money<C>, D>(deserializer)
    }
}

/// Serialize/deserialize *nullable* money as string with code formatting like `CCC amount`.
/// The separators used are from currency's locale separator.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_str_code")]
/// amount: Option<Money<USD>>,
/// ```
pub mod option_str_code {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_str_code::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_str_code::deserialize::<C, Money<C>, D>(deserializer)
    }
}

/// Serialize/deserialize money as string with symbol formatting like `S<amount>`.
/// The separators used are from currency's locale separator.
///
/// Uses [`crate::BaseMoney::format_symbol`] for serialization (e.g. `"$1,234.56"` or `"₣1'234.56"`).
/// Deserializes via [`crate::MoneyParser::from_str_symbol`].
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::str_symbol")]
/// amount: Money<USD>,
/// ```
pub mod str_symbol {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::str_symbol::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::str_symbol::deserialize::<C, Money<C>, D>(deserializer)
    }
}

/// Serialize/deserialize *nullable* money as string with symbol formatting like `S<amount>`.
/// The separators used are from currency's locale separator.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_str_symbol")]
/// amount: Option<Money<USD>>,
/// ```
pub mod option_str_symbol {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_str_symbol::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_str_symbol::deserialize::<C, Money<C>, D>(deserializer)
    }
}

// ---------------------------------------------------------------------------------
// minor: serialize/deserialize as minor amount, e.g. USD 1,234.56 -> 123456
// ---------------------------------------------------------------------------------

/// Serialize/deserialize `Money<C>` as a JSON Number of its minor amount.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::minor")]
/// amount: Money<USD>,
/// ```
pub mod minor {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Money<C>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::minor::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Money<C>, D::Error> {
        base::minor::deserialize::<C, Money<C>, D>(deserializer)
    }
}

/// Serialize/deserialize `Option<Money<C>>` as a JSON Number of its minor amount.
///
/// # Usage
///
/// ```ignore
/// #[serde(with = "moneylib::serde::money::option_minor")]
/// amount: Option<Money<USD>>,
/// ```
pub mod option_minor {

    use ::serde::{Deserializer, Serializer};

    use crate::{Currency, Money};

    use crate::serde::base;

    pub fn serialize<C: Currency, S: Serializer>(
        value: &Option<Money<C>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        base::option_minor::serialize::<C, Money<C>, S>(value, serializer)
    }

    pub fn deserialize<'de, C: Currency, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Money<C>>, D::Error> {
        base::option_minor::deserialize::<C, Money<C>, D>(deserializer)
    }
}
