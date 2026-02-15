use std::{fmt::Display, str::FromStr};

use crate::{base::MoneyAmount, money_macros::dec};
use rust_decimal::{MathematicalOps, prelude::FromPrimitive};

use crate::{
    BaseMoney, Currency, Decimal, Money, MoneyError, MoneyResult,
    base::{BaseOps, CustomMoney},
    parse::{parse_comma_thousands_separator, parse_dot_thousands_separator},
};

/// Represents a monetary value without automatic rounding.
///
/// `RawMoney` is similar to [`Money`] but does NOT automatically round amounts
/// after operations. This gives callers full control over when and how rounding occurs.
/// All decimal precision is preserved until explicitly rounded.
///
/// # Key Features
///
/// - **Type Safety**: Ensures currency consistency in operations
/// - **Precision**: Uses 128-bit fixed-precision decimal for accurate calculations
/// - **No Automatic Rounding**: Preserves all decimal places until explicitly rounded
/// - **Zero-Cost**: `Copy` type with no heap allocations
///
/// # Conversion
///
/// - Convert from `Money` using `money.into_raw()`
/// - Convert to `Money` using `raw_money.finish()` (applies rounding)
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, BaseMoney, money_macros::dec};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100.50));
/// 
/// // Convert to RawMoney to preserve precision during calculations
/// let raw = money.into_raw();
/// 
/// // Perform calculations without rounding
/// let result = raw * dec!(1.0 / 3.0);
/// 
/// // Convert back to Money when ready to round
/// let final_money = result.finish();
/// ```
///
/// # See Also
///
/// - [`Money`] for automatically-rounded monetary values
/// - [`BaseMoney`] trait for core money operations
/// - [`BaseOps`] trait for arithmetic and comparison operations
/// - [`CustomMoney`] trait for custom formatting and rounding
#[derive(Debug, Clone, Copy, Eq)]
pub struct RawMoney {
    currency: Currency,
    amount: Decimal,
}

impl RawMoney {
    /// Creates a new `RawMoney` instance with the specified currency and amount.
    ///
    /// Unlike `Money::new()`, this does NOT round the amount. All decimal precision
    /// is preserved exactly as provided.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency for this money value
    /// * `amount` - The decimal amount to store (NOT rounded)
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// 
    /// // Preserves all decimal places
    /// let raw = RawMoney::new(usd, dec!(100.567));
    /// assert_eq!(raw.amount(), dec!(100.567));
    /// 
    /// // No rounding even for currencies with limited minor units
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// let raw_jpy = RawMoney::new(jpy, dec!(100.567));
    /// assert_eq!(raw_jpy.amount(), dec!(100.567));
    /// ```
    #[inline]
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        RawMoney { currency, amount }
    }

    /// Converts this `RawMoney` to `Money`, applying rounding.
    ///
    /// This method rounds the amount to the currency's minor unit precision
    /// using the currency's rounding strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let raw = RawMoney::new(usd, dec!(100.567));
    /// 
    /// // Convert to Money with rounding
    /// let money = raw.finish();
    /// assert_eq!(money.amount(), dec!(100.57));
    /// ```
    #[inline]
    pub fn finish(self) -> Money {
        Money::new(self.currency, self.amount)
    }

    /// Creates a new `RawMoney` instance from a generic amount type.
    ///
    /// This method accepts any type that implements [`MoneyAmount`], including:
    /// - `Money` or `RawMoney` - validates currency matches
    /// - `Decimal` - creates raw money with the given amount
    /// - `f64`, `i32`, `i64`, `i128` - converts to decimal and creates raw money
    ///
    /// # Arguments
    ///
    /// * `currency` - The target currency for the money
    /// * `amount` - A value that can be converted to a money amount
    ///
    /// # Returns
    ///
    /// Returns `Ok(RawMoney)` if the conversion succeeds and currencies match (if applicable).
    /// Returns `Err(MoneyError)` if:
    /// - The amount type cannot be converted to a valid money amount
    /// - The amount is `Money` or `RawMoney` with a different currency than specified
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, Money, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // From RawMoney
    /// let raw1 = RawMoney::new(usd, dec!(100.567));
    /// let raw2 = RawMoney::from_amount(usd, raw1).unwrap();
    /// assert_eq!(raw2.amount(), dec!(100.567));
    ///
    /// // From existing Money (converts amount, preserving rounded precision)
    /// let money = Money::new(usd, dec!(50.25));
    /// let raw = RawMoney::new(usd, money.amount());
    /// assert_eq!(raw.amount(), dec!(50.25));
    ///
    /// // Error: currency mismatch
    /// let eur = Currency::from_iso("EUR").unwrap();
    /// let eur_raw = RawMoney::new(eur, dec!(50.25));
    /// assert!(RawMoney::from_amount(usd, eur_raw).is_err());
    /// ```
    pub fn from_amount<T>(currency: Currency, amount: T) -> MoneyResult<Self>
    where
        T: MoneyAmount<RawMoney>,
    {
        match (amount.get_money(), amount.get_decimal()) {
            (Some(money), _) if money.currency() == currency => Ok(money),
            (None, Some(amount)) => Ok(Self::new(currency, amount)),
            _ => Err(MoneyError::NewMoney(
                "amount type is invalid or or money's currency mismatches".into(),
            )),
        }
    }

    /// Creates a new `RawMoney` instance from an amount expressed in the currency's minor unit.
    ///
    /// The minor unit is the smallest denomination of the currency (e.g., cents for USD,
    /// pence for GBP). This method converts the minor amount to the standard amount by
    /// dividing by 10^(minor_unit).
    ///
    /// Unlike `Money::from_minor_amount()`, this does NOT round the result.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency for this money value
    /// * `minor_amount` - The amount in the currency's smallest unit
    ///
    /// # Returns
    ///
    /// Returns `Ok(RawMoney)` if the conversion succeeds.
    /// Returns `Err(MoneyError)` if:
    /// - The minor amount cannot be converted to decimal
    /// - The conversion calculation overflows
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // USD has 2 decimal places, so 12345 cents = $123.45
    /// let raw = RawMoney::from_minor_amount(usd, 12345).unwrap();
    /// assert_eq!(raw.amount(), dec!(123.45));
    ///
    /// // JPY has 0 decimal places, so the minor amount equals the amount
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// let raw = RawMoney::from_minor_amount(jpy, 1000).unwrap();
    /// assert_eq!(raw.amount(), dec!(1000));
    ///
    /// // BHD has 3 decimal places
    /// let bhd = Currency::from_iso("BHD").unwrap();
    /// let raw = RawMoney::from_minor_amount(bhd, 12345).unwrap();
    /// assert_eq!(raw.amount(), dec!(12.345));
    ///
    /// // Negative amounts are supported
    /// let raw = RawMoney::from_minor_amount(usd, -12345).unwrap();
    /// assert_eq!(raw.amount(), dec!(-123.45));
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
            .ok_or(MoneyError::ArithmeticOverflow)?;

        Ok(Self::new(currency, amount))
    }
}

