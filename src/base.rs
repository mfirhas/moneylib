use crate::Currency;
use crate::Decimal;
use crate::MoneyError;
use crate::fmt::format_with_separator;
use crate::fmt::{CODE_FORMAT, CODE_FORMAT_MINOR, SYMBOL_FORMAT, SYMBOL_FORMAT_MINOR, format};
use crate::macros::dec;
use rust_decimal::RoundingStrategy as DecimalRoundingStrategy;
use rust_decimal::{MathematicalOps, prelude::FromPrimitive, prelude::ToPrimitive};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{fmt::Debug, str::FromStr};

#[cfg(feature = "locale")]
use crate::fmt::format_with_amount;

/// Base trait for all money types in the library.
///
/// This trait provides the fundamental operations and properties for working with monetary values.
/// It combines currency information with an amount and provides various methods for accessing
/// and formatting monetary data.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, iso::USD};
/// use moneylib::macros::dec;
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

    /// Creates a new `Money` instance with amount of `Decimal`, `f64`, `i32`, `i64`, `i128`.
    ///
    /// The amount is automatically rounded to the currency's minor unit precision
    /// using the bankers rounding rule.
    ///
    /// # Arguments
    ///
    /// * `amount: impl DecimalNumber` - The amount of money accepting `Decimal`, `f64`, `i32`, `i64`, `i128`
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, RawMoney, Currency, macros::dec, BaseMoney, iso::{USD, EUR, JPY}};
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
    ///
    /// let raw_money = RawMoney::<EUR>::new(dec!(123.2323)).unwrap();
    /// assert_eq!(raw_money.amount(), dec!(123.2323));
    /// ```
    fn new(amount: impl DecimalNumber) -> Result<Self, MoneyError>;

    /// Returns the decimal amount of this money value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::BaseMoney;
    ///
    /// let money = Money::<USD>::new(dec!(123.456)).unwrap();
    /// assert_eq!(money.amount(), dec!(123.46));
    /// let rounded = money.round();
    /// assert_eq!(rounded.amount(), dec!(123.46));
    /// ```
    fn round(self) -> Self;

    /// Rounds the money amount to a specified number of decimal places using the given strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::{BaseMoney, CustomMoney};
    ///
    /// let money = Money::<USD>::new(dec!(123.456)).unwrap();
    ///
    /// let rounded = money.round_with(2, RoundingStrategy::Floor);
    /// assert_eq!(rounded.amount(), dec!(123.46));
    /// ```
    fn round_with(self, decimal_points: u32, strategy: RoundingStrategy) -> Self;

    // PROVIDED

    /// Returns the full name of the currency.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::{IDR, USD, EUR}};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::{USD, JPY, BHD}};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::{USD, JPY}};
    /// use moneylib::macros::dec;
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
    /// Returns `MoneyError::DecimalConversion` if the conversion to integer fails.
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
    /// use moneylib::{Money, Currency, iso::{USD, EUR}};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::{USD, EUR}};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::{USD, EUR}};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
