use std::marker::PhantomData;

use rust_decimal::prelude::FromPrimitive;

use crate::base::DecimalNumber;
use crate::calendar::*;
use crate::{BaseMoney, BaseOps, Currency, Decimal, base::Amount, macros::dec};

/// Trait defining interest calculation operations for fixed and compounding interest.
pub trait InterestOps<C> {
    type InterestBuilder<'a>
    where
        Self: 'a;

    /// Calculate fixed-interest on loan.
    ///
    /// # Formula
    /// FV = PV × (1 + (r × t))
    ///
    /// PV = FV / (1 + (r × t))
    ///
    /// PMT = P × r × (1+r)ᵗ / \[(1+r)ᵗ − 1\]
    ///
    /// # Argument
    /// rate: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    /// Rate is the percentage number only, e.g. 25% -> rate = 25.
    ///
    /// # Return
    /// It returns interest builder to set rate(daily, monthly, yearly) with periods(daily, monthly, yearly) of payment,
    /// along with day, month and year of calculations.
    ///
    /// The default rate is yearly and period is 12 months.
    /// The default year, month and day of calculation is current date.
    /// The default of rate days is Actual/Actual.
    fn interest_fixed<R>(&self, rate: R) -> Option<Self::InterestBuilder<'_>>
    where
        R: DecimalNumber;

    /// Calculate compounding-interest on loan.
    ///
    /// # Formula
    /// FV = PV × (1 + r)ᵗ
    ///
    /// PV = FV / (1 + r)ᵗ
    ///
    /// # Argument
    /// rate: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    /// Rate is the percentage number only, e.g. 25% -> rate = 25.
    ///
    /// # Return
    /// It returns interest builder to set rate(daily, monthly, yearly) with periods(daily, monthly, yearly) of payment,
    /// along with day, month and year of calculations.
    ///
    /// The default rate is yearly and period is 12 months.
    /// The default year, month and day of calculation is current date.
    /// The default of rate days is Actual/Actual.
    fn interest_compound<R>(&self, rate: R) -> Option<Self::InterestBuilder<'_>>
    where
        R: DecimalNumber;
}

impl<M, C> InterestOps<C> for M
where
    M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    C: Currency,
{
    type InterestBuilder<'a>
        = Interest<'a, M, C>
    where
        Self: 'a;

    fn interest_fixed<R>(&self, rate: R) -> Option<Self::InterestBuilder<'_>>
    where
        R: DecimalNumber,
    {
        let current_date = current_date()?;
        Some(Interest {
            principal: self.amount(),
            rate_percent: RatePercent::Yearly(rate.get_decimal()?), // default to annual rate
            total_period: Period::Months(12),
            interest_type: InterestType::Fixed,
            rate_days: Default::default(),
            year: current_date.0,
            month: current_date.1,
            day: current_date.2,
            contribs: None,
            tax: None,
            _output: PhantomData,
            _currency: PhantomData,
        })
    }

    fn interest_compound<R>(&self, rate: R) -> Option<Self::InterestBuilder<'_>>
    where
        R: DecimalNumber,
    {
        let current_date = current_date()?;
        Some(Interest {
            principal: self.amount(),
            rate_percent: RatePercent::Yearly(rate.get_decimal()?), // default to annual rate
            total_period: Period::Months(12),
            interest_type: InterestType::Compounding,
            rate_days: Default::default(),
            year: current_date.0,
            month: current_date.1,
            day: current_date.2,
            contribs: None,
            tax: None,
            _output: PhantomData,
            _currency: PhantomData,
        })
    }
}

/// Builder for interest calculations. Built through `self::InterestOps` trait.
#[derive(Debug, Clone, Copy)]
pub struct Interest<'a, M, C> {
    /// principal amount
    principal: Decimal,

    /// percentage of interest rate(daily, monthly, yearly)
    rate_percent: RatePercent,

    /// period of payment, including compounding points(daily, monthly, yearly)
    total_period: Period,

    /// interest type
    interest_type: InterestType,

    /// rate days for calculating interest rate
    rate_days: RateDays,

    /// year of the calculation
    year: u32,

    /// index of the month calculation, January -> 1
    month: u32,

    /// day
    day: u32,

    /// contributions each period(addition or negation)
    contribs: Option<&'a [M]>,

    /// flat-rate tax applies to each period
    tax: Option<Decimal>,

    _output: PhantomData<M>,
    _currency: PhantomData<C>,
}

