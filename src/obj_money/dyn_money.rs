use crate::{Currency, Decimal, MoneyError, RoundingStrategy, prelude::ObjMoney};
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};

use super::helpers;

/// A runtime-erased currency descriptor, mirroring the compile-time [`Currency`] trait constants.
///
/// `DynCurrency` holds every piece of metadata that a [`Currency`] implementation provides —
/// code, symbol, separators, minor-unit info, etc. — as plain `&'static str` / `u16` fields so
/// that the information can be used at runtime without a generic type parameter.
///
/// It is the currency half of [`DynMoney`] and is what [`Context`](super::Context) stores in its
/// global currency registry.
///
/// # Examples
///
/// ```
/// use moneylib::obj_money::DynCurrency;
/// use moneylib::iso::USD;
///
/// // Build from a compile-time Currency type
/// let dc = DynCurrency::from_curr::<USD>();
/// assert_eq!(dc.code(), "USD");
///
/// // Look up by ISO 4217 code at runtime
/// let dc = DynCurrency::from_code("EUR").unwrap();
/// assert_eq!(dc.code(), "EUR");
/// ```
#[derive(Debug, Clone, Copy, Eq)]
pub struct DynCurrency {
    pub(super) code: &'static str,
    pub(super) symbol: &'static str,
    pub(super) name: &'static str,
    pub(super) numeric: u16,
    pub(super) minor_unit: u16,
    pub(super) minor_unit_symbol: &'static str,
    pub(super) minor_unit_name: &'static str,
    pub(super) thousand_separator: &'static str,
    pub(super) decimal_separator: &'static str,
    pub(super) origin: &'static str,
    pub(super) locale: &'static str,
}

impl DynCurrency {
    /// Creates a `DynCurrency` from a compile-time [`Currency`] type.
    ///
    /// All fields are populated from the associated constants of `C`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::DynCurrency;
    /// use moneylib::iso::USD;
    ///
    /// let dc = DynCurrency::from_curr::<USD>();
    /// assert_eq!(dc.code(), "USD");
    /// ```
    pub fn from_curr<C: Currency>() -> Self {
        DynCurrency {
            code: C::CODE,
            symbol: C::SYMBOL,
            name: C::NAME,
            numeric: C::NUMERIC,
            minor_unit: C::MINOR_UNIT,
            minor_unit_symbol: C::MINOR_UNIT_SYMBOL,
            minor_unit_name: C::MINOR_UNIT_NAME,
            thousand_separator: C::THOUSAND_SEPARATOR,
            decimal_separator: C::DECIMAL_SEPARATOR,
            origin: C::ORIGIN,
            locale: C::LOCALE,
        }
    }

    /// Looks up a `DynCurrency` from the global [`Context`](super::Context) registry by ISO 4217
    /// code.
    ///
    /// Returns `Err` if the code is not found in the registry.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::ObjMoneyError`] when `code` is not registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::DynCurrency;
    ///
    /// let dc = DynCurrency::from_code("USD").unwrap();
    /// assert_eq!(dc.code(), "USD");
    ///
    /// assert!(DynCurrency::from_code("XYZ").is_err());
    /// ```
    pub fn from_code(code: &str) -> Result<Self, MoneyError> {
        if let Some(curr) = super::Context::get_currency(code) {
            return Ok(curr);
        }

        Err(MoneyError::ObjMoneyError(
            format!("currency {} not found", code).into(),
        ))
    }
}

impl<C: Currency> From<C> for DynCurrency {
    fn from(_: C) -> Self {
        Self::from_curr::<C>()
    }
}

impl DynCurrency {
    /// Returns the ISO 4217 currency code (e.g. `"USD"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::DynCurrency;
    /// use moneylib::iso::EUR;
    ///
    /// let dc = DynCurrency::from_curr::<EUR>();
    /// assert_eq!(dc.code(), "EUR");
    /// ```
    pub fn code(&self) -> &str {
        self.code
    }
}

