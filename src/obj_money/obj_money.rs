use super::dyn_money::DynMoney;
use crate::exchange::ObjRate;
use crate::{BaseMoney, Currency, Decimal, MoneyError, base::DecimalNumber};
use currencylib::data;
use g_string::GString;
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
    name: GString<(), 1, 50>,
    minor_unit: u16,
}

impl ObjCurrency {
    pub fn try_new(
        code: &str,
        symbol: &str,
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
    pub fn code(&self) -> &str {
        self.currency.code.as_str()
    }

    #[inline]
    pub fn symbol(&self) -> &str {
        self.currency.symbol.as_str()
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
