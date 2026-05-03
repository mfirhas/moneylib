use std::{
    fmt::{Debug, Display},
    iter::Sum,
    marker::PhantomData,
    str::FromStr,
};

#[cfg(feature = "accounting")]
use crate::AccountingOps;

use crate::{
    BaseMoney, BaseOps, Decimal, Money, MoneyError, MoneyOps,
    base::{Amount, DecimalNumber},
    macros::dec,
    parse::{
        parse_code_locale_separator, parse_comma_thousands_separator,
        parse_dot_thousands_separator, parse_symbol_comma_thousands_separator,
        parse_symbol_dot_thousands_separator, parse_symbol_locale_separator,
    },
};
use crate::{Currency, MoneyFormatter};
use rust_decimal::{MathematicalOps, prelude::FromPrimitive, prelude::ToPrimitive};

/// Represents a monetary value without automatic rounding.
///
/// `RawMoney` is exactly like [`Money`] except it doesn't automatically round
/// amounts after each operation. It keeps full decimal precision and lets
/// callers decide when to round.
///
/// # Key Features
///
/// - **Type Safety**: Provides compile-time checks to ensure valid state.
/// - **Precision**: Uses 128-bit fixed-precision decimal for accurate calculations.
/// - **No Automatic Rounding**: Preserves all decimal places until explicitly rounded.
/// - **Zero-Cost**: `Copy` type with no heap allocations and currency metadata is zero-sized type.
///
/// # Conversion
///
/// - Convert from `Money` using [`Money::into_raw`]
/// - Convert to `Money` using [`RawMoney::finish`] (applies rounding)
///
/// # Where Rounding Happens
///
/// - [`BaseMoney::round`]: rounds using currency's minor unit (bankers rounding). Returns `RawMoney`.
/// - [`BaseMoney::round_with`]: rounds using custom decimal points and strategy. Returns `RawMoney`.
/// - [`RawMoney::finish`]: rounds to currency's minor unit using bankers rounding back to `Money`.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, RawMoney, BaseMoney, macros::dec, iso::USD};
///
/// // Create RawMoney directly - no rounding
/// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
/// assert_eq!(raw.amount(), dec!(100.567));
///
/// // Convert from Money
/// let money = Money::<USD>::new(dec!(100.50)).unwrap();
/// let raw = money.into_raw();
/// assert_eq!(raw.amount(), dec!(100.50));
///
/// // Convert back to Money with rounding
/// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
/// let money = raw.finish();
/// assert_eq!(money.amount(), dec!(100.57));
/// ```
///
/// # See Also
///
/// - [`Money`] for automatically-rounded monetary values
/// - [`BaseMoney`] trait for core money operations and accessors
/// - [`BaseOps`] trait for arithmetic and comparison operations
/// - [`MoneyFormatter`] trait for custom formatting and rounding
#[derive(Copy, PartialEq, Eq)]
pub struct RawMoney<C: Currency> {
    amount: Decimal,
    _currency: PhantomData<C>,
}