impl PartialEq for DynCurrency {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

/// A type-erased, runtime-currency money value.
///
/// `DynMoney` is the dynamic counterpart of [`Money<C>`](crate::Money). It stores the amount and
/// full currency metadata at runtime, so different currencies can be handled without generic type
/// parameters — ideal for heterogeneous collections and runtime-only contexts.
///
/// # Key properties
///
/// - **`Copy`**: cheap to pass by value, no heap allocation.
/// - **Automatic rounding**: the amount is always rounded to the currency's `minor_unit`
///   (unless [`Context::is_raw()`](super::Context::is_raw) is `true`).
/// - **Implements [`ObjMoney`](super::ObjMoney)**: can be stored in `Box<dyn ObjMoney>` or
///   `Vec<Box<dyn ObjMoney>>` alongside `Money<C>` and `RawMoney<C>`.
///
/// # Examples
///
/// ```
/// use moneylib::{obj_money::{DynMoney, ObjMoney}, macros::dec, iso::USD};
///
/// let m = DynMoney::from_decimal::<USD>(dec!(1234.567));
/// assert_eq!(m.amount(), dec!(1234.57)); // rounded to 2 d.p.
/// assert_eq!(m.code(), "USD");
/// ```
#[derive(Debug, Clone, Copy, Eq)]
pub struct DynMoney {
    amount: Decimal,
    currency: DynCurrency,
}

impl DynMoney {
    /// Creates a `DynMoney` from a compile-time [`Currency`] type and a decimal amount.
    ///
    /// The amount is rounded to `C::MINOR_UNIT` decimal places unless
    /// [`Context::is_raw()`](super::Context::is_raw) is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{obj_money::{DynMoney, ObjMoney}, macros::dec, iso::JPY};
    ///
    /// // JPY has 0 minor-unit decimal places
    /// let m = DynMoney::from_decimal::<JPY>(dec!(1234.99));
    /// assert_eq!(m.amount(), dec!(1235));
    /// assert_eq!(m.code(), "JPY");
    /// ```
    #[inline(always)]
    pub fn from_decimal<C: Currency>(amount: Decimal) -> Self {
        Self {
            amount: helpers::amount::<C>(amount),
            currency: DynCurrency::from_curr::<C>(),
        }
    }

    /// Creates a `DynMoney` from an already-constructed [`DynCurrency`] and a decimal amount.
    ///
    /// The amount is rounded to `currency.minor_unit` decimal places unless
    /// [`Context::is_raw()`](super::Context::is_raw) is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::{DynCurrency, DynMoney, ObjMoney};
    /// use moneylib::{macros::dec, iso::EUR};
    ///
    /// let currency = DynCurrency::from_curr::<EUR>();
    /// let m = DynMoney::new_with_curr(currency, dec!(99.999));
    /// assert_eq!(m.amount(), dec!(100.00));
    /// assert_eq!(m.code(), "EUR");
    /// ```
    #[inline(always)]
    pub fn new_with_curr(currency: DynCurrency, amount: Decimal) -> Self {
        Self {
            amount: helpers::amount_with_curr(amount, currency),
            currency,
        }
    }

    /// Creates a `DynMoney` by looking up the currency by ISO 4217 `code` in the global
    /// [`Context`](super::Context) registry.
    ///
    /// The amount is rounded to the resolved currency's `minor_unit` unless
    /// [`Context::is_raw()`](super::Context::is_raw) is `true`.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::ObjMoneyError`] when `code` is not registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::{DynMoney, ObjMoney};
    /// use moneylib::macros::dec;
    ///
    /// let m = DynMoney::new_with_code("USD", dec!(9.999)).unwrap();
    /// assert_eq!(m.amount(), dec!(10.00));
    /// assert_eq!(m.code(), "USD");
    ///
    /// assert!(DynMoney::new_with_code("XYZ", dec!(1.0)).is_err());
    /// ```
    #[inline(always)]
    pub fn new_with_code(code: &str, amount: Decimal) -> Result<Self, MoneyError> {
        if let Some(currency) = super::Context::get_currency(code) {
            return Ok(Self {
                amount: helpers::amount_with_curr(amount, currency),
                currency,
            });
        }

        Err(MoneyError::ObjMoneyError(
            format!("currency {} not found", code).into(),
        ))
    }

