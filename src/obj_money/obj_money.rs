use super::dyn_money::DynMoney;
use crate::exchange::ObjRate;
use crate::{BaseMoney, Currency, Decimal, MoneyError, base::DecimalNumber};
use crate::{RoundingStrategy, dec};
use currencylib::data;
use g_string::GString;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::ToPrimitive;
use std::str::FromStr;
use std::sync::RwLock;
use std::{collections::HashMap, fmt::Debug, sync::OnceLock};

type CurrencyCode = GString<(), 3, 4, true>;
type CurrencySymbol = GString<(), 1, 16>;
type CurrencyMinorUnitSymbol = GString<(), 0, 16>;
type CurrencyName = GString<(), 1, 100>;

static CURRENCIES: OnceLock<RwLock<HashMap<CurrencyCode, ObjCurrency>>> = OnceLock::new();

pub(super) struct CodeToCurrencyMap(pub(super) CurrencyCode, pub(super) ObjCurrency);

impl TryFrom<(&'static str, data::Data)> for CodeToCurrencyMap {
    type Error = MoneyError;

    fn try_from((k, v): (&'static str, data::Data)) -> Result<Self, Self::Error> {
        Ok(CodeToCurrencyMap(
            CurrencyCode::try_new(k).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!("failed initializing currency code {} as key: {}", k, err).into(),
                )
            })?,
            ObjCurrency {
                code: CurrencyCode::try_new(v.code).map_err(|err| {
                    MoneyError::ObjMoneyError(
                        format!("failed initializing currency code {}: {}", v.code, err).into(),
                    )
                })?,
                symbol: CurrencySymbol::try_new(v.symbol).map_err(|err| {
                    MoneyError::ObjMoneyError(
                        format!("failed initializing currency symbol {}: {}", v.symbol, err).into(),
                    )
                })?,
                minor_unit_symbol: CurrencyMinorUnitSymbol::try_new(v.minor_unit_symbol).map_err(
                    |err| {
                        MoneyError::ObjMoneyError(
                            format!(
                                "failed initializing currency minor unit symbol {}: {}",
                                v.minor_unit_symbol, err
                            )
                            .into(),
                        )
                    },
                )?,
                name: CurrencyName::try_new(v.name).map_err(|err| {
                    MoneyError::ObjMoneyError(
                        format!("failed initializing currency name {}: {}", v.name, err).into(),
                    )
                })?,
                minor_unit: v.minor_unit,
            },
        ))
    }
}

impl From<CodeToCurrencyMap> for Result<(CurrencyCode, ObjCurrency), MoneyError> {
    fn from(value: CodeToCurrencyMap) -> Self {
        Ok((value.0, value.1))
    }
}

fn currencies() -> Result<&'static RwLock<HashMap<CurrencyCode, ObjCurrency>>, MoneyError> {
    if let Some(map) = CURRENCIES.get() {
        return Ok(map);
    }
    let map = data::entries()
        .map(
            |curr_data| -> Result<(CurrencyCode, ObjCurrency), MoneyError> {
                <(&str, currencylib::data::Data) as TryInto<CodeToCurrencyMap>>::try_into(
                    curr_data,
                )?
                .into()
            },
        )
        .collect::<Result<HashMap<_, _>, _>>()?;

    let _ = CURRENCIES.set(RwLock::new(map));

    CURRENCIES.get().ok_or(MoneyError::ObjMoneyError(
        "failed getting the currencies".into(),
    ))
}

