use crate::fmt::{
    CODE_FORMAT_NEGATIVE, CODE_FORMAT_NEGATIVE_MINOR, CODE_FORMAT_POSITIVE,
    CODE_FORMAT_POSITIVE_MINOR, SYMBOL_FORMAT_NEGATIVE, SYMBOL_FORMAT_NEGATIVE_MINOR,
    SYMBOL_FORMAT_POSITIVE, SYMBOL_FORMAT_POSITIVE_MINOR, format,
};
use crate::money_macros::dec;
use crate::{Country, Currency, MoneyError};
use crate::{Decimal, MoneyResult};
use rust_decimal::RoundingStrategy as DecimalRoundingStrategy;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{fmt::Debug, str::FromStr};

pub(crate) const COMMA_SEPARATOR: &str = ",";

pub(crate) const DOT_SEPARATOR: &str = ".";

/// Base trait for all money types in the library.
///
/// This trait provides the fundamental operations and properties for working with monetary values.
/// It combines currency information with an amount and provides various methods for accessing
/// and formatting monetary data.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency};
/// use moneylib::money_macros::dec;
/// use moneylib::BaseMoney;
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(1234.56));
///
/// // Access currency information
/// assert_eq!(money.code(), "USD");
/// assert_eq!(money.symbol(), "$");
/// assert_eq!(money.name(), "United States dollar");
/// assert_eq!(money.minor_unit(), 2);
///
/// // Check amount properties
/// assert!(money.is_positive());
/// assert!(!money.is_negative());
/// assert!(!money.is_zero());
///
/// // Format money in different ways
/// assert_eq!(money.format_code(), "USD 1,234.56");
/// assert_eq!(money.format_symbol(), "$1,234.56");
/// ```
pub trait BaseMoney:
    Sized + Debug + Display + Clone + PartialOrd + PartialEq + Eq + FromStr
{
    // REQUIRED

    /// Returns the currency of this money value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd.clone(), dec!(100));
    /// assert_eq!(money.currency(), usd);
    /// ```
    fn currency(&self) -> Currency;

    /// Returns the decimal amount of this money value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100.50));
    /// assert_eq!(money.amount(), dec!(100.50));
    /// ```
    fn amount(&self) -> Decimal;

    /// Rounds the money amount using the currency's rounding strategy to the scale of the currency's minor unit.
    ///
    /// The rounding strategy is determined by the currency's configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(123.456));
    /// let rounded = money.round();
    /// assert_eq!(rounded.amount(), dec!(123.46));
    /// ```
    fn round(self) -> Self;

    // PROVIDED

    /// Returns the full name of the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// assert_eq!(money.name(), "United States dollar");
    /// ```
    #[inline]
    fn name(&self) -> &str {
        self.currency().name()
    }

    /// Returns the currency symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// assert_eq!(money.symbol(), "$");
    ///
    /// let eur = Currency::from_iso("EUR").unwrap();
    /// let euro = Money::new(eur, dec!(100));
    /// assert_eq!(euro.symbol(), "€");
    /// ```
    #[inline]
    fn symbol(&self) -> &str {
        self.currency().symbol()
    }

    /// Returns the ISO 4217 currency code.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// assert_eq!(money.code(), "USD");
    /// ```
    #[inline]
    fn code(&self) -> &str {
        self.currency().code()
    }

    /// Returns the ISO 4217 numeric code for the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// assert_eq!(money.numeric_code(), 840);
    /// ```
    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency().numeric_code()
    }

    /// Returns the number of decimal places used by the currency's minor unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// assert_eq!(money.minor_unit(), 2);
    ///
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// let yen = Money::new(jpy, dec!(100));
    /// assert_eq!(yen.minor_unit(), 0);
    /// ```
    #[inline]
    fn minor_unit(&self) -> u16 {
        self.currency().minor_unit()
    }

    /// Returns the money amount in its smallest unit (e.g., cents for USD, pence for GBP).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(10.50));
    /// assert_eq!(money.minor_amount().unwrap(), 1050);
    ///
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// let yen = Money::new(jpy, dec!(100));
    /// assert_eq!(yen.minor_amount().unwrap(), 100);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::ArithmeticOverflow` if the calculation exceeds the maximum value.
    /// Returns `MoneyError::DecimalToInteger` if the conversion to integer fails.
    #[inline]
    fn minor_amount(&self) -> MoneyResult<i128> {
        self.amount()
            .checked_mul(
                dec!(10)
                    .checked_powu(self.minor_unit().into())
                    .ok_or(MoneyError::ArithmeticOverflow)?,
            )
            .ok_or(MoneyError::ArithmeticOverflow)?
            .to_i128()
            .ok_or(MoneyError::DecimalToInteger)
    }

    /// Returns the thousands separator used by the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(1000));
    /// assert_eq!(money.thousand_separator(), ",");
    /// ```
    #[inline]
    fn thousand_separator(&self) -> &'static str {
        self.currency().thousand_separator()
    }

    /// Returns the decimal separator used by the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(10.50));
    /// assert_eq!(money.decimal_separator(), ".");
    /// ```
    #[inline]
    fn decimal_separator(&self) -> &'static str {
        self.currency().decimal_separator()
    }

    /// Returns `true` if the amount is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let zero = Money::new(usd.clone(), dec!(0));
    /// assert!(zero.is_zero());
    ///
    /// let nonzero = Money::new(usd, dec!(1));
    /// assert!(!nonzero.is_zero());
    /// ```
    #[inline]
    fn is_zero(&self) -> bool {
        self.amount().is_zero()
    }

    /// Returns `true` if the amount is positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let positive = Money::new(usd.clone(), dec!(10));
    /// assert!(positive.is_positive());
    ///
    /// let negative_money = Money::new(usd, dec!(-10));
    /// assert!(!negative_money.is_positive());
    /// ```
    #[inline]
    fn is_positive(&self) -> bool {
        self.amount().is_sign_positive()
    }

    /// Returns `true` if the amount is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let negative = Money::new(usd.clone(), dec!(-10));
    /// assert!(negative.is_negative());
    ///
    /// let positive_money = Money::new(usd, dec!(10));
    /// assert!(!positive_money.is_negative());
    /// ```
    #[inline]
    fn is_negative(&self) -> bool {
        self.amount().is_sign_negative()
    }

    /// Formats money with currency code along with thousands and decimal separators.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd.clone(), dec!(1234.45));
    /// assert_eq!(money.format_code(), "USD 1,234.45");
    ///
    /// let negative = Money::new(usd, dec!(-1234.45));
    /// assert_eq!(negative.format_code(), "USD -1,234.45");
    /// ```
    fn format_code(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), CODE_FORMAT_NEGATIVE);
        }
        format(self.to_owned(), CODE_FORMAT_POSITIVE)
    }

    /// Formats money with currency symbol along with thousands and decimal separators.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(1234.45));
    /// assert_eq!(money.format_symbol(), "$1,234.45");
    ///
    /// let eur = Currency::from_iso("EUR").unwrap();
    /// let negative = Money::new(eur, dec!(-500.50));
    /// assert_eq!(negative.format_symbol(), "-€500.50");
    /// ```
    fn format_symbol(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), SYMBOL_FORMAT_NEGATIVE);
        }
        format(self.to_owned(), SYMBOL_FORMAT_POSITIVE)
    }

    /// Formats money with currency code in the smallest unit along with thousands separators.
    ///
    /// The amount is displayed in minor units (e.g., cents for USD).
    /// If the currency has no minor unit symbol, it defaults to "minor".
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd.clone(), dec!(1234.45));
    /// assert_eq!(money.format_code_minor(), "USD 123,445 ¢");
    ///
    /// let negative = Money::new(usd, dec!(-1234.45));
    /// assert_eq!(negative.format_code_minor(), "USD -123,445 ¢");
    /// ```
    fn format_code_minor(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), CODE_FORMAT_NEGATIVE_MINOR);
        }
        format(self.to_owned(), CODE_FORMAT_POSITIVE_MINOR)
    }

    /// Formats money with currency symbol in the smallest unit along with thousands separators.
    ///
    /// The amount is displayed in minor units (e.g., cents for USD).
    /// If the currency has no minor unit symbol, it defaults to "minor".
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(1234.45));
    /// assert_eq!(money.format_symbol_minor(), "$123,445 ¢");
    ///
    /// let negative = Money::new(usd.clone(), dec!(-10.50));
    /// assert_eq!(negative.format_symbol_minor(), "-$1,050 ¢");
    /// ```
    fn format_symbol_minor(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), SYMBOL_FORMAT_NEGATIVE_MINOR);
        }
        format(self.to_owned(), SYMBOL_FORMAT_POSITIVE_MINOR)
    }

    /// Returns the default display format for money (same as `format_code`).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(1234.45));
    /// assert_eq!(money.display(), "USD 1,234.45");
    /// ```
    fn display(&self) -> String {
        self.format_code()
    }

    /// Returns a list of countries that use this currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// let countries = money.countries();
    /// assert!(countries.is_some());
    /// ```
    fn countries(&self) -> Option<Vec<Country>> {
        self.currency().countries()
    }
}

