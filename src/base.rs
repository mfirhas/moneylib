use crate::Currency;
use crate::Decimal;
use crate::MoneyError;
use crate::fmt::{CODE_FORMAT, CODE_FORMAT_MINOR, SYMBOL_FORMAT, SYMBOL_FORMAT_MINOR, format};
use crate::money_macros::dec;
use rust_decimal::RoundingStrategy as DecimalRoundingStrategy;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{fmt::Debug, str::FromStr};

/// Base trait for all money types in the library.
///
/// This trait provides the fundamental operations and properties for working with monetary values.
/// It combines currency information with an amount and provides various methods for accessing
/// and formatting monetary data.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, USD};
/// use moneylib::money_macros::dec;
/// use moneylib::BaseMoney;
///
/// let money = Money::<USD>::new(dec!(1234.56)).unwrap();
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
pub trait BaseMoney<C: Currency>: Sized + Clone + FromStr {
    // REQUIRED

    /// Returns the decimal amount of this money value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(100.50)).unwrap();
    /// assert_eq!(money.amount(), dec!(100.50));
    /// ```
    fn amount(&self) -> Decimal;

    /// Rounds the money amount using bankers rounding rule to the scale of the currency's minor unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(123.456)).unwrap();
    /// assert_eq!(money.amount(), dec!(123.46));
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
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(100)).unwrap();
    /// assert_eq!(money.name(), "United States dollar");
    /// ```
    #[inline]
    fn name(&self) -> &str {
        C::NAME
    }

    /// Returns the currency symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, IDR, USD, EUR};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<IDR>::new(dec!(1000000)).unwrap();
    /// assert_eq!(money.symbol(), "Rp");
    ///
    /// let money = Money::<USD>::new(dec!(100)).unwrap();
    /// assert_eq!(money.symbol(), "$");
    ///
    /// let euro = Money::<EUR>::new(dec!(100)).unwrap();
    /// assert_eq!(euro.symbol(), "€");
    /// ```
    #[inline]
    fn symbol(&self) -> &str {
        C::SYMBOL
    }

    /// Returns the ISO 4217 currency code.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(100)).unwrap();
    /// assert_eq!(money.code(), "USD");
    /// ```
    #[inline]
    fn code(&self) -> &str {
        C::CODE
    }

    /// Returns the ISO 4217 numeric code for the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(100)).unwrap();
    /// assert_eq!(money.numeric_code(), 840);
    /// ```
    #[inline]
    fn numeric_code(&self) -> i32 {
        C::NUMERIC.into()
    }

    /// Returns the number of decimal places used by the currency's minor unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD, JPY, BHD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let usd = Money::<USD>::new(dec!(100)).unwrap();
    /// assert_eq!(usd.minor_unit(), 2);
    ///
    /// let yen = Money::<JPY>::new(dec!(100)).unwrap();
    /// assert_eq!(yen.minor_unit(), 0);
    ///
    /// let bhd = Money::<BHD>::new(dec!(100)).unwrap();
    /// assert_eq!(bhd.minor_unit(), 3);
    /// ```
    #[inline]
    fn minor_unit(&self) -> u16 {
        C::MINOR_UNIT
    }

    /// Returns the money amount in its smallest unit (e.g., cents for USD, pence for GBP).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD, JPY};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(10.50)).unwrap();
    /// assert_eq!(money.minor_amount().unwrap(), 1050);
    ///
    /// let yen = Money::<JPY>::new(dec!(100)).unwrap();
    /// assert_eq!(yen.minor_amount().unwrap(), 100);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::ArithmeticOverflow` if the calculation exceeds the maximum value.
    /// Returns `MoneyError::DecimalToInteger` if the conversion to integer fails.
    #[inline]
    fn minor_amount(&self) -> Result<i128, MoneyError> {
        self.amount()
            .checked_mul(
                dec!(10)
                    .checked_powu(self.minor_unit().into())
                    .ok_or(MoneyError::ArithmeticOverflow)?,
            )
            .ok_or(MoneyError::ArithmeticOverflow)?
            .to_i128()
            .ok_or(MoneyError::DecimalConversion)
    }

    /// Returns the thousands separator used by the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD, EUR};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(1000)).unwrap();
    /// assert_eq!(money.thousand_separator(), ",");
    /// let money = Money::<EUR>::new(dec!(1000)).unwrap();
    /// assert_eq!(money.thousand_separator(), ".");
    /// ```
    #[inline]
    fn thousand_separator(&self) -> &str {
        C::THOUSAND_SEPARATOR
    }

    /// Returns the decimal separator used by the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD, EUR};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(10.50)).unwrap();
    /// assert_eq!(money.decimal_separator(), ".");
    ///
    /// let money = Money::<EUR>::new(dec!(10.50)).unwrap();
    /// assert_eq!(money.decimal_separator(), ",");
    /// ```
    #[inline]
    fn decimal_separator(&self) -> &str {
        C::DECIMAL_SEPARATOR
    }

    /// Returns `true` if the amount is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let zero = Money::<USD>::new(dec!(0)).unwrap();
    /// assert!(zero.is_zero());
    ///
    /// let nonzero = Money::<USD>::new(dec!(1)).unwrap();
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
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let positive = Money::<USD>::new(dec!(10)).unwrap();
    /// assert!(positive.is_positive());
    ///
    /// let negative_money = Money::<USD>::new(dec!(-10)).unwrap();
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
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let negative = Money::<USD>::new(dec!(-10)).unwrap();
    /// assert!(negative.is_negative());
    ///
    /// let positive_money = Money::<USD>::new(dec!(10)).unwrap();
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
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(1234.45)).unwrap();
    /// assert_eq!(money.format_code(), "USD 1,234.45");
    ///
    /// let negative = Money::<USD>::new(dec!(-1234.45)).unwrap();
    /// assert_eq!(negative.format_code(), "USD -1,234.45");
    /// ```
    fn format_code(&self) -> String {
        format(self.to_owned(), CODE_FORMAT)
    }

    /// Formats money with currency symbol along with thousands and decimal separators.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD, EUR};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(1234.45)).unwrap();
    /// assert_eq!(money.format_symbol(), "$1,234.45");
    ///
    /// let negative = Money::<EUR>::new(dec!(-500.50)).unwrap();
    /// assert_eq!(negative.format_symbol(), "-€500,50");
    /// ```
    fn format_symbol(&self) -> String {
        format(self.to_owned(), SYMBOL_FORMAT)
    }

    /// Formats money with currency code in the smallest unit along with thousands separators.
    ///
    /// The amount is displayed in minor units (e.g., cents for USD).
    /// If the currency has no minor unit symbol, it defaults to "minor".
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(1234.45)).unwrap();
    /// assert_eq!(money.format_code_minor(), "USD 123,445 ¢");
    ///
    /// let negative = Money::<USD>::new(dec!(-1234.45)).unwrap();
    /// assert_eq!(negative.format_code_minor(), "USD -123,445 ¢");
    /// ```
    fn format_code_minor(&self) -> String {
        format(self.to_owned(), CODE_FORMAT_MINOR)
    }

    /// Formats money with currency symbol in the smallest unit along with thousands separators.
    ///
    /// The amount is displayed in minor units (e.g., cents for USD).
    /// If the currency has no minor unit symbol, it defaults to "minor".
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(1234.45)).unwrap();
    /// assert_eq!(money.format_symbol_minor(), "$123,445 ¢");
    ///
    /// let negative = Money::<USD>::new(dec!(-10.50)).unwrap();
    /// assert_eq!(negative.format_symbol_minor(), "-$1,050 ¢");
    /// ```
    fn format_symbol_minor(&self) -> String {
        format(self.to_owned(), SYMBOL_FORMAT_MINOR)
    }

    /// Returns the default display format for money (same as `format_code`).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(1234.45)).unwrap();
    /// assert_eq!(money.display(), "USD 1,234.45");
    /// ```
    fn display(&self) -> String {
        self.format_code()
    }
}

