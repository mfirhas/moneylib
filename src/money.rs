use std::{fmt::Display, str::FromStr};

use crate::{base::MoneyAmount, money_macros::dec};
use rust_decimal::{MathematicalOps, prelude::FromPrimitive};

use crate::{
    BaseMoney, Currency, Decimal, MoneyError, MoneyResult,
    base::{BaseOps, COMMA_SEPARATOR, CustomMoney, DOT_SEPARATOR},
    parse::{parse_comma_thousands_separator, parse_dot_thousands_separator},
};

/// Represents a monetary value with a specific currency and amount.
///
/// `Money` is a value type that combines a [`Currency`] with a [`Decimal`] amount.
/// It automatically rounds the amount to the currency's minor unit precision using
/// the currency's rounding strategy (default: Banker's rounding).
///
/// # Key Features
///
/// - **Type Safety**: Ensures currency consistency in operations
/// - **Precision**: Uses 128-bit fixed-point decimal for accurate calculations
/// - **Automatic Rounding**: Rounds to currency's minor unit after each operation
/// - **Zero-Cost**: `Copy` type with no heap allocations
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, BaseMoney, money_macros::dec};
/// use std::str::FromStr;
///
/// // Create money from currency and amount
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.50));
/// assert_eq!(money.amount(), dec!(100.50));
///
/// // Parse money from string
/// let money = Money::from_str("USD 1,234.56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// ```
///
/// # See Also
///
/// - [`BaseMoney`] trait for core money operations and accessors
/// - [`BaseOps`] trait for arithmetic and comparison operations
/// - [`CustomMoney`] trait for custom formatting and rounding
#[derive(Debug, Clone, Copy, Eq)]
pub struct Money {
    currency: Currency,
    amount: Decimal,
}