/// Trait for arithmetic and comparison operations on money values.
///
/// This trait extends `BaseMoney` with mathematical operations (addition, subtraction,
/// multiplication, division) and comparison methods. All operations ensure currency
/// compatibility and return appropriate errors when currencies don't match.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let m1 = Money::new(usd.clone(), dec!(100));
/// let m2 = Money::new(usd, dec!(50));
///
/// // Arithmetic operations
/// let sum = m1.add(m2.clone()).unwrap();
/// assert_eq!(sum.amount(), dec!(150));
///
/// let diff = m1.sub(m2.clone()).unwrap();
/// assert_eq!(diff.amount(), dec!(50));
///
/// // Comparison operations
/// assert!(m1.is_bigger(m2.clone()).unwrap());
/// assert!(!m1.is_smaller(m2).unwrap());
/// ```
pub trait BaseOps:
    Sized
    + BaseMoney
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg<Output = Self>
{
    // REQUIRED

    /// Returns the absolute value of the money amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let negative = Money::new(usd, dec!(-100));
    /// let positive = negative.abs();
    /// assert_eq!(positive.amount(), dec!(100));
    /// ```
    fn abs(&self) -> Self;

    /// Returns the minimum of two money values.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(100));
    /// let m2 = Money::new(usd, dec!(50));
    /// let minimum = m1.min(m2);
    /// assert_eq!(minimum.amount(), dec!(50));
    /// ```
    fn min(&self, rhs: Self) -> Self;

    /// Returns the maximum of two money values.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(100));
    /// let m2 = Money::new(usd, dec!(50));
    /// let maximum = m1.max(m2);
    /// assert_eq!(maximum.amount(), dec!(100));
    /// ```
    fn max(&self, rhs: Self) -> Self;

    /// Clamps the money amount between `from` and `to` inclusively.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// let clamped = money.clamp(dec!(20), dec!(80));
    /// assert_eq!(clamped.amount(), dec!(80));
    /// ```
    fn clamp(&self, from: Decimal, to: Decimal) -> Self;

    /// Adds another money value to this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(100));
    /// let m2 = Money::new(usd, dec!(50));
    /// let sum = m1.add(m2).unwrap();
    /// assert_eq!(sum.amount(), dec!(150));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if the currencies don't match.
    fn add<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    /// Subtracts another money value from this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(100));
    /// let m2 = Money::new(usd, dec!(30));
    /// let diff = m1.sub(m2).unwrap();
    /// assert_eq!(diff.amount(), dec!(70));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if the currencies don't match.
    fn sub<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    /// Multiplies this money value by another value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(10));
    /// let product = money.mul(dec!(3)).unwrap();
    /// assert_eq!(product.amount(), dec!(30));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if multiplying by money with different currency.
    fn mul<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    /// Divides this money value by another value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100));
    /// let quotient = money.div(dec!(4)).unwrap();
    /// assert_eq!(quotient.amount(), dec!(25));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if dividing by money with different currency.
    fn div<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    // PROVIDED

    /// Returns `true` if this money value is greater than another.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(100));
    /// let m2 = Money::new(usd, dec!(50));
    /// assert!(m1.is_bigger(m2).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if the currencies don't match.
    fn is_bigger(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() > rhs.amount())
    }

    /// Returns `true` if this money value is less than another.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(50));
    /// let m2 = Money::new(usd, dec!(100));
    /// assert!(m1.is_smaller(m2).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if the currencies don't match.
    fn is_smaller(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() < rhs.amount())
    }

    /// Returns `true` if this money value is greater than or equal to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(100));
    /// let m2 = Money::new(usd, dec!(100));
    /// assert!(m1.is_bigger_equal(m2).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if the currencies don't match.
    fn is_bigger_equal(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() >= rhs.amount())
    }

    /// Returns `true` if this money value is less than or equal to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let m1 = Money::new(usd.clone(), dec!(50));
    /// let m2 = Money::new(usd, dec!(50));
    /// assert!(m1.is_smaller_equal(m2).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::CurrencyMismatch` if the currencies don't match.
    fn is_smaller_equal(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() <= rhs.amount())
    }
}

