use crate::{Currency, Decimal, MoneyError};
use currencylib::data::Data;

use super::helpers;

#[derive(Debug, Clone, Copy, Eq)]
pub struct DynCurrency(pub(super) Data);

impl PartialEq for DynCurrency {
    fn eq(&self, other: &Self) -> bool {
        self.0.code == other.0.code
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct DynMoney {
    pub(super) amount: Decimal,
    pub(super) currency: DynCurrency,
}

impl DynMoney {
    #[inline(always)]
    pub fn new<C: Currency>(amount: Decimal) -> Self {
        Self {
            amount: helpers::amount::<C>(amount),
            currency: helpers::dyn_curr_from::<C>(),
        }
    }

    #[inline(always)]
    pub fn new_with_curr(currency: DynCurrency, amount: Decimal) -> Self {
        Self {
            amount: helpers::amount_with_curr(amount, currency),
            currency,
        }
    }

    #[inline(always)]
    pub fn new_with_code(code: &str, amount: Decimal) -> Result<Self, MoneyError> {
        if let Some(currency) = super::Context::get_currency(code) {
            return Ok(Self {
                amount: helpers::amount_with_curr(amount, currency),
                currency,
            });
        }

        Err(MoneyError::Other(
            format!("currency {} not found", code).into(),
        ))
    }

    #[inline(always)]
    pub fn set_amount(&self, amount: Decimal) -> Self {
        Self {
            amount: helpers::amount_with_curr(amount, self.currency),
            ..*self
        }
    }

    #[inline(always)]
    pub fn set_curr<C: Currency>(&self) -> Self {
        Self {
            currency: helpers::dyn_curr_from::<C>(),
            ..*self
        }
    }

    pub fn set_curr_from_code(&self, code: &str) -> Result<Self, MoneyError> {
        if let Some(currency) = super::Context::get_currency(code) {
            return Ok(Self { currency, ..*self });
        }

        Err(MoneyError::Other(
            format!("currency {} not found", code).into(),
        ))
    }
}

impl PartialEq for DynMoney {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency.0.code == other.currency.0.code
    }
}

impl PartialOrd for DynMoney {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency.0.code == other.currency.0.code {
            return Some(self.amount.cmp(&other.amount));
        }
        None
    }
}
