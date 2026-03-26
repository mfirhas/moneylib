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

/// Helper used by the `money!`, `raw!`, and `dec!` macros to construct a [`Decimal`] from a string
/// without requiring `rust_decimal` to be a direct dependency of the caller's crate.
///
/// # Panics
///
/// Panics if `s` is not a valid decimal literal.  In practice this can only happen when the
/// macro is invoked with something that is not a numeric literal.
#[doc(hidden)]
pub fn __parse_decimal(s: &str) -> Decimal {
    s.trim().trim_end_matches(',').parse().expect("invalid decimal literal passed to money!/raw!/dec! macro")
}

/// Contains helper macros.
pub mod macros;

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
mod iter_ops;
mod ops;
mod percent_ops;
pub use percent_ops::PercentOps;

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

#[cfg(feature = "accounting")]
/// Accounting module
pub mod accounting;

mod fmt;

mod parse;

#[cfg(feature = "accounting")]
mod calendar;

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

#[cfg(test)]
mod percent_ops_test;

#[cfg(all(test, feature = "exchange"))]
mod exchange_test;

#[cfg(all(test, feature = "accounting"))]
mod calendar_test;