/// Trait for types that can represent a money amount.
///
/// This trait allows for flexible input types in arithmetic operations. A type can represent
/// either a money value or a decimal number for scaling operations.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, BaseOps};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(100));
///
/// // Can add another money value
/// let other = Money::new(money.currency(), dec!(50));
/// let sum = money.add(other).unwrap();
/// assert_eq!(sum.amount(), dec!(150));
///
/// // Can multiply by a decimal
/// let scaled = money.mul(dec!(2)).unwrap();
/// assert_eq!(scaled.amount(), dec!(200));
/// ```
pub trait MoneyAmount<T>: Sized
where
    T: BaseMoney,
{
    /// Returns the money value if this type represents money, otherwise `None`.
    fn get_money(&self) -> Option<T>;

    /// Returns the decimal value if this type represents a decimal, otherwise `None`.
    fn get_decimal(&self) -> Option<Decimal>;
}

/// Defines the strategy for rounding decimal money amounts.
///
/// Different rounding strategies can produce different results when rounding values that fall
/// exactly between two possible rounded values (e.g., 2.5 rounded to one decimal place).
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, RoundingStrategy};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, CustomMoney};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let money = Money::new(usd, dec!(123.456));
///
/// // Round using different strategies
/// let floor = money.clone().round_with(2, RoundingStrategy::Floor);
/// assert_eq!(floor.amount(), dec!(123.46));
///
/// let ceil = money.clone().round_with(2, RoundingStrategy::Ceil);
/// assert_eq!(ceil.amount(), dec!(123.46));
///
/// let half_up = money.clone().round_with(2, RoundingStrategy::HalfUp);
/// assert_eq!(half_up.amount(), dec!(123.46));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoundingStrategy {
    /// Banker's Rounding (Round Half to Even).
    ///
    /// Rounds to the nearest even number when the value is exactly halfway between two numbers.
    /// This is the default rounding strategy and helps reduce cumulative rounding errors in
    /// financial calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // 2.5 rounds to 2 (even)
    /// let m1 = Money::new(usd.clone(), dec!(2.5));
    /// let rounded = m1.round_with(0, RoundingStrategy::BankersRounding);
    /// assert_eq!(rounded.amount(), dec!(2));
    ///
    /// // 3.5 rounds to 4 (even)
    /// let m2 = Money::new(usd, dec!(3.5));
    /// let rounded = m2.round_with(0, RoundingStrategy::BankersRounding);
    /// assert_eq!(rounded.amount(), dec!(4));
    /// ```
    #[default]
    BankersRounding,

    /// Rounds half values away from zero.
    ///
    /// When a value is exactly halfway between two numbers, it rounds away from zero.
    /// This is the common rounding taught in schools.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // 2.5 rounds to 3
    /// let m1 = Money::new(usd.clone(), dec!(2.5));
    /// let rounded = m1.round_with(0, RoundingStrategy::HalfUp);
    /// assert_eq!(rounded.amount(), dec!(3));
    ///
    /// // -2.5 rounds to -3
    /// let m2 = Money::new(usd, dec!(-2.5));
    /// let rounded = m2.round_with(0, RoundingStrategy::HalfUp);
    /// assert_eq!(rounded.amount(), dec!(-3));
    /// ```
    HalfUp,

    /// Rounds half values toward zero.
    ///
    /// When a value is exactly halfway between two numbers, it rounds toward zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // 2.5 rounds to 2
    /// let m1 = Money::new(usd.clone(), dec!(2.5));
    /// let rounded = m1.round_with(0, RoundingStrategy::HalfDown);
    /// assert_eq!(rounded.amount(), dec!(2));
    ///
    /// // -2.5 rounds to -2
    /// let m2 = Money::new(usd, dec!(-2.5));
    /// let rounded = m2.round_with(0, RoundingStrategy::HalfDown);
    /// assert_eq!(rounded.amount(), dec!(-2));
    /// ```
    HalfDown,

    /// Rounds away from zero (toward positive/negative infinity).
    ///
    /// Always rounds to the next number away from zero, regardless of the fractional part.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // 2.1 rounds to 3
    /// let m1 = Money::new(usd.clone(), dec!(2.1));
    /// let rounded = m1.round_with(0, RoundingStrategy::Ceil);
    /// assert_eq!(rounded.amount(), dec!(3));
    ///
    /// // -2.1 rounds to -3
    /// let m2 = Money::new(usd, dec!(-2.1));
    /// let rounded = m2.round_with(0, RoundingStrategy::Ceil);
    /// assert_eq!(rounded.amount(), dec!(-3));
    /// ```
    Ceil,

    /// Rounds toward zero (truncates).
    ///
    /// Always rounds to the next number closer to zero, effectively truncating the decimal part.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    ///
    /// // 2.9 rounds to 2
    /// let m1 = Money::new(usd.clone(), dec!(2.9));
    /// let rounded = m1.round_with(0, RoundingStrategy::Floor);
    /// assert_eq!(rounded.amount(), dec!(2));
    ///
    /// // -2.9 rounds to -2
    /// let m2 = Money::new(usd, dec!(-2.9));
    /// let rounded = m2.round_with(0, RoundingStrategy::Floor);
    /// assert_eq!(rounded.amount(), dec!(-2));
    /// ```
    Floor,
}

