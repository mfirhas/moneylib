use crate::Decimal;
use crate::{Country, Currency, MoneyError};
use accounting::Accounting;
use regex::Regex;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};
use rust_decimal_macros::dec;
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

pub type MoneyResult<T> = Result<T, MoneyError>;

/// BaseMoney is the base trait for dealing with money type.
pub trait BaseMoney: Debug + Clone + PartialOrd + PartialEq + FromStr {
    // REQUIRED

    /// Get currency of money
    fn currency(&self) -> Currency;

    /// Get amount of money
    fn amount(&self) -> Decimal;

    /// Round money using Banker's Rounding rule to the scale of currency's minor unit
    fn round(self) -> Self;

    // PROVIDED

    /// Get currency name
    fn name(&self) -> &str {
        self.currency().name
    }

    /// Get money symbol
    fn symbol(&self) -> &str {
        self.currency().symbol
    }

    /// Get money ISO 4217 code
    #[inline]
    fn code(&self) -> &str {
        self.currency().code
    }

    /// Get currency ISO 4217 numeric code
    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency().numeric_code
    }

    /// Get money minor unit
    #[inline]
    fn minor_unit(&self) -> u16 {
        self.currency().minor_unit
    }

    /// Get money amount in its smallest unit
    #[inline]
    fn minor_amount(&self) -> MoneyResult<i128> {
        let minor_amount_dec = self
            .amount()
            .round_dp(self.minor_unit() as u32)
            .checked_mul(dec!(10).powu(self.minor_unit() as u64))
            .ok_or(MoneyError::ArithmeticOverflow)?;
        let minor_amount_int = minor_amount_dec
            .to_i128()
            .ok_or(MoneyError::DecimalToInteger)?;
        Ok(minor_amount_int)
    }

    /// Get money thousands separator
    #[inline]
    fn thousand_separator(&self) -> &'static str {
        // COMMA_SEPARATOR
        self.currency().thousand_separator
    }

    /// Get money decimal separator
    #[inline]
    fn decimal_separator() -> &'static str {
        DOT_SEPARATOR
    }

    /// Format money with code along with thousands and decimal separators.
    /// Example: USD 1,234.45
    fn format_code(&self) -> String {
        let mut fmt = Accounting::new_from_seperator(
            self.code(),
            self.minor_unit() as usize,
            self.thousand_separator(),
            Self::decimal_separator(),
        );
        fmt.set_format("{s} {v}");
        fmt.format_money(self.amount())
    }

    /// Format money with symbol along with thousands and decimal separators.
    /// Example: $1,234.45
    fn format_symbol(&self) -> String {
        let mut fmt = Accounting::new_from_seperator(
            self.symbol(),
            self.minor_unit() as usize,
            self.thousand_separator(),
            Self::decimal_separator(),
        );
        fmt.set_format("{s}{v}");
        fmt.format_money(self.amount())
    }

    /// Get countries using this currency
    fn countries(&self) -> Vec<Country> {
        self.currency().countries()
    }
}
