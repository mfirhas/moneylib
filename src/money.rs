use std::{fmt::Display, str::FromStr};

use crate::money_macros::dec;
use rust_decimal::{MathematicalOps, prelude::FromPrimitive};

use crate::{
    BaseMoney, Currency, Decimal, MoneyError, MoneyResult,
    base::{BaseOps, COMMA_SEPARATOR, CustomMoney, DOT_SEPARATOR},
    parse::{parse_comma_thousands_separator, parse_dot_thousands_separator},
};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Money {
    currency: Currency,
    amount: Decimal,
}

impl Money {
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        Money { currency, amount }.round()
    }

    pub fn from_amount<T>(currency: Currency, amount: T) -> MoneyResult<Self>
    where
        T: Sized + Into<MoneyAmount>,
    {
        let money_amount: MoneyAmount = amount.into();

        let money: MoneyResult<Money> = money_amount.try_into();

        let decimal: MoneyResult<Decimal> = money_amount.try_into();

        let minor_amount: MoneyResult<i128> = money_amount.try_into();

        match (money, decimal, minor_amount) {
            (Ok(val), _, _) => {
                if val.currency() != currency {
                    return Err(MoneyError::NewMoney("creating Money from MoneyAmount with Money but the Money has different currency than `currency`".into()));
                }
                Ok(val)
            }
            (_, Ok(val), _) => Ok(Self::new(currency, val)),
            (_, _, Ok(val)) => {
                if let Some(amount) = Decimal::from_i128(val) {
                    return Ok(Self::new(currency, amount));
                }
                Err(MoneyError::NewMoney(
                    "failed creating money from i128, converting to decimal".into(),
                ))
            }
            (money_ret, dec_ret, int_ret) => {
                let err_msg = if let Err(err) = money_ret {
                    err.to_string()
                } else if let Err(err) = dec_ret {
                    err.to_string()
                } else if let Err(err) = int_ret {
                    err.to_string()
                } else {
                    "failed creating money from amount".to_string()
                };
                Err(MoneyError::NewMoney(err_msg))
            }
        }
    }

    pub fn from_minor_amount(currency: Currency, minor_amount: i128) -> MoneyResult<Self> {
        let dec = Decimal::from_i128(minor_amount).ok_or(MoneyError::NewMoney(
            "failed converting i128 to decimal".into(),
        ))?;

        let amount = dec
            .checked_div(dec!(10).powu(currency.minor_unit() as u64))
            .ok_or(MoneyError::NewMoney(
                "failed converting minor amount into amount".into(),
            ))?;

        Ok(Self::new(currency, amount))
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency == other.currency {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Try parsing with comma thousands separator first
        if let Some((currency_code, amount_str)) = parse_comma_thousands_separator(s) {
            let mut currency = currency_code
                .parse::<Currency>()
                .map_err(|_| MoneyError::InvalidCurrency)?;

            currency.set_thousand_separator(COMMA_SEPARATOR);
            currency.set_decimal_separator(DOT_SEPARATOR);

            let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?;

            return Ok(Self::new(currency, amount));
        }

        // Try parsing with dot thousands separator
        if let Some((currency_code, amount_str)) = parse_dot_thousands_separator(s) {
            let mut currency = currency_code
                .parse::<Currency>()
                .map_err(|_| MoneyError::InvalidCurrency)?;

            currency.set_thousand_separator(DOT_SEPARATOR);
            currency.set_decimal_separator(COMMA_SEPARATOR);

            let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?;

            return Ok(Self::new(currency, amount));
        }

        // Neither format matched
        Err(MoneyError::ParseStr)
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl From<Money> for Decimal {
    /// Get the amount of money
    fn from(value: Money) -> Self {
        value.amount()
    }
}

/// Types accepted as amount of money
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MoneyAmount {
    Money(Money),
    Decimal(Decimal),
    Float64(f64),
    Integer64(i64),
    Integer128(i128),
}

//// --- MoneyAmount

impl From<Money> for MoneyAmount {
    fn from(value: Money) -> Self {
        Self::Money(value)
    }
}

impl From<Decimal> for MoneyAmount {
    fn from(value: Decimal) -> Self {
        Self::Decimal(value)
    }
}

impl From<f64> for MoneyAmount {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl From<i64> for MoneyAmount {
    fn from(value: i64) -> Self {
        Self::Integer64(value)
    }
}

impl From<i128> for MoneyAmount {
    fn from(value: i128) -> Self {
        Self::Integer128(value)
    }
}

impl TryFrom<MoneyAmount> for Decimal {
    type Error = MoneyError;

    fn try_from(value: MoneyAmount) -> Result<Self, Self::Error> {
        match value {
            MoneyAmount::Money(val) => Ok(val.amount()),
            MoneyAmount::Decimal(val) => Ok(val),
            MoneyAmount::Float64(val) => Decimal::from_f64(val).ok_or(MoneyError::MoneyAmount(
                "failed converting f64 to decimal".into(),
            )),
            MoneyAmount::Integer64(val) => Decimal::from_i64(val).ok_or(MoneyError::MoneyAmount(
                "failed converting i64 to decimal".into(),
            )),
            MoneyAmount::Integer128(val) => Decimal::from_i128(val).ok_or(MoneyError::MoneyAmount(
                "failed converting i128 to decimal".into(),
            )),
        }
    }
}

impl TryFrom<MoneyAmount> for Money {
    type Error = MoneyError;

    fn try_from(value: MoneyAmount) -> Result<Self, Self::Error> {
        if let MoneyAmount::Money(val) = value {
            return Ok(val);
        }
        Err(MoneyError::MoneyAmount(
            "MoneyAmount must be in form of Money".into(),
        ))
    }
}

impl TryFrom<MoneyAmount> for i128 {
    type Error = MoneyError;

    fn try_from(value: MoneyAmount) -> Result<Self, Self::Error> {
        if let MoneyAmount::Integer128(val) = value {
            return Ok(val);
        }
        Err(MoneyError::MoneyAmount(
            "MoneyAmount must be in form of i128".into(),
        ))
    }
}

//// MoneyAmount ---

impl BaseMoney for Money {
    /// Get currency of money
    fn currency(&self) -> Currency {
        self.currency
    }

    /// Get amount of money
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Round money using `Currency`'s rounding strategy to the scale of currency's minor unit
    fn round(self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.round_dp_with_strategy(
                self.currency().minor_unit() as u32,
                self.currency.rounding_strategy().into(),
            ),
        }
    }
}

impl BaseOps for Money {
    fn abs(&self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.abs(),
        }
        .round()
    }

    fn min(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.min(rhs.amount),
        }
        .round()
    }

    fn max(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.max(rhs.amount),
        }
        .round()
    }

    /// clamp the money amount between `from` and `to` inclusively.
    fn clamp(&self, from: Decimal, to: Decimal) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.clamp(from, to),
        }
        .round()
    }

    fn add(&self, rhs: Decimal) -> MoneyResult<Self> {
        Ok(Self {
            currency: self.currency,
            amount: self
                .amount
                .checked_add(rhs)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        }
        .round())
    }

    fn sub(&self, rhs: Decimal) -> MoneyResult<Self> {
        Ok(Self {
            currency: self.currency,
            amount: self
                .amount
                .checked_sub(rhs)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        }
        .round())
    }

    fn mul(&self, rhs: Decimal) -> MoneyResult<Self> {
        Ok(Self {
            currency: self.currency,
            amount: self
                .amount
                .checked_mul(rhs)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        }
        .round())
    }

    fn div(&self, rhs: Decimal) -> MoneyResult<Self> {
        Ok(Self {
            currency: self.currency,
            amount: self
                .amount
                .checked_div(rhs)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        }
        .round())
    }
}

impl CustomMoney for Money {
    fn set_thousand_separator(&mut self, separator: &'static str) {
        self.currency.set_thousand_separator(separator);
    }

    fn set_decimal_separator(&mut self, separator: &'static str) {
        self.currency.set_decimal_separator(separator);
    }

    fn round_with(self, decimal_points: u32, strategy: crate::base::RoundingStrategy) -> Self {
        Self {
            currency: self.currency,
            amount: self
                .amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
        }
    }
}