impl Money {
    /// Creates a new `Money` instance with the specified currency and amount.
    ///
    /// The amount is automatically rounded to the currency's minor unit precision
    /// using the currency's rounding strategy (default: Banker's rounding).
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency for this money value
    /// * `amount` - The decimal amount to store
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100.50));
    /// assert_eq!(money.amount(), dec!(100.50));
    ///
    /// // Amount is rounded to currency's minor unit (2 decimal places for USD)
    /// let money = Money::new(usd, dec!(100.567));
    /// assert_eq!(money.amount(), dec!(100.57));
    ///
    /// // JPY has 0 decimal places, so it rounds to whole numbers
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// let money = Money::new(jpy, dec!(100.5));
    /// assert_eq!(money.amount(), dec!(100));
    /// ```
    #[inline]
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        Money { currency, amount }.round()
    }

    /// Creates a new `Money` instance from a generic amount type.
    ///
    /// This method accepts any type that implements [`MoneyAmount`], including:
    /// - `Money` - validates currency matches
    /// - `Decimal` - creates money with the given amount
    /// - `f64`, `i32`, `i64`, `i128` - converts to decimal and creates money
    ///
    /// # Arguments
    ///
    /// * `currency` - The target currency for the money
    /// * `amount` - A value that can be converted to a money amount
    ///
    /// # Returns
    ///
    /// Returns `Ok(Money)` if the conversion succeeds and currencies match (if applicable).
    /// Returns `Err(MoneyError)` if:
    /// - The amount type cannot be converted to a valid money amount
    /// - The amount is `Money` with a different currency than specified
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // From Decimal
    /// let money = Money::from_amount(usd, dec!(100.50)).unwrap();
    /// assert_eq!(money.amount(), dec!(100.50));
    ///
    /// // From f64
    /// let money = Money::from_amount(usd, 100.50).unwrap();
    /// assert_eq!(money.amount(), dec!(100.50));
    ///
    /// // From i32
    /// let money = Money::from_amount(usd, 100_i32).unwrap();
    /// assert_eq!(money.amount(), dec!(100.00));
    ///
    /// // From existing Money with matching currency
    /// let existing = Money::new(usd, dec!(50.25));
    /// let money = Money::from_amount(usd, existing).unwrap();
    /// assert_eq!(money.amount(), dec!(50.25));
    ///
    /// // Error: currency mismatch
    /// let eur = Currency::from_iso("EUR").unwrap();
    /// let eur_money = Money::new(eur, dec!(50.25));
    /// assert!(Money::from_amount(usd, eur_money).is_err());
    /// ```
    pub fn from_amount<T>(currency: Currency, amount: T) -> MoneyResult<Self>
    where
        T: MoneyAmount<Money>,
    {
        match (amount.get_money(), amount.get_decimal()) {
            (Some(money), _) if money.currency() == currency => Ok(money),
            (None, Some(amount)) => Ok(Self::new(currency, amount)),
            _ => Err(MoneyError::NewMoney(
                "amount type is invalid or or money's currency mismatches".into(),
            )),
        }
    }

    /// Creates a new `Money` instance from an amount expressed in the currency's minor unit.
    ///
    /// The minor unit is the smallest denomination of the currency (e.g., cents for USD,
    /// pence for GBP). This method converts the minor amount to the standard amount by
    /// dividing by 10^(minor_unit).
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency for this money value
    /// * `minor_amount` - The amount in the currency's smallest unit
    ///
    /// # Returns
    ///
    /// Returns `Ok(Money)` if the conversion succeeds.
    /// Returns `Err(MoneyError)` if:
    /// - The minor amount cannot be converted to decimal
    /// - The conversion calculation overflows
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // USD has 2 decimal places, so 12345 cents = $123.45
    /// let money = Money::from_minor_amount(usd, 12345).unwrap();
    /// assert_eq!(money.amount(), dec!(123.45));
    ///
    /// // JPY has 0 decimal places, so the minor amount equals the amount
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// let money = Money::from_minor_amount(jpy, 1000).unwrap();
    /// assert_eq!(money.amount(), dec!(1000));
    ///
    /// // BHD has 3 decimal places
    /// let bhd = Currency::from_iso("BHD").unwrap();
    /// let money = Money::from_minor_amount(bhd, 12345).unwrap();
    /// assert_eq!(money.amount(), dec!(12.345));
    ///
    /// // Negative amounts are supported
    /// let money = Money::from_minor_amount(usd, -12345).unwrap();
    /// assert_eq!(money.amount(), dec!(-123.45));
    /// ```
    pub fn from_minor_amount(currency: Currency, minor_amount: i128) -> MoneyResult<Self> {
        let dec = Decimal::from_i128(minor_amount).ok_or(MoneyError::NewMoney(
            "failed converting i128 to decimal".into(),
        ))?;

        let amount = dec
            .checked_div(
                dec!(10)
                    .checked_powu(currency.minor_unit().into())
                    .ok_or(MoneyError::ArithmeticOverflow)?,
            )
            .ok_or(MoneyError::NewMoney(
                "failed converting minor amount into amount".into(),
            ))?;

        Ok(Self::new(currency, amount))
    }
}

/// Implementation of equality comparison for `Money`.
///
/// Two `Money` values are equal if both their currency and amount are equal.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let eur = Currency::from_iso("EUR").unwrap();
///
/// let money1 = Money::new(usd, dec!(100.50));
/// let money2 = Money::new(usd, dec!(100.50));
/// let money3 = Money::new(usd, dec!(100.51));
/// let money4 = Money::new(eur, dec!(100.50));
///
/// // Same currency and amount
/// assert_eq!(money1, money2);
///
/// // Different amount
/// assert_ne!(money1, money3);
///
/// // Different currency
/// assert_ne!(money1, money4);
/// ```
impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}

