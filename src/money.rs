use std::{fmt::Display, str::FromStr};

use crate::{
    BaseMoney, Currency, Decimal, MoneyError,
    base::{COMMA_SEPARATOR, COMMA_THOUSANDS_SEPARATOR_REGEX, DOT_THOUSANDS_SEPARATOR_REGEX},
};

#[derive(Debug, Clone, Copy)]
pub struct Money {
    currency: Currency,
    amount: Decimal,
}

impl Money {
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        Money { currency, amount }
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

        let currency = money_parts[0]
            .parse::<Currency>()
            .map_err(|_| MoneyError::InvalidCurrency)?;

        let amount_str = if currency.thousand_separator == COMMA_SEPARATOR
            && COMMA_THOUSANDS_SEPARATOR_REGEX.is_match(money_parts[1])
        {
            let comma = ',';
            // remove commas
            let amount_str: String = money_parts[1].chars().filter(|&c| c != comma).collect();
            amount_str
        } else {
            if !DOT_THOUSANDS_SEPARATOR_REGEX.is_match(money_parts[1]) {
                return Err(MoneyError::ParseStr);
            }
            let dot = '.';
            // remove dots
            let amount_str: String = money_parts[1].chars().filter(|&c| c != dot).collect();
            // convert comma to dot
            let amount_str: String = amount_str.replace(',', ".");
            amount_str
        };

        let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::ParseStr)?;
        let amount = amount.round_dp(currency.minor_unit as u32);

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
            amount: self.amount.round_dp(self.currency().minor_unit as u32),
        }
    }
}