impl<'a, M, C> Interest<'a, M, C> {
    /// Add contributions each period.
    /// Length of contribs is at MOST length of period - 1,
    /// since contribution add on the second time of the period length.
    /// Value can be positive or negative and can be skipped by adding zero.
    pub fn with_contribs(self, contribs: &'a [M]) -> Option<Self> {
        let period_length: usize = self.total_period.get_period_length().try_into().ok()?;
        if contribs.len() >= period_length {
            return None;
        }
        Some(Self {
            contribs: Some(contribs),
            ..self
        })
    }

    /// Set tax flat-rate percentage applied to interest returns.
    /// The tax is expressed as a percentage number, e.g. 20% -> tax = 20.
    ///
    /// The tax is flat-rate, meaning it applies no matter how much the return is.
    ///
    /// # Argument
    /// D: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    ///
    pub fn with_tax<D>(self, tax: D) -> Option<Self>
    where
        D: crate::base::DecimalNumber,
    {
        Some(Self {
            tax: Some(tax.get_decimal()?),
            ..self
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum RatePercent {
    Daily(Decimal),
    Monthly(Decimal),
    Yearly(Decimal),
}

impl RatePercent {
    const fn get_rate_amount(&self) -> Decimal {
        match self {
            Self::Daily(v) | Self::Monthly(v) | Self::Yearly(v) => *v,
        }
    }

    /// Get the actual rate relative to daily payment period.
    ///
    /// - if rate is daily then r = r
    /// - if rate is monthly then r = r / number of days in that month
    /// - if rate is yearly/annual then r = r / number of days in that year
    fn get_daily_rate(&self, rate_days: RateDays, year: u32, month: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => v.checked_div(dec!(100)),
            Self::Monthly(v) => v
                .checked_div(Decimal::from_u32(rate_days.days_in_month(year, month)?)?)?
                .checked_div(dec!(100)),
            Self::Yearly(v) => v
                .checked_div(Decimal::from_u32(rate_days.days_in_year(year)?)?)?
                .checked_div(dec!(100)),
        }
    }

    /// Get the actual rate relative to monthly payment period.
    ///
    /// - if rate is daily then r = r * number of days in that month
    /// - if rate is monthly then r = r
    /// - if rate is yearly then r = r / 12
    fn get_monthly_rate(&self, rate_days: RateDays, year: u32, month: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => v
                .checked_mul(Decimal::from_u32(rate_days.days_in_month(year, month)?)?)?
                .checked_div(dec!(100)),
            Self::Monthly(v) => v.checked_div(dec!(100)),
            Self::Yearly(v) => v
                .checked_div(Decimal::from_u32(12)?)?
                .checked_div(dec!(100)),
        }
    }

    /// Get the actual rate relative to yearly/annual payment period.
    ///
    /// - if rate is daily then r = r * number of days in that year
    /// - if rate is monthly then r = r * 12
    /// - if rate is yearly then r = r
    fn get_yearly_rate(&self, rate_days: RateDays, year: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => v
                .checked_mul(Decimal::from_u32(rate_days.days_in_year(year)?)?)?
                .checked_div(dec!(100)),
            Self::Monthly(v) => v
                .checked_mul(Decimal::from_u32(12)?)?
                .checked_div(dec!(100)),
            Self::Yearly(v) => v.checked_div(dec!(100)),
        }
    }

    fn get_quarterly_rate(&self, rate_days: RateDays, year: u32, month: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => {
                let (first_month_year, first_month) = (year, month);
                let (second_month_year, second_month, _) =
                    first_month.next_month(first_month_year)?;
                let (third_month_year, third_month, _) =
                    second_month.next_month(second_month_year)?;

                let total_days = rate_days
                    .days_in_month(year, first_month)?
                    .checked_add(rate_days.days_in_month(second_month_year, second_month)?)?
                    .checked_add(rate_days.days_in_month(third_month_year, third_month)?)?;

                v.checked_mul(Decimal::from_u32(total_days)?)?
                    .checked_div(dec!(100))
            }
            Self::Monthly(v) => {
                let quarter_rate = v.checked_mul(dec!(3))?;
                quarter_rate.checked_div(dec!(100))
            }
            Self::Yearly(v) => {
                let quarter_rate = v.checked_div(dec!(4))?;
                quarter_rate.checked_div(dec!(100))
            }
        }
    }

