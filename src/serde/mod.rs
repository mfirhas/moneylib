/// Shared serde building blocks for `Money<C>` and `RawMoney<C>`.
pub mod base;

/// `Money<C>` serde implementations
pub mod money;

/// `RawMoney<C>` serde implementations
#[cfg(feature = "raw_money")]
pub mod raw_money;

/// `ObjMoney` serde implementations
#[cfg(feature = "obj_money")]
pub mod obj_money;

#[cfg(test)]
mod money_test;

#[cfg(all(test, feature = "raw_money"))]
mod raw_money_test;

#[cfg(all(test, feature = "obj_money"))]
mod obj_money_test;
