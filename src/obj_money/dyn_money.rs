use std::str::FromStr;

use crate::{
    BaseMoney, BaseOps, Currency, Decimal, MoneyError, MoneyFormatter, MoneyParser,
    RoundingStrategy,
    base::{Amount, DecimalNumber},
    fmt::{CODE_FORMAT, CODE_FORMAT_MINOR, SYMBOL_FORMAT, SYMBOL_FORMAT_MINOR},
};
use currencylib::data::Data;
use rust_decimal::{MathematicalOps, prelude::ToPrimitive};

#[derive(Debug, Clone, Copy, Eq)]
pub struct DynCurrency(pub(super) Data);

impl PartialEq for DynCurrency {
    fn eq(&self, other: &Self) -> bool {
        self.0.code == other.0.code
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct DynMoney {
    pub(super) amount: Decimal,
    pub(super) currency: DynCurrency,
}

impl DynMoney {
    pub fn new<C: Currency>(amount: Decimal) -> Self {
        Self {
            amount: super::context::amount::<C>(amount),
            currency: DynCurrency(super::context::into_currency_data::<C>()),
        }
    }

    pub fn set_curr<C: Currency>(&mut self) {
        self.currency = DynCurrency(super::context::into_currency_data::<C>());
    }

    pub fn set_curr_from_code(&mut self, code: &str) -> Result<(), MoneyError> {
        if let Some(curr) = super::context::get_currency(code) {
            self.currency = curr;
            return Ok(());
        }

        Err(MoneyError::Other(
            format!("currency {} not found", code).into(),
        ))
    }

    /// Build a `DynMoney` from a raw `Decimal` and a `DynCurrency`, rounding to the
    /// currency's minor unit unless the global context is set to raw mode.
    fn from_decimal_with_currency(amount: Decimal, currency: DynCurrency) -> Self {
        let amount = if super::context::is_raw() {
            amount
        } else {
            amount.round_dp(currency.0.minor_unit.into())
        };
        Self { amount, currency }
    }
}

impl PartialEq for DynMoney {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency.0.code == other.currency.0.code
    }
}

impl PartialOrd for DynMoney {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency.0.code == other.currency.0.code {
            return Some(self.amount.cmp(&other.amount));
        }
        None
    }
}

impl<C: Currency> Amount<C> for DynMoney {
    fn get_decimal(&self) -> Option<Decimal> {
        if C::CODE == self.currency.0.code {
            return Some(self.amount);
        }
        None
    }
}

impl<C: Currency> BaseMoney<C> for DynMoney {
    // ---- Required methods ----

    #[inline]
    fn from_decimal(amount: Decimal) -> Self {
        Self::new::<C>(amount)
    }

    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    fn minor_amount(&self) -> Option<i128> {
        self.amount
            .round_dp(self.currency.0.minor_unit.into())
            .checked_mul(crate::dec!(10).checked_powu(self.currency.0.minor_unit.into())?)?
            .to_i128()
    }

    #[inline]
    fn name(&self) -> &str {
        self.currency.0.name
    }

    #[inline]
    fn symbol(&self) -> &str {
        self.currency.0.symbol
    }

    #[inline]
    fn code(&self) -> &str {
        self.currency.0.code
    }

    #[inline]
    fn numeric_code(&self) -> i32 {
        self.currency.0.numeric.into()
    }

    #[inline]
    fn minor_unit(&self) -> u16 {
        self.currency.0.minor_unit
    }

    #[inline]
    fn thousand_separator(&self) -> &str {
        self.currency.0.thousand_separator
    }

    #[inline]
    fn decimal_separator(&self) -> &str {
        self.currency.0.decimal_separator
    }

    // ---- Provided methods overridden to use runtime currency data ----

    /// Rounds to the runtime currency's minor unit (ignores `C::MINOR_UNIT`).
    #[inline]
    fn round(self) -> Self {
        Self {
            amount: self.amount.round_dp(self.currency.0.minor_unit.into()),
            currency: self.currency,
        }
    }

