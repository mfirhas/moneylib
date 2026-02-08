use std::{fmt::Display, str::FromStr};

use crate::{
    BaseMoney, Currency, Decimal, MoneyError, MoneyResult,
    base::{
        BaseOps, COMMA_SEPARATOR, COMMA_THOUSANDS_SEPARATOR_REGEX, CustomMoney, DOT_SEPARATOR,
        DOT_THOUSANDS_SEPARATOR_REGEX,
    },
};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Money {
    currency: Currency,
    amount: Decimal,
}

impl Money {
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        Money {
            currency,
            amount: amount.round_dp(currency.minor_unit() as u32),
        }
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
        let money_parts: Vec<&str> = s.split_whitespace().collect();
        if money_parts.len() != 2 {
            return Err(MoneyError::ParseStr);
        }

        let mut currency = money_parts[0]
            .parse::<Currency>()
            .map_err(|_| MoneyError::InvalidCurrency)?;

        let amount_str = if COMMA_THOUSANDS_SEPARATOR_REGEX.is_match(s) {
            currency.set_thousand_separator(COMMA_SEPARATOR);
            currency.set_decimal_separator(DOT_SEPARATOR);

            let comma = ',';
            // remove commas
            let amount_str: String = money_parts[1].chars().filter(|&c| c != comma).collect();
            amount_str
        } else if DOT_THOUSANDS_SEPARATOR_REGEX.is_match(s) {
            currency.set_thousand_separator(DOT_SEPARATOR);
            currency.set_decimal_separator(COMMA_SEPARATOR);

            let dot = '.';
            // remove dots
            let amount_str: String = money_parts[1].chars().filter(|&c| c != dot).collect();
            // convert comma to dot
            let amount_str: String = amount_str.replace(',', ".");
            amount_str
        } else {
            return Err(MoneyError::ParseStr);
        };

        let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?;
        let amount = amount.round_dp(currency.minor_unit() as u32);

        Ok(Self { currency, amount })
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl BaseMoney for Money {
    /// Get currency of money
    fn currency(&self) -> Currency {
        self.currency
    }

    /// Get amount of money
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Round money using Banker's Rounding rule to the scale of currency's minor unit
    fn round(self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.round_dp(self.currency().minor_unit() as u32),
        }
    }
}

impl BaseOps for Money {
    fn abs(&self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.abs(),
        }
    }

    fn min(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.min(rhs.amount),
        }
    }

    fn max(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.max(rhs.amount),
        }
    }

    /// clamp the money amount between `from` and `to` inclusively.
    fn clamp(&self, from: Decimal, to: Decimal) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.clamp(from, to),
        }
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