    /// Returns a new `DynMoney` with the same currency but a different amount.
    ///
    /// The new amount is rounded to the currency's `minor_unit` unless
    /// [`Context::is_raw()`](super::Context::is_raw) is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{obj_money::{DynMoney, ObjMoney}, macros::dec, iso::USD};
    ///
    /// let m = DynMoney::from_decimal::<USD>(dec!(10.00));
    /// let m2 = m.set_amount(dec!(99.999));
    /// assert_eq!(m2.amount(), dec!(100.00));
    /// assert_eq!(m2.code(), "USD");
    /// ```
    #[inline(always)]
    pub fn set_amount(&self, amount: Decimal) -> Self {
        Self {
            amount: helpers::amount_with_curr(amount, self.currency),
            ..*self
        }
    }

    /// Returns a new `DynMoney` with the same amount re-rounded for currency `C`.
    ///
    /// The amount is rounded to `C::MINOR_UNIT` decimal places, so switching to a currency with a
    /// different precision (e.g. JPY with 0 decimal places) will truncate/round the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{obj_money::{DynMoney, ObjMoney}, macros::dec, iso::{USD, JPY}};
    ///
    /// let m = DynMoney::from_decimal::<USD>(dec!(100.75));
    /// let jpy = m.set_curr::<JPY>();
    /// assert_eq!(jpy.code(), "JPY");
    /// // 100.75 rounded to 0 decimal places (JPY) → 101
    /// assert_eq!(jpy.amount(), dec!(101));
    /// ```
    #[inline(always)]
    pub fn set_curr<C: Currency>(&self) -> Self {
        let currency = DynCurrency::from_curr::<C>();
        Self {
            amount: helpers::amount_with_curr(self.amount, currency),
            currency,
        }
    }

    /// Returns a new `DynMoney` with the currency swapped to the one identified by `code`, with
    /// the amount re-rounded for that currency.
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::ObjMoneyError`] when `code` is not registered in the
    /// [`Context`](super::Context).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{obj_money::{DynMoney, ObjMoney}, macros::dec, iso::USD};
    ///
    /// let m = DynMoney::from_decimal::<USD>(dec!(100.75));
    /// let jpy = m.set_curr_from_code("JPY").unwrap();
    /// assert_eq!(jpy.code(), "JPY");
    /// // 100.75 rounded to 0 decimal places (JPY) → 101
    /// assert_eq!(jpy.amount(), dec!(101));
    ///
    /// assert!(m.set_curr_from_code("XYZ").is_err());
    /// ```
    pub fn set_curr_from_code(&self, code: &str) -> Result<Self, MoneyError> {
        if let Some(currency) = super::Context::get_currency(code) {
            return Ok(Self {
                amount: helpers::amount_with_curr(self.amount, currency),
                currency,
            });
        }

        Err(MoneyError::ObjMoneyError(
            format!("currency {} not found", code).into(),
        ))
    }
}

impl PartialEq for DynMoney {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency.code == other.currency.code
    }
}

impl PartialOrd for DynMoney {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency.code == other.currency.code {
            return Some(self.amount.cmp(&other.amount));
        }
        None
    }
}

impl super::ObjMoney for DynMoney {
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    fn code(&self) -> &str {
        self.currency.code
    }

    #[inline]
    fn symbol(&self) -> &str {
        self.currency.symbol
    }

    #[inline]
    fn name(&self) -> &str {
        self.currency.name
    }

    #[inline]
    fn minor_unit(&self) -> u16 {
        self.currency.minor_unit
    }

    #[inline]
    fn thousand_separator(&self) -> &str {
        self.currency.thousand_separator
    }

    #[inline]
    fn decimal_separator(&self) -> &str {
        self.currency.decimal_separator
    }

    #[inline]
    fn minor_unit_symbol(&self) -> &str {
        self.currency.minor_unit_symbol
    }

    #[inline]
    fn minor_unit_name(&self) -> &str {
        self.currency.minor_unit_name
    }

    #[inline]
    fn origin(&self) -> &str {
        self.currency.origin
    }

    #[inline]
    fn locale(&self) -> &str {
        self.currency.locale
    }

    #[inline]
    fn minor_amount(&self) -> Option<i128> {
        self.amount
            .round_dp(self.currency.minor_unit.into())
            .checked_mul(crate::dec!(10).checked_powu(self.currency.minor_unit.into())?)?
            .to_i128()
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency.numeric.into()
    }

