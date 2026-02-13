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

/// BaseMoney is the base trait for dealing with money type.
pub trait BaseMoney:
    Sized + Debug + Display + Clone + PartialOrd + PartialEq + Eq + FromStr
{
    // REQUIRED

    /// Get currency of money
    fn currency(&self) -> Currency;

    /// Get amount of money
    fn amount(&self) -> Decimal;

    /// Round money using `Currency`'s rounding strategy to the scale of currency's minor unit
    fn round(self) -> Self;

    // PROVIDED

    /// Get currency name
    #[inline]
    fn name(&self) -> &str {
        self.currency().name()
    }

    /// Get money symbol
    #[inline]
    fn symbol(&self) -> &str {
        self.currency().symbol()
    }

    /// Get money ISO 4217 code
    #[inline]
    fn code(&self) -> &str {
        self.currency().code()
    }

    /// Get currency ISO 4217 numeric code
    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency().numeric_code()
    }

    /// Get money minor unit
    #[inline]
    fn minor_unit(&self) -> u16 {
        self.currency().minor_unit()
    }

    /// Get money amount in its smallest unit
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

    /// Get money thousands separator
    #[inline]
    fn thousand_separator(&self) -> &'static str {
        self.currency().thousand_separator()
    }

    /// Get money decimal separator
    #[inline]
    fn decimal_separator(&self) -> &'static str {
        self.currency().decimal_separator()
    }

    /// Check if amount is 0
    #[inline]
    fn is_zero(&self) -> bool {
        self.amount().is_zero()
    }

    /// Check if sign is +
    #[inline]
    fn is_positive(&self) -> bool {
        self.amount().is_sign_positive()
    }

    /// Check if sign is -
    #[inline]
    fn is_negative(&self) -> bool {
        self.amount().is_sign_negative()
    }

    /// Format money with code along with thousands and decimal separators.
    /// Example: USD 1,234.45
    fn format_code(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), CODE_FORMAT_NEGATIVE);
        }
        format(self.to_owned(), CODE_FORMAT_POSITIVE)
    }

    /// Format money with symbol along with thousands and decimal separators.
    /// Example: $1,234.45
    fn format_symbol(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), SYMBOL_FORMAT_NEGATIVE);
        }
        format(self.to_owned(), SYMBOL_FORMAT_POSITIVE)
    }

    /// Format money with code in the smallest unit along with thousands separators.
    /// Example USD 1,234.45 --> USD 123,445 ¢
    /// If the currency has no minor unit symbol, it defaults to "minor".
    /// You can set the minor unit symbol in `Currency` type's setter.
    fn format_code_minor(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), CODE_FORMAT_NEGATIVE_MINOR);
        }
        format(self.to_owned(), CODE_FORMAT_POSITIVE_MINOR)
    }

    /// Format money with code in the smallest unit along with thousands separators.
    /// Example $1,234.45 --> $123,445 ¢
    /// If the currency has no minor unit symbol, it defaults to "minor".
    /// You can set the minor unit symbol in `Currency` type's setter.
    fn format_symbol_minor(&self) -> String {
        if self.is_negative() {
            return format(self.to_owned(), SYMBOL_FORMAT_NEGATIVE_MINOR);
        }
        format(self.to_owned(), SYMBOL_FORMAT_POSITIVE_MINOR)
    }

    /// Default display of money
    fn display(&self) -> String {
        self.format_code()
    }

    /// Get countries using this currency
    fn countries(&self) -> Option<Vec<Country>> {
        self.currency().countries()
    }
}

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

    /// make money positive
    fn abs(&self) -> Self;

    fn min(&self, rhs: Self) -> Self;

    fn max(&self, rhs: Self) -> Self;

    /// clamp the money amount between `from` and `to` inclusively.
    fn clamp(&self, from: Decimal, to: Decimal) -> Self;

    fn add<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    fn sub<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    fn mul<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    fn div<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney;

    // PROVIDED

    fn is_bigger(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() > rhs.amount())
    }

    fn is_smaller(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() < rhs.amount())
    }

    fn is_bigger_equal(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() >= rhs.amount())
    }

    fn is_smaller_equal(&self, rhs: impl BaseMoney) -> MoneyResult<bool> {
        if self.currency() != rhs.currency() {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(self.amount() <= rhs.amount())
    }
}

pub trait MoneyAmount<T>: Sized
where
    T: BaseMoney,
{
    fn get_money(&self) -> Option<T>;

    fn get_decimal(&self) -> Option<Decimal>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoundingStrategy {
    #[default]
    BankersRounding,

    HalfUp,

    HalfDown,

    Ceil,

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

pub trait CustomMoney: Sized + BaseMoney {
    // REQUIRED

    fn set_thousand_separator(&mut self, separator: &'static str);

    fn set_decimal_separator(&mut self, separator: &'static str);

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
