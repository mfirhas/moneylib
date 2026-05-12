//--------- Ops for RawMoney (without automatic rounding)

use super::RawMoney;
use crate::impl_money_ops;

impl_money_ops!(RawMoney);