/// Implementation of ordering comparison for `Money`.
///
/// Compares two `Money` values by their amounts. Both values must have the same
/// currency, otherwise the comparison will **panic**.
///
/// # Panics
///
/// Panics if the two `Money` values have different currencies. Use the comparison
/// methods in [`BaseOps`] trait for non-panicking comparisons.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money1 = Money::new(usd, dec!(100.00));
/// let money2 = Money::new(usd, dec!(200.00));
///
/// assert!(money1 < money2);
/// assert!(money2 > money1);
/// assert!(money1 <= money2);
/// assert!(money2 >= money1);
/// ```
///
/// ```should_panic
/// use moneylib::{Money, Currency, money_macros::dec};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let eur = Currency::from_iso("EUR").unwrap();
/// let money1 = Money::new(usd, dec!(100.00));
/// let money2 = Money::new(eur, dec!(100.00));
///
/// // This will panic because currencies differ
/// let _ = money1 < money2;
/// ```
impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // WARN: PANIC!
        assert_eq!(
            self.currency, other.currency,
            "cannot compare 2 money with different currencies"
        );
        self.amount.partial_cmp(&other.amount)
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
/// use moneylib::{Money, money_macros::dec, BaseMoney};
/// use std::str::FromStr;
///
/// // Comma as thousand separator, dot as decimal
/// let money = Money::from_str("USD 1,234.56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// assert_eq!(money.code(), "USD");
///
/// // Dot as thousand separator, comma as decimal
/// let money = Money::from_str("EUR 1.234,56").unwrap();
/// assert_eq!(money.amount(), dec!(1234.56));
/// assert_eq!(money.code(), "EUR");
///
/// // No thousand separator
/// let money = Money::from_str("GBP 123.45").unwrap();
/// assert_eq!(money.amount(), dec!(123.45));
///
/// // Error: invalid format (currency must come first)
/// assert!(Money::from_str("100.00 USD").is_err());
/// ```
impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Try parsing with comma thousands separator first
        if let Some((currency_code, amount_str)) = parse_comma_thousands_separator(s) {
            let mut currency = currency_code
                .parse::<Currency>()
                .map_err(|_| MoneyError::InvalidCurrency)?;

            currency.set_thousand_separator(COMMA_SEPARATOR);
            currency.set_decimal_separator(DOT_SEPARATOR);

            let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?;

            return Ok(Self::new(currency, amount));
        }

        // Try parsing with dot thousands separator
        if let Some((currency_code, amount_str)) = parse_dot_thousands_separator(s) {
            let mut currency = currency_code
                .parse::<Currency>()
                .map_err(|_| MoneyError::InvalidCurrency)?;

            currency.set_thousand_separator(DOT_SEPARATOR);
            currency.set_decimal_separator(COMMA_SEPARATOR);

            let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?;

            return Ok(Self::new(currency, amount));
        }

        // Neither format matched
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
/// use moneylib::{Money, Currency, money_macros::dec};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(1234.56));
/// assert_eq!(format!("{}", money), "USD 1,234.56");
///
/// let jpy = Currency::from_iso("JPY").unwrap();
/// let money = Money::new(jpy, dec!(1234));
/// assert_eq!(format!("{}", money), "JPY 1,234");
///
/// // Negative amounts
/// let money = Money::new(usd, dec!(-1234.56));
/// assert_eq!(format!("{}", money), "USD -1,234.56");
/// ```
impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Implementation of conversion from `Money` to `Decimal`.
///
/// Extracts the amount from a `Money` value as a `Decimal`.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, Decimal, money_macros::dec};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(123.45));
///
/// let amount: Decimal = money.into();
/// assert_eq!(amount, dec!(123.45));
/// ```
impl From<Money> for Decimal {
    /// Get the amount of money
    fn from(value: Money) -> Self {
        value.amount()
    }
}

// --- MoneyAmount

