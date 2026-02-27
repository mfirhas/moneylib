use std::{fmt::Display, marker::PhantomData, str::FromStr};

use crate::{
    BaseMoney, BaseOps, Decimal, Money, MoneyError,
    base::Amount,
    money_macros::dec,
    parse::{
        parse_comma_thousands_separator, parse_dot_thousands_separator,
        parse_symbol_comma_thousands_separator, parse_symbol_dot_thousands_separator,
    },
};
use crate::{Currency, CustomMoney};
use rust_decimal::{MathematicalOps, prelude::FromPrimitive, prelude::ToPrimitive};

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
/// - [`CustomMoney::round_with`]: rounds using custom decimal points and strategy. Returns `RawMoney`.
/// - [`RawMoney::finish`]: rounds to currency's minor unit using bankers rounding back to `Money`.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, RawMoney, BaseMoney, money_macros::dec, USD};
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
/// - [`CustomMoney`] trait for custom formatting and rounding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawMoney<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C> RawMoney<C>
where
    C: Currency + Clone,
{
    /// Creates a new `RawMoney` instance without rounding.
    ///
    /// Unlike [`Money::new`], the amount is NOT rounded to the currency's minor unit.
    /// All decimal precision is preserved exactly as provided.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of money (not rounded)
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD, JPY};
    ///
    /// // Preserves all decimal places even for USD (2 decimal places)
    /// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    /// assert_eq!(raw.amount(), dec!(100.567));
    ///
    /// // No rounding even for currencies with limited minor units
    /// let raw_jpy = RawMoney::<JPY>::new(dec!(100.567)).unwrap();
    /// assert_eq!(raw_jpy.amount(), dec!(100.567));
    ///
    /// // Accepts i32, i64, i128 amounts
    /// let raw = RawMoney::<USD>::new(300_i32).unwrap();
    /// assert_eq!(raw.amount(), dec!(300));
    /// ```
    #[inline]
    pub fn new<T>(amount: T) -> Result<Self, MoneyError>
    where
        T: Amount<C>,
    {
        Ok(Self {
            amount: amount.get_decimal().ok_or(MoneyError::DecimalConversion)?,
            _currency: PhantomData,
        })
    }

    /// Creates a new `RawMoney` instance from Decimal without rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD};
    ///
    /// let raw = RawMoney::<USD>::from_decimal(dec!(123.309));
    /// assert_eq!(raw.amount(), dec!(123.309));
    /// ```
    #[inline]
    pub const fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount,
            _currency: PhantomData,
        }
    }

    /// Creates a new `RawMoney` from minor amount i128 without rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD, BHD, JPY};
    ///
    /// // USD has 2 decimal places, so 12302 cents = $123.02
    /// let raw = RawMoney::<USD>::from_minor(12302).unwrap();
    /// assert_eq!(raw.amount(), dec!(123.02));
    ///
    /// // JPY has 0 decimal places
    /// let raw = RawMoney::<JPY>::from_minor(1000).unwrap();
    /// assert_eq!(raw.amount(), dec!(1000));
    ///
    /// // BHD has 3 decimal places
    /// let raw = RawMoney::<BHD>::from_minor(12345).unwrap();
    /// assert_eq!(raw.amount(), dec!(12.345));
    /// ```
    #[inline]
    pub fn from_minor(minor_amount: i128) -> Result<Self, MoneyError> {
        Ok(Self {
            amount: Decimal::from_i128(minor_amount)
                .ok_or(MoneyError::DecimalConversion)?
                .checked_div(
                    dec!(10)
                        .checked_powu(C::MINOR_UNIT.into())
                        .ok_or(MoneyError::ArithmeticOverflow)?,
                )
                .ok_or(MoneyError::ArithmeticOverflow)?,
            _currency: PhantomData,
        })
    }

    /// Converts this `RawMoney` to `Money`, applying rounding.
    ///
    /// Rounds the amount to the currency's minor unit precision using the
    /// bankers rounding rule, then returns a `Money` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD, JPY, BHD};
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
        Money::from_decimal(self.amount())
    }

    /// Creates a new `RawMoney` from a string with dot as the thousands separator
    /// and comma as the decimal separator (e.g., `"EUR 1.234,56"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, EUR, USD};
    ///
    /// let raw = RawMoney::<EUR>::from_str_dot_thousands("EUR 1.234,56").unwrap();
    /// assert_eq!(raw.code(), "EUR");
    ///
    /// assert!(RawMoney::<USD>::from_str_dot_thousands("EUR 1.234,56").is_err());
    /// ```
    pub fn from_str_dot_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

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

    /// Parse from string with symbol and comma-separated thousands, no rounding
    /// Example: $1,234.2249 into USD 1234.2249
    pub fn from_symbol_comma_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((symbol, amount_str)) = parse_symbol_comma_thousands_separator::<C>(s)
            && symbol == C::SYMBOL
        {
            return Ok(Self::from_decimal(
                Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?,
            ));
        }

        Err(MoneyError::ParseStr)
    }

    /// Parse from string with symbol and dot-separated thousands, no rounding
    /// Example: $1.234,2249 into USD 1234.2249
    pub fn from_symbol_dot_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((symbol, amount_str)) = parse_symbol_dot_thousands_separator::<C>(s)
            && symbol == C::SYMBOL
        {
            return Ok(Self::from_decimal(
                Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?,
            ));
        }

        Err(MoneyError::ParseStr)
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
    C: Currency + Clone,
{
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl<C> FromStr for RawMoney<C>
where
    C: Currency + Clone,
{
    type Err = MoneyError;

    /// Parses a string in the format `"CCC amount"` (comma thousands separator).
    ///
    /// For dot thousands separator format (e.g., `"EUR 1.234,56"`), use
    /// [`RawMoney::from_str_dot_thousands`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD};
    /// use std::str::FromStr;
    ///
    /// let raw = RawMoney::<USD>::from_str("USD 1,234.56789").unwrap();
    /// assert_eq!(raw.amount(), dec!(1234.56789));
    /// assert_eq!(raw.code(), "USD");
    ///
    /// assert!(RawMoney::<USD>::from_str("EUR 100.00").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Some((currency_code, amount_str)) = parse_comma_thousands_separator(s) {
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

/// Formats `RawMoney` using the currency code and full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{RawMoney, money_macros::dec, USD};
///
/// let raw = RawMoney::<USD>::from_decimal(dec!(1234.567));
/// assert_eq!(format!("{}", raw), "USD 1,234.567");
///
/// let raw = RawMoney::<USD>::from_decimal(dec!(-1234.56));
/// assert_eq!(format!("{}", raw), "USD -1,234.56");
/// ```
impl<C> Display for RawMoney<C>
where
    C: Currency + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<C> BaseMoney<C> for RawMoney<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Rounds explicitly to the currency's minor unit using bankers rounding.
    ///
    /// Unlike `Money`, this must be called explicitly. Returns `RawMoney`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD};
    ///
    /// let raw = RawMoney::<USD>::from_decimal(dec!(100.567));
    /// let rounded = raw.round();
    /// assert_eq!(rounded.amount(), dec!(100.57));
    ///
    /// // round() returns RawMoney, not Money
    /// let rounded_again = rounded.round();
    /// assert_eq!(rounded_again.amount(), dec!(100.57));
    /// ```
    #[inline]
    fn round(self) -> Self {
        Self {
            amount: self.amount().round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        }
    }

    /// Returns the money amount in its smallest unit, rounding as needed.
    ///
    /// Since minor amounts must be integers, this rounds the raw amount
    /// to the currency's minor unit precision before computing the minor amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, money_macros::dec, USD};
    ///
    /// let raw = RawMoney::<USD>::from_decimal(dec!(123.45));
    /// assert_eq!(raw.minor_amount().unwrap(), 12345_i128);
    ///
    /// // Extra precision is rounded before computing minor amount
    /// let raw = RawMoney::<USD>::from_decimal(dec!(123.238533));
    /// assert_eq!(raw.minor_amount().unwrap(), 12324_i128);
    /// ```
    #[inline]
    fn minor_amount(&self) -> Result<i128, MoneyError> {
        self.amount()
            .round_dp(C::MINOR_UNIT.into())
            .checked_mul(
                dec!(10)
                    .checked_powu(C::MINOR_UNIT.into())
                    .ok_or(MoneyError::ArithmeticOverflow)?,
            )
            .ok_or(MoneyError::ArithmeticOverflow)?
            .to_i128()
            .ok_or(MoneyError::DecimalConversion)
    }
}

impl<C> BaseOps<C> for RawMoney<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn abs(&self) -> Self {
        Self::from_decimal(self.amount.abs())
    }

    #[inline]
    fn add<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_add(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    #[inline]
    fn sub<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_sub(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    #[inline]
    fn mul<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>,
    {
        Ok(Self::from_decimal(
            self.amount
                .checked_mul(rhs.get_decimal().ok_or(MoneyError::ArithmeticOverflow)?)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    #[inline]
    fn div<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
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

impl<C> CustomMoney<C> for RawMoney<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn round_with(self, decimal_points: u32, strategy: crate::base::RoundingStrategy) -> Self {
        Self {
            amount: self
                .amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
            _currency: PhantomData,
        }
    }
}
