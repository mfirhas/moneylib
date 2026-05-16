use crate::{BaseMoney, BaseOps, Currency, Decimal, Money, MoneyError, RoundingStrategy};

impl<C: Currency + Copy + 'static + Send + Sync> super::ObjMoney for Money<C> {
    #[inline]
    fn amount(&self) -> Decimal {
        BaseMoney::amount(self)
    }
    #[inline]
    fn code(&self) -> &str {
        C::CODE
    }
    #[inline]
    fn symbol(&self) -> &str {
        C::SYMBOL
    }
    #[inline]
    fn name(&self) -> &str {
        C::NAME
    }
    #[inline]
    fn minor_unit(&self) -> u16 {
        C::MINOR_UNIT
    }
    #[inline]
    fn thousand_separator(&self) -> &str {
        C::THOUSAND_SEPARATOR
    }
    #[inline]
    fn decimal_separator(&self) -> &str {
        C::DECIMAL_SEPARATOR
    }
    #[inline]
    fn minor_unit_symbol(&self) -> &str {
        C::MINOR_UNIT_SYMBOL
    }

    #[inline]
    fn minor_amount(&self) -> Option<i128> {
        BaseMoney::minor_amount(self)
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn numeric_code(&self) -> i32 {
        C::NUMERIC.into()
    }

    #[inline]
    fn abs(&self) -> Box<dyn super::ObjMoney> {
        Box::new(BaseOps::abs(self))
    }

    #[inline]
    fn round(&self) -> Box<dyn super::ObjMoney> {
        Box::new(BaseMoney::round(*self))
    }

    #[inline]
    fn round_with(
        &self,
        decimal_points: u32,
        strategy: RoundingStrategy,
    ) -> Box<dyn super::ObjMoney> {
        Box::new(BaseMoney::round_with(*self, decimal_points, strategy))
    }

    #[inline]
    fn truncate(&self) -> Box<dyn super::ObjMoney> {
        Box::new(BaseMoney::truncate(self))
    }

    #[inline]
    fn truncate_with(&self, scale: u32) -> Box<dyn super::ObjMoney> {
        Box::new(BaseMoney::truncate_with(self, scale))
    }

    #[inline]
    fn checked_add(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(BaseOps::checked_add(self, rhs)?))
    }

    #[inline]
    fn checked_sub(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(BaseOps::checked_sub(self, rhs)?))
    }

    #[inline]
    fn checked_mul(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(BaseOps::checked_mul(self, rhs)?))
    }

    #[inline]
    fn checked_div(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(BaseOps::checked_div(self, rhs)?))
    }

    #[inline]
    fn checked_rem(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(BaseOps::checked_rem(self, rhs)?))
    }

    #[cfg(feature = "exchange")]
    fn convert(
        &self,
        to_code: &str,
        rate: &dyn crate::exchange::ObjRate,
    ) -> Result<Box<dyn super::ObjMoney>, crate::MoneyError> {
        if C::CODE == to_code {
            let current_is_raw = super::Context::is_raw();
            super::Context::set_raw(false);
            let ret = Box::new(super::DynMoney::from_decimal::<C>(BaseMoney::amount(self)));
            super::Context::set_raw(current_is_raw);

            return Ok(ret);
        }

        let rate_amount = rate.get_rate(C::CODE, to_code).ok_or_else(|| {
            MoneyError::ExchangeError(
                format!(
                    "overflowed or failed getting rate from: {} to: {}",
                    BaseMoney::code(self),
                    to_code
                )
                .into(),
            )
        })?;

        let result = BaseMoney::amount(self)
            .checked_mul(rate_amount)
            .ok_or(MoneyError::OverflowError)?;

        let current_is_raw = super::Context::is_raw();
        super::Context::set_raw(false);
        let ret = super::DynMoney::new_with_code(to_code, result)?;
        super::Context::set_raw(current_is_raw);

        Ok(Box::new(ret))
    }
}

/// Converts a reference to an [`ObjMoney`](super::ObjMoney) trait object into `Money<C>`.
///
/// The conversion succeeds when the currency code of the trait object matches `C::CODE`.
/// On success, the amount is rounded to the currency's minor unit (bankers rounding), exactly
/// as [`Money::from_decimal`] does.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatchError`] when the currency codes do not match.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, obj_money::ObjMoney, BaseMoney, MoneyError, macros::dec, iso::{USD, EUR}};
///
/// let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.50)).unwrap());
///
/// // Successful conversion
/// let money = Money::<USD>::try_from(obj.as_ref()).unwrap();
/// assert_eq!(BaseMoney::amount(&money), dec!(100.50));
/// assert_eq!(BaseMoney::code(&money), "USD");
///
/// // Currency mismatch returns an error
/// assert!(Money::<EUR>::try_from(obj.as_ref()).is_err());
/// ```
impl<C: Currency + Copy + 'static + Send + Sync> TryFrom<&dyn super::ObjMoney> for Money<C> {
    type Error = MoneyError;

    fn try_from(value: &dyn super::ObjMoney) -> Result<Self, Self::Error> {
        if value.code() != C::CODE {
            return Err(MoneyError::CurrencyMismatchError(
                value.code().into(),
                C::CODE.into(),
            ));
        }
        Ok(Self::from_decimal(value.amount()))
    }
}

impl<C: Currency + Copy + Send + Sync + 'static> TryFrom<Box<dyn super::ObjMoney>> for Money<C> {
    type Error = MoneyError;

    fn try_from(value: Box<dyn super::ObjMoney>) -> Result<Self, Self::Error> {
        Money::<C>::try_from(value.as_ref())
    }
}

impl<C: Currency> From<Money<C>> for super::DynMoney {
    fn from(value: Money<C>) -> Self {
        Self::from_decimal::<C>(value.amount())
    }
}

// equality

use crate::obj_money::{DynMoney, ObjMoney};
impl<C: Currency> PartialEq<&dyn ObjMoney> for Money<C> {
    fn eq(&self, other: &&dyn ObjMoney) -> bool {
        if self.code() != other.code() {
            return false;
        }
        self.amount() == other.amount()
    }
}

impl<C: Currency> PartialEq<Box<dyn ObjMoney>> for Money<C> {
    fn eq(&self, other: &Box<dyn ObjMoney>) -> bool {
        if self.code() != other.code() {
            return false;
        }
        self.amount() == other.amount()
    }
}

impl<C: Currency> PartialEq<DynMoney> for Money<C> {
    fn eq(&self, other: &DynMoney) -> bool {
        if self.code() != C::CODE {
            return false;
        }
        self.amount() == other.amount()
    }
}

// ordering

impl<C: Currency> PartialOrd<&dyn ObjMoney> for Money<C> {
    fn partial_cmp(&self, other: &&dyn ObjMoney) -> Option<std::cmp::Ordering> {
        if self.code() != other.code() {
            return None;
        }
        self.amount().partial_cmp(&other.amount())
    }
}

impl<C: Currency> PartialOrd<Box<dyn ObjMoney>> for Money<C> {
    fn partial_cmp(&self, other: &Box<dyn ObjMoney>) -> Option<std::cmp::Ordering> {
        if self.code() != other.code() {
            return None;
        }
        self.amount().partial_cmp(&other.amount())
    }
}

impl<C: Currency> PartialOrd<DynMoney> for Money<C> {
    fn partial_cmp(&self, other: &DynMoney) -> Option<std::cmp::Ordering> {
        if self.code() != other.code() {
            return None;
        }
        self.amount().partial_cmp(&other.amount())
    }
}