/// Implementation of `MoneyAmount<Money>` for `Money`.
///
/// Allows `Money` to be used as an amount in operations that accept
/// generic amount types.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money1 = Money::new(usd, dec!(100.00));
/// let money2 = Money::new(usd, dec!(50.00));
///
/// // Add Money to Money
/// let result = money1.add(money2).unwrap();
/// assert_eq!(result.amount(), dec!(150.00));
/// ```
impl MoneyAmount<Money> for Money {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        Some(*self)
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

/// Implementation of `MoneyAmount<Money>` for `Decimal`.
///
/// Allows `Decimal` values to be used as amounts in money operations.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.00));
///
/// // Add Decimal to Money
/// let result = money.add(dec!(50.00)).unwrap();
/// assert_eq!(result.amount(), dec!(150.00));
/// ```
impl MoneyAmount<Money> for Decimal {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(*self)
    }
}

/// Implementation of `MoneyAmount<Money>` for `f64`.
///
/// Allows `f64` values to be used as amounts in money operations.
/// The value is converted to `Decimal` automatically.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.00));
///
/// // Add f64 to Money
/// let result = money.add(50.0).unwrap();
/// assert_eq!(result.amount(), dec!(150.00));
/// ```
impl MoneyAmount<Money> for f64 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_f64(*self)
    }
}

/// Implementation of `MoneyAmount<Money>` for `i32`.
///
/// Allows `i32` values to be used as amounts in money operations.
/// The value is converted to `Decimal` automatically.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.00));
///
/// // Add i32 to Money
/// let result = money.add(50_i32).unwrap();
/// assert_eq!(result.amount(), dec!(150.00));
/// ```
impl MoneyAmount<Money> for i32 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i32(*self)
    }
}

/// Implementation of `MoneyAmount<Money>` for `i64`.
///
/// Allows `i64` values to be used as amounts in money operations.
/// The value is converted to `Decimal` automatically.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.00));
///
/// // Add i64 to Money
/// let result = money.add(50_i64).unwrap();
/// assert_eq!(result.amount(), dec!(150.00));
/// ```
impl MoneyAmount<Money> for i64 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i64(*self)
    }
}

/// Implementation of `MoneyAmount<Money>` for `i128`.
///
/// Allows `i128` values to be used as amounts in money operations.
/// The value is converted to `Decimal` automatically.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, money_macros::dec, BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.00));
///
/// // Add i128 to Money
/// let result = money.add(50_i128).unwrap();
/// assert_eq!(result.amount(), dec!(150.00));
/// ```
impl MoneyAmount<Money> for i128 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i128(*self)
    }
}

// MoneyAmount ---

impl BaseMoney for Money {
    /// Get currency of money
    #[inline]
    fn currency(&self) -> Currency {
        self.currency
    }

    /// Get amount of money
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Round money using `Currency`'s rounding strategy to the scale of currency's minor unit
    #[inline]
    fn round(self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.round_dp_with_strategy(
                self.currency().minor_unit().into(),
                self.currency.rounding_strategy().into(),
            ),
        }
    }
}

impl BaseOps for Money {
    #[inline]
    fn abs(&self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.abs(),
        }
        .round()
    }

    #[inline]
    fn min(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.min(rhs.amount),
        }
        .round()
    }

    #[inline]
    fn max(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.max(rhs.amount),
        }
        .round()
    }

    /// clamp the money amount between `from` and `to` inclusively.
    #[inline]
    fn clamp(&self, from: Decimal, to: Decimal) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.clamp(from, to),
        }
        .round()
    }

    fn add<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_add(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn sub<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_sub(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn mul<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_mul(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn div<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_div(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }
}

impl CustomMoney for Money {
    #[inline]
    fn set_thousand_separator(&mut self, separator: &'static str) {
        self.currency.set_thousand_separator(separator);
    }

    #[inline]
    fn set_decimal_separator(&mut self, separator: &'static str) {
        self.currency.set_decimal_separator(separator);
    }

    #[inline]
    fn round_with(self, decimal_points: u32, strategy: crate::base::RoundingStrategy) -> Self {
        Self {
            currency: self.currency,
            amount: self
                .amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
        }
    }
}