impl<C> RawMoney<C>
where
    C: Currency,
{
    /// Creates a new `RawMoney` instance from Decimal without rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::USD};
    ///
    /// let raw = RawMoney::<USD>::from_decimal(dec!(123.309));
    /// assert_eq!(raw.amount(), dec!(123.309));
    /// ```
    #[inline]
    pub const fn from_decimal(amount: Decimal) -> Self {
        Self {
            amount,
            _currency: PhantomData,
        }
    }

    /// Creates a new `RawMoney` from minor amount i128 without rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::{USD, BHD, JPY}};
    ///
    /// // USD has 2 decimal places, so 12302 cents = $123.02
    /// let raw = RawMoney::<USD>::from_minor(12302).unwrap();
    /// assert_eq!(raw.amount(), dec!(123.02));
    ///
    /// // JPY has 0 decimal places
    /// let raw = RawMoney::<JPY>::from_minor(1000).unwrap();
    /// assert_eq!(raw.amount(), dec!(1000));
    ///
    /// // BHD has 3 decimal places
    /// let raw = RawMoney::<BHD>::from_minor(12345).unwrap();
    /// assert_eq!(raw.amount(), dec!(12.345));
    /// ```
    #[inline]
    pub fn from_minor(minor_amount: i128) -> Result<Self, MoneyError> {
        Ok(Self {
            amount: Decimal::from_i128(minor_amount)
                .ok_or(MoneyError::OverflowError)?
                .checked_div(
                    dec!(10)
                        .checked_powu(C::MINOR_UNIT.into())
                        .ok_or(MoneyError::OverflowError)?,
                )
                .ok_or(MoneyError::OverflowError)?,
            _currency: PhantomData,
        })
    }

    /// Converts this `RawMoney` to `Money`, applying rounding.
    ///
    /// Rounds the amount to the currency's minor unit precision using the
    /// bankers rounding rule, then returns a `Money` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::{USD, JPY, BHD}};
    ///
    /// let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    /// let money = raw.finish();
    /// assert_eq!(money.amount(), dec!(100.57));
    ///
    /// let raw_jpy = RawMoney::<JPY>::new(dec!(100.567)).unwrap();
    /// let money = raw_jpy.finish();
    /// assert_eq!(money.amount(), dec!(101));
    ///
    /// let raw_bhd = RawMoney::<BHD>::new(dec!(100.9999)).unwrap();
    /// let money = raw_bhd.finish();
    /// assert_eq!(money.amount(), dec!(101.000));
    /// ```
    #[inline]
    pub fn finish(self) -> Money<C> {
        Money::from_decimal(self.amount())
    }

    /// Parses a string in the format `"CCC amount"` (comma thousands separator and dot decimal separator).
    ///
    /// The format is `"CCC amount"` where `CCC` is a currency code (1-15 letters).
    ///
    /// For dot thousands separator format (e.g., `"EUR 1.234,56"`), use
    /// [`RawMoney::from_code_dot_thousands`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::USD};
    /// use std::str::FromStr;
    ///
    /// let raw = RawMoney::<USD>::from_code_comma_thousands("USD 1,234.56789").unwrap();
    /// assert_eq!(raw.amount(), dec!(1234.56789));
    /// assert_eq!(raw.code(), "USD");
    ///
    /// assert!(RawMoney::<USD>::from_code_comma_thousands("EUR 100.00").is_err());
    /// ```
    pub fn from_code_comma_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((currency_code, amount_str)) = parse_comma_thousands_separator(s) {
            if currency_code != C::CODE {
                return Err(MoneyError::CurrencyMismatchError(
                    currency_code.into(),
                    C::CODE.into(),
                ));
            }
            return Ok(Self::from_decimal(Decimal::from_str(&amount_str).map_err(
                |err| MoneyError::ParseStrError(err.to_string().into()),
            )?));
        }

        Err(MoneyError::ParseStrError(format!(
            "failed parsing {}, use format: <CODE> <AMOUNT> where <CODE> is defined and <AMOUNT> is comma-separated thousands(optional) and dot-separated decimal",
            s
        ).into()))
    }

    /// Creates a new `RawMoney` from a string with dot as the thousands separator
    /// and comma as the decimal separator (e.g., `"EUR 1.234,56"`).
    ///
    /// The format is `"CCC amount"` where `CCC` is a currency code (1-15 letters) and
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, iso::{EUR, USD}};
    ///
    /// let raw = RawMoney::<EUR>::from_code_dot_thousands("EUR 1.234,56").unwrap();
    /// assert_eq!(raw.code(), "EUR");
    ///
    /// assert!(RawMoney::<USD>::from_code_dot_thousands("EUR 1.234,56").is_err());
    /// ```
    pub fn from_code_dot_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((currency_code, amount_str)) = parse_dot_thousands_separator(s) {
            if currency_code != C::CODE {
                return Err(MoneyError::CurrencyMismatchError(
                    currency_code.into(),
                    C::CODE.into(),
                ));
            }
            return Ok(Self::from_decimal(Decimal::from_str(&amount_str).map_err(
                |err| MoneyError::ParseStrError(err.to_string().into()),
            )?));
        }

        Err(MoneyError::ParseStrError(format!(
            "failed parsing {}, use format: <CODE> <AMOUNT> where <CODE> is defined and <AMOUNT> is dot-separated thousands(optional) and comma-separated decimal",
            s
        ).into()))
    }

    /// Parse from string with symbol, comma-separated thousands, dot-separated decimal, no rounding
    /// Example: $1,234.2249 into USD 1234.2249
    pub fn from_symbol_comma_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((symbol, amount_str)) = parse_symbol_comma_thousands_separator::<C>(s) {
            if symbol != C::SYMBOL {
                return Err(MoneyError::CurrencyMismatchError(
                    symbol.into(),
                    C::SYMBOL.into(),
                ));
            }

            return Ok(Self::from_decimal(Decimal::from_str(&amount_str).map_err(
                |err| MoneyError::ParseStrError(err.to_string().into()),
            )?));
        }

        Err(MoneyError::ParseStrError(format!(
            "failed parsing {}, use format: <SYMBOL><AMOUNT> where <SYMBOL> is defined and <AMOUNT> is comma-separated thousands(optional) and dot-separated decimal",
            s
        ).into()))
    }

    /// Parse from string with symbol, dot-separated thousands, comma-separated decimal, no rounding
    /// Example: $1.234,2249 into USD 1234.2249
    pub fn from_symbol_dot_thousands(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((symbol, amount_str)) = parse_symbol_dot_thousands_separator::<C>(s) {
            if symbol != C::SYMBOL {
                return Err(MoneyError::CurrencyMismatchError(
                    symbol.into(),
                    C::SYMBOL.into(),
                ));
            }

            return Ok(Self::from_decimal(Decimal::from_str(&amount_str).map_err(
                |err| MoneyError::ParseStrError(err.to_string().into()),
            )?));
        }

        Err(MoneyError::ParseStrError(format!(
            "failed parsing {}, use format: <SYMBOL><AMOUNT> where <SYMBOL> is defined and <AMOUNT> is dot-separated thousands(optional) and comma-separated decimal",
            s
        ).into()))
    }

    /// Parse from string with code, locale thousands and decimal separators.
    ///
    /// Code is space separated with amount.
    ///
    /// Currencies locale separators are from here: <https://docs.rs/currencylib>
    ///
    /// # Example
    /// ```
    /// use moneylib::{RawMoney, raw, iso::CHF, dec, BaseMoney};
    ///
    /// let money = RawMoney::<CHF>::from_code_locale_separator("CHF 1'123'456.2223").unwrap();
    /// assert_eq!(money.code(), "CHF");
    /// assert_eq!(money.symbol(), "₣");
    /// assert_eq!(money.amount(), dec!(1123456.2223));
    /// assert_eq!(money, raw!(CHF, 1123456.2223));
    /// ```
    pub fn from_code_locale_separator(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((code, amount_str)) = parse_code_locale_separator::<C>(s) {
            if code != C::CODE {
                return Err(MoneyError::CurrencyMismatchError(
                    code.into(),
                    C::CODE.into(),
                ));
            }

            return Self::from_str(&amount_str)
                .map_err(|err| MoneyError::ParseStrError(err.to_string().into()));
        }

        Err(MoneyError::ParseStrError(format!(
            "failed parsing {}, use format: <CODE> <AMOUNT> where <CODE> is defined and <AMOUNT> is separated by locale separators",
            s
        ).into()))
    }

    /// Parse from string with symbol, locale thousands and decimal separators.
    ///
    /// There's no space between symbol and amount.
    ///
    /// Currencies locale separators are from here: <https://docs.rs/currencylib>
    ///
    /// # Example
    /// ```
    /// use moneylib::{RawMoney, raw, iso::CHF, dec, BaseMoney};
    ///
    /// let money = RawMoney::<CHF>::from_symbol_locale_separator("₣1'123'456.2223").unwrap();
    /// assert_eq!(money.code(), "CHF");
    /// assert_eq!(money.symbol(), "₣");
    /// assert_eq!(money.amount(), dec!(1123456.2223));
    /// assert_eq!(money, raw!(CHF, 1123456.2223));
    /// ```
    pub fn from_symbol_locale_separator(s: &str) -> Result<Self, MoneyError> {
        let s = s.trim();

        if let Some((symbol, amount_str)) = parse_symbol_locale_separator::<C>(s) {
            if symbol != C::SYMBOL {
                return Err(MoneyError::CurrencyMismatchError(
                    symbol.into(),
                    C::SYMBOL.into(),
                ));
            }

            return Self::from_str(&amount_str)
                .map_err(|err| MoneyError::ParseStrError(err.to_string().into()));
        }

        Err(MoneyError::ParseStrError(format!(
            "failed parsing {}, use format: <SYMBOL><AMOUNT> where <SYMBOL> is defined and <AMOUNT> is separated by locale separators",
            s
        ).into()))
    }
}

