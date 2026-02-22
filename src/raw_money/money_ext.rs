use crate::{BaseMoney, Currency, Money};

use super::RawMoney;

impl<C> Money<C>
where
    C: Currency + Clone,
{
    /// Converts this `Money` into `RawMoney`, preserving the current (rounded) amount.
    ///
    /// The resulting `RawMoney` will not automatically round during operations,
    /// giving full control over when rounding occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, BaseMoney, money_macros::dec, USD};
    ///
    /// let money = Money::<USD>::new(dec!(100.50)).unwrap();
    ///
    /// // Convert to RawMoney
    /// let raw = money.into_raw();
    /// assert_eq!(raw.amount(), dec!(100.50));
    ///
    /// // Perform calculations without auto-rounding
    /// let result = raw * (dec!(1) / dec!(3));
    ///
    /// // Convert back when ready to round
    /// let final_money = result.finish();
    /// assert_eq!(final_money.amount(), dec!(33.50));
    /// ```
    #[inline]
    pub fn into_raw(self) -> RawMoney<C> {
        RawMoney::from_decimal(self.amount())
    }
}
