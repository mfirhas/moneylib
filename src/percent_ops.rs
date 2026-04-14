//! percent_ops contains trait for percentage operations.
//!
//! It has blanket implementation for types implementing BaseMoney.

use crate::{
    BaseMoney, BaseOps, Currency,
    base::{Amount, DecimalNumber},
    macros::dec,
};

/// Trait for percentage operations.
///
/// It has blanket implementation for types implementing BaseMoney.
pub trait PercentOps<C: Currency> {
    type Output;

    /// Calculates what a certain percentage of a money amount equals.
    ///
    /// `pcn` is the percentage, 20% -> pcn = 20.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let price = money!(USD, 200);
    /// let tax = price.percent(15).unwrap(); // 15% of $200
    /// assert_eq!(tax.amount(), dec!(30));
    ///
    /// // Returns None on overflow
    /// let none_on_overflow = price.percent(moneylib::Decimal::MAX);
    /// assert!(none_on_overflow.is_none());
    /// ```
    fn percent<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber;

    /// Adds amount by percentage
    ///
    /// `pcn` is the percentage, 20% -> pcn = 20.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let price = money!(USD, 100);
    /// let after_tax = price.percent_add(20).unwrap(); // $100 + 20% = $120
    /// assert_eq!(after_tax.amount(), dec!(120));
    ///
    /// // Returns None on overflow
    /// let none_on_overflow = price.percent_add(moneylib::Decimal::MAX);
    /// assert!(none_on_overflow.is_none());
    /// ```
    fn percent_add<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber;

    /// Adds self by multiple percentages from original amount.
    ///
    /// Each items in `pcns` are percentage, 20% -> 20.
    ///
    /// Order of `pcns` does **NOT** matter.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let base = money!(USD, 1_000);
    /// // All percentages are applied to the original base amount:
    /// // $1000 + 10% of $1000 + 5% of $1000 = $1000 + $100 + $50 = $1150
    /// let total = base.percent_adds_fixed([10, 5]).unwrap();
    /// assert_eq!(total.amount(), dec!(1150));
    ///
    /// // Returns None on overflow
    /// let none_on_overflow = base.percent_adds_fixed([moneylib::Decimal::MAX]);
    /// assert!(none_on_overflow.is_none());
    /// ```
    fn percent_adds_fixed<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber;

    /// Adds self by multiple percentages compounding.
    ///
    /// Each items in `pcns` are percentage, 20% -> 20.
    ///
    /// Order of `pcns` **DOES** matter.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let base = money!(USD, 1_000);
    /// // Percentages compound on the running total:
    /// // Step 1: $1000 + 10% of $1000  = $1100
    /// // Step 2: $1100 + 5%  of $1100  = $1155
    /// let total = base.percent_adds_compound([10, 5]).unwrap();
    /// assert_eq!(total.amount(), dec!(1155));
    ///
    /// // Returns None on overflow
    /// let none_on_overflow = base.percent_adds_compound([moneylib::Decimal::MAX]);
    /// assert!(none_on_overflow.is_none());
    /// ```
    fn percent_adds_compound<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber;

    /// Substracts amount by percentage(discount)
    ///
    /// `pcn` is the percentage, 20% -> pcn = 20.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let price = money!(USD, 200);
    /// let after_discount = price.percent_sub(25).unwrap(); // $200 - 25% = $150
    /// assert_eq!(after_discount.amount(), dec!(150));
    ///
    /// // Returns None on overflow
    /// let none_on_overflow = price.percent_sub(moneylib::Decimal::MAX);
    /// assert!(none_on_overflow.is_none());
    /// ```
    fn percent_sub<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber;

    /// Substracts self by multiple percentages in sequence.
    ///
    /// Each items in `pcns` are percentage, 20% -> 20.
    ///
    /// Order of `pcns` **DOES** matter.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let gross = money!(USD, 1_000);
    /// // Deductions compound on the running total:
    /// // Step 1: $1000 - 10% of $1000 = $900
    /// // Step 2: $900  - 5%  of $900  = $855
    /// let net = gross.percent_subs_sequence([10, 5]).unwrap();
    /// assert_eq!(net.amount(), dec!(855));
    ///
    /// // Returns None on overflow
    /// let none_on_overflow = gross.percent_subs_sequence([moneylib::Decimal::MAX]);
    /// assert!(none_on_overflow.is_none());
    /// ```
    fn percent_subs_sequence<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber;

    /// Determines what percentage one money is of another.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{BaseMoney, PercentOps, macros::{dec, money}};
    ///
    /// let profit = money!(USD, 50);
    /// let revenue = money!(USD, 200);
    /// let margin = profit.percent_of(revenue).unwrap(); // $50 is 25% of $200
    /// assert_eq!(margin.amount(), dec!(25));
    ///
    /// // Returns None when dividing by zero
    /// let zero = money!(USD, 0);
    /// assert!(profit.percent_of(zero).is_none());
    /// ```
    fn percent_of<M>(&self, rhs: M) -> Option<Self::Output>
    where
        M: Amount<C>;
}

impl<M, C> PercentOps<C> for M
where
    M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    C: Currency,
{
    type Output = M;

    fn percent<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber,
    {
        self.checked_mul(pcn.get_decimal()?)?.checked_div(dec!(100))
    }

    fn percent_add<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber,
    {
        self.percent(pcn)?.checked_add(self.to_owned())
    }

    fn percent_adds_fixed<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber,
    {
        let mut result = self.amount();
        for pcn in pcns.into_iter() {
            result = result.checked_add(self.percent(pcn.get_decimal()?)?.amount())?;
        }
        Self::Output::new(result).ok()
    }

    fn percent_adds_compound<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber,
    {
        let mut result = self.clone();
        let mut current = self.clone();
        for pcn in pcns.into_iter() {
            result = result.checked_add(current.percent(pcn.get_decimal()?)?)?;
            current = result.clone();
        }
        Some(result)
    }

    fn percent_sub<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber,
    {
        self.checked_sub(self.percent(pcn)?)
    }

    fn percent_subs_sequence<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber,
    {
        let mut result = self.clone();
        let mut current = self.clone();
        for pcn in pcns.into_iter() {
            result = result.checked_sub(current.percent(pcn.get_decimal()?)?)?;
            current = result.clone();
        }
        Some(result)
    }

    fn percent_of<D>(&self, rhs: D) -> Option<Self::Output>
    where
        D: Amount<C>,
    {
        (self.checked_div(rhs.get_decimal()?)?).checked_mul(dec!(100))
    }
}
