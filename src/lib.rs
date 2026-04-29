#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![forbid(clippy::float_arithmetic)]
#![forbid(clippy::float_cmp)]
#![forbid(clippy::as_conversions)]
#![forbid(clippy::cast_possible_truncation)]
#![forbid(clippy::cast_sign_loss)]
#![forbid(clippy::cast_possible_wrap)]
#![forbid(clippy::unwrap_used)]

/// Contains all types and traits of moneylib.
pub mod prelude {
    pub use crate::BaseMoney;
    pub use crate::BaseOps;
    pub use crate::Currency;
    pub use crate::IterOps;
    pub use crate::MoneyFormatter;
    pub use crate::MoneyOps;
    pub use crate::PercentOps;
    pub use crate::RoundingStrategy;
    pub use crate::{Decimal, Money, MoneyError};

    pub use crate::iso;

    pub use crate::macros::{dec, money};

    #[cfg(feature = "raw_money")]
    pub use crate::RawMoney;
    #[cfg(feature = "raw_money")]
    pub use crate::macros::raw;

    #[cfg(feature = "exchange")]
    pub use crate::exchange::{Exchange, ExchangeRates};

    #[cfg(feature = "obj_money")]
    pub use crate::ObjMoney;

    #[cfg(feature = "accounting")]
    pub use crate::AccountingOps;

    #[cfg(feature = "accounting")]
    pub use crate::accounting;

    #[cfg(feature = "serde")]
    pub use crate::serde;
}

// ------------------ MoneyOps contains all ops traits for money instance ------------------

#[cfg(not(any(feature = "exchange", feature = "accounting")))]
/// MoneyOps\<C\> trait contains all traits on money instance.
pub trait MoneyOps<C>: BaseOps<C> + MoneyFormatter<C> + PercentOps<C>
where
    C: Currency,
{
}

#[cfg(all(feature = "exchange", not(feature = "accounting")))]
/// MoneyOps\<C\> trait contains all traits on money instance.
pub trait MoneyOps<C>: BaseOps<C> + MoneyFormatter<C> + PercentOps<C> + Exchange<C>
where
    C: Currency,
{
}

#[cfg(feature = "accounting")]
/// Contains all ops traits inside accounting module
pub trait AccountingOps<C>: accounting::interest::InterestOps<C> {}

#[cfg(all(feature = "accounting", not(feature = "exchange")))]
/// MoneyOps\<C\> trait contains all traits on money instance.
pub trait MoneyOps<C>: BaseOps<C> + MoneyFormatter<C> + PercentOps<C> + AccountingOps<C>
where
    C: Currency,
{
}

#[cfg(all(feature = "exchange", feature = "accounting"))]
/// MoneyOps\<C\> trait contains all traits on money instance.
pub trait MoneyOps<C>:
    BaseOps<C> + MoneyFormatter<C> + PercentOps<C> + Exchange<C> + AccountingOps<C>
where
    C: Currency,
{
}

// -----------------------------------------------------------------------------------------

pub use rust_decimal::Decimal;

/// Contains helper macros.
pub mod macros;

mod base;
pub use base::{BaseMoney, BaseOps, IterOps, MoneyFormatter, RoundingStrategy};

mod error;
pub use error::MoneyError;

pub use currencylib::Currency;

/// Contains all ISO 4217 currencies.
pub mod iso {
    pub use currencylib::*;
}

mod money;
pub use money::Money;

#[cfg(feature = "raw_money")]
mod raw_money;
#[cfg(feature = "raw_money")]
pub use raw_money::RawMoney;

mod dec_ops;
mod iter_ops;
mod ops;
mod percent_ops;
pub use percent_ops::PercentOps;
mod split_alloc_ops;

#[cfg(feature = "exchange")]
mod exchange;
#[cfg(feature = "exchange")]
pub use exchange::{Exchange, ExchangeRates};

#[cfg(feature = "accounting")]
/// Accounting module
pub mod accounting;

#[cfg(feature = "accounting")]
mod calendar;

#[cfg(feature = "serde")]
/// Serde implementations
pub mod serde;

mod fmt;

mod parse;

#[cfg(feature = "obj_money")]
mod obj_money;
#[cfg(feature = "obj_money")]
pub use obj_money::{ObjIterOps, ObjMoney, ObjRate};

// ----------------- test modules -----------------

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

#[cfg(test)]
mod split_alloc_ops_test;

#[cfg(all(test, feature = "exchange"))]
mod exchange_test;

#[cfg(all(test, feature = "accounting"))]
mod calendar_test;
