use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use crate::{
    BaseMoney, BaseOps, Decimal, MoneyError, MoneyResult,
    base::Amount,
    money_macros::dec,
    parse::{parse_comma_thousands_separator, parse_dot_thousands_separator},
};
use crate::{Currency, CustomMoney};
use rust_decimal::{MathematicalOps, prelude::FromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    #[inline]
    pub fn new<T>(amount: T) -> MoneyResult<Self>
    where
        T: Amount<C>,
    {
        Ok(Self {
            amount: amount
                .get_decimal()
                .ok_or(MoneyError::NewMoney(
                    "failed converting into Decimal".into(),
                ))?
                .round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        })
    }

    #[inline]
    pub fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount: amount.round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        }
    }

    #[inline]
    pub fn from_minor(minor_amount: i128) -> MoneyResult<Self> {
        Ok(Self {
            amount: Decimal::from_i128(minor_amount)
                .ok_or(MoneyError::ArithmeticOverflow)?
                .checked_mul(
                    dec!(10)
                        .checked_powu(C::MINOR_UNIT.into())
                        .ok_or(MoneyError::ArithmeticOverflow)?,
                )
                .ok_or(MoneyError::ArithmeticOverflow)?
                .round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        })
    }
}

impl<C: Currency> Ord for Money<C>
where
    C: Currency + PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl<C> PartialOrd for Money<C>
where
    C: Currency + PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Amount<C> for Money<C>
where
    C: Currency + Clone,
{
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl<C: Currency> Amount<C> for Decimal {
    fn get_decimal(&self) -> Option<Decimal> {
        Some(*self)
    }
}

impl<C: Currency> Amount<C> for f64 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_f64(*self)
    }
}

impl<C: Currency> Amount<C> for i32 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i32(*self)
    }
}

impl<C: Currency> Amount<C> for i64 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i64(*self)
    }
}

impl<C: Currency> Amount<C> for i128 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i128(*self)
    }
}

impl<C: Currency> FromStr for Money<C> {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Try parsing with comma thousands separator first
        if let Some((currency_code, amount_str)) = parse_comma_thousands_separator(s) {
            if currency_code != C::CODE {
                return Err(MoneyError::CurrencyMismatch);
            }
            return Ok(Self::from_decimal(
                Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?,
            ));
        }

        // Try parsing with dot thousands separator
        if let Some((currency_code, amount_str)) = parse_dot_thousands_separator(s) {
            if currency_code != C::CODE {
                return Err(MoneyError::CurrencyMismatch);
            }
            return Ok(Self::from_decimal(
                Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?,
            ));
        }

        Err(MoneyError::ParseStr)
    }
}

impl<C> Display for Money<C>
where
    C: Currency + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_code())
    }
}

impl<C> BaseMoney<C> for Money<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    fn round(self) -> Self {
        Self::from_decimal(self.amount().round_dp(C::MINOR_UNIT.into()))
    }
}

impl<C> BaseOps<C> for Money<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn abs(&self) -> Self {
        Self::from_decimal(self.amount.abs()).round()
    }

    #[inline]
    fn min(&self, rhs: Self) -> Self {
        Self::from_decimal(self.amount.min(rhs.amount)).round()
    }

    #[inline]
    fn max(&self, rhs: Self) -> Self {
        Self::from_decimal(self.amount.max(rhs.amount)).round()
    }

    #[inline]
    fn clamp(&self, from: Decimal, to: Decimal) -> Self {
        Self::from_decimal(self.amount.clamp(from, to)).round()
    }

    fn add<RHS>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_add(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn sub<RHS>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_sub(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn mul<RHS>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_mul(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn div<RHS>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_div(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }
}

impl<C> CustomMoney<C> for Money<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn round_with(self, decimal_points: u32, strategy: crate::base::RoundingStrategy) -> Self {
        Self::from_decimal(
            self.amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
        )
    }
}
