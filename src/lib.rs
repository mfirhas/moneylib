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
pub mod money_macros {
    pub use rust_decimal::dec;
}

mod base;
pub use base::{BaseMoney, BaseOps, CustomMoney, RoundingStrategy};

mod error;
pub use error::MoneyError;

pub use currencylib::*;

mod money;
pub use money::Money;

mod dec_ops;
mod ops;

#[cfg(feature = "raw_money")]
mod raw_money;
#[cfg(feature = "raw_money")]
pub use raw_money::RawMoney;

mod serde;

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
