use super::dyn_money::DynMoney;
use crate::exchange::ObjRate;
use crate::{BaseMoney, Currency, Decimal, MoneyError, base::DecimalNumber};
use crate::{RoundingStrategy, dec};
use currencylib::data;
use g_string::GString;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::ToPrimitive;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Debug, sync::OnceLock};

static CURRENCIES: OnceLock<HashMap<GString<(), 3, 4, true>, ObjCurrency>> = OnceLock::new();

fn currencies() -> Result<&'static HashMap<GString<(), 3, 4, true>, ObjCurrency>, MoneyError> {
    if let Some(map) = CURRENCIES.get() {
        return Ok(map);
    }
    let map = data::entries()
        .map(|(k, v)| -> Result<_, MoneyError> {
            Ok((
                GString::try_new(k).map_err(|err| {
                    MoneyError::ObjMoneyError(
                        format!("failed initializing currency code {} as key: {}", k, err).into(),
                    )
                })?,
                ObjCurrency {
                    code: GString::try_new(v.code).map_err(|err| {
                        MoneyError::ObjMoneyError(
                            format!("failed initializing currency code: {}", err).into(),
                        )
                    })?,
                    symbol: GString::try_new(v.symbol).map_err(|err| {
                        MoneyError::ObjMoneyError(
                            format!("failed initializing currency symbol: {}", err).into(),
                        )
                    })?,
                    minor_unit_symbol: GString::try_new(v.minor_unit_symbol).map_err(|err| {
                        MoneyError::ObjMoneyError(
                            format!("failed initializing currency minor unit symbol: {}", err)
                                .into(),
                        )
                    })?,
                    name: GString::try_new(v.name).map_err(|err| {
                        MoneyError::ObjMoneyError(
                            format!("failed initializing currency name: {}", err).into(),
                        )
                    })?,
                    minor_unit: v.minor_unit,
                },
            ))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    // if another thread won the race, ignore — fall through to get()
    let _ = CURRENCIES.set(map);

    CURRENCIES.get().ok_or(MoneyError::ObjMoneyError(
        "failed getting the currencies".into(),
    ))
}

pub fn register_currency(
    code: &str,
    symbol: &str,
    minor_unit_symbol: &str,
    name: &str,
    minor_unit: u16,
) -> Result<(), MoneyError> {
    let existing = currencies()?;

    if existing.contains_key(code) {
        return Err(MoneyError::ObjMoneyError(
            format!("currency code {} is already existed", code).into(),
        ));
    }

    let code_key = GString::try_new(code).map_err(|err| {
        MoneyError::ObjMoneyError(format!("failed initializing currency code: {}", err).into())
    })?;
    let curr = ObjCurrency {
        code: code_key,
        symbol: GString::try_new(symbol).map_err(|err| {
            MoneyError::ObjMoneyError(
                format!("failed initializing currency symbol: {}", err).into(),
            )
        })?,
        minor_unit_symbol: GString::try_new(minor_unit_symbol).map_err(|err| {
            MoneyError::ObjMoneyError(
                format!("failed initializing currency minor unit symbol: {}", err).into(),
            )
        })?,
        name: GString::try_new(name).map_err(|err| {
            MoneyError::ObjMoneyError(format!("failed initializing currency name: {}", err).into())
        })?,
        minor_unit,
    };

    let mut updated_data = existing.clone();
    updated_data.insert(code_key, curr);

    let _ = CURRENCIES.set(updated_data);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct ObjMoney<const IS_RAW: bool = false> {
    amount: Decimal,
    currency: ObjCurrency,
}

#[derive(Clone, Copy, Debug)]
pub struct ObjCurrency {
    code: GString<(), 3, 4, true>,
    symbol: GString<(), 1, 16>,
    minor_unit_symbol: GString<(), 1, 16>,
    name: GString<(), 1, 50>,
    minor_unit: u16,
}

impl ObjCurrency {
    pub fn try_new(
        code: &str,
        symbol: &str,
        minor_unit_symbol: &str,
        name: &str,
        minor_unit: u16,
    ) -> Result<ObjCurrency, MoneyError> {
        Ok(ObjCurrency {
            code: GString::try_new(code).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!(
                        "failed constructing currency code {} with error: {}",
                        code, err
                    )
                    .into(),
                )
            })?,
            symbol: GString::try_new(symbol).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!(
                        "failed constructing currency symbol {} with error: {}",
                        symbol, err
                    )
                    .into(),
                )
            })?,
            minor_unit_symbol: GString::try_new(minor_unit_symbol).map_err(|err| {
                MoneyError::ObjMoneyError(
                    format!(
                        "failed constructing currency minor unit symbol {} with error: {}",
                        minor_unit_symbol, err
                    )
                    .into(),
                )
            })?,
            name: GString::try_new(name).map_err(|err| {
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
        if IS_RAW { amount.round_dp(dp) } else { amount }
    }

    #[inline]
    pub fn new(currency: ObjCurrency, amount: Decimal) -> Self {
        Self {
            amount: Self::round_amount(amount, currency.minor_unit.into()),
            currency,
        }
    }

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
        let obj_curr = currencies()?
            .get(&code_key)
            .ok_or(MoneyError::ObjMoneyError(
                format!("currency {} is not found", currency_code).into(),
            ))?;

        Ok(Self {
            amount: Self::round_amount(amount, obj_curr.minor_unit.into()),
            currency: *obj_curr,
        })
    }

    pub fn set_amount(mut self, new_amount: Decimal) -> Self {
        self.amount = new_amount;
        self
    }

    #[inline]
    pub fn amount(&self) -> Decimal {
        self.amount
    }

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

    #[inline]
    pub fn round(&self) -> Self {
        self.set_amount(self.amount().round_dp(self.minor_unit().into()));
        *self
    }

    #[inline]
    pub fn round_with(&self, decimal_points: u32, strategy: RoundingStrategy) -> Self {
        self.set_amount(
            self.amount()
                .round_dp_with_strategy(decimal_points, strategy.into()),
        );
        *self
    }

    #[inline]
    pub fn code(&self) -> &str {
        self.currency.code.as_str()
    }

    #[inline]
    pub fn symbol(&self) -> &str {
        self.currency.symbol.as_str()
    }

    #[inline]
    pub fn minor_unit_symbol(&self) -> &str {
        self.currency.minor_unit_symbol.as_str()
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.currency.name.as_str()
    }

    #[inline]
    pub fn minor_unit(&self) -> u16 {
        self.currency.minor_unit
    }
}

