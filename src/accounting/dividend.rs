use std::marker::PhantomData;

use crate::base::DecimalNumber;
use crate::{BaseMoney, BaseOps, Currency, Decimal, base::Amount, macros::dec};

/// Trait defining dividend calculation operations for common stocks.
///
/// Provides a builder-based API for computing dividend metrics
/// such as yield, income, payout ratio, and reinvestment growth.
pub trait DividendOps<C> {
    type DividendBuilder;

    /// Create a dividend calculator from a share price and dividend per share.
    ///
    /// # Arguments
    /// * `self` – The current share price per share.
    /// * `dividend_per_share` – The dividend amount paid per share per payment period.
    ///
    /// # Return
    /// Returns a builder to configure shares owned, payment frequency, years, etc.
    ///
    /// Defaults: 1 share, 1 period per year, 1 year, no reinvestment, no tax.
    fn dividend<R>(&self, dividend_per_share: R) -> Option<Self::DividendBuilder>
    where
        R: DecimalNumber;
}

impl<M, C> DividendOps<C> for M
where
    M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    C: Currency + Clone,
{
    type DividendBuilder = Dividend<M, C>;

    fn dividend<R>(&self, dividend_per_share: R) -> Option<Self::DividendBuilder>
    where
        R: DecimalNumber,
    {
        Some(Dividend {
            share_price: self.amount(),
            dividend_per_share: dividend_per_share.get_decimal()?,
            shares: dec!(1),
            periods_per_year: 1,
            years: 1,
            reinvest: false,
            tax: None,
            _output: PhantomData,
            _currency: PhantomData,
        })
    }
}

/// Builder for dividend calculations. Built through `self::DividendOps` trait.
///
/// Supports both simple (non-reinvested) and DRIP (Dividend Reinvestment Plan) calculations.
#[derive(Debug, Clone, Copy)]
pub struct Dividend<M, C> {
    /// Current share price
    share_price: Decimal,

    /// Dividend per share per payment period
    dividend_per_share: Decimal,

    /// Number of shares owned
    shares: Decimal,

    /// Number of dividend payment periods per year (e.g. 4 for quarterly, 2 for semi-annual, 1 for annual)
    periods_per_year: u32,

    /// Number of years
    years: u32,

    /// Whether dividends are reinvested (DRIP – Dividend Reinvestment Plan)
    reinvest: bool,

    /// Flat-rate tax on dividend income (percentage, e.g. 20 for 20%)
    tax: Option<Decimal>,

    _output: PhantomData<M>,
    _currency: PhantomData<C>,
}

impl<M, C> Dividend<M, C>
where
    M: BaseMoney<C> + BaseOps<C>,
    C: Currency,
{
    /// Set the number of shares owned.
    ///
    /// # Argument
    /// D: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    pub fn shares<D>(self, shares: D) -> Option<Self>
    where
        D: DecimalNumber,
    {
        Some(Self {
            shares: shares.get_decimal()?,
            ..self
        })
    }

    /// Set the number of dividend payment periods per year.
    ///
    /// Common values: 1 (annual), 2 (semi-annual), 4 (quarterly), 12 (monthly).
    pub const fn periods_per_year(self, periods_per_year: u32) -> Self {
        Self {
            periods_per_year,
            ..self
        }
    }

    /// Set the number of years to calculate over.
    pub const fn years(self, years: u32) -> Self {
        Self { years, ..self }
    }

    /// Enable dividend reinvestment (DRIP – Dividend Reinvestment Plan).
    ///
    /// When enabled, dividend income is used to purchase additional shares
    /// at the current share price each period, compounding the dividend income.
    pub const fn with_reinvest(self) -> Self {
        Self {
            reinvest: true,
            ..self
        }
    }

    /// Set a flat-rate tax percentage applied to dividend income.
    ///
    /// The tax is expressed as a percentage number, e.g. 20% → tax = 20.
    ///
    /// # Argument
    /// D: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    pub fn with_tax<D>(self, tax: D) -> Option<Self>
    where
        D: DecimalNumber,
    {
        Some(Self {
            tax: Some(tax.get_decimal()?),
            ..self
        })
    }

    /// Calculate the annual dividend yield as a percentage.
    ///
    /// # Formula
    /// `Yield = (DPS × periods_per_year) / share_price × 100`
    ///
    /// Returns `None` if share_price is zero or arithmetic overflows.
    pub fn dividend_yield(&self) -> Option<Decimal> {
        dividend_impl::get_dividend_yield(self)
    }

    /// Calculate the total dividend income over all years.
    ///
    /// Without reinvestment:
    /// `Income = DPS × shares × periods_per_year × years`
    ///
    /// With reinvestment (DRIP):
    /// Each period, dividend income buys additional shares at the current share price,
    /// which increases future dividend payments (similar to compound interest).
    ///
    /// If tax is set, it is applied to each period's dividend income before
    /// potential reinvestment.
    pub fn income(&self) -> Option<M> {
        dividend_impl::get_income(self)
    }

    /// Calculate the payout ratio as a percentage.
    ///
    /// # Formula
    /// `Payout Ratio = (DPS × periods_per_year) / EPS × 100`
    ///
    /// # Argument
    /// * `earnings_per_share` – The company's earnings per share (EPS) for the period.
    ///
    /// Returns `None` if EPS is zero or arithmetic overflows.
    pub fn payout_ratio<D>(&self, earnings_per_share: D) -> Option<Decimal>
    where
        D: DecimalNumber,
    {
        dividend_impl::get_payout_ratio(self, earnings_per_share.get_decimal()?)
    }

    /// Calculate the retention ratio as a percentage.
    ///
    /// # Formula
    /// `Retention Ratio = 100 − Payout Ratio`
    ///
    /// # Argument
    /// * `earnings_per_share` – The company's earnings per share (EPS) for the period.
    ///
    /// Returns `None` if EPS is zero or arithmetic overflows.
    pub fn retention_ratio<D>(&self, earnings_per_share: D) -> Option<Decimal>
    where
        D: DecimalNumber,
    {
        dividend_impl::get_retention_ratio(self, earnings_per_share.get_decimal()?)
    }

    /// Calculate dividend per share given total dividends and shares outstanding.
    ///
    /// # Formula
    /// `DPS = total_dividends / shares_outstanding`
    ///
    /// This is a standalone calculation that does not use the builder's
    /// dividend_per_share value.
    ///
    /// Returns `None` if shares_outstanding is zero or arithmetic overflows.
    pub fn dividend_per_share<D>(total_dividends: D, shares_outstanding: D) -> Option<Decimal>
    where
        D: DecimalNumber,
    {
        dividend_impl::get_dividend_per_share(
            total_dividends.get_decimal()?,
            shares_outstanding.get_decimal()?,
        )
    }
}

