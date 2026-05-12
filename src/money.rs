use rust_decimal::prelude::ToPrimitive;
use std::{
    fmt::{Debug, Display},
    iter::Sum,
    marker::PhantomData,
    str::FromStr,
};

use crate::{
    BaseMoney, BaseOps, Decimal, MoneyError, MoneyOps,
    base::{Amount, MoneyParser},
    macros::dec,
};
use crate::{Currency, MoneyFormatter};
use rust_decimal::MathematicalOps;

/// Represents a monetary value with a specific currency and amount.
///
/// `Money` is a value type that represents amount of money along with its currency.
/// It's statically checked at compile time for currency match so it will not mixing with other currencies.
/// It automatically rounds the amount to the currency's minor unit precision using
/// bankers rounding rule.
///
/// # Key Features
///
/// - **Type Safety**: Provides compile-time and runtime checks to ensure valid state.
/// - **Precision**: Uses 128-bit fixed-precision decimal for accurate calculations.
/// - **Automatic Rounding**: Rounds to currency's minor unit after each operation.
/// - **Zero-Cost**: `Copy` type with no heap allocations and currency metadata is zero-sized type.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, BaseMoney, macros::dec, iso::USD};
/// use std::str::FromStr;
///
/// // Create money from currency and amount
/// let money = Money::<USD>::new(dec!(100.50)).unwrap();
/// assert_eq!(money.amount(), dec!(100.50));
///
/// // Parse money from string
/// let money = Money::<USD>::from_str("1234.56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// ```
///
/// # See Also
///
/// - [`BaseMoney`] trait for core money operations and accessors
/// - [`BaseOps`] trait for arithmetic and comparison operations
/// - [`MoneyFormatter`] trait for custom formatting and rounding
#[derive(Copy, PartialEq, Eq)]
pub struct Money<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C: Currency> Default for Money<C> {
    /// Returns money with zero amount.
    fn default() -> Self {
        Self {
            amount: Decimal::default(),
            _currency: PhantomData,
        }
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
    C: Currency,
{
    #[inline(always)]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl<C> FromStr for Money<C>
where
    C: Currency,
{
    type Err = MoneyError;

    /// Parse money from string number.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, Money, iso::USD, money, dec};
    /// use std::str::FromStr;
    ///
    /// let money = Money::<USD>::from_str("12334.4439").unwrap();
    /// assert_eq!(money, money!(USD, 12334.44));
    /// assert_eq!(money.amount(), dec!(12334.44));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let dec_num = Decimal::from_str(s).map_err(|err| {
            MoneyError::ParseStrError(format!("failed parsing money from string: {}", err).into())
        })?;
        Ok(Self::from_decimal(dec_num))
    }
}

impl<C: Currency> Clone for Money<C> {
    fn clone(&self) -> Self {
        Self {
            amount: self.amount,
            _currency: PhantomData,
        }
    }
}

/// Implementation of formatted display for `Money`.
///
/// Displays the money using the default format, which is the currency code
/// followed by the amount with thousand and decimal separators.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, Money, Currency, macros::dec, iso::{USD, JPY}};
///
/// let money = Money::<USD>::from_decimal(dec!(1234.56));
/// assert_eq!(format!("{}", money), "USD 1,234.56");
///
/// let money = Money::<JPY>::from_minor(1234).unwrap();
/// assert_eq!(format!("{}", money), "JPY 1,234");
///
/// // Negative amounts
/// let money = Money::<USD>::new(dec!(-1234.56)).unwrap();
/// assert_eq!(format!("{}", money), "USD -1,234.56");
/// ```
impl<C> Display for Money<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<C> Debug for Money<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Money({}, {})", C::CODE, self.amount)
    }
}

impl<C: Currency> Sum for Money<C> {
    /// Sum all moneys
    ///
    /// WARN: PANIC!!! if overflowed.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Money::default(), |acc, b| acc + b)
    }
}

impl<'a, C: Currency> Sum<&'a Money<C>> for Money<C> {
    /// Sum all moneys(borrowed)
    ///
    /// WARN: PANIC!!! if overflowed.
    fn sum<I: Iterator<Item = &'a Money<C>>>(iter: I) -> Self {
        iter.fold(Money::default(), |acc, b| acc + b.clone())
    }
}

impl<C> BaseMoney<C> for Money<C>
where
    C: Currency,
{
    #[inline(always)]
    fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount: amount.round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        }
    }

    #[inline(always)]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline(always)]
    fn minor_amount(&self) -> Option<i128> {
        self.amount()
            .checked_mul(dec!(10).checked_powu(self.minor_unit().into())?)?
            .to_i128()
    }
}

impl<C> BaseOps<C> for Money<C> where C: Currency {}

impl<C> MoneyParser<C> for Money<C> where C: Currency {}

impl<C> MoneyFormatter<C> for Money<C> where C: Currency {}

impl<C> MoneyOps<C> for Money<C> where C: Currency {}