/// use moneylib::{Money, Currency, iso::USD};
/// use moneylib::macros::dec;
/// use moneylib::{BaseMoney, BaseOps};
///
/// let m1 = Money::<USD>::new(dec!(100)).unwrap();
/// let m2 = Money::<USD>::new(dec!(50)).unwrap();
///
/// // Arithmetic operations
/// let sum = m1.checked_add(m2).unwrap();
/// assert_eq!(sum.amount(), dec!(150));
///
/// let diff = m1.checked_sub(m2).unwrap();
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
    // PROVIDED

    /// Compare 2 moneys within tolerance(inclusive).
    ///
    /// # Arguments
    /// - m: `impl BaseMoney<C>`, applied for `Money<C>` and `RawMoney<C>`
    /// - tolerance: `impl DecimalNumber`, if return `None`, false returned.
    ///
    /// ```rust
    /// use moneylib::{Money, BaseOps, BaseMoney, iso::USD, macros::dec};
    ///
    /// let calculated = Money::<USD>::from_decimal(dec!(100.01));
    /// let expected = Money::<USD>::from_decimal(dec!(100.00));
    /// // Check within $0.05 tolerance
    /// let is_close = calculated.is_approx(expected, dec!(0.05));
    /// assert!(is_close);
    /// // Result: true (difference is only $0.01)
    ///
    /// // Strict check within 1 cent
    /// let is_exact = calculated.is_approx(expected, dec!(0.01));
    /// assert!(is_exact);
    /// // Result: true (difference is exactly $0.01, inclusive)
    ///
    /// let converted1 = Money::<USD>::from_decimal(dec!(100.02));  // From source 1
    /// let converted2 = Money::<USD>::from_decimal(dec!(100.05));  // From source 2
    /// let matches = converted1.is_approx(converted2, 0.02);
    /// assert!(!matches);
    /// // Result: false (different is 0.03, outside 0.02 tolerance)
    ///
    /// // Exchange rate reconciliation
    /// let converted1 = Money::<USD>::from_decimal(dec!(100.89));  // From source 1
    /// let converted2 = Money::<USD>::from_decimal(dec!(100.90));  // From source 2
    /// let matches = converted1.is_approx(converted2, dec!(0.02));
    /// assert!(matches);
    /// // Result: true (within 2 cent tolerance)
    /// ```
    fn is_approx<M, T>(&self, m: M, tolerance: T) -> bool
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C>,
        T: DecimalNumber,
    {
        self.checked_sub(m).is_some_and(|diff| {
            tolerance
                .get_decimal()
                .is_some_and(|tol| tol >= diff.abs().amount())
        })
    }

    // REQUIRED

    /// Returns the absolute value of the money amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let negative = Money::<USD>::new(dec!(-100)).unwrap();
    /// let positive = negative.abs();
    /// assert_eq!(positive.amount(), dec!(100));
    /// ```
    fn abs(&self) -> Self;

    /// Adds another money value to this one.
    ///
    /// # Argument
    /// - `rhs: impl Amount<C>` accepts: `BaseMoney<C>`(`Money<C>`/`RawMoney<C>`), `Decimal`, `f64`, `i32`, `i64`, `i128`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let m1 = Money::<USD>::new(dec!(100)).unwrap();
    /// let m2 = Money::<USD>::new(dec!(50)).unwrap();
    /// let sum = m1.checked_add(m2).unwrap();
    /// assert_eq!(sum.amount(), dec!(150));
    /// ```
    fn checked_add<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: Amount<C>;

    /// Subtracts another money value from this one.
    ///
    /// # Argument
    /// - `rhs: impl Amount<C>` accepts: `BaseMoney<C>`(`Money<C>`/`RawMoney<C>`), `Decimal`, `f64`, `i32`, `i64`, `i128`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let m1 = Money::<USD>::new(dec!(100)).unwrap();
    /// let m2 = Money::<USD>::new(dec!(30)).unwrap();
    /// let diff = m1.checked_sub(m2).unwrap();
    /// assert_eq!(diff.amount(), dec!(70));
    /// ```
    fn checked_sub<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: Amount<C>;

    /// Multiplies this money value by another value.
    ///
    /// # Argument
    /// - `rhs: impl DecimalNumber` accepts: `Decimal`, `f64`, `i32`, `i64`, `i128`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let money = Money::<USD>::new(dec!(10)).unwrap();
    /// let product = money.checked_mul(dec!(3)).unwrap();
    /// assert_eq!(product.amount(), dec!(30));
    /// ```
    fn checked_mul<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber;

    /// Divides this money value by another value.
    ///
    /// # Argument
    /// - `rhs: impl DecimalNumber` accepts: `Decimal`, `f64`, `i32`, `i64`, `i128`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let money = Money::<USD>::new(dec!(100)).unwrap();
    /// let quotient = money.checked_div(dec!(4)).unwrap();
    /// assert_eq!(quotient.amount(), dec!(25));
    /// ```
    fn checked_div<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber;

    /// Split money into equal same parts leaving a remainder(if any).
    ///
    /// # Argument
    /// n: u32, how many parts splitted.
    ///
    /// # Return
    /// `Option<(Self, Self)>`, returns equal parts of split(0) and remainder(1) if any, if no remainder, it defaults to zero.
    ///
    /// # Example
    /// ```rust
    /// use moneylib::money;
    /// use moneylib::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let money = money!(USD, 100);
    /// let split3 = money.split(3).unwrap();
    /// assert_eq!(split3.0.amount(), dec!(33.33)); // all equal parts after split.
    /// assert_eq!(split3.1.amount(), dec!(0.01)); // remainder 1 cent.
    /// // result: 100 = 33.33 + 33.33 + 33.33 + 0.01
    ///
    /// let money = money!(USD, 500);
    /// let split4 = money.split(4).unwrap();
    /// assert_eq!(split4.0.amount(), dec!(125.00)); // all equal parts
    /// assert!(split4.1.is_zero()); // no remainder
    /// ```
    fn split(&self, n: u32) -> Option<(Self, Self)>
    where
        Self: Default + Amount<C> + Ord,
    {
        crate::split_alloc_ops::split(self, n)
    }

    /// Split money into equal parts and distribute the remainder(if any) equally into parts.
    ///
    /// # Argument
    /// n: u32, how many parts splitted.
    ///
    /// # Return
    /// `Option<Vec<T>>`, returns list of parts where remainder distribute among them(beginning from first).
    ///
    /// # Example
    /// ```rust
    /// use moneylib::money;
    /// use moneylib::dec;
    /// use moneylib::{BaseMoney, BaseOps};
    ///
    /// let money = money!(USD, 100);
    /// let split3 = money.split_dist(3).unwrap();
    /// assert_eq!(split3, vec![money!(USD, 33.34), money!(USD, 33.33), money!(USD, 33.33)]); // all equal parts after split and remainder distributed starting from first element.
    /// assert_eq!(split3.len(), 3);
    ///
    /// let money = money!(USD, 500);
    /// let split4 = money.split_dist(4).unwrap();
    /// assert_eq!(split4, vec![money!(USD, 125), money!(USD, 125), money!(USD, 125), money!(USD, 125)]); // all equal parts after split and leave no remainder.
    /// assert_eq!(split4.len(), 4);
    /// ```
    fn split_dist(&self, n: u32) -> Option<Vec<Self>>
    where
        Self: Default + Amount<C> + Ord,
    {
        crate::split_alloc_ops::split_dist(self, n)
    }

    /// Allocate money by percentages and distribute the remainder(if any).
    ///
    /// Total percentages must be equal to 100.
    ///
    /// # Argument
    /// pcns: list of DecimalNumber: Decimal, f64, i32, i64, i128 -> denoting percentage, e.g. 20% -> 20.
    ///
    /// # Return
    /// Return list of allocated money all summed back into original amount.
    /// Returns `None` if the percentages list is empty or does not sum to 100.
    ///
    /// # Example
    /// ```rust
    /// use moneylib::{Money, BaseMoney, BaseOps, macros::dec, iso::USD};
    ///
    /// // percentage ratios: 60%, 40%
    /// let profit = Money::<USD>::new(dec!(10000.00)).unwrap();
    /// let shares = profit.allocate(&[60, 40]).unwrap();  // 60/40 split
    /// assert_eq!(shares[0].amount(), dec!(6000.00));
    /// assert_eq!(shares[1].amount(), dec!(4000.00));
    ///
    /// // Budget allocation by priority weights
    /// let budget = Money::<USD>::new(dec!(100000.00)).unwrap();
    /// let depts = budget.allocate(&[35, 25, 20, 15, 5]).unwrap();
    /// assert_eq!(depts[0].amount(), dec!(35000.00));
    /// assert_eq!(depts[4].amount(), dec!(5000.00));
    /// ```
    fn allocate<D>(&self, pcns: &[D]) -> Option<Vec<Self>>
    where
        Self: Default + Amount<C> + CustomMoney<C>,
        D: DecimalNumber + Copy,
    {
        crate::split_alloc_ops::allocate(self, pcns)
    }

    /// Allocate money by ratios and distribute the remainder(if any).
    ///
    /// # Argument
    /// ratios: list of DecimalNumber: Decimal, f64, i32, i64, i128 -> denoting ratios.
    ///
    /// # Return
    /// Return list of allocated money all summed back into original amount.
    /// Returns `None` if the ratios list is empty or all ratios are zero.
    ///
    /// # Example
    /// ```rust
    /// use moneylib::{Money, BaseMoney, BaseOps, macros::dec, iso::USD};
    ///
    /// // Unequal ratios: 1:2:1 means 25%, 50%, 25%
    /// let amount = Money::<USD>::new(dec!(400.00)).unwrap();
    /// let parts = amount.allocate_by_ratios(&[1, 2, 1]).unwrap();
    /// assert_eq!(parts[0].amount(), dec!(100.00));
    /// assert_eq!(parts[1].amount(), dec!(200.00));
    /// assert_eq!(parts[2].amount(), dec!(100.00));
    /// ```
    fn allocate_by_ratios<D>(&self, ratios: &[D]) -> Option<Vec<Self>>
    where
        Self: Default + Amount<C> + CustomMoney<C>,
        D: DecimalNumber + Copy,
    {
        crate::split_alloc_ops::allocate_by_ratios(self, ratios)
    }
}

