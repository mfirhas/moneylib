use crate::{BaseMoney, Money};
use super::RawMoney;

/// Extension trait for Money to support conversion to RawMoney
impl Money {
    /// Converts this `Money` to `RawMoney`, preserving the current amount.
    ///
    /// The resulting `RawMoney` will not automatically round during operations,
    /// giving full control over when rounding occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, Currency, money_macros::dec, BaseMoney};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let money = Money::new(usd, dec!(100.50));
    /// 
    /// // Convert to RawMoney
    /// let raw = money.into_raw();
    /// assert_eq!(raw.amount(), dec!(100.50));
    /// 
    /// // Perform calculations without auto-rounding
    /// let result = raw * dec!(1.0 / 3.0);
    /// 
    /// // Convert back when ready to round
    /// let final_money = result.finish();
    /// ```
    #[inline]
    pub fn into_raw(self) -> RawMoney {
        RawMoney::new(self.currency(), self.amount())
    }
}
