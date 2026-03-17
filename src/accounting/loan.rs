use rust_decimal::MathematicalOps;

use crate::base::DecimalNumber;
use crate::{BaseMoney, BaseOps, Currency, base::Amount, macros::dec};

/// Trait defining loan and time-value-of-money calculations.
///
/// Provides three core financial formulas:
/// - [`LoanOps::loan_payment`]: periodic payment for a loan (PMT)
/// - [`LoanOps::present_value`]: current worth of a future amount (PV)
/// - [`LoanOps::future_value`]: projected worth of a present amount (FV)
///
/// All rate arguments are **decimal fractions**, not percentages
/// (e.g. pass `dec!(0.005)` for 0.5 %, not `dec!(0.5)`).
///
/// # Examples
///
/// ```
/// use moneylib::{Money, BaseMoney, macros::dec, iso::USD};
/// use moneylib::accounting::LoanOps;
///
/// // 30-year mortgage: $300,000 at 4% annual interest, monthly payments
/// let principal = Money::<USD>::new(dec!(300000)).unwrap();
/// let monthly_rate = dec!(0.04) / dec!(12);
/// let payment = principal.loan_payment(monthly_rate, 360).unwrap();
///
/// // Goal: $10,000 in 5 years at 6% – how much to invest today?
/// let future = Money::<USD>::new(dec!(10000)).unwrap();
/// let present = Money::<USD>::present_value(future, dec!(0.06), 5).unwrap();
///
/// // Invest $5,000 for 10 years at 8%
/// let investment = Money::<USD>::new(dec!(5000)).unwrap();
/// let at_maturity = investment.future_value(dec!(0.08), 10).unwrap();
/// ```
pub trait LoanOps<C: Currency>: Sized {
    /// Calculate the periodic payment amount for a loan (PMT).
    ///
    /// **Formula:** PMT = Principal × \[r(1+r)^n\] / \[(1+r)^n − 1\]
    ///
    /// # Arguments
    /// - `rate`: Period interest rate as a decimal fraction (e.g. `dec!(0.005)` for 0.5 %).
    /// - `periods`: Total number of payment periods.
    ///
    /// # Returns
    /// `None` if:
    /// - `rate` cannot be converted to `Decimal`
    /// - `rate` is zero (the denominator `(1+r)^n − 1` collapses to zero)
    /// - arithmetic overflow occurs
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, BaseMoney, macros::dec, iso::USD};
    /// use moneylib::accounting::LoanOps;
    ///
    /// // Auto loan: $25,000 at 6% annual, 5-year term (60 monthly payments)
    /// let price = Money::<USD>::new(dec!(25000)).unwrap();
    /// let monthly_rate = dec!(0.06) / dec!(12);
    /// let payment = price.loan_payment(monthly_rate, 60).unwrap();
    /// ```
    fn loan_payment<R>(&self, rate: R, periods: u32) -> Option<Self>
    where
        R: DecimalNumber;

    /// Calculate the present value of a future amount (PV).
    ///
    /// **Formula:** PV = FV / (1 + rate)^periods
    ///
    /// # Arguments
    /// - `future_value`: The future money amount.
    /// - `rate`: Period discount rate as a decimal fraction (e.g. `dec!(0.06)` for 6 %).
    /// - `periods`: Number of compounding periods.
    ///
    /// # Returns
    /// `None` if:
    /// - `rate` cannot be converted to `Decimal`
    /// - arithmetic overflow occurs
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, BaseMoney, macros::dec, iso::USD};
    /// use moneylib::accounting::LoanOps;
    ///
    /// // How much to invest today to have $10,000 in 5 years at 6%?
    /// let future = Money::<USD>::new(dec!(10000)).unwrap();
    /// let present = Money::<USD>::present_value(future, dec!(0.06), 5).unwrap();
    /// ```
    fn present_value<R>(future_value: Self, rate: R, periods: u32) -> Option<Self>
    where
        R: DecimalNumber;

    /// Calculate the future value of a present investment (FV).
    ///
    /// **Formula:** FV = PV × (1 + rate)^periods
    ///
    /// # Arguments
    /// - `rate`: Period growth rate as a decimal fraction (e.g. `dec!(0.08)` for 8 %).
    /// - `periods`: Number of compounding periods.
    ///
    /// # Returns
    /// `None` if:
    /// - `rate` cannot be converted to `Decimal`
    /// - arithmetic overflow occurs
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, BaseMoney, macros::dec, iso::USD};
    /// use moneylib::accounting::LoanOps;
    ///
    /// // $5,000 invested for 10 years at 8%
    /// let investment = Money::<USD>::new(dec!(5000)).unwrap();
    /// let at_maturity = investment.future_value(dec!(0.08), 10).unwrap();
    /// ```
    fn future_value<R>(&self, rate: R, periods: u32) -> Option<Self>
    where
        R: DecimalNumber;
}

impl<M, C> LoanOps<C> for M
where
    M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    C: Currency + Clone,
{
    fn loan_payment<R>(&self, rate: R, periods: u32) -> Option<Self>
    where
        R: DecimalNumber,
    {
        let rate = rate.get_decimal()?;
        let principal = self.amount();
        let n = u64::from(periods);

        // (1 + r)^n
        let one_plus_r = dec!(1).checked_add(rate)?;
        let one_plus_r_n = one_plus_r.checked_powu(n)?;

        // numerator: r × (1 + r)^n
        let numerator = rate.checked_mul(one_plus_r_n)?;

        // denominator: (1 + r)^n − 1
        let denominator = one_plus_r_n.checked_sub(dec!(1))?;

        // PMT = P × numerator / denominator
        let pmt = principal
            .checked_mul(numerator)?
            .checked_div(denominator)?;

        M::new(pmt).ok()
    }

    fn present_value<R>(future_value: Self, rate: R, periods: u32) -> Option<Self>
    where
        R: DecimalNumber,
    {
        let rate = rate.get_decimal()?;
        let fv = future_value.amount();
        let n = u64::from(periods);

        // (1 + rate)^periods
        let one_plus_r_n = dec!(1).checked_add(rate)?.checked_powu(n)?;

        // PV = FV / (1 + rate)^periods
        let pv = fv.checked_div(one_plus_r_n)?;

        M::new(pv).ok()
    }

    fn future_value<R>(&self, rate: R, periods: u32) -> Option<Self>
    where
        R: DecimalNumber,
    {
        let rate = rate.get_decimal()?;
        let pv = self.amount();
        let n = u64::from(periods);

        // (1 + rate)^periods
        let one_plus_r_n = dec!(1).checked_add(rate)?.checked_powu(n)?;

        // FV = PV × (1 + rate)^periods
        let fv = pv.checked_mul(one_plus_r_n)?;

        M::new(fv).ok()
    }
}