// Ops
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    #[inline]
    pub fn abs(&self) -> Self {
        self.set_amount(self.amount().abs())
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.amount().is_zero()
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        if self.is_zero() {
            return false;
        }
        self.amount().is_sign_positive()
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        if self.is_zero() {
            return false;
        }
        self.amount().is_sign_negative()
    }

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
        Ok(self.set_amount(
            self.amount()
                .checked_add(rhs.amount())
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

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
        Ok(self.set_amount(
            self.amount()
                .checked_sub(rhs.amount())
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    #[inline]
    pub fn checked_mul<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
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
        Ok(self.set_amount(
            self.amount()
                .checked_mul(rhs.amount())
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    #[inline]
    pub fn checked_div<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
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
        Ok(self.set_amount(
            self.amount()
                .checked_div(rhs.amount())
                .ok_or(MoneyError::OverflowError)?,
        ))
    }

    #[inline]
    pub fn checked_rem<RHS>(&self, rhs: RHS) -> Result<Self, MoneyError>
    where
        RHS: DecimalNumber,
    {
        Ok(self.set_amount(
            self.amount()
                .checked_rem(rhs.get_decimal().ok_or(MoneyError::OverflowError)?)
                .ok_or(MoneyError::OverflowError)?,
        ))
    }
}

// parsing
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
    pub fn from_str_code(
        money_str: &str,
        code: &str,
        thousand_separator: &str,
        decimal_separator: &str,
    ) -> Result<Self, MoneyError> {
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

impl std::ops::Neg for ObjMoney {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

// conversion
impl<const IS_RAW: bool> ObjMoney<IS_RAW> {
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