impl<C: Currency> Default for RawMoney<C> {
    /// Returns money with zero amount.
    fn default() -> Self {
        Self {
            amount: Decimal::default(),
            _currency: PhantomData,
        }
    }
}

impl<C: Currency> Ord for RawMoney<C>
where
    C: Currency + PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl<C> PartialOrd for RawMoney<C>
where
    C: Currency + PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Amount<C> for RawMoney<C>
where
    C: Currency,
{
    #[inline(always)]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl<C> FromStr for RawMoney<C>
where
    C: Currency,
{
    type Err = MoneyError;

    /// Parse money from string number.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, RawMoney, iso::USD, raw, dec};
    /// use std::str::FromStr;
    ///
    /// let money = RawMoney::<USD>::from_str("12334.4439").unwrap();
    /// assert_eq!(money, raw!(USD, 12334.4439));
    /// assert_eq!(money.amount(), dec!(12334.4439));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let dec_num = Decimal::from_str(s).map_err(|err| {
            MoneyError::ParseStrError(format!("failed parsing money from string: {}", err).into())
        })?;
        Ok(Self::from_decimal(dec_num))
    }
}

impl<C: Currency> Clone for RawMoney<C> {
    fn clone(&self) -> Self {
        Self {
            amount: self.amount,
            _currency: PhantomData,
        }
    }
}

