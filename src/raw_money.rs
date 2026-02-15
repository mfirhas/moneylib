//! RawMoney is money type that contains raw value of money amount without rounding to keep the precision, and choose when to round it into tender money.
//!

use crate::{Currency, Decimal};

#[derive(Debug, Clone, Copy, Eq)]
pub struct RawMoney {
    currency: Currency,
    amount: Decimal,
}

impl RawMoney {
    #[inline]
    pub const fn new(currency: Currency, amount: Decimal) -> Self {
        Self { currency, amount }
    }
}

impl PartialEq for RawMoney {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}