/// Register new currency for runtime validation.
///
/// The currency will be added into existing currencies of ISO 4217.
///
/// Currency code is the identity for a currency, so it cannot be duplicated.
pub fn register_currency(
    code: &str,
    symbol: &str,
    minor_unit_symbol: &str,
    name: &str,
    minor_unit: u16,
) -> Result<(), MoneyError> {
    let mut existing = currencies()?
        .write()
        .map_err(|_| MoneyError::ObjMoneyError("failed getting lock to write".into()))?;

    if existing.contains_key(code) {
        return Err(MoneyError::ObjMoneyError(
            format!("currency code {} is already existed", code).into(),
        ));
    }

    let code_key = CurrencyCode::try_new(code).map_err(|err| {
        MoneyError::ObjMoneyError(format!("failed initializing currency code: {}", err).into())
    })?;
    let curr = ObjCurrency {
        code: code_key,
        symbol: CurrencySymbol::try_new(symbol).map_err(|err| {
            MoneyError::ObjMoneyError(
                format!("failed initializing currency symbol: {}", err).into(),
            )
        })?,
        minor_unit_symbol: CurrencyMinorUnitSymbol::try_new(minor_unit_symbol).map_err(|err| {
            MoneyError::ObjMoneyError(
                format!("failed initializing currency minor unit symbol: {}", err).into(),
            )
        })?,
        name: CurrencyName::try_new(name).map_err(|err| {
            MoneyError::ObjMoneyError(format!("failed initializing currency name: {}", err).into())
        })?,
        minor_unit,
    };

    existing.insert(code_key, curr);

    Ok(())
}

/// `ObjMoney` is type for runtime money where currency is resolved at runtime.
///
/// This is useful for user-specified currencies and aggregating multiple currencies.
#[derive(Clone, Copy, Debug)]
pub struct ObjMoney<const IS_RAW: bool = false> {
    amount: Decimal,
    currency: ObjCurrency,
}

/// `ObjCurrency` is runtime currency type.
#[derive(Clone, Copy, Debug)]
pub struct ObjCurrency {
    code: CurrencyCode,
    symbol: CurrencySymbol,
    minor_unit_symbol: CurrencyMinorUnitSymbol,
    name: CurrencyName,
    minor_unit: u16,
}

impl ObjCurrency {
    /// Constructs new ObjCurrency.
    pub fn try_new(
        code: &str,
        symbol: &str,
        minor_unit_symbol: &str,
        name: &str,
        minor_unit: u16,
    ) -> Result<ObjCurrency, MoneyError> {
        Ok(ObjCurrency {
            code: CurrencyCode::try_new(code).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!(
                        "failed constructing currency code {} with error: {}",
                        code, err
                    )
                    .into(),
                )
            })?,
            symbol: CurrencySymbol::try_new(symbol).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!(
                        "failed constructing currency symbol {} with error: {}",
                        symbol, err
                    )
                    .into(),
                )
            })?,
            minor_unit_symbol: CurrencyMinorUnitSymbol::try_new(minor_unit_symbol).map_err(
                |err| {
                    MoneyError::ObjMoneyError(
                        format!(
                            "failed constructing currency minor unit symbol {} with error: {}",
                            minor_unit_symbol, err
                        )
                        .into(),
                    )
                },
            )?,
            name: CurrencyName::try_new(name).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!(
                        "failed constructing currency name {} with error: {}",
                        name, err
                    )
                    .into(),
                )
            })?,
            minor_unit,
        })
    }
}

impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    #[inline(always)]
    fn round_amount(amount: Decimal, dp: u32) -> Decimal {
        if IS_RAW { amount } else { amount.round_dp(dp) }
    }

    /// Constructs new ObjMoney from ObjCurrency.
    #[inline]
    pub fn new(currency: ObjCurrency, amount: Decimal) -> Self {
        Self {
            amount: Self::round_amount(amount, currency.minor_unit.into()),
            currency,
        }
    }

    /// Constructs new ObjMoney from currency code.
    ///
    /// It checks for registered currencies, including ISO 4217.
    pub fn try_new(currency_code: &str, amount: Decimal) -> Result<Self, MoneyError> {
        let code_key = GString::try_new(currency_code).map_err(|err| {
            MoneyError::ObjMoneyError(
                format!(
                    "failed parsing currency code {} with error: {}",
                    currency_code, err
                )
                .into(),
            )
        })?;
        let currencies = currencies()?
            .read()
            .map_err(|_| MoneyError::ObjMoneyError("failed reading currencies lock".into()))?;
        let obj_curr = currencies.get(&code_key).ok_or(MoneyError::ObjMoneyError(
            format!("currency {} is not found", currency_code).into(),
        ))?;

        Ok(Self {
            amount: Self::round_amount(amount, obj_curr.minor_unit.into()),
            currency: *obj_curr,
        })
    }

    #[inline]
    pub(super) fn set_amount(mut self, new_amount: Decimal) -> Self {
        self.amount = new_amount;
        self
    }

    /// Update amount.
    ///
    /// It rounds the `new_amount` if `IS_RAW` is false.
    #[inline]
    pub fn update_amount(self, new_amount: Decimal) -> Self {
        self.set_amount(Self::round_amount(new_amount, self.minor_unit().into()))
    }

    /// Amount.
    #[inline]
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// Minor amount. It rounds first if it's in raw.
    #[inline]
    pub fn minor_amount(&self) -> Option<i128> {
        // if the amount is raw, round it first
        if IS_RAW {
            self.amount().round_dp(self.minor_unit().into())
        } else {
            self.amount()
        }
        .checked_mul(dec!(10).checked_powu(self.minor_unit().into())?)?
        .to_i128()
    }

    /// Rounds to currency's minor unit using banker's rounding rule.
    #[inline]
    pub fn round(self) -> Self {
        self.set_amount(self.amount().round_dp(self.minor_unit().into()))
    }

    /// Rounds to selected minor unit/decimal points and rounding strategy.
    #[inline]
    pub fn round_with(self, decimal_points: u32, strategy: RoundingStrategy) -> Self {
        self.set_amount(
            self.amount()
                .round_dp_with_strategy(decimal_points, strategy.into()),
        )
    }

    /// Currency code.
    #[inline]
    pub fn code(&self) -> &str {
        self.currency.code.as_str()
    }

    /// Currency symbol.
    #[inline]
    pub fn symbol(&self) -> &str {
        self.currency.symbol.as_str()
    }

    /// Currency minor unit symbol.
    #[inline]
    pub fn minor_unit_symbol(&self) -> &str {
        self.currency.minor_unit_symbol.as_str()
    }

    /// Currency name.
    #[inline]
    pub fn name(&self) -> &str {
        self.currency.name.as_str()
    }

    /// Currency minor unit.
    #[inline]
    pub fn minor_unit(&self) -> u16 {
        self.currency.minor_unit
    }
}