impl From<RoundingStrategy> for DecimalRoundingStrategy {
    fn from(value: RoundingStrategy) -> Self {
        match value {
            RoundingStrategy::BankersRounding => DecimalRoundingStrategy::MidpointNearestEven,
            RoundingStrategy::HalfUp => DecimalRoundingStrategy::MidpointAwayFromZero,
            RoundingStrategy::HalfDown => DecimalRoundingStrategy::MidpointTowardZero,
            RoundingStrategy::Ceil => DecimalRoundingStrategy::AwayFromZero,
            RoundingStrategy::Floor => DecimalRoundingStrategy::ToZero,
        }
    }
}

/// Trait for customizing money formatting and rounding behavior.
///
/// This trait extends `BaseMoney` with methods to customize how money is displayed and rounded.
/// It allows you to change separators and apply specific rounding strategies.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, RoundingStrategy};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, CustomMoney};
///
/// let usd = Currency::from_iso("USD").unwrap();
/// let mut money = Money::new(usd, dec!(1234.56));
///
/// // Customize separators
/// money.set_thousand_separator(".");
/// money.set_decimal_separator(",");
/// assert_eq!(money.format_code(), "USD 1.234,56");
///
/// // Custom rounding
/// let value = Money::new(money.currency(), dec!(123.456));
/// let rounded = value.round_with(2, RoundingStrategy::Floor);
/// assert_eq!(rounded.amount(), dec!(123.46));
/// ```
pub trait CustomMoney: Sized + BaseMoney {
    // REQUIRED

