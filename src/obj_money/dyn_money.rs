use crate::{Currency, Decimal, MoneyError, RoundingStrategy, prelude::ObjMoney};
use currencylib::data::Data;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};

use super::helpers;

#[derive(Debug, Clone, Copy, Eq)]
pub struct DynCurrency(pub(super) Data);

impl DynCurrency {
    pub fn code(&self) -> &str {
        self.0.code
    }
}

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
    pub fn from_decimal<C: Currency>(amount: Decimal) -> Self {
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
    fn minor_unit_name(&self) -> &str {
        self.currency.0.minor_unit_name
    }

    #[inline]
    fn origin(&self) -> &str {
        self.currency.0.origin
    }

    #[inline]
    fn locale(&self) -> &str {
        self.currency.0.locale
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
        Box::new(
            self.set_amount(
                self.amount
                    .round_dp_with_strategy(decimal_points, strategy.into()),
            ),
        )
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

    #[inline]
    fn neg(&self) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(-self.amount))
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

        Ok(Box::new(Self::new_with_code(to_code, new_amount)?))
    }
}

impl TryFrom<&dyn super::ObjMoney> for DynMoney {
    type Error = MoneyError;

    fn try_from(value: &dyn super::ObjMoney) -> Result<Self, Self::Error> {
        Self::new_with_code(value.code(), value.amount())
    }
}

impl TryFrom<Box<dyn super::ObjMoney>> for DynMoney {
    type Error = MoneyError;

    fn try_from(value: Box<dyn super::ObjMoney>) -> Result<Self, Self::Error> {
        DynMoney::try_from(value.as_ref())
    }
}

impl<C: Currency> TryFrom<DynMoney> for crate::Money<C> {
    type Error = MoneyError;

    fn try_from(value: DynMoney) -> Result<Self, Self::Error> {
        if value.currency.0.code != C::CODE {
            return Err(MoneyError::CurrencyMismatchError(
                value.currency.0.code.into(),
                C::CODE.into(),
            ));
        }

        use crate::BaseMoney;
        Ok(Self::from_decimal(value.amount))
    }
}

#[cfg(feature = "raw_money")]
impl<C: Currency> TryFrom<DynMoney> for crate::RawMoney<C> {
    type Error = MoneyError;

    fn try_from(value: DynMoney) -> Result<Self, Self::Error> {
        if value.currency.0.code != C::CODE {
            return Err(MoneyError::CurrencyMismatchError(
                value.currency.0.code.into(),
                C::CODE.into(),
            ));
        }

        use crate::BaseMoney;
        Ok(Self::from_decimal(value.amount))
    }
}

// Equality

impl PartialEq<&dyn ObjMoney> for DynMoney {
    fn eq(&self, other: &&dyn ObjMoney) -> bool {
        self.currency.0.code == other.code() && self.amount == other.amount()
    }
}

impl PartialEq<Box<dyn ObjMoney>> for DynMoney {
    fn eq(&self, other: &Box<dyn ObjMoney>) -> bool {
        self.currency.0.code == other.code() && self.amount == other.amount()
    }
}

use crate::{BaseMoney, Money};
impl<C: Currency> PartialEq<Money<C>> for DynMoney {
    fn eq(&self, other: &Money<C>) -> bool {
        self.currency.0.code == C::CODE && self.amount == other.amount()
    }
}

#[cfg(feature = "raw_money")]
use crate::RawMoney;

#[cfg(feature = "raw_money")]
impl<C: Currency> PartialEq<RawMoney<C>> for DynMoney {
    fn eq(&self, other: &RawMoney<C>) -> bool {
        self.currency.0.code == C::CODE && self.amount == other.amount()
    }
}

// Ordering

impl PartialOrd<&dyn ObjMoney> for DynMoney {
    fn partial_cmp(&self, other: &&dyn ObjMoney) -> Option<std::cmp::Ordering> {
        if self.currency.0.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}

impl PartialOrd<Box<dyn ObjMoney>> for DynMoney {
    fn partial_cmp(&self, other: &Box<dyn ObjMoney>) -> Option<std::cmp::Ordering> {
        if self.currency.0.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}

impl<C: Currency> PartialOrd<Money<C>> for DynMoney {
    fn partial_cmp(&self, other: &Money<C>) -> Option<std::cmp::Ordering> {
        if self.currency.0.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}

#[cfg(feature = "raw_money")]
impl<C: Currency> PartialOrd<RawMoney<C>> for DynMoney {
    fn partial_cmp(&self, other: &RawMoney<C>) -> Option<std::cmp::Ordering> {
        if self.currency.0.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}
