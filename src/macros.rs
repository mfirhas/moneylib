/// Creates a [`Money`] instance using an ISO 4217 currency code and a decimal amount.
///
/// The currency code is resolved from [`crate::iso`] automatically, so no separate
/// `use` import is required. The amount is passed to [`dec!`](rust_decimal::dec) and
/// then wrapped in a [`Money`] value, applying the currency's rounding rules.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, macros::{dec, money}};
///
/// // No `use moneylib::iso::USD;` needed
/// let m = money!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.24)); // rounded to 2 decimal places for USD
///
/// // Negative amounts
/// let m = money!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.00)); // banker's rounding
/// ```
#[macro_export]
macro_rules! money {
    ($currency:ident, $($amount:tt)+) => {
        $crate::Money::<$crate::iso::$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
}

/// Creates a [`RawMoney`] instance using an ISO 4217 currency code and a decimal amount.
///
/// The currency code is resolved from [`crate::iso`] automatically, so no separate
/// `use` import is required. The amount is passed to [`dec!`](rust_decimal::dec) without
/// any rounding, preserving the full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, macros::{dec, raw}};
///
/// // No `use moneylib::iso::USD;` needed
/// let m = raw!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.237));
///
/// // Negative amounts
/// let m = raw!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.005));
/// ```
#[cfg(feature = "raw_money")]
#[macro_export]
macro_rules! raw {
    ($currency:ident, $($amount:tt)+) => {
        $crate::RawMoney::<$crate::iso::$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
}

pub use rust_decimal::dec;
pub use crate::money;
#[cfg(feature = "raw_money")]
pub use crate::raw;