/// Trait for statistical and aggregate operations on collections of money values.
///
/// This trait is automatically implemented for any type whose references implement
/// [`IntoIterator`] over money values (e.g. `Vec<Money<C>>`, slices `&[Money<C>]`, etc.).
///
/// All methods return `None` when the collection is empty or when an arithmetic
/// overflow occurs, making them safe to use without panicking.
pub trait IterOps<C: Currency> {
    type Item;

    /// Returns the sum of all money values in the collection, or `None` if
    /// arithmetic overflow occurs. Returns `None` for an empty collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, IterOps, BaseMoney, macros::dec, iso::USD};
    ///
    /// let moneys = vec![
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(20.00)).unwrap(),
    ///     Money::<USD>::new(dec!(30.00)).unwrap(),
    /// ];
    /// assert_eq!(moneys.checked_sum().unwrap().amount(), dec!(60.00));
    ///
    /// // Empty collection returns Some(zero)
    /// let empty: Vec<Money<USD>> = vec![];
    /// assert!(empty.checked_sum().is_none());
    /// ```
    fn checked_sum(&self) -> Option<Self::Item>;

    /// Returns the arithmetic mean (average) of all money values in the collection,
    /// or `None` if the collection is empty or if arithmetic overflow occurs.
    ///
    /// The result is rounded to the currency's minor unit using bankers rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, IterOps, BaseMoney, macros::dec, iso::USD};
    ///
    /// let moneys = vec![
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(20.00)).unwrap(),
    ///     Money::<USD>::new(dec!(30.00)).unwrap(),
    /// ];
    /// assert_eq!(moneys.mean().unwrap().amount(), dec!(20.00));
    ///
    /// // Empty collection returns None
    /// let empty: Vec<Money<USD>> = vec![];
    /// assert!(empty.mean().is_none());
    /// ```
    fn mean(&self) -> Option<Self::Item>;

    /// Returns the median money value of the collection, or `None` if the collection
    /// is empty or if arithmetic overflow occurs.
    ///
    /// The collection is sorted by amount in ascending order. For an odd-length
    /// collection the middle element is returned. For an even-length collection the
    /// arithmetic mean of the two middle elements is returned, rounded to the
    /// currency's minor unit using bankers rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, IterOps, BaseMoney, macros::dec, iso::USD};
    ///
    /// // Odd number of elements – returns the middle value
    /// let moneys = vec![
    ///     Money::<USD>::new(dec!(30.00)).unwrap(),
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(20.00)).unwrap(),
    /// ];
    /// assert_eq!(moneys.median().unwrap().amount(), dec!(20.00));
    ///
    /// // Even number of elements – returns average of the two middle values
    /// let moneys = vec![
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(20.00)).unwrap(),
    ///     Money::<USD>::new(dec!(30.00)).unwrap(),
    ///     Money::<USD>::new(dec!(40.00)).unwrap(),
    /// ];
    /// assert_eq!(moneys.median().unwrap().amount(), dec!(25.00));
    ///
    /// // Empty collection returns None
    /// let empty: Vec<Money<USD>> = vec![];
    /// assert!(empty.median().is_none());
    /// ```
    fn median(&self) -> Option<Self::Item>;

    /// Returns the most frequently occurring money value(s) in the collection as a
    /// `Vec`, or `None` if the collection is empty or if all distinct values share
    /// the same frequency (no single dominant mode group).
    ///
    /// # Behavior
    ///
    /// - Empty collection → `None`
    /// - Single element → `Some(vec![element])`
    /// - All elements equal → `Some(vec![that element])`
    /// - All distinct values have the same occurrence count → `None`
    ///   (e.g. `[1,1,2,2,3,3]` → `None`)
    /// - Some values occur more than others → `Some(vec![…values at max frequency…])`
    ///   in the order they first appear in the collection
    ///   (e.g. `[1,1,1,2,2,3,3,3]` → `Some(vec![1, 3])`)
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, IterOps, BaseMoney, macros::dec, iso::USD};
    ///
    /// // Single clear mode
    /// let moneys = vec![
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(20.00)).unwrap(),
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(30.00)).unwrap(),
    /// ];
    /// assert_eq!(moneys.mode().unwrap()[0].amount(), dec!(10.00));
    ///
    /// // Empty collection returns None
    /// let empty: Vec<Money<USD>> = vec![];
    /// assert!(empty.mode().is_none());
    ///
    /// // All distinct values with equal frequency – no mode
    /// let all_distinct = vec![
    ///     Money::<USD>::new(dec!(10.00)).unwrap(),
    ///     Money::<USD>::new(dec!(20.00)).unwrap(),
    ///     Money::<USD>::new(dec!(30.00)).unwrap(),
    /// ];
    /// assert!(all_distinct.mode().is_none());
    /// ```
    fn mode(&self) -> Option<Vec<Self::Item>>;
}

