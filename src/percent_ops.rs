//! percent_ops contains trait for percentage operations.
//!
//! It has blanket implementation for types implementing BaseMoney.

use crate::{
    BaseMoney, BaseOps, Currency,
    base::{Amount, DecimalNumber},
    macros::dec,
};

pub trait PercentOps<C: Currency> {
    type Output;

    /// Calculates what a certain percentage of a money amount equals.
    ///
    /// `pcn` is the percentage, 20% -> pcn = 20.
    fn percent<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber;

    /// Adds amount by percentage
    ///
    /// `pcn` is the percentage, 20% -> pcn = 20.
    fn percent_add<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber;

    /// Adds self by multiple percentages from original amount.
    ///
    /// Each items in `pcns` are percentage, 20% -> 20.
    ///
    /// Order of `pcns` does **NOT** matter.
    fn percent_adds_fixed<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber;

    /// Adds self by multiple percentages compounding.
    ///
    /// Each items in `pcns` are percentage, 20% -> 20.
    ///
    /// Order of `pcns` **DOES** matter.
    fn percent_adds_compound<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber;

    /// Substracts amount by percentage(discount)
    ///
    /// `pcn` is the percentage, 20% -> pcn = 20.
    fn percent_sub<D>(&self, pcn: D) -> Option<Self::Output>
    where
        D: DecimalNumber;

    /// Substracts self by multiple percentages in sequence.
    ///
    /// Each items in `pcns` are percentage, 20% -> 20.
    ///
    /// Order of `pcns` **DOES** matter.
    fn percent_subs_sequence<D, I>(&self, pcns: I) -> Option<Self::Output>
    where
        for<'a> &'a I: IntoIterator<Item = &'a D>,
        D: DecimalNumber;

    /// Determines what percentage one money is of another.
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