/// Formats `RawMoney` using the currency code and full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{RawMoney, macros::dec, iso::USD};
///
/// let raw = RawMoney::<USD>::from_decimal(dec!(1234.567));
/// assert_eq!(format!("{}", raw), "USD 1,234.567");
///
/// let raw = RawMoney::<USD>::from_decimal(dec!(-1234.56));
/// assert_eq!(format!("{}", raw), "USD -1,234.56");
/// ```
impl<C> Display for RawMoney<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<C> Debug for RawMoney<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawMoney({}, {})", C::CODE, self.amount)
    }
}

impl<C: Currency> Sum for RawMoney<C> {
    /// Sum all moneys
    ///
    /// WARN: PANIC! if overflowed.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(RawMoney::default(), |acc, b| acc + b)
    }
}

impl<'a, C: Currency> Sum<&'a RawMoney<C>> for RawMoney<C> {
    /// Sum all moneys(borrowed)
    ///
    /// WARN: PANIC!!! if overflowed.
    fn sum<I: Iterator<Item = &'a RawMoney<C>>>(iter: I) -> Self {
        iter.fold(RawMoney::default(), |acc, b| acc + b.clone())
    }
}

impl<C> BaseMoney<C> for RawMoney<C>
where
    C: Currency,
{
    #[inline]
    fn new(amount: impl DecimalNumber) -> Result<Self, MoneyError> {
        Ok(Self {
            amount: amount.get_decimal().ok_or(MoneyError::OverflowError)?,
            _currency: PhantomData,
        })
    }

    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Rounds explicitly to the currency's minor unit using bankers rounding.
    ///
    /// Unlike `Money`, this must be called explicitly. Returns `RawMoney`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::USD};
    ///
    /// let raw = RawMoney::<USD>::from_decimal(dec!(100.567));
    /// let rounded = raw.round();
    /// assert_eq!(rounded.amount(), dec!(100.57));
    ///
    /// // round() returns RawMoney, not Money
    /// let rounded_again = rounded.round();
    /// assert_eq!(rounded_again.amount(), dec!(100.57));
    /// ```
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

    /// Returns the money amount in its smallest unit, rounding as needed.
    ///
    /// Since minor amounts must be integers, this rounds the raw amount
    /// to the currency's minor unit precision before computing the minor amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{RawMoney, BaseMoney, macros::dec, iso::USD};
    ///
    /// let raw = RawMoney::<USD>::from_decimal(dec!(123.45));
    /// assert_eq!(raw.minor_amount().unwrap(), 12345_i128);
    ///
    /// // Extra precision is rounded before computing minor amount
    /// let raw = RawMoney::<USD>::from_decimal(dec!(123.238533));
    /// assert_eq!(raw.minor_amount().unwrap(), 12324_i128);
    /// ```
    #[inline]
    fn minor_amount(&self) -> Option<i128> {
        self.amount()
            .round_dp(C::MINOR_UNIT.into())
            .checked_mul(dec!(10).checked_powu(C::MINOR_UNIT.into())?)?
            .to_i128()
    }
}

impl<C> BaseOps<C> for RawMoney<C>
where
    C: Currency,
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

impl<C> MoneyFormatter<C> for RawMoney<C> where C: Currency {}

#[cfg(feature = "accounting")]
impl<C> AccountingOps<C> for RawMoney<C> where C: Currency {}

impl<C> MoneyOps<C> for RawMoney<C> where C: Currency {}
