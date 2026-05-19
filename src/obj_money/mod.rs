//! Runtime-validated money types and trait along with currency.

mod context;
pub use context::Context;

mod fmt;

#[allow(clippy::module_inception)]
mod obj_money;
pub use obj_money::{ObjIterOps, ObjMoney};

mod dyn_money;
pub use dyn_money::{DynCurrency, DynMoney};

mod ops;

mod money_impl;

#[cfg(feature = "raw_money")]
mod raw_money_impl;

#[cfg(test)]
mod obj_money_test;

mod helpers {
    /// get the amount rounded or not depends on Context's config.
    #[inline(always)]
    pub(super) fn amount<C: crate::Currency>(amount: crate::Decimal) -> crate::Decimal {
        if super::Context::is_raw() {
            return amount;
        }

        amount.round_dp(C::MINOR_UNIT.into())
    }

    /// get amount rounded or not depends on Context's config.
    #[inline(always)]
    pub(super) fn amount_with_curr(
        amount: crate::Decimal,
        currency: super::DynCurrency,
    ) -> crate::Decimal {
        if super::Context::is_raw() {
            return amount;
        }

        amount.round_dp(currency.minor_unit.into())
    }
}