impl BaseMoney for RawMoney {
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
    ///
    /// Note: Unlike `Money`, `RawMoney` operations don't automatically round.
    /// This method is provided to allow explicit rounding when needed.
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

impl BaseOps for RawMoney {
    #[inline]
    fn abs(&self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.abs(),
        }
    }

    #[inline]
    fn min(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.min(rhs.amount),
        }
    }

    #[inline]
    fn max(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.max(rhs.amount),
        }
    }

    /// clamp the money amount between `from` and `to` inclusively.
    #[inline]
    fn clamp(&self, from: Decimal, to: Decimal) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.clamp(from, to),
        }
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

impl CustomMoney for RawMoney {
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

// Implement Display using the same formatting logic as Money
impl Display for RawMoney {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::fmt::format(*self, crate::fmt::CODE_FORMAT))
    }
}

// Implement PartialEq
impl PartialEq for RawMoney {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}

// Implement PartialOrd
impl PartialOrd for RawMoney {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency != other.currency {
            None
        } else {
            self.amount.partial_cmp(&other.amount)
        }
    }
}

// Implement FromStr using the same parsing logic as Money
impl FromStr for RawMoney {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing with comma thousands separator first (e.g., "USD 1,234.56")
        if let Some((code, amount_str)) = parse_comma_thousands_separator(s) {
            let currency = Currency::from_iso(code)?;
            let amount = Decimal::from_str(&amount_str)
                .map_err(|_| MoneyError::ParseStr)?;
            return Ok(Self::new(currency, amount));
        }

        // Try parsing with dot thousands separator (e.g., "USD 1.234,56")
        if let Some((code, amount_str)) = parse_dot_thousands_separator(s) {
            let currency = Currency::from_iso(code)?;
            let amount = Decimal::from_str(&amount_str)
                .map_err(|_| MoneyError::ParseStr)?;
            return Ok(Self::new(currency, amount));
        }

        Err(MoneyError::ParseStr)
    }
}

// Add MoneyAmount implementation for RawMoney
impl MoneyAmount<RawMoney> for RawMoney {
    fn get_money(&self) -> Option<RawMoney> {
        Some(*self)
    }

    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

// Add Money conversion method
impl Money {
    /// Converts this `Money` to `RawMoney`, preserving the current amount.
    ///
    /// The resulting `RawMoney` will not automatically round during operations,
    /// giving full control over when rounding occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100.50));
    /// 
    /// // Convert to RawMoney
    /// let raw = money.into_raw();
    /// assert_eq!(raw.amount(), dec!(100.50));
    /// 
    /// // Perform calculations without auto-rounding
    /// let result = raw * dec!(1.0 / 3.0);
    /// 
    /// // Convert back when ready to round
    /// let final_money = result.finish();
    /// ```
    #[inline]
    pub fn into_raw(self) -> RawMoney {
        RawMoney::new(self.currency(), self.amount())
    }
}
