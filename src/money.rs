use std::{fmt::Display, str::FromStr};

use crate::{base::MoneyAmount, money_macros::dec};
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
    #[inline]
    pub fn new(currency: Currency, amount: Decimal) -> Self {
        Money { currency, amount }.round()
    }

    pub fn from_amount<T>(currency: Currency, amount: T) -> MoneyResult<Self>
    where
        T: MoneyAmount<Money>,
    {
        match (amount.get_money(), amount.get_decimal()) {
            (Some(money), _) if money.currency() == currency => Ok(money),
            (None, Some(amount)) => Ok(Self::new(currency, amount)),
            _ => Err(MoneyError::NewMoney(
                "amount type is invalid or or money's currency mismatches".into(),
            )),
        }
    }

    pub fn from_minor_amount(currency: Currency, minor_amount: i128) -> MoneyResult<Self> {
        let dec = Decimal::from_i128(minor_amount).ok_or(MoneyError::NewMoney(
            "failed converting i128 to decimal".into(),
        ))?;

        let amount = dec
            .checked_div(
                dec!(10)
                    .checked_powu(currency.minor_unit().into())
                    .ok_or(MoneyError::ArithmeticOverflow)?,
            )
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
        // WARN: PANIC!
        assert_eq!(
            self.currency, other.currency,
            "cannot compare 2 money with different currencies"
        );
        self.amount.partial_cmp(&other.amount)
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

// --- MoneyAmount

impl MoneyAmount<Money> for Money {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        Some(*self)
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(self.amount())
    }
}

impl MoneyAmount<Money> for Decimal {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Some(*self)
    }
}

impl MoneyAmount<Money> for f64 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_f64(*self)
    }
}

impl MoneyAmount<Money> for i32 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i32(*self)
    }
}

impl MoneyAmount<Money> for i64 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i64(*self)
    }
}

impl MoneyAmount<Money> for i128 {
    #[inline]
    fn get_money(&self) -> Option<Money> {
        None
    }

    #[inline]
    fn get_decimal(&self) -> Option<Decimal> {
        Decimal::from_i128(*self)
    }
}

// MoneyAmount ---

impl BaseMoney for Money {
    /// Get currency of money
    #[inline]
    fn currency(&self) -> Currency {
        self.currency
    }

    /// Get amount of money
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount
    }

    /// Round money using `Currency`'s rounding strategy to the scale of currency's minor unit
    #[inline]
    fn round(self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.round_dp_with_strategy(
                self.currency().minor_unit().into(),
                self.currency.rounding_strategy().into(),
            ),
        }
    }
}

impl BaseOps for Money {
    #[inline]
    fn abs(&self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.abs(),
        }
        .round()
    }

    #[inline]
    fn min(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.min(rhs.amount),
        }
        .round()
    }

    #[inline]
    fn max(&self, rhs: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.max(rhs.amount),
        }
        .round()
    }

    /// clamp the money amount between `from` and `to` inclusively.
    #[inline]
    fn clamp(&self, from: Decimal, to: Decimal) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount.clamp(from, to),
        }
        .round()
    }

    fn add<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_add(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn sub<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_sub(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn mul<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_mul(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }

    fn div<RHS, T>(&self, rhs: RHS) -> MoneyResult<Self>
    where
        RHS: MoneyAmount<T>,
        T: BaseMoney,
    {
        let amount = match (rhs.get_money(), rhs.get_decimal()) {
            (Some(money), _) if money.currency() == self.currency() => money.amount(),
            (None, Some(amount)) => amount,
            _ => return Err(MoneyError::CurrencyMismatch),
        };
        Ok(Self::new(
            self.currency,
            self.amount
                .checked_div(amount)
                .ok_or(MoneyError::ArithmeticOverflow)?,
        ))
    }
}

impl CustomMoney for Money {
    #[inline]
    fn set_thousand_separator(&mut self, separator: &'static str) {
        self.currency.set_thousand_separator(separator);
    }

    #[inline]
    fn set_decimal_separator(&mut self, separator: &'static str) {
        self.currency.set_decimal_separator(separator);
    }

    #[inline]
    fn round_with(self, decimal_points: u32, strategy: crate::base::RoundingStrategy) -> Self {
        Self {
            currency: self.currency,
            amount: self
                .amount
                .round_dp_with_strategy(decimal_points, strategy.into()),
        }
    }
}
