use std::{
    fmt::{Debug, Display},
    iter::Sum,
    marker::PhantomData,
    str::FromStr,
};

use crate::{
    BaseMoney, BaseOps, Decimal, Money, MoneyError, MoneyOps,
    base::{Amount, MoneyParser},
    macros::dec,
};
use crate::{Currency, MoneyFormatter};
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};

/// Represents a monetary value without automatic rounding.
///
/// `RawMoney` is exactly like [`Money`] except it doesn't automatically round
/// amounts after each operation. It keeps full decimal precision and lets
/// callers decide when to round.
///
/// # Key Features
///
/// - **Type Safety**: Provides compile-time checks to ensure valid state.
/// - **Precision**: Uses 128-bit fixed-precision decimal for accurate calculations.
/// - **No Automatic Rounding**: Preserves all decimal places until explicitly rounded.
/// - **Zero-Cost**: `Copy` type with no heap allocations and currency metadata is zero-sized type.
///
/// # Conversion
///
/// - Convert from `Money` using [`Money::into_raw`]
/// - Convert to `Money` using [`RawMoney::finish`] (applies rounding)
///
/// # Where Rounding Happens
///
/// - [`BaseMoney::round`]: rounds using currency's minor unit (bankers rounding). Returns `RawMoney`.
/// - [`BaseMoney::round_with`]: rounds using custom decimal points and strategy. Returns `RawMoney`.
/// - [`RawMoney::finish`]: rounds to currency's minor unit using bankers rounding back to `Money`.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, RawMoney, BaseMoney, macros::dec, iso::USD};
///
/// // Create RawMoney directly - no rounding
/// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
/// assert_eq!(raw.amount(), dec!(100.567));
///
/// // Convert from Money
/// let money = Money::<USD>::new(dec!(100.50)).unwrap();
/// let raw = money.into_raw();
/// assert_eq!(raw.amount(), dec!(100.50));
///
/// // Convert back to Money with rounding
/// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
/// let money = raw.finish();
/// assert_eq!(money.amount(), dec!(100.57));
/// ```
///
/// # See Also
///
/// - [`Money`] for automatically-rounded monetary values
/// - [`BaseMoney`] trait for core money operations and accessors
/// - [`BaseOps`] trait for arithmetic and comparison operations
/// - [`MoneyFormatter`] trait for custom formatting and rounding
#[derive(Copy, PartialEq, Eq)]
pub struct RawMoney<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C> RawMoney<C>
where
    C: Currency,
{
    /// Converts this `RawMoney` to `Money`, applying rounding.
    ///
    /// Rounds the amount to the currency's minor unit precision using the
    /// bankers rounding rule, then returns a `Money` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::{USD, JPY, BHD}};
    ///
    /// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    /// let money = raw.finish();
    /// assert_eq!(money.amount(), dec!(100.57));
    ///
    /// let raw_jpy = RawMoney::<JPY>::new(dec!(100.567)).unwrap();
    /// let money = raw_jpy.finish();
    /// assert_eq!(money.amount(), dec!(101));
    ///
    /// let raw_bhd = RawMoney::<BHD>::new(dec!(100.9999)).unwrap();
    /// let money = raw_bhd.finish();
    /// assert_eq!(money.amount(), dec!(101.000));
    /// ```
    #[inline]
    pub fn finish(self) -> Money<C> {
        Money::from_decimal(self.amount)
    }
}

impl<C: Currency> Default for RawMoney<C> {
    /// Returns money with zero amount.
    fn default() -> Self {
        Self {
            amount: Decimal::default(),
            _currency: PhantomData,
        }
    }
}

impl<C: Currency> Ord for RawMoney<C>
where
    C: Currency + PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl<C> PartialOrd for RawMoney<C>
where
    C: Currency + PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Amount<C> for RawMoney<C>
where
    C: Currency,
{
    #[inline(always)]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl<C> FromStr for RawMoney<C>
where
    C: Currency,
{
    type Err = MoneyError;

    /// Parse money from string number.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, RawMoney, iso::USD, raw, dec};
    /// use std::str::FromStr;
    ///
    /// let money = RawMoney::<USD>::from_str("12334.4439").unwrap();
    /// assert_eq!(money, raw!(USD, 12334.4439));
    /// assert_eq!(money.amount(), dec!(12334.4439));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let dec_num = Decimal::from_str(s).map_err(|err| {
            MoneyError::ParseStrError(format!("failed parsing money from string: {}", err).into())
        })?;
        Ok(Self::from_decimal(dec_num))
    }
}

impl<C: Currency> Clone for RawMoney<C> {
    fn clone(&self) -> Self {
        Self {
            amount: self.amount,
            _currency: PhantomData,
        }
    }
}

/// Formats `RawMoney` using the currency code and full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, RawMoney, macros::dec, iso::USD};
///
/// let raw = RawMoney::<USD>::from_decimal(dec!(1234.567));
/// assert_eq!(format!("{}", raw), "USD 1,234.567");
///
/// let raw = RawMoney::<USD>::from_decimal(dec!(-1234.56));
/// assert_eq!(format!("{}", raw), "USD -1,234.56");
/// ```
impl<C> Display for RawMoney<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<C> Debug for RawMoney<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawMoney({}, {})", C::CODE, self.amount)
    }
}

impl<C: Currency> Sum for RawMoney<C> {
    /// Sum all moneys
    ///
    /// WARN: PANIC! if overflowed.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(RawMoney::default(), |acc, b| acc + b)
    }
}

impl<'a, C: Currency> Sum<&'a RawMoney<C>> for RawMoney<C> {
    /// Sum all moneys(borrowed)
    ///
    /// WARN: PANIC!!! if overflowed.
    fn sum<I: Iterator<Item = &'a RawMoney<C>>>(iter: I) -> Self {
        iter.fold(RawMoney::default(), |acc, b| acc + b.clone())
    }
}

impl<C> BaseMoney<C> for RawMoney<C>
where
    C: Currency,
{
    #[inline(always)]
    fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount,
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
            .round_dp(C::MINOR_UNIT.into())
            .checked_mul(dec!(10).checked_powu(C::MINOR_UNIT.into())?)?
            .to_i128()
    }
}

impl<C> BaseOps<C> for RawMoney<C> where C: Currency {}

impl<C> MoneyParser<C> for RawMoney<C> where C: Currency {}

impl<C> MoneyFormatter<C> for RawMoney<C> where C: Currency {}

impl<C> MoneyOps<C> for RawMoney<C> where C: Currency {}
