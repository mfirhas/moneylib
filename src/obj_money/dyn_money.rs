use crate::{Currency, Decimal, MoneyError, RoundingStrategy};
use currencylib::data::Data;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};

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

impl super::ObjMoney for DynMoney {
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    fn code(&self) -> &str {
        self.currency.0.code
    }

    #[inline]
    fn symbol(&self) -> &str {
        self.currency.0.symbol
    }

    #[inline]
    fn name(&self) -> &str {
        self.currency.0.name
    }

    #[inline]
    fn minor_unit(&self) -> u16 {
        self.currency.0.minor_unit
    }

    #[inline]
    fn thousand_separator(&self) -> &str {
        self.currency.0.thousand_separator
    }

    #[inline]
    fn decimal_separator(&self) -> &str {
        self.currency.0.decimal_separator
    }

    #[inline]
    fn minor_unit_symbol(&self) -> &str {
        self.currency.0.minor_unit_symbol
    }

    #[inline]
    fn minor_amount(&self) -> Option<i128> {
        self.amount
            .round_dp(self.currency.0.minor_unit.into())
            .checked_mul(crate::dec!(10).checked_powu(self.currency.0.minor_unit.into())?)?
            .to_i128()
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency.0.numeric.into()
    }

    #[inline]
    fn abs(&self) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.abs()))
    }

    #[inline]
    fn round(&self) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.round_dp(self.currency.0.minor_unit.into())))
    }

    #[inline]
    fn round_with(
        &self,
        decimal_points: u32,
        strategy: RoundingStrategy,
    ) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(
            self.amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
        ))
    }

    #[inline]
    fn truncate(&self) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.trunc()))
    }

    #[inline]
    fn truncate_with(&self, scale: u32) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.trunc_with_scale(scale)))
    }

    #[inline]
    fn checked_add(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_add(rhs)?)))
    }

    #[inline]
    fn checked_sub(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_sub(rhs)?)))
    }

    #[inline]
    fn checked_mul(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_mul(rhs)?)))
    }

    #[inline]
    fn checked_div(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_div(rhs)?)))
    }

    #[inline]
    fn checked_rem(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_rem(rhs)?)))
    }

    #[cfg(feature = "exchange")]
    fn convert(
        &self,
        to_code: &str,
        rate: &dyn crate::exchange::ObjRate,
    ) -> Result<Box<dyn super::ObjMoney>, MoneyError> {
        if self.currency.0.code == to_code {
            return Ok(Box::new(*self));
        }

        let rate_val = rate
            .get_rate(self.currency.0.code, to_code)
            .ok_or_else(|| {
                MoneyError::ExchangeError(
                    format!(
                        "overflowed or failed getting rate from: {} to: {}",
                        self.currency.0.code, to_code
                    )
                    .into(),
                )
            })?;

        let new_amount = self
            .amount
            .checked_mul(rate_val)
            .ok_or(MoneyError::OverflowError)?;

        let to_currency = super::Context::get_currency(to_code)
            .ok_or_else(|| MoneyError::Other(format!("currency {} not found", to_code).into()))?;

        Ok(Box::new(Self::new_with_curr(to_currency, new_amount)))
    }
}