/// Trait for arithmetic and comparison operations on money values.
///
/// This trait extends `BaseMoney` with mathematical operations (addition, subtraction,
/// multiplication, division) and absolute value. All arithmetic operations ensure runtime safety like overflowed and wrapped values.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, USD};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, BaseOps};
///
/// let m1 = Money::<USD>::new(dec!(100)).unwrap();
/// let m2 = Money::<USD>::new(dec!(50)).unwrap();
///
/// // Arithmetic operations
/// let sum = m1.add(m2).unwrap();
/// assert_eq!(sum.amount(), dec!(150));
///
/// let diff = m1.sub(m2).unwrap();
/// assert_eq!(diff.amount(), dec!(50));
///
/// // Comparison operations
/// assert_eq!(m1.max(m2), m1);
/// assert_eq!(m1.min(m2), m2);
/// ```
pub trait BaseOps<C: Currency>:
    Sized
    + BaseMoney<C>
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
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let negative = Money::<USD>::new(dec!(-100)).unwrap();
    /// let positive = negative.abs();
    /// assert_eq!(positive.amount(), dec!(100));
    /// ```
    fn abs(&self) -> Self;

    /// Adds another money value to this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let m1 = Money::<USD>::new(dec!(100)).unwrap();
    /// let m2 = Money::<USD>::new(dec!(50)).unwrap();
    /// let sum = m1.add(m2).unwrap();
    /// assert_eq!(sum.amount(), dec!(150));
    /// ```
    fn add<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>;

    /// Subtracts another money value from this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let m1 = Money::<USD>::new(dec!(100)).unwrap();
    /// let m2 = Money::<USD>::new(dec!(30)).unwrap();
    /// let diff = m1.sub(m2).unwrap();
    /// assert_eq!(diff.amount(), dec!(70));
    /// ```
    fn sub<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>;

    /// Multiplies this money value by another value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let money = Money::<USD>::new(dec!(10)).unwrap();
    /// let product = money.mul(dec!(3)).unwrap();
    /// assert_eq!(product.amount(), dec!(30));
    /// ```
    fn mul<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>;

    /// Divides this money value by another value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let money = Money::<USD>::new(dec!(100)).unwrap();
    /// let quotient = money.div(dec!(4)).unwrap();
    /// assert_eq!(quotient.amount(), dec!(25));
    /// ```
    fn div<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: Amount<C>;
}

