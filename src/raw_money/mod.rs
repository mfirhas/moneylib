//! RawMoney module - provides monetary types without automatic rounding.
//!
//! This module contains the `RawMoney` type and related functionality for performing
//! precise monetary calculations without automatic rounding. Use this when you need
//! full control over when and how rounding occurs.

mod raw_money;
pub use raw_money::RawMoney;

mod dec_ops;
mod ops;

mod money_ext;

#[cfg(test)]
mod raw_money_test;