    /// Sets the thousands separator for formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let mut money = Money::new(usd, dec!(1234.56));
    ///
    /// money.set_thousand_separator(" ");
    /// assert_eq!(money.format_code(), "USD 1 234.56");
    /// ```
    fn set_thousand_separator(&mut self, separator: &'static str);

    /// Sets the decimal separator for formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let mut money = Money::new(usd, dec!(1234.56));
    ///
    /// money.set_decimal_separator(",");
    /// assert_eq!(money.format_code(), "USD 1,234,56");
    /// ```
    fn set_decimal_separator(&mut self, separator: &'static str);

    /// Rounds the money amount to a specified number of decimal places using the given strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(123.456));
    ///
    /// let rounded = money.round_with(2, RoundingStrategy::Floor);
    /// assert_eq!(rounded.amount(), dec!(123.46));
    /// ```
    fn round_with(self, decimal_points: u32, strategy: RoundingStrategy) -> Self;

    // PROVIDED

    /// Format money according to the provided format string `f`.
    ///
    /// Format symbols:
    /// - 'a': amount (displayed as absolute value)
    /// - 'c': currency code (e.g., "USD")
    /// - 's': currency symbol (e.g., "$")
    /// - 'm': minor symbol (e.g., "cents")
    /// - 'n': negative sign (-), only displayed when amount is negative
    ///
    /// # Escaping Format Symbols
    ///
    /// To display format symbols as literal characters, prefix them with a backslash (\).
    /// This allows you to:
    /// 1. Insert literal format symbol characters (a, c, s, m, n) into the output
    /// 2. Mix escaped symbols with actual format symbols in the same string
    ///
    /// Escape sequences:
    /// - `\a` outputs literal "a"
    /// - `\c` outputs literal "c"
    /// - `\s` outputs literal "s"
    /// - `\m` outputs literal "m"
    /// - `\n` outputs literal "n"
    /// - `\\` (double backslash in source) outputs literal "\"
    /// - `\x` (where x is not a format symbol or backslash) outputs literal "\x"
    ///
    /// # Arguments
    ///
    /// * `money` - The Money value to format
    /// * `format_str` - The format string containing format symbols and optional literal text
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency};
    /// use moneylib::money_macros::dec;
    /// use moneylib::CustomMoney;
    ///
    /// let currency = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(currency, dec!(100.50));
    ///
    /// // Basic formatting
    /// // "USD 100.50"
    /// assert_eq!(money.format("c a"), "USD 100.50");
    ///
    /// // "$100.50"
    /// assert_eq!(money.format("sa"), "$100.50");
    ///
    /// // "USD 10,050 ¢" (amount in minor units when 'm' is present)
    /// assert_eq!(money.format("c a m"), "USD 10,050 ¢");
    ///
    /// // adding `n` to positive money will be ignored
    /// assert_eq!(money.format("c na"), "USD 100.50");
    ///
    /// // Mixing literals with format symbols
    /// // "Total: $100.50"
    /// assert_eq!(money.format("Tot\\al: sa"), "Total: $100.50");
    ///
    /// // Escaping format symbols to display them as literals
    /// // "a=100.50, c=USD"
    /// assert_eq!(money.format("\\a=a, \\c=c"), "a=100.50, c=USD");
    ///
    /// let negative = Money::new(currency, dec!(-50.00));
    /// // "USD -50.00"
    /// assert_eq!(negative.format("c na"), "USD -50.00");
    /// // "-$50.00"
    /// assert_eq!(negative.format("nsa"), "-$50.00");
    ///
    /// // not specifying the `n` for negative sign will omit the negative sign.
    /// assert_eq!(negative.format("sa"), "$50.00")
    ///
    ///
    /// ```
    fn format(&self, f: &str) -> String {
        format(self.to_owned(), f)
    }
}