    #[inline]
    fn abs(&self) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.abs()))
    }

    #[inline]
    fn round(&self) -> Box<dyn super::ObjMoney> {
        Box::new(Self {
            amount: self.amount.round_dp(self.currency.minor_unit.into()),
            currency: self.currency,
        })
    }

    #[inline]
    fn round_with(
        &self,
        decimal_points: u32,
        strategy: RoundingStrategy,
    ) -> Box<dyn super::ObjMoney> {
        Box::new(
            self.set_amount(
                self.amount
                    .round_dp_with_strategy(decimal_points, strategy.into()),
            ),
        )
    }

    #[inline]
    fn truncate(&self) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.trunc()))
    }

    #[inline]
    fn truncate_with(&self, scale: u32) -> Box<dyn super::ObjMoney> {
        Box::new(self.set_amount(self.amount.trunc_with_scale(scale)))
    }

    #[inline]
    fn checked_add(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_add(rhs)?)))
    }

    #[inline]
    fn checked_sub(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_sub(rhs)?)))
    }

    #[inline]
    fn checked_mul(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_mul(rhs)?)))
    }

    #[inline]
    fn checked_div(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_div(rhs)?)))
    }

    #[inline]
    fn checked_rem(&self, rhs: Decimal) -> Option<Box<dyn super::ObjMoney>> {
        Some(Box::new(self.set_amount(self.amount.checked_rem(rhs)?)))
    }

    #[cfg(feature = "exchange")]
    fn convert(
        &self,
        to_code: &str,
        rate: &dyn crate::exchange::ObjRate,
    ) -> Result<Box<dyn super::ObjMoney>, MoneyError> {
        if self.currency.code == to_code {
            return Ok(Box::new(*self));
        }

        let rate_val = rate.get_rate(self.currency.code, to_code).ok_or_else(|| {
            MoneyError::ExchangeError(
                format!(
                    "overflowed or failed getting rate from: {} to: {}",
                    self.currency.code, to_code
                )
                .into(),
            )
        })?;

        let new_amount = self
            .amount
            .checked_mul(rate_val)
            .ok_or(MoneyError::OverflowError)?;

        Ok(Box::new(Self::new_with_code(to_code, new_amount)?))
    }
}

/// Converts a `&dyn ObjMoney` trait object into a `DynMoney`.
///
/// The currency is looked up by code in the global [`Context`](super::Context) registry, and the
/// amount is rounded to that currency's `minor_unit`.
///
/// # Errors
///
/// Returns [`MoneyError::ObjMoneyError`] if the trait object's currency code is not registered.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, BaseMoney, obj_money::{DynMoney, ObjMoney}, macros::dec, iso::USD};
///
/// let boxed: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(42.50)).unwrap());
/// let dyn_m = DynMoney::try_from(boxed.as_ref()).unwrap();
/// assert_eq!(dyn_m.amount(), dec!(42.50));
/// assert_eq!(dyn_m.code(), "USD");
/// ```
impl TryFrom<&dyn super::ObjMoney> for DynMoney {
    type Error = MoneyError;

    fn try_from(value: &dyn super::ObjMoney) -> Result<Self, Self::Error> {
        Self::new_with_code(value.code(), value.amount())
    }
}

/// Converts a `Box<dyn ObjMoney>` into a `DynMoney`.
///
/// Delegates to the `TryFrom<&dyn ObjMoney>` implementation.
///
/// # Errors
///
/// Returns [`MoneyError::ObjMoneyError`] if the boxed value's currency code is not registered.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, BaseMoney, obj_money::{DynMoney, ObjMoney}, macros::dec, iso::USD};
///
/// let boxed: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(55.00)).unwrap());
/// let dyn_m = DynMoney::try_from(boxed).unwrap();
/// assert_eq!(dyn_m.amount(), dec!(55.00));
/// assert_eq!(dyn_m.code(), "USD");
/// ```
impl TryFrom<Box<dyn super::ObjMoney>> for DynMoney {
    type Error = MoneyError;

    fn try_from(value: Box<dyn super::ObjMoney>) -> Result<Self, Self::Error> {
        DynMoney::try_from(value.as_ref())
    }
}

