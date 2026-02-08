#![forbid(unsafe_code)]

pub use iso_currency::Country;
pub use rust_decimal::Decimal;
pub mod money_macros {
    pub use rust_decimal_macros::dec;
}

mod base;
pub use base::{BaseMoney, BaseOps, CustomMoney, RoundingStrategy};

mod error;
pub use error::MoneyError;

/// Money result type
pub type MoneyResult<T> = Result<T, MoneyError>;

mod money;
pub use money::Money;

mod dec_ops;
mod ops;

mod currency;
pub use currency::Currency;

#[cfg(test)]
mod currency_test;

#[cfg(test)]
mod money_test;