    fn get_semi_annualy_rate(&self, rate_days: RateDays, year: u32, month: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => {
                let (first_month_year, first_month) = (year, month);
                let (second_month_year, second_month, _) =
                    first_month.next_month(first_month_year)?;
                let (third_month_year, third_month, _) =
                    second_month.next_month(second_month_year)?;
                let (fourth_month_year, fourth_month, _) =
                    third_month.next_month(third_month_year)?;
                let (fifth_month_year, fifth_month, _) =
                    fourth_month.next_month(fourth_month_year)?;
                let (sixth_month_year, sixth_month, _) =
                    fifth_month.next_month(fifth_month_year)?;

                let total_days = rate_days
                    .days_in_month(first_month_year, first_month)?
                    .checked_add(rate_days.days_in_month(second_month_year, second_month)?)?
                    .checked_add(rate_days.days_in_month(third_month_year, third_month)?)?
                    .checked_add(rate_days.days_in_month(fourth_month_year, fourth_month)?)?
                    .checked_add(rate_days.days_in_month(fifth_month_year, fifth_month)?)?
                    .checked_add(rate_days.days_in_month(sixth_month_year, sixth_month)?)?;

                v.checked_mul(Decimal::from_u32(total_days)?)?
                    .checked_div(dec!(100))
            }
            Self::Monthly(v) => {
                let quarter_rate = v.checked_mul(dec!(6))?;
                quarter_rate.checked_div(dec!(100))
            }
            Self::Yearly(v) => {
                let quarter_rate = v.checked_div(dec!(2))?;
                quarter_rate.checked_div(dec!(100))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum InterestType {
    Fixed,
    Compounding,
}

#[derive(Debug, Clone, Copy)]
enum Period {
    Days(u32),
    Months(u32),
    Years(u32),
    Quarters(u32),
    SemiAnnuals(u32),
}

impl Period {
    fn get_period_length(&self) -> u32 {
        match self {
            Period::Days(t)
            | Period::Months(t)
            | Period::Years(t)
            | Period::Quarters(t)
            | Period::SemiAnnuals(t) => *t,
        }
    }
}

/// Get number of days in a month and in a year according to ISO 15022 MT565: (16) Field 22F
#[derive(Debug, Clone, Copy, Default)]
pub enum RateDays {
    /// 30 days per month / 360 days per year (Bond Basis)
    Rate30360,
    /// 30 days per month / 365 days per year
    Rate30365,
    /// 30 days per month / Actual days in the year (365 or 366)
    Rate30Actual,
    /// Actual days per month / 360 days per year (Money Market)
    RateActual360,
    /// Actual days per month / 365 days per year (Retail Banking)
    RateActual365,
    /// Actual days per month / Actual days in the year (Treasury)
    #[default]
    RateActualActual,
}

impl RateDays {
    fn days_in_month(&self, year: u32, month: u32) -> Option<u32> {
        match self {
            Self::Rate30360 | Self::Rate30365 | Self::Rate30Actual => Some(30),
            Self::RateActual360 | Self::RateActual365 | Self::RateActualActual => {
                days_in_month(year, month)
            }
        }
    }

    fn days_in_year(&self, year: u32) -> Option<u32> {
        match self {
            Self::Rate30360 | Self::RateActual360 => Some(360),
            Self::Rate30365 | Self::RateActual365 => Some(365),
            Self::Rate30Actual | Self::RateActualActual => Some(days_in_year(year)),
        }
    }
}

impl<'a, M, C> Interest<'a, M, C>
where
    M: BaseMoney<C> + BaseOps<C> + Default,
    C: Currency,
{
    /// Sets the interest rate to daily.
    pub const fn daily(self) -> Self {
        Self {
            rate_percent: RatePercent::Daily(self.rate_percent.get_rate_amount()),
            ..self
        }
    }

    /// Sets the period of interest payments each day for n days.
    pub const fn days(self, n: u32) -> Self {
        Self {
            total_period: Period::Days(n),
            ..self
        }
    }

    /// Sets the interest rate to monthly.
    pub const fn monthly(self) -> Self {
        Self {
            rate_percent: RatePercent::Monthly(self.rate_percent.get_rate_amount()),
            ..self
        }
    }

    /// Sets the period of interest payments each month for n months.
    pub const fn months(self, n: u32) -> Self {
        Self {
            total_period: Period::Months(n),
            ..self
        }
    }

    /// Sets the interest rate to yearly/annual.
    pub const fn yearly(self) -> Self {
        Self {
            rate_percent: RatePercent::Yearly(self.rate_percent.get_rate_amount()),
            ..self
        }
    }

    /// Sets the period of interest payments each year for n years.
    pub const fn years(self, n: u32) -> Self {
        Self {
            total_period: Period::Years(n),
            ..self
        }
    }

    /// Sets the period of interest payments every 3 months.
    pub const fn quarters(self, n: u32) -> Self {
        Self {
            total_period: Period::Quarters(n),
            ..self
        }
    }

    /// Sets the period of interest payments every 6 months.
    pub const fn semi_annuals(self, n: u32) -> Self {
        Self {
            total_period: Period::SemiAnnuals(n),
            ..self
        }
    }

    /// Sets start of the calculation year.
    pub const fn year(self, year: u32) -> Self {
        Self { year, ..self }
    }

    /// Sets start of the calculation month by index, January = 1.
    pub const fn month(self, month: u32) -> Self {
        Self { month, ..self }
    }

    /// Set start of the calculation day date.
    pub const fn day(self, day: u32) -> Self {
        Self { day, ..self }
    }

    /// Set rate days
    pub const fn rate_days(self, rate_days: RateDays) -> Self {
        Self { rate_days, ..self }
    }

    /// Calculate the returns(total interests)
    pub fn returns(&self) -> Option<M> {
        match self.interest_type {
            InterestType::Fixed => interest_impl::get_returns_fixed(self),
            InterestType::Compounding => interest_impl::get_returns_compounding(self),
        }
    }

    /// Calculate the future value of a money: Principal + Interests.
    pub fn future_value(&self) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    {
        interest_impl::get_future_value(self)
    }

    /// Calculate the present value of future goal.
    pub fn present_value(&self) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    {
        match self.interest_type {
            InterestType::Fixed => interest_impl::get_present_value_fixed(self),
            InterestType::Compounding => interest_impl::get_present_value_compounding(self),
        }
    }

    /// Calculate amortized payment(PMT)
    /// PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
    /// where r is the period rate and n is the total number of periods.
    /// PMT is amortized against fixed-rate loan.
    pub fn payment(&self) -> Option<M> {
        match self.interest_type {
            InterestType::Fixed => interest_impl::get_pmt(self),
            _ => None,
        }
    }
}

// ===========================================================================
// ============================= Implementations =============================
// ===========================================================================

mod interest_impl {
    use crate::{
        BaseMoney, BaseOps, Currency, IterOps,
        accounting::interest::{Interest, InterestType, Period},
        base::Amount,
        calendar::{AddMonths, get_years_months, get_years_months_days},
        macros::dec,
    };

    /// Get total returns of fixed-rate
    ///
    /// FV = PV * (1 + (r * t))
    pub(crate) fn get_returns_fixed<M, C>(bld: &Interest<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Default,
        C: Currency,
    {
        let total_interest = match bld.total_period {
            Period::Days(t) => {
                let years_months_days = get_years_months_days(bld.year, bld.month, bld.day, t)?;
                let mut interest_total = dec!(0);
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for year in years_months_days {
                    for month in year.1 {
                        for _day in month.1 {
                            interest_total = interest_total.checked_add(
                                current_principal.checked_mul(bld.rate_percent.get_daily_rate(
                                    bld.rate_days,
                                    year.0,
                                    month.0,
                                )?)?,
                            )?;

                            if let Some(contribs) = bld.contribs {
                                let contrib = if let Some(c) = contribs.get(contrib_index) {
                                    c.amount()
                                } else {
                                    dec!(0)
                                };
                                current_principal = current_principal.checked_add(contrib)?;
                                if current_principal <= dec!(0) {
                                    return Some(M::default());
                                }
                                contrib_index = contrib_index.checked_add(1)?;
                            }
                        }
                    }
                }

                Some(interest_total)
            }
            Period::Months(t) => {
                let year_months = get_years_months(bld.year, bld.month, t)?;
                let mut total_interest = dec!(0);
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for (year, months) in year_months {
                    for month in months {
                        total_interest = total_interest.checked_add(
                            current_principal.checked_mul(bld.rate_percent.get_monthly_rate(
                                bld.rate_days,
                                year,
                                month,
                            )?)?,
                        )?;

                        if let Some(contribs) = bld.contribs {
                            let contrib = if let Some(c) = contribs.get(contrib_index) {
                                c.amount()
                            } else {
                                dec!(0)
                            };
                            current_principal = current_principal.checked_add(contrib)?;
                            if current_principal <= dec!(0) {
                                return Some(M::default());
                            }
                            contrib_index = contrib_index.checked_add(1)?;
                        }
                    }
                }
                Some(total_interest)
            }
            Period::Years(t) => {
                let mut interest_total = dec!(0);
                let mut current_year = bld.year;
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for _y in 0..t {
                    interest_total = interest_total.checked_add(
                        current_principal.checked_mul(
                            bld.rate_percent
                                .get_yearly_rate(bld.rate_days, current_year)?,
                        )?,
                    )?;
                    current_year = current_year.checked_add(1)?;

                    if let Some(contribs) = bld.contribs {
                        let contrib = if let Some(c) = contribs.get(contrib_index) {
                            c.amount()
                        } else {
                            dec!(0)
                        };
                        current_principal = current_principal.checked_add(contrib)?;
                        if current_principal <= dec!(0) {
                            return Some(M::default());
                        }
                        contrib_index = contrib_index.checked_add(1)?;
                    }
                }
                Some(interest_total)
            }
            Period::Quarters(t) => {
                let mut total_interest = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for _q in 0..t {
                    total_interest = total_interest.checked_add(current_principal.checked_mul(
                        bld.rate_percent.get_quarterly_rate(
                            bld.rate_days,
                            current_year,
                            current_month,
                        )?,
                    )?)?;
                    let (next_quarter_year, next_quarter_month, _) =
                        current_month.add_months(current_year, 3)?;
                    current_year = next_quarter_year;
                    current_month = next_quarter_month;

                    if let Some(contribs) = bld.contribs {
                        let contrib = if let Some(c) = contribs.get(contrib_index) {
                            c.amount()
                        } else {
                            dec!(0)
                        };
                        current_principal = current_principal.checked_add(contrib)?;
                        if current_principal <= dec!(0) {
                            return Some(M::default());
                        }
                        contrib_index = contrib_index.checked_add(1)?;
                    }
                }

                Some(total_interest)
            }
            Period::SemiAnnuals(t) => {
                let mut total_interest = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for _s in 0..t {
                    total_interest = total_interest.checked_add(current_principal.checked_mul(
                        bld.rate_percent.get_semi_annualy_rate(
                            bld.rate_days,
                            current_year,
                            current_month,
                        )?,
                    )?)?;
                    let (next_halfyear_year, next_halfyear_month, _) =
                        current_month.add_months(current_year, 6)?;
                    current_year = next_halfyear_year;
                    current_month = next_halfyear_month;

                    if let Some(contribs) = bld.contribs {
                        let contrib = if let Some(c) = contribs.get(contrib_index) {
                            c.amount()
                        } else {
                            dec!(0)
                        };
                        current_principal = current_principal.checked_add(contrib)?;
                        if current_principal <= dec!(0) {
                            return Some(M::default());
                        }
                        contrib_index = contrib_index.checked_add(1)?;
                    }
                }

                Some(total_interest)
            }
        };

        M::new(total_interest?).ok()
    }

    /// Get total returns of compounding rate.
    ///
    /// FV = PV * (1 + r)^t
    pub(crate) fn get_returns_compounding<M, C>(bld: &Interest<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Default,
        C: Currency,
    {
        let total_interest = match bld.total_period {
            Period::Days(t) => {
                let years_months_days = get_years_months_days(bld.year, bld.month, bld.day, t)?;
                let mut total_interest = dec!(0);
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for year in years_months_days {
                    for month in year.1 {
                        for _day in month.1 {
                            let current_interest = current_principal.checked_mul(
                                bld.rate_percent
                                    .get_daily_rate(bld.rate_days, year.0, month.0)?,
                            )?;
                            total_interest = total_interest.checked_add(current_interest)?;
                            current_principal = current_principal.checked_add(current_interest)?;

                            if let Some(contribs) = bld.contribs {
                                let contrib = if let Some(c) = contribs.get(contrib_index) {
                                    c.amount()
                                } else {
                                    dec!(0)
                                };
                                current_principal = current_principal.checked_add(contrib)?;
                                if current_principal <= dec!(0) {
                                    return Some(M::default());
                                }
                                contrib_index = contrib_index.checked_add(1)?;
                            }
                        }
                    }
                }
                Some(total_interest)
            }
            Period::Months(t) => {
                let years_months = get_years_months(bld.year, bld.month, t)?;
                let mut total_interest = dec!(0);
                let mut current_principal = bld.principal;
                let mut contrib_index = 0;
                for year in years_months {
                    for month in year.1 {
                        let current_interest = current_principal.checked_mul(
                            bld.rate_percent
                                .get_monthly_rate(bld.rate_days, year.0, month)?,
                        )?;
                        total_interest = total_interest.checked_add(current_interest)?;
                        current_principal = current_principal.checked_add(current_interest)?;

                        if let Some(contribs) = bld.contribs {
                            let contrib = if let Some(c) = contribs.get(contrib_index) {
                                c.amount()
                            } else {
                                dec!(0)
                            };
                            current_principal = current_principal.checked_add(contrib)?;
                            if current_principal <= dec!(0) {
                                return Some(M::default());
                            }
                            contrib_index = contrib_index.checked_add(1)?;
                        }
                    }
                }
                Some(total_interest)
            }
            Period::Years(t) => {
                let mut total_interest = dec!(0);
                let mut current_principal = bld.principal;
                let mut current_year = bld.year;
                let mut contrib_index = 0;
                for _y in 0..t {
                    let current_interest = current_principal.checked_mul(
                        bld.rate_percent
                            .get_yearly_rate(bld.rate_days, current_year)?,
                    )?;
                    total_interest = total_interest.checked_add(current_interest)?;
                    current_principal = current_principal.checked_add(current_interest)?;
                    current_year = current_year.checked_add(1)?;

                    if let Some(contribs) = bld.contribs {
                        let contrib = if let Some(c) = contribs.get(contrib_index) {
                            c.amount()
                        } else {
                            dec!(0)
                        };
                        current_principal = current_principal.checked_add(contrib)?;
                        if current_principal <= dec!(0) {
                            return Some(M::default());
                        }
                        contrib_index = contrib_index.checked_add(1)?;
                    }
                }
                Some(total_interest)
            }
            Period::Quarters(t) => {
                let mut current_principal = bld.principal;
                let mut total_interest = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;
                let mut contrib_index = 0;
                for _q in 0..t {
                    let current_interest =
                        current_principal.checked_mul(bld.rate_percent.get_quarterly_rate(
                            bld.rate_days,
                            current_year,
                            current_month,
                        )?)?;
                    total_interest = total_interest.checked_add(current_interest)?;
                    current_principal = current_principal.checked_add(current_interest)?;

                    let (next_quarter_year, next_quarter_month, _) =
                        current_month.add_months(current_year, 3)?;
                    current_year = next_quarter_year;
                    current_month = next_quarter_month;

                    if let Some(contribs) = bld.contribs {
                        let contrib = if let Some(c) = contribs.get(contrib_index) {
                            c.amount()
                        } else {
                            dec!(0)
                        };
                        current_principal = current_principal.checked_add(contrib)?;
                        if current_principal <= dec!(0) {
                            return Some(M::default());
                        }
                        contrib_index = contrib_index.checked_add(1)?;
                    }
                }

                Some(total_interest)
            }
            Period::SemiAnnuals(t) => {
                let mut current_principal = bld.principal;
                let mut total_interest = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;
                let mut contrib_index = 0;
                for _q in 0..t {
                    let current_interest =
                        current_principal.checked_mul(bld.rate_percent.get_semi_annualy_rate(
                            bld.rate_days,
                            current_year,
                            current_month,
                        )?)?;
                    total_interest = total_interest.checked_add(current_interest)?;
                    current_principal = current_principal.checked_add(current_interest)?;

                    let (next_halfyear_year, next_halfyear_month, _) =
                        current_month.add_months(current_year, 6)?;
                    current_year = next_halfyear_year;
                    current_month = next_halfyear_month;

                    if let Some(contribs) = bld.contribs {
                        let contrib = if let Some(c) = contribs.get(contrib_index) {
                            c.amount()
                        } else {
                            dec!(0)
                        };
                        current_principal = current_principal.checked_add(contrib)?;
                        if current_principal <= dec!(0) {
                            return Some(M::default());
                        }
                        contrib_index = contrib_index.checked_add(1)?;
                    }
                }

                Some(total_interest)
            }
        };

        M::new(total_interest?).ok()
    }

    use crate::PercentOps;
    /// Get future value: principal + contributions + total interests
    ///
    /// FV = PV * (1 + (r * t))
    pub(crate) fn get_future_value<C, M>(bld: &Interest<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C> + Default + PercentOps<C, Output = M>,
        C: Currency,
    {
        let principal = bld.principal;

        let returns = {
            let mut returns = match bld.interest_type {
                InterestType::Fixed => get_returns_fixed(bld)?,
                InterestType::Compounding => get_returns_compounding(bld)?,
            };

            // apply tax if it exists
            if let Some(tax) = bld.tax {
                returns = returns.percent_sub(tax)?;
            }

            returns
        };

        let mut total_contribs = M::default();
        if let Some(contribs) = bld.contribs
            && !contribs.is_empty()
        {
            total_contribs = total_contribs.checked_add(contribs.checked_sum()?)?;
        }

        returns.checked_add(principal)?.checked_add(total_contribs)
    }

    /// Get present value on fixed-rate interest
    ///
    /// PV = FV / (1 + (r * t))
    pub(crate) fn get_present_value_fixed<C, M>(bld: &Interest<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let ret = match bld.total_period {
            Period::Days(t) => {
                let years_months_days = get_years_months_days(bld.year, bld.month, bld.day, t)?;
                let mut actual_r = dec!(0);
                for year in years_months_days {
                    for month in year.1 {
                        for _day in month.1 {
                            actual_r = actual_r.checked_add(bld.rate_percent.get_daily_rate(
                                bld.rate_days,
                                year.0,
                                month.0,
                            )?)?;
                        }
                    }
                }
                let divisor = dec!(1).checked_add(actual_r)?;

                bld.principal.checked_div(divisor)
            }
            Period::Months(t) => {
                let years_months = get_years_months(bld.year, bld.month, t)?;
                let mut actual_r = dec!(0);
                for year in years_months {
                    for month in year.1 {
                        actual_r = actual_r.checked_add(bld.rate_percent.get_monthly_rate(
                            bld.rate_days,
                            year.0,
                            month,
                        )?)?;
                    }
                }
                let divisor = dec!(1).checked_add(actual_r)?;

                bld.principal.checked_div(divisor)
            }
            Period::Years(t) => {
                let mut actual_r = dec!(0);
                let mut current_year = bld.year;
                for _y in 0..t {
                    actual_r = actual_r.checked_add(
                        bld.rate_percent
                            .get_yearly_rate(bld.rate_days, current_year)?,
                    )?;
                    current_year = current_year.checked_add(1)?;
                }
                let divisor = dec!(1).checked_add(actual_r)?;

                bld.principal.checked_div(divisor)
            }
            Period::Quarters(t) => {
                let mut actual_r = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;

                for _q in 0..t {
                    actual_r = actual_r.checked_add(bld.rate_percent.get_quarterly_rate(
                        bld.rate_days,
                        current_year,
                        current_month,
                    )?)?;

                    let (next_quarter_year, next_quarter_month, _) =
                        current_month.add_months(current_year, 3)?;
                    current_year = next_quarter_year;
                    current_month = next_quarter_month;
                }

                let divisor = dec!(1).checked_add(actual_r)?;

                bld.principal.checked_div(divisor)
            }
            Period::SemiAnnuals(t) => {
                let mut actual_r = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;

                for _q in 0..t {
                    actual_r = actual_r.checked_add(bld.rate_percent.get_semi_annualy_rate(
                        bld.rate_days,
                        current_year,
                        current_month,
                    )?)?;

                    let (next_halfyear_year, next_halfyear_month, _) =
                        current_month.add_months(current_year, 6)?;
                    current_year = next_halfyear_year;
                    current_month = next_halfyear_month;
                }

                let divisor = dec!(1).checked_add(actual_r)?;

                bld.principal.checked_div(divisor)
            }
        };

        M::new(ret?).ok()
    }

    /// Get present value on compounding-rate interest
    ///
    /// PV = FV / (1 + r)^t
    pub(crate) fn get_present_value_compounding<C, M>(bld: &Interest<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let ret = match bld.total_period {
            Period::Days(t) => {
                let years_months_days = get_years_months_days(bld.year, bld.month, bld.day, t)?;
                let mut divisor = dec!(1);
                for year in years_months_days {
                    for month in year.1 {
                        for _day in month.1 {
                            let d = dec!(1).checked_add(bld.rate_percent.get_daily_rate(
                                bld.rate_days,
                                year.0,
                                month.0,
                            )?)?;
                            divisor = divisor.checked_mul(d)?;
                        }
                    }
                }

                bld.principal.checked_div(divisor)
            }
            Period::Months(t) => {
                let years_months = get_years_months(bld.year, bld.month, t)?;
                let mut divisor = dec!(1);
                for year in years_months {
                    for month in year.1 {
                        let d = dec!(1).checked_add(bld.rate_percent.get_monthly_rate(
                            bld.rate_days,
                            year.0,
                            month,
                        )?)?;
                        divisor = divisor.checked_mul(d)?;
                    }
                }

                bld.principal.checked_div(divisor)
            }
            Period::Years(t) => {
                let mut current_year = bld.year;
                let mut divisor = dec!(1);
                for _y in 0..t {
                    let d = dec!(1).checked_add(
                        bld.rate_percent
                            .get_yearly_rate(bld.rate_days, current_year)?,
                    )?;
                    divisor = divisor.checked_mul(d)?;

                    current_year = current_year.checked_add(1)?;
                }

                bld.principal.checked_div(divisor)
            }
            Period::Quarters(t) => {
                let mut divisor = dec!(1);
                let mut current_year = bld.year;
                let mut current_month = bld.month;

                for _q in 0..t {
                    let d = dec!(1).checked_add(bld.rate_percent.get_quarterly_rate(
                        bld.rate_days,
                        current_year,
                        current_month,
                    )?)?;
                    divisor = divisor.checked_mul(d)?;

                    let (next_quarter_year, next_quarter_month, _) =
                        current_month.add_months(current_year, 3)?;
                    current_year = next_quarter_year;
                    current_month = next_quarter_month;
                }

                bld.principal.checked_div(divisor)
            }
            Period::SemiAnnuals(t) => {
                let mut divisor = dec!(1);
                let mut current_year = bld.year;
                let mut current_month = bld.month;

                for _q in 0..t {
                    let d = dec!(1).checked_add(bld.rate_percent.get_semi_annualy_rate(
                        bld.rate_days,
                        current_year,
                        current_month,
                    )?)?;
                    divisor = divisor.checked_mul(d)?;

                    let (next_halfyear_year, next_halfyear_month, _) =
                        current_month.add_months(current_year, 6)?;
                    current_year = next_halfyear_year;
                    current_month = next_halfyear_month;
                }

                bld.principal.checked_div(divisor)
            }
        };

        M::new(ret?).ok()
    }

    /// Get PMT payment on fixed-rate interest
    ///
    /// PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
    ///
    /// PMT is calculated against fixed-rate loan.
    pub(crate) fn get_pmt<C, M>(bld: &Interest<M, C>) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C>,
        C: Currency,
    {
        let pmt = match bld.total_period {
            Period::Years(t) => {
                // c accumulates (1+r)ⁿ; r_period holds the last period rate.
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let mut current_year = bld.year;
                for _ in 0..t {
                    let r = bld
                        .rate_percent
                        .get_yearly_rate(bld.rate_days, current_year)?;
                    r_period = r;
                    // (1+r)ⁿ
                    c = c.checked_mul(dec!(1).checked_add(r)?)?;
                    current_year = current_year.checked_add(1)?;
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                bld.principal.checked_mul(d)
            }
            Period::Months(t) => {
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let years_months = get_years_months(bld.year, bld.month, t)?;
                for year in years_months {
                    for month in year.1.iter() {
                        let r = bld
                            .rate_percent
                            .get_monthly_rate(bld.rate_days, year.0, *month)?;
                        r_period = r;
                        // (1+r)ⁿ
                        c = c.checked_mul(dec!(1).checked_add(r)?)?;
                    }
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                bld.principal.checked_mul(d)
            }
            Period::Days(t) => {
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let years_months_days = get_years_months_days(bld.year, bld.month, bld.day, t)?;

                for year in years_months_days {
                    for month in year.1 {
                        for _day in month.1.iter() {
                            let r =
                                bld.rate_percent
                                    .get_daily_rate(bld.rate_days, year.0, month.0)?;
                            r_period = r;
                            // (1+r)ⁿ
                            c = c.checked_mul(dec!(1).checked_add(r)?)?;
                        }
                    }
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                bld.principal.checked_mul(d)
            }

            Period::Quarters(t) => {
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;

                for _ in 0..t {
                    let r = bld.rate_percent.get_quarterly_rate(
                        bld.rate_days,
                        current_year,
                        current_month,
                    )?;
                    r_period = r;
                    c = c.checked_mul(dec!(1).checked_add(r)?)?;
                    let (next_year, next_month, _) = current_month.add_months(current_year, 3)?;
                    current_year = next_year;
                    current_month = next_month;
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                bld.principal.checked_mul(d)
            }
            Period::SemiAnnuals(t) => {
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let mut current_year = bld.year;
                let mut current_month = bld.month;

                for _ in 0..t {
                    let r = bld.rate_percent.get_semi_annualy_rate(
                        bld.rate_days,
                        current_year,
                        current_month,
                    )?;
                    r_period = r;
                    c = c.checked_mul(dec!(1).checked_add(r)?)?;
                    let (next_year, next_month, _) = current_month.add_months(current_year, 6)?;
                    current_year = next_year;
                    current_month = next_month;
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                bld.principal.checked_mul(d)
            }
        };

        M::new(pmt?).ok()
    }
}