    /// Rounds to `decimal_points` with `strategy`, preserving the runtime currency.
    #[inline]
    fn round_with(self, decimal_points: u32, strategy: RoundingStrategy) -> Self {
        Self {
            amount: self
                .amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
            currency: self.currency,
        }
    }

    /// Truncates the fractional part, preserving the runtime currency.
    #[inline]
    fn truncate(&self) -> Self {
        Self {
            amount: self.amount.trunc(),
            currency: self.currency,
        }
    }

    /// Truncates to `scale` decimal places, preserving the runtime currency.
    #[inline]
    fn truncate_with(&self, scale: u32) -> Self {
        Self {
            amount: self.amount.trunc_with_scale(scale),
            currency: self.currency,
        }
    }

    fn format_code(&self) -> String {
        super::fmt::format_obj_money(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            self.currency.0.thousand_separator,
            self.currency.0.decimal_separator,
            CODE_FORMAT,
        )
    }

    fn format_symbol(&self) -> String {
        super::fmt::format_obj_money(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            self.currency.0.thousand_separator,
            self.currency.0.decimal_separator,
            SYMBOL_FORMAT,
        )
    }

    fn format_code_minor(&self) -> String {
        super::fmt::format_obj_money(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            self.currency.0.thousand_separator,
            self.currency.0.decimal_separator,
            CODE_FORMAT_MINOR,
        )
    }

    fn format_symbol_minor(&self) -> String {
        super::fmt::format_obj_money(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            self.currency.0.thousand_separator,
            self.currency.0.decimal_separator,
            SYMBOL_FORMAT_MINOR,
        )
    }
}

impl<C: Currency> BaseOps<C> for DynMoney {
    // All BaseOps methods are overridden to preserve the runtime currency stored in `self`
    // instead of creating a new DynMoney tied to the `C` type parameter.

    /// Returns the absolute value while preserving the runtime currency.
    #[inline(always)]
    fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            currency: self.currency,
        }
    }

    /// Adds `rhs` to this money value, preserving the runtime currency.
    ///
    /// Returns `None` on overflow or when `rhs` is a `DynMoney` with a different currency code.
    #[inline(always)]
    fn checked_add<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: Amount<C>,
    {
        Some(Self {
            amount: self.amount.checked_add(rhs.get_decimal()?)?,
            currency: self.currency,
        })
    }

    /// Subtracts `rhs` from this money value, preserving the runtime currency.
    ///
    /// Returns `None` on overflow or when `rhs` is a `DynMoney` with a different currency code.
    fn checked_sub<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: Amount<C>,
    {
        Some(Self {
            amount: self.amount.checked_sub(rhs.get_decimal()?)?,
            currency: self.currency,
        })
    }

    /// Multiplies this money value by `rhs`, preserving the runtime currency.
    fn checked_mul<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber,
    {
        Some(Self {
            amount: self.amount.checked_mul(rhs.get_decimal()?)?,
            currency: self.currency,
        })
    }

    /// Divides this money value by `rhs`, preserving the runtime currency.
    fn checked_div<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber,
    {
        Some(Self {
            amount: self.amount.checked_div(rhs.get_decimal()?)?,
            currency: self.currency,
        })
    }

    /// Returns the remainder of dividing this money value by `rhs`, preserving the runtime currency.
    fn checked_rem<RHS>(&self, rhs: RHS) -> Option<Self>
    where
        RHS: DecimalNumber,
    {
        Some(Self {
            amount: self.amount.checked_rem(rhs.get_decimal()?)?,
            currency: self.currency,
        })
    }
}

impl<C: Currency> MoneyParser<C> for DynMoney {
    // MoneyParser methods are overridden so that code-based parsing derives the currency
    // from the string itself (looked up in the runtime CURRENCIES map) rather than
    // requiring the currency code to match the `C` type parameter.
    //
    // Symbol-based parsing still uses `C::SYMBOL` to identify the currency symbol,
    // since symbols are not unique across currencies (e.g. `$` is used by USD, AUD, CAD …).

