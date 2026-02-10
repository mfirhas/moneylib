use std::{fmt::Display, str::FromStr};

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
