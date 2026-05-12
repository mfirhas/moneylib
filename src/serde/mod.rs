/// Shared serde building blocks for `Money<C>` and `RawMoney<C>`.
pub mod base;

/// `Money<C>` serde implementations
pub mod money;

#[cfg(feature = "raw_money")]
/// `RawMoney<C>` serde implementations
pub mod raw_money;

#[cfg(test)]
mod money_test;

#[cfg(all(test, feature = "raw_money"))]
mod raw_money_test;
