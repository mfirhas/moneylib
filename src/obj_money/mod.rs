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
    /// converts Currency into DynCurrency
    #[inline(always)]
    pub(super) const fn dyn_curr_from<C: crate::Currency>() -> super::DynCurrency {
        super::DynCurrency {
            code: C::CODE,
            symbol: C::SYMBOL,
            name: C::NAME,
            numeric: C::NUMERIC,
            minor_unit: C::MINOR_UNIT,
            minor_unit_symbol: C::MINOR_UNIT_SYMBOL,
            minor_unit_name: C::MINOR_UNIT_NAME,
            thousand_separator: C::THOUSAND_SEPARATOR,
            decimal_separator: C::DECIMAL_SEPARATOR,
            origin: C::ORIGIN,
            locale: C::LOCALE,
        }
    }

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
