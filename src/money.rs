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
/// use moneylib::{Money, Currency, BaseMoney, money_macros::dec, USD};
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
/// - [`CustomMoney`] trait for custom formatting and rounding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    /// Creates a new `Money` instance with amount of Decimal, f64, i32, i64, i128,
    /// or taking amount from another instance of money of same currency.
    ///
    /// The amount is automatically rounded to the currency's minor unit precision
    /// using the bankers rounding rule.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of money
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, USD, JPY};
    ///
    /// let money = Money::<USD>::new(dec!(100.50)).unwrap();
    /// assert_eq!(money.amount(), dec!(100.50));
    ///
    /// // Amount is rounded to currency's minor unit (2 decimal places for USD)
    /// let money = Money::<USD>::new(100.567).unwrap();
    /// assert_eq!(money.amount(), dec!(100.57));
    ///
    /// // JPY has 0 decimal places, so it rounds to whole numbers
    /// let money = Money::<JPY>::new(dec!(100.5)).unwrap();
    /// assert_eq!(money.amount(), dec!(100));
    ///
    /// // Amount is i32
    /// let money = Money::<USD>::new(300).unwrap();
    /// assert_eq!(money.amount(), dec!(300));
    /// // Amount is i64
    /// let money = Money::<USD>::new(300_i64).unwrap();
    /// assert_eq!(money.amount(), dec!(300));
    ///
    /// // Amount is i128
    /// let money = Money::<USD>::new(3000_i128).unwrap();
    /// assert_eq!(money.amount(), dec!(3000));
    /// ```
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

    /// Creates a new `Money` instance from Decimal
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, USD};
    ///
    /// let money = Money::<USD>::from_decimal(dec!(123.309));
    /// assert_eq!(money.amount(), dec!(123.31));
    /// ```
    #[inline]
    pub fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount: amount.round_dp(C::MINOR_UNIT.into()),
            _currency: PhantomData,
        }
    }

    /// Creates a new `Money` from minor amount i128.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, USD};
    ///
    /// let money = Money::<USD>::from_minor(12302).unwrap();
    /// assert_eq!(money.amount(), dec!(123.02));
    /// ```
    #[inline]
    pub fn from_minor(minor_amount: i128) -> MoneyResult<Self> {
        Ok(Self {
            amount: Decimal::from_i128(minor_amount)
                .ok_or(MoneyError::ArithmeticOverflow)?
                .checked_div(
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

/// Implementation of string parsing for `Money`.
///
/// Parses a string representation of money in the format "CURRENCY AMOUNT".
/// Supports two thousand separator formats:
/// - Comma as thousand separator, dot as decimal separator (e.g., "USD 1,234.56")
/// - Dot as thousand separator, comma as decimal separator (e.g., "EUR 1.234,56")
///
/// The currency code must be a valid ISO 4217 alphabetic code.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, money_macros::dec, BaseMoney, USD, EUR, GBP};
/// use std::str::FromStr;
///
/// // Comma as thousand separator, dot as decimal
/// let money = Money::<USD>::from_str("USD 1,234.56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// assert_eq!(money.code(), "USD");
///
/// // Dot as thousand separator, comma as decimal
/// let money = Money::<EUR>::from_str("EUR 1.234,56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// assert_eq!(money.code(), "EUR");
///
/// // No thousand separator
/// let money = Money::<GBP>::from_str("GBP 123.45").unwrap();
/// assert_eq!(money.amount(), dec!(123.45));
///
/// // Error: invalid format (currency must come first)
/// assert!(Money::<USD>::from_str("100.00 USD").is_err());
///
/// // Error: currencies mismatch
/// assert!(Money::<EUR>::from_str("USD 100.00").is_err());
/// ```
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

/// Implementation of formatted display for `Money`.
///
/// Displays the money using the default format, which is the currency code
/// followed by the amount with thousand and decimal separators.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, USD, JPY};
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

    #[inline]
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

    #[inline]
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

    #[inline]
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