/// Trait for types that can represent a money amount.
///
/// This trait allows for flexible input types in constructing and arithmetic operations.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, USD};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, BaseOps};
///
/// let money = Money::<USD>::new(dec!(100)).unwrap();
/// let money2 = Money::<USD>::new(money).unwrap();
/// let money3 = Money::<USD>::new(123.000_f64).unwrap();
/// let money4 = Money::<USD>::new(123_i32).unwrap();
/// let money5 = Money::<USD>::new(123_i64).unwrap();
/// let money6 = Money::<USD>::new(123_i128).unwrap();
///
/// let check = money3 == money4;
/// assert!(check);
/// let check = money5 == money6;
/// assert!(check);
/// let check = money4 == money5;
/// assert!(check);
/// let check = money3 == money6;
/// assert!(check);
///
/// assert_eq!(money, money2);
/// assert_eq!(money.amount(), money2.amount());
/// ```
pub trait Amount<C: Currency>: Sized {
    /// Get decimal amount of Self.
    ///
    /// Returns `None` if Self cannot be converted into Decimal.
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
/// use moneylib::{Money, Currency, RoundingStrategy, USD};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, CustomMoney};
///
/// // Note: Money values are rounded to currency minor unit on creation,
/// // so we use round_with with more precision to demonstrate differences
/// let money1 = Money::<USD>::new(dec!(2.5)).unwrap();
/// let bankers = money1.round_with(0, RoundingStrategy::BankersRounding);
/// assert_eq!(bankers.amount(), dec!(2));  // Rounds to even
///
/// let money2 = Money::<USD>::new(dec!(2.5)).unwrap();
/// let half_up = money2.round_with(0, RoundingStrategy::HalfUp);
/// assert_eq!(half_up.amount(), dec!(3));  // Always rounds up at halfway
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
    /// use moneylib::{Money, Currency, RoundingStrategy, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// // 2.5 rounds to 2 (even)
    /// let m1 = Money::<USD>::new(dec!(2.5)).unwrap();
    /// let rounded = m1.round_with(0, RoundingStrategy::BankersRounding);
    /// assert_eq!(rounded.amount(), dec!(2));
    ///
    /// // 3.5 rounds to 4 (even)
    /// let m2 = Money::<USD>::new(dec!(3.5)).unwrap();
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
    /// use moneylib::{Money, Currency, RoundingStrategy, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// // 2.5 rounds to 3
    /// let m1 = Money::<USD>::new(dec!(2.5)).unwrap();
    /// let rounded = m1.round_with(0, RoundingStrategy::HalfUp);
    /// assert_eq!(rounded.amount(), dec!(3));
    ///
    /// // -2.5 rounds to -3
    /// let m2 = Money::<USD>::new(dec!(-2.5)).unwrap();
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
    /// use moneylib::{Money, Currency, RoundingStrategy, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// // 2.5 rounds to 2
    /// let m1 = Money::<USD>::new(dec!(2.5)).unwrap();
    /// let rounded = m1.round_with(0, RoundingStrategy::HalfDown);
    /// assert_eq!(rounded.amount(), dec!(2));
    ///
    /// // -2.5 rounds to -2
    /// let m2 = Money::<USD>::new(dec!(-2.5)).unwrap();
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
    /// use moneylib::{Money, Currency, RoundingStrategy, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// // 2.1 rounds to 3
    /// let m1 = Money::<USD>::new(dec!(2.1)).unwrap();
    /// let rounded = m1.round_with(0, RoundingStrategy::Ceil);
    /// assert_eq!(rounded.amount(), dec!(3));
    ///
    /// // -2.1 rounds to -3
    /// let m2 = Money::<USD>::new(dec!(-2.1)).unwrap();
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
    /// use moneylib::{Money, Currency, RoundingStrategy, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// // 2.9 rounds to 2
    /// let m1 = Money::<USD>::new(dec!(2.9)).unwrap();
    /// let rounded = m1.round_with(0, RoundingStrategy::Floor);
    /// assert_eq!(rounded.amount(), dec!(2));
    ///
    /// // -2.9 rounds to -2
    /// let m2 = Money::<USD>::new(dec!(-2.9)).unwrap();
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
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, RoundingStrategy, USD};
/// use moneylib::money_macros::dec;
/// use moneylib::{BaseMoney, CustomMoney};
///
/// let mut money = Money::<USD>::new(dec!(1234.56)).unwrap();
///
/// // Custom rounding
/// let value = Money::<USD>::new(dec!(123.456)).unwrap();
/// let rounded = value.round_with(2, RoundingStrategy::Floor);
/// assert_eq!(rounded.amount(), dec!(123.46));
/// ```
pub trait CustomMoney<C: Currency>: Sized + BaseMoney<C> {
    // REQUIRED

    /// Rounds the money amount to a specified number of decimal places using the given strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let money = Money::<USD>::new(dec!(123.456)).unwrap();
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
    /// use moneylib::{Money, Currency, USD};
    /// use moneylib::money_macros::dec;
    /// use moneylib::CustomMoney;
    ///
    /// let money = Money::<USD>::new(dec!(100.50)).unwrap();
    ///
    /// // Basic formatting
    /// // "USD 100.50"
    /// assert_eq!(money.format("c a"), "USD 100.50");
    ///
    /// // "$100.50"
    /// assert_eq!(money.format("sa"), "$100.50");
    ///
    /// assert_eq!(money.format("c nsa"), "USD $100.50");
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
    /// let negative = Money::<USD>::new(dec!(-50.00)).unwrap();
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