// Ops
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    /// Absolute value.
    #[inline]
    pub fn abs(&self) -> Self {
        self.update_amount(self.amount().abs())
    }

    /// Check if amount is zero
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.amount().is_zero()
    }

    /// Check if amount is bigger than zero.
    #[inline]
    pub fn is_positive(&self) -> bool {
        if self.is_zero() {
            return false;
        }
        self.amount().is_sign_positive()
    }

    /// Check if amount is smaller than zero.
    #[inline]
    pub fn is_negative(&self) -> bool {
        if self.is_zero() {
            return false;
        }
        self.amount().is_sign_negative()
    }

    /// Adds ObjMoney to `impl DynMoney`: `ObjMoney`, `Money<C>`, `RawMoney<C>`.
    ///
    /// Currency is checked at runtime.
    #[inline]
    pub fn checked_add<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: DynMoney,
    {
        if self.code() != rhs.code() {
            return Err(MoneyError::ObjMoneyError(
                format!(
                    "currency mismatch, got {}, expected {}",
                    rhs.code(),
                    self.code()
                )
                .into(),
            ));
        }
        Ok(self.update_amount(
            self.amount()
                .checked_add(rhs.amount())
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    /// Substracts ObjMoney to `impl DynMoney`: `ObjMoney`, `Money<C>`, `RawMoney<C>`.
    ///
    /// Currency is checked at runtime.
    #[inline]
    pub fn checked_sub<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: DynMoney,
    {
        if self.code() != rhs.code() {
            return Err(MoneyError::ObjMoneyError(
                format!(
                    "currency mismatch, got {}, expected {}",
                    rhs.code(),
                    self.code()
                )
                .into(),
            ));
        }
        Ok(self.update_amount(
            self.amount()
                .checked_sub(rhs.amount())
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    /// Multiplies ObjMoney to `impl DecimalNumber`: Decimal, f64, i32, i64, i128.
    ///
    /// Currency is checked at runtime.
    #[inline]
    pub fn checked_mul<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: DecimalNumber,
    {
        Ok(self.update_amount(
            self.amount()
                .checked_mul(rhs.get_decimal().ok_or(MoneyError::OverflowError)?)
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    /// Divides ObjMoney to `impl DecimalNumber`: Decimal, f64, i32, i64, i128.
    ///
    /// Currency is checked at runtime.
    #[inline]
    pub fn checked_div<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: DecimalNumber,
    {
        Ok(self.update_amount(
            self.amount()
                .checked_div(rhs.get_decimal().ok_or(MoneyError::OverflowError)?)
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    /// Get remainder.
    #[inline]
    pub fn checked_rem<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: DecimalNumber,
    {
        Ok(self.update_amount(
            self.amount()
                .checked_rem(rhs.get_decimal().ok_or(MoneyError::OverflowError)?)
                .ok_or(MoneyError::OverflowError)?,
        ))
    }
}

impl From<ObjMoney<false>> for ObjMoney<true> {
    fn from(value: ObjMoney<false>) -> Self {
        Self::new(value.currency, value.amount())
    }
}

impl From<ObjMoney<true>> for ObjMoney<false> {
    fn from(value: ObjMoney<true>) -> Self {
        Self::new(value.currency, value.amount())
    }
}

// parsing
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    /// Parse ObjMoney from string with format `<CODE> <AMOUNT>`.
    ///
    /// `<CODE>` must be registered or valid ISO 4217 currencies.
    pub fn from_str_code(
        money_str: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> Result<Self, MoneyError> {
        let parts: Vec<&str> = money_str.split_whitespace().collect();
        if parts.is_empty() {
            return Err(MoneyError::ObjMoneyError(
                format!("invalid string: {}", money_str).into(),
            ));
        }
        let code = parts[0];
        let amount_str = crate::parse::parse_str_code_internal(
            code,
            money_str,
            thousand_separator,
            decimal_separator,
        )?;
        ObjMoney::try_new(
            code,
            Decimal::from_str(&amount_str).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!("failed parsing string amount \"{}\": {}", &amount_str, err).into(),
                )
            })?,
        )
    }
}

// formatting
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    /// Format ObjMoney into `format_str` with specified thousand and decimal separators.
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
    pub fn format(
        &self,
        format_str: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> String {
        crate::fmt::format_with_separator_internal(
            self.code(),
            self.symbol(),
            self.minor_unit(),
            self.minor_unit_symbol(),
            self.is_negative(),
            self.amount(),
            self.minor_amount(),
            format_str,
            thousand_separator,
            decimal_separator,
        )
    }
}

impl<const IS_RAW: bool> std::ops::Neg for ObjMoney<IS_RAW> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

// conversion
#[cfg(feature = "exchange")]
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    /// Converts ObjMoney into `target` currency code.
    ///
    /// `target` currency code must be registered or valid ISO 4217.
    ///
    /// `rate` is the [`crate::ExchangeRates`].
    #[inline]
    pub fn convert(
        &self,
        target: &str,
        rate: &impl ObjRate,
    ) -> Result<ObjMoney<IS_RAW>, MoneyError> {
        ObjMoney::try_new(
            target,
            self.amount()
                .checked_mul(rate.get_rate(self.code(), target).ok_or(
                    MoneyError::ObjMoneyError(
                        format!("fail getting rate for {}/{}", self.code(), target).into(),
                    ),
                )?)
                .ok_or(MoneyError::OverflowError)?,
        )
    }

    /// Converts ObjMoney into multiple `targets` of currency codes.
    ///
    /// `targets` currency codes must be registered or valid ISO 4217.
    ///
    /// `rate` is the [`crate::ExchangeRates`].
    #[inline]
    pub fn convert_multi<'a, 'b, I>(
        &'a self,
        targets: I,
        rate: &impl ObjRate,
    ) -> Result<Vec<ObjMoney<IS_RAW>>, MoneyError>
    where
        I: IntoIterator<Item = &'b str>,
        'b: 'a,
    {
        targets
            .into_iter()
            .map(|to| {
                self.convert(to, rate).map_err(|err| {
                    MoneyError::ObjMoneyError(
                        format!("fail converting from {} to {}: {err}", self.code(), to).into(),
                    )
                })
            })
            .collect::<Result<Vec<ObjMoney<IS_RAW>>, MoneyError>>()
    }
}

impl<const IS_RAW: bool, C: Currency> TryFrom<crate::Money<C>> for ObjMoney<IS_RAW> {
    type Error = MoneyError;

    fn try_from(value: crate::Money<C>) -> Result<Self, Self::Error> {
        ObjMoney::try_new(C::CODE, BaseMoney::amount(&value))
    }
}

impl<const IS_RAW: bool, C: Currency> TryFrom<ObjMoney<IS_RAW>> for crate::Money<C> {
    type Error = MoneyError;

    fn try_from(value: ObjMoney<IS_RAW>) -> Result<Self, Self::Error> {
        if value.code() != C::CODE {
            return Err(MoneyError::ObjMoneyError(
                format!(
                    "failed converting from ObjMoney {} to Money {}",
                    value.code(),
                    C::CODE
                )
                .into(),
            ));
        }
        Ok(Self::from_decimal(value.amount()))
    }
}

#[cfg(feature = "raw_money")]
impl<const IS_RAW: bool, C: Currency> TryFrom<crate::RawMoney<C>> for ObjMoney<IS_RAW> {
    type Error = MoneyError;

    fn try_from(value: crate::RawMoney<C>) -> Result<Self, Self::Error> {
        ObjMoney::try_new(C::CODE, BaseMoney::amount(&value))
    }
}

#[cfg(feature = "raw_money")]
impl<const IS_RAW: bool, C: Currency> TryFrom<ObjMoney<IS_RAW>> for crate::RawMoney<C> {
    type Error = MoneyError;

    fn try_from(value: ObjMoney<IS_RAW>) -> Result<Self, Self::Error> {
        if value.code() != C::CODE {
            return Err(MoneyError::ObjMoneyError(
                format!(
                    "failed converting from ObjMoney {} to RawMoney {}",
                    value.code(),
                    C::CODE
                )
                .into(),
            ));
        }
        Ok(Self::from_decimal(value.amount()))
    }
}

impl<const IS_RAW_LHS: bool, const IS_RAW_RHS: bool> PartialEq<ObjMoney<IS_RAW_RHS>>
    for ObjMoney<IS_RAW_LHS>
{
    fn eq(&self, other: &ObjMoney<IS_RAW_RHS>) -> bool {
        self.code() == other.code() && self.amount() == other.amount()
    }
}

impl<const IS_RAW_LHS: bool, const IS_RAW_RHS: bool> PartialOrd<ObjMoney<IS_RAW_RHS>>
    for ObjMoney<IS_RAW_LHS>
{
    fn partial_cmp(&self, other: &ObjMoney<IS_RAW_RHS>) -> Option<std::cmp::Ordering> {
        if self.code() != other.code() {
            return None;
        }
        Some(self.amount().cmp(&other.amount()))
    }
}
