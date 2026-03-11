#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(clippy::float_arithmetic)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(clippy::cast_possible_truncation)]
#![forbid(clippy::cast_sign_loss)]
#![forbid(clippy::cast_possible_wrap)]
#![forbid(clippy::unwrap_used)]

pub use rust_decimal::Decimal;

/// Creates a [`Money`] instance with the given currency type and decimal amount.
///
/// The amount is passed to [`dec!`] (from [`rust_decimal`]) and then wrapped in a
/// [`Money`] value, applying the currency's rounding rules.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, iso::USD, macros::{dec, money}};
///
/// // Basic usage
/// let m = money!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.24)); // rounded to 2 decimal places for USD
///
/// // Negative amounts
/// let m = money!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.00)); // banker's rounding
/// ```
#[macro_export]
macro_rules! money {
    ($currency:ty, $($amount:tt)+) => {
        $crate::Money::<$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
}

/// Creates a [`RawMoney`] instance with the given currency type and decimal amount.
///
/// The amount is passed to [`dec!`] (from [`rust_decimal`]) without any rounding,
/// preserving the full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, iso::USD, macros::{dec, raw_money}};
///
/// // Basic usage – no rounding applied
/// let m = raw_money!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.237));
///
/// // Negative amounts
/// let m = raw_money!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.005));
/// ```
#[cfg(feature = "raw_money")]
#[macro_export]
macro_rules! raw_money {
    ($currency:ty, $($amount:tt)+) => {
        $crate::RawMoney::<$currency>::from_decimal($crate::macros::dec!($($amount)+))
    };
}

/// Contains helper macros.
pub mod macros {
    pub use rust_decimal::dec;
    pub use crate::money;
    #[cfg(feature = "raw_money")]
    pub use crate::raw_money;
}

mod base;
pub use base::{BaseMoney, BaseOps, CustomMoney, IterOps, RoundingStrategy};

mod error;
pub use error::MoneyError;

pub use currencylib::Currency;

/// Contains all ISO 4217 currencies.
pub mod iso {
    pub use currencylib::*;
}

mod money;
pub use money::Money;

mod dec_ops;
mod ops;

#[cfg(feature = "raw_money")]
mod raw_money;
#[cfg(feature = "raw_money")]
pub use raw_money::RawMoney;

#[cfg(feature = "serde")]
/// Serde implementations
pub mod serde;

#[cfg(feature = "exchange")]
mod exchange;
#[cfg(feature = "exchange")]
pub use exchange::{Exchange, ExchangeRates};

mod fmt;

mod parse;

#[cfg(test)]
mod parse_test;

#[cfg(test)]
mod fmt_test;

#[cfg(test)]
mod money_test;

#[cfg(test)]
mod error_test;

#[cfg(test)]
mod ops_test;

#[cfg(test)]
mod iter_ops_test;

#[cfg(all(test, feature = "exchange"))]
mod exchange_test;
