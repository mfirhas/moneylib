use crate::Decimal;
use currencylib::data::Data;

#[derive(Debug, Clone, Copy, Eq)]
pub struct DynMoney {
    amount: Decimal,
    currency: Data,
}

impl PartialEq for DynMoney {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency.code == other.currency.code
    }
}

impl PartialOrd for DynMoney {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency.code == other.currency.code {
            return Some(self.amount.cmp(&other.amount));
        }
        None
    }
}
