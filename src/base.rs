use crate::{Country, Currency, MoneyError};
use crate::{Decimal, MoneyResult};
use accounting::Accounting;
use regex::Regex;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};
use rust_decimal_macros::dec;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{fmt::Debug, str::FromStr, sync::LazyLock};

pub(crate) const COMMA_SEPARATOR: &'static str = ",";

pub(crate) const DOT_SEPARATOR: &'static str = ".";

pub static COMMA_THOUSANDS_SEPARATOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^([A-Z]{3})\s+((?:\d{1,3}(?:,\d{3})*|\d+)(?:\.\d+)?)$")
        .expect("failed compiling money format regex: comma thousands separator")
});

pub static DOT_THOUSANDS_SEPARATOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^([A-Z]{3})\s+((?:\d{1,3}(?:.\d{3})*|\d+)(?:\,\d+)?)$")
        .expect("failed compiling money format regex: dot thousands separator")
});

/// BaseMoney is the base trait for dealing with money type.
pub trait BaseMoney: Sized + Debug + Display + Clone + PartialOrd + PartialEq + FromStr {
    // REQUIRED

    /// Get currency of money
    fn currency(&self) -> Currency;

    /// Get amount of money
    fn amount(&self) -> Decimal;

    /// Round money using Banker's Rounding rule to the scale of currency's minor unit
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
        Ok(self
            .amount()
            .round_dp(self.minor_unit() as u32)
            .checked_mul(dec!(10).powu(self.minor_unit() as u64))
            .ok_or(MoneyError::ArithmeticOverflow)?
            .to_i128()
            .ok_or(MoneyError::DecimalToInteger)?)
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
        let mut fmt = Accounting::new_from_seperator(
            self.code(),
            self.minor_unit() as usize,
            self.thousand_separator(),
            self.decimal_separator(),
        );
        if self.amount().is_sign_negative() {
            let abs = self.amount().abs();
            fmt.set_format("{s} -{v}");
            fmt.format_money(abs)
        } else {
            fmt.set_format("{s} {v}");
            fmt.format_money(self.amount())
        }
    }

    /// Format money with symbol along with thousands and decimal separators.
    /// Example: $1,234.45
    fn format_symbol(&self) -> String {
        let mut fmt = Accounting::new_from_seperator(
            self.symbol(),
            self.minor_unit() as usize,
            self.thousand_separator(),
            self.decimal_separator(),
        );
        fmt.set_format("{s}{v}");
        fmt.format_money(self.amount())
    }

    /// Format money with code in the smallest unit along with thousands separators.
    /// Example USD 1,234.45 --> USD 123,445 ¢
    /// If the currency has no minor unit symbol, it defaults to "minor".
    /// You can set the minor unit symbol in `Currency` type's setter.
    fn format_code_minor(&self) -> MoneyResult<String> {
        let minor_amount = self.minor_amount()?;
        let mut fmt = Accounting::new_from_seperator(
            self.code(),
            0,
            self.thousand_separator(),
            self.decimal_separator(),
        );
        if minor_amount.is_negative() {
            let abs = minor_amount.abs();
            let f = format!("{{s}} -{{v}} {}", self.currency().minor_symbol());
            fmt.set_format(&f);
            Ok(fmt.format_money(abs))
        } else {
            let f = format!("{{s}} {{v}} {}", self.currency().minor_symbol());
            fmt.set_format(&f);
            Ok(fmt.format_money(minor_amount))
        }
    }

    /// Format money with code in the smallest unit along with thousands separators.
    /// Example $1,234.45 --> $123,445 ¢
    /// If the currency has no minor unit symbol, it defaults to "minor".
    /// You can set the minor unit symbol in `Currency` type's setter.
    fn format_symbol_minor(&self) -> MoneyResult<String> {
        let minor_amount = self.minor_amount()?;
        let mut fmt = Accounting::new_from_seperator(
            self.symbol(),
            0,
            self.thousand_separator(),
            self.decimal_separator(),
        );
        let f = format!("{{s}}{{v}} {}", self.currency().minor_symbol());
        fmt.set_format(&f);
        Ok(fmt.format_money(minor_amount))
    }

    /// Default display of money
    fn display(&self) -> String {
        self.format_code()
    }

    /// Get countries using this currency
    fn countries(&self) -> Option<Vec<Country>> {
        self.currency().countries()
    }

    fn set_thousand_separator(&mut self, separator: &'static str) {
        self.currency().set_thousand_separator(separator);
    }

    fn set_decimal_separator(&mut self, separator: &'static str) {
        self.currency().set_decimal_separator(separator);
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

    fn add(&self, rhs: Decimal) -> MoneyResult<Self>;

    fn sub(&self, rhs: Decimal) -> MoneyResult<Self>;

    fn mul(&self, rhs: Decimal) -> MoneyResult<Self>;

    fn div(&self, rhs: Decimal) -> MoneyResult<Self>;
}