// ===========================================================================
// ============================= Implementations =============================
// ===========================================================================

mod dividend_impl {
    use rust_decimal::prelude::FromPrimitive;

    use crate::{BaseMoney, BaseOps, Currency, Decimal, macros::dec};

    use super::Dividend;

    /// Yield = (DPS × periods_per_year) / share_price × 100
    pub(crate) fn get_dividend_yield<M, C>(bld: &Dividend<M, C>) -> Option<Decimal>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let annual_dps = bld
            .dividend_per_share
            .checked_mul(Decimal::from_u32(bld.periods_per_year)?)?;
        annual_dps
            .checked_div(bld.share_price)?
            .checked_mul(dec!(100))
    }

    /// Total income = DPS × shares × periods_per_year × years (without reinvestment)
    /// With reinvestment: compound each period
    pub(crate) fn get_income<M, C>(bld: &Dividend<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let total_periods = bld.periods_per_year.checked_mul(bld.years)?;

        if bld.reinvest {
            // DRIP: each period, dividend income buys more shares
            let mut current_shares = bld.shares;
            let mut total_income = dec!(0);

            for _period in 0..total_periods {
                let mut period_income = bld.dividend_per_share.checked_mul(current_shares)?;

                // apply tax if set
                if let Some(tax) = bld.tax {
                    let tax_amount = period_income.checked_mul(tax)?.checked_div(dec!(100))?;
                    period_income = period_income.checked_sub(tax_amount)?;
                }

                total_income = total_income.checked_add(period_income)?;

                // reinvest: buy more shares
                let new_shares = period_income.checked_div(bld.share_price)?;
                current_shares = current_shares.checked_add(new_shares)?;
            }

            M::new(total_income).ok()
        } else {
            // Simple: total = DPS × shares × total_periods
            let mut period_income = bld.dividend_per_share.checked_mul(bld.shares)?;

            // apply tax if set
            if let Some(tax) = bld.tax {
                let tax_amount = period_income.checked_mul(tax)?.checked_div(dec!(100))?;
                period_income = period_income.checked_sub(tax_amount)?;
            }

            let total_income = period_income.checked_mul(Decimal::from_u32(total_periods)?)?;

            M::new(total_income).ok()
        }
    }

    /// Payout Ratio = (DPS × periods_per_year) / EPS × 100
    pub(crate) fn get_payout_ratio<M, C>(bld: &Dividend<M, C>, eps: Decimal) -> Option<Decimal>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let annual_dps = bld
            .dividend_per_share
            .checked_mul(Decimal::from_u32(bld.periods_per_year)?)?;
        annual_dps.checked_div(eps)?.checked_mul(dec!(100))
    }

    /// Retention Ratio = 100 − Payout Ratio
    pub(crate) fn get_retention_ratio<M, C>(bld: &Dividend<M, C>, eps: Decimal) -> Option<Decimal>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let payout = get_payout_ratio(bld, eps)?;
        dec!(100).checked_sub(payout)
    }

    /// DPS = total_dividends / shares_outstanding
    pub(crate) fn get_dividend_per_share(
        total_dividends: Decimal,
        shares_outstanding: Decimal,
    ) -> Option<Decimal> {
        total_dividends.checked_div(shares_outstanding)
    }
}
