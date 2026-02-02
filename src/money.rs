use std::str::FromStr;

use crate::{BaseMoney, Currency, MoneyError, base::COMMA_SEPARATOR};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Money {
    currency: Currency,
    symbol: String,
    amount: Decimal,
    minor_unit: u16,
}

impl Money {
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        Money {
            currency,
            symbol: currency.symbol().symbol,
            amount,
            minor_unit: currency.exponent().unwrap_or_default(),
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency
            && self.amount == other.amount
            && self.symbol == other.symbol
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency == other.currency && self.symbol == other.symbol {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let money_parts: Vec<&str> = s.split_whitespace().collect();
        if money_parts.len() != 2 {
            return Err(MoneyError::ParseStr);
        }

        // 3. parse currency code
        let currency = money_parts[0]
            .parse::<Currency>()
            .map_err(|_| MoneyError::InvalidCurrency)?;

        let symbol = currency.symbol().symbol;

        let minor_unit = currency.exponent().unwrap_or_default();

        let amount_str = if <Money as BaseMoney>::thousand_separator() == COMMA_SEPARATOR {
            let comma = ',';
            // remove commas
            let amount_str: String = money_parts[1].chars().filter(|&c| c != comma).collect();
            amount_str
        } else {
            let dot = '.';
            // remove dots
            let amount_str: String = money_parts[1].chars().filter(|&c| c != dot).collect();
            // convert comma to dot
            let amount_str: String = amount_str.replace(',', ".");
            amount_str
        };

        let amount = Decimal::from_str(&amount_str).map_err(|_| MoneyError::InvalidAmount)?;

        Ok(Self {
            currency,
            symbol,
            minor_unit,
            amount,
        })
    }
}

impl BaseMoney for Money {
    /// Get currency of money
    fn currency(&self) -> Currency {
        self.currency
    }

    /// Get currency name
    fn name(&self) -> &str {
        self.currency.name()
    }

    /// Get money symbol
    fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Get amount of money
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Round money using Banker's Rounding rule to the scale of currency's minor unit
    fn round(self) -> Self {
        Self {
            currency: self.currency,
            symbol: self.symbol,
            minor_unit: self.minor_unit,
            amount: self.amount.round_dp(self.minor_unit as u32),
        }
    }
}
