/// Creates a [`Money`] instance using a currency type and a decimal amount.
///
/// **Short form (ISO currencies):** pass a bare ISO 4217 currency code — it is resolved from
/// [`crate::iso`] automatically, so no separate `use` import is required.
///
/// **Long form (custom currencies):** pass any path that resolves to a type implementing
/// [`Currency`](crate::Currency). The path is used directly, so the type must be in scope.
///
/// The amount is passed to [`dec!`](rust_decimal::dec) and then wrapped in a [`Money`] value,
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
        $crate::Money::<$crate::iso::$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
    // Long form: explicit path for custom currency types (must be in scope)
    ($currency:path, $($amount:tt)+) => {
        $crate::Money::<$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
}

/// Creates a [`RawMoney`] instance using a currency type and a decimal amount.
///
/// **Short form (ISO currencies):** pass a bare ISO 4217 currency code — it is resolved from
/// [`crate::iso`] automatically, so no separate `use` import is required.
///
/// **Long form (custom currencies):** pass any path that resolves to a type implementing
/// [`Currency`](crate::Currency). The path is used directly, so the type must be in scope.
///
/// The amount is passed to [`dec!`](rust_decimal::dec) without any rounding, preserving
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
        $crate::RawMoney::<$crate::iso::$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
    // Long form: explicit path for custom currency types (must be in scope)
    ($currency:path, $($amount:tt)+) => {
        $crate::RawMoney::<$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
}

pub use rust_decimal::dec;
pub use crate::money;
#[cfg(feature = "raw_money")]
pub use crate::raw;