/// Converts a [`DynMoney`] into a typed [`Money<C>`](crate::Money).
///
/// Succeeds only when the `DynMoney`'s runtime currency code matches `C::CODE`.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatchError`] when the currency codes differ.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, BaseMoney, obj_money::{DynMoney, ObjMoney}, macros::dec, iso::{USD, EUR}};
///
/// let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
/// let money = Money::<USD>::try_from(dyn_m).unwrap();
/// assert_eq!(BaseMoney::amount(&money), dec!(100.00));
///
/// // Currency mismatch returns an error
/// assert!(Money::<EUR>::try_from(dyn_m).is_err());
/// ```
impl<C: Currency> TryFrom<DynMoney> for crate::Money<C> {
    type Error = MoneyError;

    fn try_from(value: DynMoney) -> Result<Self, Self::Error> {
        if value.currency.code != C::CODE {
            return Err(MoneyError::CurrencyMismatchError(
                value.currency.code.into(),
                C::CODE.into(),
            ));
        }

        use crate::BaseMoney;
        Ok(Self::from_decimal(value.amount))
    }
}

#[cfg(feature = "raw_money")]
/// Converts a [`DynMoney`] into a typed [`RawMoney<C>`](crate::RawMoney).
///
/// Succeeds only when the `DynMoney`'s runtime currency code matches `C::CODE`.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatchError`] when the currency codes differ.
///
/// # Examples
///
/// ```
/// use moneylib::{RawMoney, BaseMoney, obj_money::{DynMoney, ObjMoney}, macros::dec, iso::{USD, EUR}};
///
/// let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
/// let raw = RawMoney::<USD>::try_from(dyn_m).unwrap();
/// assert_eq!(BaseMoney::amount(&raw), dec!(100.00));
///
/// // Currency mismatch returns an error
/// assert!(RawMoney::<EUR>::try_from(dyn_m).is_err());
/// ```
impl<C: Currency> TryFrom<DynMoney> for crate::RawMoney<C> {
    type Error = MoneyError;

    fn try_from(value: DynMoney) -> Result<Self, Self::Error> {
        if value.currency.code != C::CODE {
            return Err(MoneyError::CurrencyMismatchError(
                value.currency.code.into(),
                C::CODE.into(),
            ));
        }

        use crate::BaseMoney;
        Ok(Self::from_decimal(value.amount))
    }
}

// Equality

impl PartialEq<&dyn ObjMoney> for DynMoney {
    fn eq(&self, other: &&dyn ObjMoney) -> bool {
        self.currency.code == other.code() && self.amount == other.amount()
    }
}

impl PartialEq<Box<dyn ObjMoney>> for DynMoney {
    fn eq(&self, other: &Box<dyn ObjMoney>) -> bool {
        self.currency.code == other.code() && self.amount == other.amount()
    }
}

use crate::{BaseMoney, Money};
impl<C: Currency> PartialEq<Money<C>> for DynMoney {
    fn eq(&self, other: &Money<C>) -> bool {
        self.currency.code == C::CODE && self.amount == other.amount()
    }
}

#[cfg(feature = "raw_money")]
use crate::RawMoney;

#[cfg(feature = "raw_money")]
impl<C: Currency> PartialEq<RawMoney<C>> for DynMoney {
    fn eq(&self, other: &RawMoney<C>) -> bool {
        self.currency.code == C::CODE && self.amount == other.amount()
    }
}

// Ordering

impl PartialOrd<&dyn ObjMoney> for DynMoney {
    fn partial_cmp(&self, other: &&dyn ObjMoney) -> Option<std::cmp::Ordering> {
        if self.currency.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}

impl PartialOrd<Box<dyn ObjMoney>> for DynMoney {
    fn partial_cmp(&self, other: &Box<dyn ObjMoney>) -> Option<std::cmp::Ordering> {
        if self.currency.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}

impl<C: Currency> PartialOrd<Money<C>> for DynMoney {
    fn partial_cmp(&self, other: &Money<C>) -> Option<std::cmp::Ordering> {
        if self.currency.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}

#[cfg(feature = "raw_money")]
impl<C: Currency> PartialOrd<RawMoney<C>> for DynMoney {
    fn partial_cmp(&self, other: &RawMoney<C>) -> Option<std::cmp::Ordering> {
        if self.currency.code != other.code() {
            return None;
        }
        self.amount.partial_cmp(&other.amount())
    }
}
