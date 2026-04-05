use std::{
    fmt::{Debug, Display},
    iter::Sum,
    marker::PhantomData,
    str::FromStr,
};

#[cfg(feature = "accounting")]
use crate::AccountingOps;

use crate::{
    BaseMoney, BaseOps, Decimal, MoneyError, MoneyOps,
    base::{Amount, DecimalNumber},
    macros::dec,
    parse::{
        parse_comma_thousands_separator, parse_dot_thousands_separator,
        parse_symbol_comma_thousands_separator, parse_symbol_dot_thousands_separator,
    },
};
use crate::{Currency, MoneyFormatter};
use rust_decimal::{MathematicalOps, prelude::FromPrimitive};

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
/// let money = Money::<USD>::from_str("USD 1,234.56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// ```
///
/// # See Also
///
/// - [`BaseMoney`] trait for core money operations and accessors
/// - [`BaseOps`] trait for arithmetic and comparison operations
/// - [`MoneyFormatter`] trait for custom formatting and rounding
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Money<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C> Money<C>
where
    C: Currency + Clone,
{
    /// Creates a new `Money` instance from Decimal
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, macros::dec, BaseMoney, iso::USD};
    ///
    /// let money = Money::<USD>::from_decimal(dec!(123.309));
    /// assert_eq!(money.amount(), dec!(123.31));
    /// ```
    #[inline]
    pub fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount,
            _currency: PhantomData,
        }
        .round()
    }

    /// Creates a new `Money` from minor amount i128.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, macros::dec, BaseMoney, iso::USD};
    ///
    /// let money = Money::<USD>::from_minor(12302).unwrap();
    /// assert_eq!(money.amount(), dec!(123.02));
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
        }
        .round())
    }

    /// Creates a new `Money` instance by parsing a string that uses dot as the
    /// thousands separator and comma as the decimal separator.
    ///
    /// The format is `"CCC amount"` where `CCC` is a currency code (1-15 letters) and
    /// `amount` uses dots for thousand grouping and an optional comma for the decimal
    /// separator (e.g., `"EUR 1.234,56"`).
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice in the format `"CCC amount"`, e.g. `"EUR 1.234,56"`
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::CurrencyMismatch`] if the currency code in the string does
    /// not match the currency type parameter `C`.
    ///
    /// Returns [`MoneyError::ParseStr`] if the string is not in the expected format.
    ///
    /// Accepts negative amount CCC -amount
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, macros::dec, BaseMoney, iso::{EUR, USD}};
    ///
    /// // Dot as thousand separator, comma as decimal
    /// let money = Money::<EUR>::from_str_dot_thousands("EUR 1.234,56").unwrap();
    /// assert_eq!(money.amount(), dec!(1234.56));
    /// assert_eq!(money.code(), "EUR");
    ///
    /// // Large amount with multiple dot thousand separators
    /// let money = Money::<EUR>::from_str_dot_thousands("EUR 1.000.000,99").unwrap();
    /// assert_eq!(money.amount(), dec!(1000000.99));
    ///
    /// // No thousand separator, only decimal comma
    /// let money = Money::<EUR>::from_str_dot_thousands("EUR 100,50").unwrap();
    /// assert_eq!(money.amount(), dec!(100.50));
    ///
    /// // Integer amount without decimal part
    /// let money = Money::<EUR>::from_str_dot_thousands("EUR 1.234").unwrap();
    /// assert_eq!(money.amount(), dec!(1234.00));
    ///
    /// // Error: currencies mismatch
    /// assert!(Money::<USD>::from_str_dot_thousands("EUR 1.234,56").is_err());
    ///
    /// // Error: invalid format (wrong separator style)
    /// assert!(Money::<EUR>::from_str_dot_thousands("EUR 1,234.56").is_err());
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

    /// Parse from string with symbol and comma-separated thousands.
    /// Example: $1,234.22 into USD 1234.22
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

    /// Parse from string with symbol and dot-separated thousands.
    /// Example: $1.234,22 into USD 1234.22
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
    C: Currency + Clone,
{
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl<C> FromStr for Money<C>
where
    C: Currency + Clone,
{
    type Err = MoneyError;

    /// Implementation of string parsing for `Money` using comma as the thousands separator.
    ///
    /// Parses a string representation of money in the format `"CCC amount"` where
    /// `CCC` is a currency code (1-15 letters) and `amount` uses commas for thousand grouping
    /// and an optional dot for the decimal separator (e.g., `"USD 1,234.56"`).
    ///
    /// The currency code must be a valid ISO 4217 alphabetic code.
    ///
    /// For strings that use dot as the thousands separator and comma as the decimal
    /// separator (e.g., `"EUR 1.234,56"`), use
    /// [`Money::from_str_dot_thousands`] instead.
    ///
    /// Accepts negative amount CCC -amount
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, macros::dec, BaseMoney, iso::{USD, GBP}};
    /// use std::str::FromStr;
    ///
    /// // Comma as thousand separator, dot as decimal
    /// let money = Money::<USD>::from_str("USD 1,234.56").unwrap();
    /// assert_eq!(money.amount(), dec!(1234.56));
    /// assert_eq!(money.code(), "USD");
    ///
    /// // No thousand separator
    /// let money = Money::<GBP>::from_str("GBP 123.45").unwrap();
    /// assert_eq!(money.amount(), dec!(123.45));
    ///
    /// // Large amount with multiple comma thousand separators
    /// let money = Money::<USD>::from_str("USD 1,000,000.99").unwrap();
    /// assert_eq!(money.amount(), dec!(1000000.99));
    ///
    /// // Error: invalid format (currency must come first)
    /// assert!(Money::<USD>::from_str("100.00 USD").is_err());
    ///
    /// // Error: currencies mismatch
    /// assert!(Money::<USD>::from_str("EUR 100.00").is_err());
    ///
    /// // Error: dot thousands / comma decimal format not supported here
    /// assert!(Money::<USD>::from_str("USD 1.234,56").is_err());
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
    C: Currency + Clone,
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

impl<C: Currency + Clone> Sum for Money<C> {
    /// Sum all moneys
    ///
    /// WARN: PANIC!!! if overflowed.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Money::default(), |acc, b| acc + b)
    }
}

impl<'a, C: Currency + Clone> Sum<&'a Money<C>> for Money<C> {
    /// Sum all moneys(borrowed)
    ///
    /// WARN: PANIC!!! if overflowed.
    fn sum<I: Iterator<Item = &'a Money<C>>>(iter: I) -> Self {
        iter.fold(Money::default(), |acc, b| acc + b.clone())
    }
}

impl<C> BaseMoney<C> for Money<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn new(amount: impl DecimalNumber) -> Result<Self, MoneyError> {
        Ok(Self {
            amount: amount.get_decimal().ok_or(MoneyError::DecimalConversion)?,
            _currency: PhantomData,
        }
        .round())
    }

    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    fn round(self) -> Self {
        Self {
            amount: self.amount().round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        }
    }

    #[inline]
    fn round_with(self, decimal_points: u32, strategy: crate::base::RoundingStrategy) -> Self {
        Self {
            amount: self
                .amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
            _currency: PhantomData,
        }
    }

    #[inline]
    fn truncate(&self) -> Self {
        Self::from_decimal(self.amount.trunc())
    }

    #[inline]
    fn truncate_with(&self, scale: u32) -> Self {
        Self::from_decimal(self.amount.trunc_with_scale(scale))
    }
}

impl<C> BaseOps<C> for Money<C>
where
    C: Currency + Clone,
{
    #[inline]
    fn abs(&self) -> Self {
        Self::from_decimal(self.amount.abs())
    }

    #[inline]
    fn checked_add<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: Amount<C>,
    {
        Some(Self::from_decimal(
            self.amount.checked_add(rhs.get_decimal()?)?,
        ))
    }

    #[inline]
    fn checked_sub<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: Amount<C>,
    {
        Some(Self::from_decimal(
            self.amount.checked_sub(rhs.get_decimal()?)?,
        ))
    }

    #[inline]
    fn checked_mul<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber,
    {
        Some(Self::from_decimal(
            self.amount.checked_mul(rhs.get_decimal()?)?,
        ))
    }

    #[inline]
    fn checked_div<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber,
    {
        Some(Self::from_decimal(
            self.amount.checked_div(rhs.get_decimal()?)?,
        ))
    }
}

impl<C> MoneyFormatter<C> for Money<C> where C: Currency + Clone {}

#[cfg(feature = "accounting")]
impl<C> AccountingOps<C> for Money<C> where C: Currency + Clone {}

impl<C> MoneyOps<C> for Money<C> where C: Currency + Clone {}