    /// Parses `"<CODE> <AMOUNT>"` using explicit separators.
    ///
    /// The currency code is extracted from the string and looked up in the global currencies map;
    /// the `C` type parameter is **not** used for code validation.
    fn from_str_code_with(
        amount_str: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> Result<Self, MoneyError> {
        let str_code = amount_str.trim();

        let parts: Vec<&str> = str_code.split_whitespace().collect();
        if parts.len() != 2
            || parts[0].is_empty()
            || !parts[0].chars().all(|c| c.is_ascii_alphabetic())
            || parts[1].is_empty()
        {
            return Err(MoneyError::ParseStrError(
                format!(
                    "invalid currency with code, expected: <CODE> <AMOUNT> with <CODE> and <AMOUNT> all in ascii, found: {}",
                    str_code
                )
                .into(),
            ));
        }

        let code = parts[0];
        let currency = super::context::get_currency(code).ok_or_else(|| {
            MoneyError::CurrencyMismatchError(code.into(), "unknown (not in currencies map)".into())
        })?;

        let cleaned =
            crate::parse::parse_amount_str(parts[1], thousand_separator, decimal_separator)?;
        let amount = Decimal::from_str(&cleaned).map_err(|err| {
            MoneyError::ParseStrError(
                format!("failed parsing {} into decimal: {}", cleaned, err).into(),
            )
        })?;

        Ok(Self::from_decimal_with_currency(amount, currency))
    }

    /// Parses `"<SYMBOL><AMOUNT>"` using explicit separators.
    ///
    /// `C::SYMBOL` is used to strip the currency symbol from the string, and the corresponding
    /// currency data is looked up by `C::CODE` in the runtime currencies map.
    fn from_str_symbol_with(
        amount_str: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> Result<Self, MoneyError> {
        let str_symbol = amount_str.trim();

        let (abs_money, is_negative) = if let Some(trimmed) = str_symbol.strip_prefix('-') {
            (trimmed, true)
        } else {
            (str_symbol, false)
        };

        let amount_part = abs_money.strip_prefix(C::SYMBOL);
        let amount_part = if let Some(amount) = amount_part
            && !amount.is_empty()
        {
            amount
        } else {
            return Err(MoneyError::CurrencyMismatchError(
                str_symbol.into(),
                C::SYMBOL.into(),
            ));
        };

        // Look up the currency by code in the global map.
        let currency = super::context::get_currency(C::CODE).ok_or_else(|| {
            MoneyError::CurrencyMismatchError(
                C::CODE.into(),
                "unknown (not in currencies map)".into(),
            )
        })?;

        let amount_with_sign = if is_negative {
            format!("-{}", amount_part)
        } else {
            amount_part.to_string()
        };
        let cleaned = crate::parse::parse_amount_str(
            &amount_with_sign,
            thousand_separator,
            decimal_separator,
        )?;
        let amount = Decimal::from_str(&cleaned).map_err(|err| {
            MoneyError::ParseStrError(
                format!("failed parsing {} into decimal: {}", cleaned, err).into(),
            )
        })?;

        Ok(Self::from_decimal_with_currency(amount, currency))
    }

    /// Parses `"<CODE> <AMOUNT>"` using the runtime currency's own locale separators.
    ///
    /// The currency code is extracted from the string and looked up in the global currencies map.
    /// Its `thousand_separator` and `decimal_separator` fields are used automatically.
    fn from_str_code(amount_str: &str) -> Result<Self, MoneyError> {
        let str_code = amount_str.trim();

        let parts: Vec<&str> = str_code.split_whitespace().collect();
        if parts.len() != 2
            || parts[0].is_empty()
            || !parts[0].chars().all(|c| c.is_ascii_alphabetic())
            || parts[1].is_empty()
        {
            return Err(MoneyError::ParseStrError(
                format!(
                    "invalid currency with code, expected: <CODE> <AMOUNT> with <CODE> and <AMOUNT> all in ascii, found: {}",
                    str_code
                )
                .into(),
            ));
        }

        let code = parts[0];
        let currency = super::context::get_currency(code).ok_or_else(|| {
            MoneyError::CurrencyMismatchError(code.into(), "unknown (not in currencies map)".into())
        })?;

        // Use the runtime currency's own separators.
        let cleaned = crate::parse::parse_amount_str(
            parts[1],
            currency.0.thousand_separator,
            currency.0.decimal_separator,
        )?;
        let amount = Decimal::from_str(&cleaned).map_err(|err| {
            MoneyError::ParseStrError(
                format!("failed parsing {} into decimal: {}", cleaned, err).into(),
            )
        })?;

        Ok(Self::from_decimal_with_currency(amount, currency))
    }

    /// Parses `"<SYMBOL><AMOUNT>"` using `C::SYMBOL` and the runtime currency's locale separators.
    fn from_str_symbol(amount_str: &str) -> Result<Self, MoneyError> {
        let str_symbol = amount_str.trim();

        let (abs_money, is_negative) = if let Some(trimmed) = str_symbol.strip_prefix('-') {
            (trimmed, true)
        } else {
            (str_symbol, false)
        };

        let amount_part = abs_money.strip_prefix(C::SYMBOL);
        let amount_part = if let Some(amount) = amount_part
            && !amount.is_empty()
        {
            amount
        } else {
            return Err(MoneyError::CurrencyMismatchError(
                str_symbol.into(),
                C::SYMBOL.into(),
            ));
        };

        // Look up the currency by code in the global map and use its own separators.
        let currency = super::context::get_currency(C::CODE).ok_or_else(|| {
            MoneyError::CurrencyMismatchError(
                C::CODE.into(),
                "unknown (not in currencies map)".into(),
            )
        })?;

        let amount_with_sign = if is_negative {
            format!("-{}", amount_part)
        } else {
            amount_part.to_string()
        };
        let cleaned = crate::parse::parse_amount_str(
            &amount_with_sign,
            currency.0.thousand_separator,
            currency.0.decimal_separator,
        )?;
        let amount = Decimal::from_str(&cleaned).map_err(|err| {
            MoneyError::ParseStrError(
                format!("failed parsing {} into decimal: {}", cleaned, err).into(),
            )
        })?;

        Ok(Self::from_decimal_with_currency(amount, currency))
    }
}

impl<C: Currency> MoneyFormatter<C> for DynMoney {
    // MoneyFormatter methods are overridden to use the runtime currency's locale separators,
    // minor unit, code, symbol and minor-unit symbol stored in `self.currency.0`, rather
    // than the compile-time `C::*` constants.

    /// Formats this `DynMoney` with `format_str` using the runtime currency's locale separators.
    fn format(&self, format_str: &str) -> String {
        super::fmt::format_obj_money(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            self.currency.0.thousand_separator,
            self.currency.0.decimal_separator,
            format_str,
        )
    }

    /// Formats this `DynMoney` with explicit separators, using the runtime minor unit.
    fn format_with_separator(
        &self,
        format_str: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> String {
        super::fmt::format_obj_money(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            thousand_separator,
            decimal_separator,
            format_str,
        )
    }

    /// Locale-aware formatting using the runtime currency's code, symbol and minor-unit data.
    #[cfg(feature = "locale")]
    fn format_locale_amount(
        &self,
        locale_str: &str,
        format_str: &str,
    ) -> Result<String, MoneyError> {
        super::fmt::format_obj_money_locale(
            self.amount,
            self.currency.0.code,
            self.currency.0.symbol,
            self.currency.0.minor_unit_symbol,
            self.currency.0.minor_unit,
            locale_str,
            format_str,
        )
    }
}
