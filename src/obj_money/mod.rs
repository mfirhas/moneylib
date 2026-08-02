mod dyn_money;
pub use dyn_money::DynMoney;

#[allow(clippy::module_inception)]
mod obj_money;
pub use obj_money::{ObjCurrency, ObjMoney, register_currency};

#[cfg(test)]
mod obj_money_test;
