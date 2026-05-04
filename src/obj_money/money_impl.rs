use crate::{BaseMoney, Currency, Decimal, Money, MoneyError};

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

    #[cfg(feature = "exchange")]
    fn convert(
        &self,
        to_code: &str,
        rate: &dyn crate::exchange::ObjRate,
    ) -> Result<Box<dyn super::ObjMoney>, crate::MoneyError> {
        if BaseMoney::code(self) == to_code {
            return Ok(Box::new(*self));
        }

        Ok(Box::new(Self::from_decimal(
            BaseMoney::amount(self)
                .checked_mul(
                    rate.get_rate(BaseMoney::code(self), to_code).ok_or(
                        crate::MoneyError::ExchangeError(
                            format!(
                                "overflowed or failed getting rate from: {} to: {}",
                                BaseMoney::code(self),
                                to_code
                            )
                            .into(),
                        ),
                    )?,
                )
                .ok_or(crate::MoneyError::OverflowError)?,
        )))
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
/// use moneylib::{Money, ObjMoney, BaseMoney, MoneyError, macros::dec, iso::{USD, EUR}};
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
