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
    C: Currency + Clone,
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

#[cfg(test)]
mod percent_ops_tests {
    use crate::{
        BaseMoney,
        macros::{dec, money, raw},
        percent_ops::PercentOps,
    };

    #[test]
    fn test_percent_ops() {
        let money = money!(USD, 100);
        let tips = money.percent(15).unwrap();
        assert_eq!(tips.amount(), dec!(15));

        let money = raw!(USD, 100.3434);
        let tips = money.percent(15).unwrap();
        assert_eq!(tips.amount(), dec!(15.051510));

        let money = money!(USD, 100);
        let tips = money.percent(i128::MAX);
        assert!(tips.is_none());

        let money = money!(USD, 100);
        let tips = money.percent(crate::Decimal::MAX);
        assert!(tips.is_none());

        let money = money!(USD, 100);
        let after_tax = money.percent_add(50).unwrap();
        assert_eq!(after_tax.amount(), dec!(150));

        let money = raw!(USD, 100.3434);
        let after_tax = money.percent_add(40).unwrap();
        assert_eq!(after_tax.amount(), dec!(140.480760));

        let money = money!(USD, 100);
        let after_discount = money.percent_sub(50).unwrap();
        assert_eq!(after_discount.amount(), dec!(50));

        let money = raw!(USD, 100.3434);
        let after_discount = money.percent_sub(40).unwrap();
        assert_eq!(after_discount.amount(), dec!(60.206040));

        let profit = money!(USD, 200);
        let revenue = money!(USD, 300);
        let margin = profit.percent_of(revenue).unwrap();
        assert_eq!(margin.amount(), dec!(67));

        let profit = raw!(USD, 200.4004);
        let revenue = raw!(USD, 300.123123);
        let margin = profit.percent_of(revenue).unwrap();
        assert_eq!(margin.amount(), dec!(66.772729137567984056996501400));

        let profit = money!(USD, 200);
        let margin = profit.percent_of(i128::MAX);
        assert!(margin.is_none());

        let profit = money!(USD, 200);
        let margin = profit.percent_of(0);
        assert!(margin.is_none());

        // fixed additions
        let base = money!(USD, 1_000_000);
        let insurance_percent = 5;
        let transportation_percent = 10;
        let royalty = 5;
        let ret = base
            .percent_adds_fixed([insurance_percent, transportation_percent, royalty])
            .unwrap();
        assert_eq!(ret.amount(), dec!(1_200_000));

        let base = money!(USD, 1_000_000);
        let insurance_percent = 5;
        let transportation_percent = 10;
        let royalty = i128::MAX;
        let ret = base.percent_adds_fixed([insurance_percent, transportation_percent, royalty]);
        assert!(ret.is_none());

        // compounding additions
        let base = money!(USD, 1_000_000);
        let insurance_percent = 5;
        let transportation_percent = 10;
        let royalty = 5;
        let ret = base
            .percent_adds_compound([insurance_percent, transportation_percent, royalty])
            .unwrap();
        assert_eq!(ret.amount(), dec!(1_212_750));

        let base = money!(USD, 1_000_000);
        let insurance_percent = 5;
        let transportation_percent = 10;
        let royalty = i128::MAX;
        let ret = base.percent_adds_compound([insurance_percent, transportation_percent, royalty]);
        assert!(ret.is_none());

        // sequence reduction
        let gross = money!(IDR, 50_000_000);
        let pph21 = 20;
        let bpjs = 10;
        let tapera = 5;
        let ret = gross.percent_subs_sequence([pph21, bpjs, tapera]).unwrap();
        assert_eq!(ret.amount(), dec!(34_200_000));

        let gross = money!(IDR, 50_000_000);
        let pph21 = 20;
        let bpjs = 10;
        let tapera = i128::MAX;
        let ret = gross.percent_subs_sequence([pph21, bpjs, tapera]);
        assert!(ret.is_none());
    }
}