/// Trait for types that can represent a money amount: BaseMoney<C>, Decimal, f64, i32, i64, i128.
///
/// This trait allows for flexible input types in constructing and arithmetic operations.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, iso::USD};
/// use moneylib::macros::dec;
/// use moneylib::{BaseMoney, BaseOps};
///
/// let money = Money::<USD>::new(dec!(100)).unwrap();
/// let money2 = Money::<USD>::new(100.0_f64).unwrap();
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

/// Trait to represents numbers to work with money amounts.
///
/// It supports Decimal, f64, i32, i64, i128.
pub trait DecimalNumber: Sized {
    fn get_decimal(&self) -> Option<Decimal>;
}

impl DecimalNumber for Decimal {
    fn get_decimal(&self) -> Option<Decimal> {
        Some(*self)
    }
}

impl DecimalNumber for f64 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_f64(*self)
    }
}

impl DecimalNumber for i32 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i32(*self)
    }
}

impl DecimalNumber for i64 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i64(*self)
    }
}

impl DecimalNumber for i128 {
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i128(*self)
    }
}

/// Defines the strategy for rounding decimal money amounts.
///
/// Different rounding strategies can produce different results when rounding values that fall
/// exactly between two possible rounded values (e.g., 2.5 rounded to one decimal place).
///
/// # Examples
///
/// ```
/// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
/// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
    /// use moneylib::macros::dec;
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
    /// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
    /// use moneylib::macros::dec;
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
/// use moneylib::{Money, Currency, RoundingStrategy, iso::USD};
/// use moneylib::macros::dec;
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
    // PROVIDED

    /// Format money according to the provided format string `format_str`.
    ///
    /// `format_str` contains these symbols as parts of money display.
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
    /// # Literal Blocks
    ///
    /// Use `\{...}` to print the contents of the curly braces literally, without any
    /// interpretation of format symbols inside. This is an alternative to escaping
    /// individual characters.
    ///
    /// Examples:
    /// - `\{Total:} c na` outputs "Total: USD 1,000.23"
    /// - `\{Price (USD):} na` outputs "Price (USD): 1,000.23"
    /// - `\{a, c, s} a` outputs "a, c, s 100.50"
    ///
    /// If the closing `}` is omitted, the contents are still printed literally to the end.
    ///
    /// # Arguments
    ///
    /// * `money` - The Money value to format
    /// * `format_str` - The format string containing format symbols and optional literal text
    ///
    /// *NOTE*: It's preferable to include `n` to avoid negative money printed as positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, Money, Currency, iso::USD};
    /// use moneylib::macros::dec;
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
    fn format(&self, format_str: &str) -> String {
        format(self.to_owned(), format_str)
    }

    /// Format money according to the provided format string `format_str`.
    ///
    /// `format_str` contains these symbols as parts of money display.
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
    /// # Literal Blocks
    ///
    /// Use `\{...}` to print the contents of the curly braces literally, without any
    /// interpretation of format symbols inside. This is an alternative to escaping
    /// individual characters.
    ///
    /// Examples:
    /// - `\{Total:} c na` outputs "Total: USD 1,000.23"
    /// - `\{Price (USD):} na` outputs "Price (USD): 1,000.23"
    /// - `\{a, c, s} a` outputs "a, c, s 100.50"
    ///
    /// If the closing `}` is omitted, the contents are still printed literally to the end.
    ///
    /// # Arguments
    ///
    /// * `money` - The Money value to format
    /// * `format_str` - The format string containing format symbols and optional literal text
    /// * `thousand_separator` - separator for thousands grouping
    /// * `decimal_separator` - separator for decimal fractions
    ///
    /// *NOTE*: It's preferable to include `n` to avoid negative money printed as positive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moneylib::{Money, RawMoney, Currency, iso::{USD, EUR}};
    /// use moneylib::macros::dec;
    /// use moneylib::CustomMoney;
    ///
    /// let money = Money::<USD>::from_decimal(dec!(93009.446688));
    /// let ret = money.format_with_separator("c na", "*", "#");
    /// assert_eq!(ret, "USD 93*009#45");
    ///
    /// let money = Money::<EUR>::from_decimal(dec!(93009.446688));
    /// let ret = money.format_with_separator("s na", " ", ",");
    /// assert_eq!(ret, "€ 93 009,45");
    ///
    /// let money = RawMoney::<USD>::from_decimal(dec!(93009.446688));
    /// let ret = money.format_with_separator("c na", "*", "#");
    /// assert_eq!(ret, "USD 93*009#446688");
    ///
    /// let money = RawMoney::<EUR>::from_decimal(dec!(93009.446688));
    /// let ret = money.format_with_separator("s na", " ", ",");
    /// assert_eq!(ret, "€ 93 009,446688");
    /// ```
    fn format_with_separator(
        &self,
        format_str: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> String {
        format_with_separator(
            self.to_owned(),
            format_str,
            thousand_separator,
            decimal_separator,
        )
    }

    #[cfg(feature = "locale")]
    /// Format money's amount using locale standard with `format_str` format.
    ///
    /// `locale_str` supports ISO 639 lowercase language code, ISO 639 with ISO 3166-1 alpha‑2 uppercase region code,
    /// also support BCP 47 locale extensions. Such as:
    /// - Languages code only: en, id, de, fr, zh etc.
    /// - Languages code with region: en-US, id-ID, de-DE, fr-FR, ar-SA, zh-CN, etc.
    /// - BCP 47 extension: zh-CN-u-nu-hanidec (Chinese locale with Chinese numbers).
    ///
    /// Some locales are default to latin numberings, some default to their own local numberings.
    ///
    /// `format_str` contains these symbols as parts of money display.
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
    /// # Literal Blocks
    ///
    /// Use `\{...}` to print the contents of the curly braces literally, without any
    /// interpretation of format symbols inside. This is an alternative to escaping
    /// individual characters.
    ///
    /// Examples:
    /// - `\{Total:} c na` outputs "Total: USD 1,000.23"
    /// - `\{Price (USD):} na` outputs "Price (USD): 1,000.23"
    /// - `\{a, c, s} a` outputs "a, c, s 100.50"
    ///
    /// If the closing `}` is omitted, the contents are still printed literally to the end.
    ///
    /// # Arguments
    ///
    /// * `locale_str` - Locale code, e.g. en-US, en-GB, fr-FR, id-ID, ar-SA, ar-AE
    /// * `format_str` - The format string containing format symbols and optional literal text
    ///
    /// *NOTE*: It's preferable to include `n` to avoid negative money printed as positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, Money, Currency, iso::{USD, EUR, INR}};
    /// use moneylib::macros::dec;
    /// use moneylib::CustomMoney;
    ///
    /// // English (US) locale: comma thousands separator, dot decimal separator
    /// let money = Money::<USD>::new(dec!(1234.56)).unwrap();
    /// assert_eq!(money.format_locale_amount("en-US", "c na").unwrap(), "USD 1,234.56");
    ///
    /// // Arabic (Saudi Arabia) locale: Arabic-Indic numerals
    /// let money = Money::<USD>::new(dec!(1234.56)).unwrap();
    /// assert_eq!(money.format_locale_amount("ar-SA", "c na").unwrap(), "USD ١٬٢٣٤٫٥٦");
    ///
    /// // Negative amount: include `n` in format_str to show the negative sign
    /// let money = Money::<USD>::new(dec!(-1234.56)).unwrap();
    /// assert_eq!(money.format_locale_amount("en-US", "c na").unwrap(), "USD -1,234.56");
    ///
    /// // Indian numbers and group formatting.
    /// let money = Money::<INR>::new(dec!(-1234012.52498)).unwrap();
    /// let result = money.format_locale_amount("hi-IN-u-nu-deva", "s na");
    /// assert_eq!(result.unwrap(), "₹ -१२,३४,०१२.५२");
    ///
    /// // Invalid locale returns an error
    /// let money = Money::<USD>::new(dec!(1234.56)).unwrap();
    /// assert!(money.format_locale_amount("!!!invalid", "c na").is_err());
    /// ```
    fn format_locale_amount(
        &self,
        locale_str: &str,
        format_str: &str,
    ) -> Result<String, MoneyError> {
        use icu_decimal::{DecimalFormatter, input::Decimal as LocaleDecimal};
        use icu_locale::Locale;

        let loc: Locale = locale_str.parse().map_err(|_| MoneyError::ParseLocale)?;
        let formatter = DecimalFormatter::try_new(loc.into(), Default::default())
            .map_err(|_| MoneyError::ParseLocale)?;

        let is_negative = self.is_negative();
        let abs_amount = self.amount().abs().to_string();

        let decimal =
            LocaleDecimal::try_from_str(&abs_amount).map_err(|_| MoneyError::DecimalConversion)?;

        let formatted_decimal = formatter.format(&decimal).to_string();

        let ret = format_with_amount::<C>(&formatted_decimal, is_negative, format_str);

        Ok(ret)
    }
}
