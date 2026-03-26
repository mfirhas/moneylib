/// Creates a [`Money`](crate::Money) instance using a currency type and a decimal amount.
///
/// **Short form (ISO currencies):** pass a bare ISO 4217 currency code — it is resolved from
/// [`crate::iso`] automatically, so no separate `use` import is required.
///
/// **Long form (custom currencies):** pass any path that resolves to a type implementing
/// [`Currency`](crate::Currency). The path is used directly, so the type must be in scope.
///
/// The amount is parsed as a decimal string at initialization time and then wrapped in a [`Money`](crate::Money) value,
/// applying the currency's rounding rules.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, macros::{dec, money}};
///
/// // Short form: no `use moneylib::iso::USD;` needed
/// let m = money!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.24)); // rounded to 2 decimal places for USD
///
/// // Negative amounts
/// let m = money!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.00)); // banker's rounding
/// ```
///
/// ```
/// use moneylib::{BaseMoney, Currency, macros::{dec, money}, iso::USD};
///
/// // Long form: path to a custom currency type (must be in scope)
/// let m = money!(USD, 100.00);
/// assert_eq!(m.amount(), dec!(100.00));
/// ```
#[macro_export]
macro_rules! money {
    // Short form: bare ISO currency identifier, auto-resolved from crate::iso
    ($currency:ident, $($amount:tt)+) => {
        $crate::Money::<$crate::iso::$currency>::from_decimal($crate::dec!($($amount)+))
    };
    // Long form: explicit path for custom currency types (must be in scope)
    ($currency:path, $($amount:tt)+) => {
        $crate::Money::<$currency>::from_decimal($crate::dec!($($amount)+))
    };
}

/// Creates a [`RawMoney`](crate::RawMoney) instance using a currency type and a decimal amount.
///
/// **Short form (ISO currencies):** pass a bare ISO 4217 currency code — it is resolved from
/// [`crate::iso`] automatically, so no separate `use` import is required.
///
/// **Long form (custom currencies):** pass any path that resolves to a type implementing
/// [`Currency`](crate::Currency). The path is used directly, so the type must be in scope.
///
/// The amount is parsed as a decimal string at initialization time without any rounding, preserving
/// the full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, macros::{dec, raw}};
///
/// // Short form: no `use moneylib::iso::USD;` needed
/// let m = raw!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.237));
///
/// // Negative amounts
/// let m = raw!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.005));
/// ```
///
/// ```
/// use moneylib::{BaseMoney, Currency, macros::{dec, raw}, iso::USD};
///
/// // Long form: path to a custom currency type (must be in scope)
/// let m = raw!(USD, 100.123);
/// assert_eq!(m.amount(), dec!(100.123));
/// ```
#[cfg(feature = "raw_money")]
#[macro_export]
macro_rules! raw {
    // Short form: bare ISO currency identifier, auto-resolved from crate::iso
    ($currency:ident, $($amount:tt)+) => {
        $crate::RawMoney::<$crate::iso::$currency>::from_decimal($crate::dec!($($amount)+))
    };
    // Long form: explicit path for custom currency types (must be in scope)
    ($currency:path, $($amount:tt)+) => {
        $crate::RawMoney::<$currency>::from_decimal($crate::dec!($($amount)+))
    };
}

/// Creates a [`Decimal`](crate::Decimal) value from a numeric literal without requiring
/// `rust_decimal` to be a direct dependency of the caller's crate.
///
/// # Examples
///
/// ```
/// use moneylib::macros::dec;
///
/// let d = dec!(2.51);
/// assert_eq!(d.to_string(), "2.51");
/// ```
#[macro_export]
macro_rules! dec {
    ($($amount:tt)+) => {
        $crate::__parse_decimal(concat!($(stringify!($amount)),+))
    };
}

pub use crate::money;

#[cfg(feature = "raw_money")]
pub use crate::raw;

pub use crate::dec;
