use crate::{
    BaseMoney, BaseOps, Currency, Decimal, MoneyError, MoneyFormatter, MoneyParser, base::Amount,
};
use currencylib::data::Data;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};

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
    pub fn new<C: Currency>(amount: Decimal) -> Self {
        Self {
            amount: super::context::amount::<C>(amount),
            currency: DynCurrency(super::context::into_currency_data::<C>()),
        }
    }

    pub fn set_curr<C: Currency>(&mut self) {
        self.currency = DynCurrency(super::context::into_currency_data::<C>());
    }

    pub fn set_curr_from_code(&mut self, code: &str) -> Result<(), MoneyError> {
        if let Some(curr) = super::context::get_currency(code) {
            self.currency = curr;
            return Ok(());
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

impl<C: Currency> Amount<C> for DynMoney {
    fn get_decimal(&self) -> Option<Decimal> {
        if C::CODE == self.currency.0.code {
            return Some(self.amount);
        }
        None
    }
}

impl<C: Currency> BaseMoney<C> for DynMoney {
    #[inline]
    fn from_decimal(amount: Decimal) -> Self {
        Self::new::<C>(amount)
    }

    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    fn minor_amount(&self) -> Option<i128> {
        self.amount
            .round_dp(self.currency.0.minor_unit.into())
            .checked_mul(crate::dec!(10).checked_powu(self.currency.0.minor_unit.into())?)?
            .to_i128()
    }

    #[inline]
    fn name(&self) -> &str {
        self.currency.0.name
    }

    #[inline]
    fn symbol(&self) -> &str {
        self.currency.0.symbol
    }

    #[inline]
    fn code(&self) -> &str {
        self.currency.0.code
    }

    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency.0.numeric.into()
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
}

impl<C: Currency> BaseOps<C> for DynMoney {}

impl<C: Currency> MoneyParser<C> for DynMoney {}

impl<C: Currency> MoneyFormatter<C> for DynMoney {}
