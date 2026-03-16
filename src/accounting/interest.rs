use std::marker::PhantomData;

use rust_decimal::prelude::FromPrimitive;

use crate::base::DecimalNumber;
use crate::calendar::*;
use crate::{BaseMoney, BaseOps, Currency, Decimal, base::Amount, macros::dec};

/// Trait defining interest calculation operations for fixed and compounding interest.
pub trait InterestOps<C> {
    type InterestBuilder;

    /// Calculate fixed-interest on loan.
    ///
    /// # Argument
    /// rate: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    ///
    /// # Return
    /// It returns interest builder to set rate(daily, monthly, yearly) with periods(daily, monthly, yearly) of payment,
    /// along with day, month and year of calculations.
    ///
    /// The default rate is yearly and period is 12 months.
    fn interest_fixed<R>(&self, rate: R) -> Option<Self::InterestBuilder>
    where
        R: DecimalNumber;

    /// Calculate compounding-interest on loan.
    ///
    /// # Argument
    /// rate: impl DecimalNumber, supports Decimal, f64, i32, i64, i128.
    ///
    /// # Return
    /// It returns interest builder to set rate(daily, monthly, yearly) with periods(daily, monthly, yearly) of payment,
    /// along with day, month and year of calculations.
    ///
    /// The default rate is yearly and period is 12 months.
    fn interest_compound<R>(&self, rate: R) -> Option<Self::InterestBuilder>
    where
        R: DecimalNumber;
}

impl<M, C> InterestOps<C> for M
where
    M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    C: Currency + Clone,
{
    type InterestBuilder = Interest<M, C>;

    fn interest_fixed<R>(&self, rate: R) -> Option<Self::InterestBuilder>
    where
        R: DecimalNumber,
    {
        let current_date = current_date()?;
        Some(Interest {
            principal: self.amount(),
            rate_percent: RatePercent::Yearly(rate.get_decimal()?), // default to annual rate
            total_period: Period::Months(12),
            interest_type: InterestType::Fixed,
            year: current_date.0,
            month: current_date.1,
            day: current_date.2,
            _output: PhantomData,
            _currency: PhantomData,
        })
    }

    fn interest_compound<R>(&self, rate: R) -> Option<Self::InterestBuilder>
    where
        R: DecimalNumber,
    {
        let current_date = current_date()?;
        Some(Interest {
            principal: self.amount(),
            rate_percent: RatePercent::Yearly(rate.get_decimal()?), // default to annual rate
            total_period: Period::Months(12),
            interest_type: InterestType::Compounding,
            year: current_date.0,
            month: current_date.1,
            day: current_date.2,
            _output: PhantomData,
            _currency: PhantomData,
        })
    }
}

/// Builder for interest calculations. Built through `self::InterestOps` trait.
#[derive(Debug, Clone, Copy)]
pub struct Interest<M, C> {
    /// principal amount
    principal: Decimal,

    /// percentage of interest rate(daily, monthly, yearly)
    rate_percent: RatePercent,

    /// period of payment, including compounding points(daily, monthly, yearly)
    total_period: Period,

    /// interest type
    interest_type: InterestType,

    /// year of the calculation
    year: u32,

    /// index of the month calculation, January -> 1
    month: u32,

    /// day
    day: u32,

    _output: PhantomData<M>,
    _currency: PhantomData<C>,
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
    fn get_daily_rate(&self, month: u32, year: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => v.checked_div(dec!(100)),
            Self::Monthly(v) => v
                .checked_div(Decimal::from_u32(days_in_month(year, month)?)?)?
                .checked_div(dec!(100)),
            Self::Yearly(v) => v
                .checked_div(Decimal::from_u32(days_in_year(year))?)?
                .checked_div(dec!(100)),
        }
    }

    /// Get the actual rate relative to monthly payment period.
    ///
    /// - if rate is daily then r = r * number of days in that month
    /// - if rate is monthly then r = r
    /// - if rate is yearly then r = r / 12
    fn get_monthly_rate(&self, month: u32, year: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => v
                .checked_mul(Decimal::from_u32(days_in_month(year, month)?)?)?
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
    fn get_yearly_rate(&self, year: u32) -> Option<Decimal> {
        match self {
            Self::Daily(v) => v
                .checked_mul(Decimal::from_u32(days_in_year(year))?)?
                .checked_div(dec!(100)),
            Self::Monthly(v) => v
                .checked_mul(Decimal::from_u32(12)?)?
                .checked_div(dec!(100)),
            Self::Yearly(v) => v.checked_div(dec!(100)),
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
}

impl<M, C> Interest<M, C>
where
    M: BaseMoney<C>,
    C: Currency,
{
    /// Sets the interest rate to daily.
    pub const fn daily(self) -> Self {
        Self {
            principal: self.principal,
            rate_percent: RatePercent::Daily(self.rate_percent.get_rate_amount()),
            total_period: self.total_period,
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets the period of interest payments each day for n days.
    pub const fn days(self, n: u32) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: Period::Days(n),
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets the interest rate to monthly.
    pub const fn monthly(self) -> Self {
        Self {
            principal: self.principal,
            rate_percent: RatePercent::Monthly(self.rate_percent.get_rate_amount()),
            total_period: self.total_period,
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets the period of interest payments each month for n months.
    pub const fn months(self, n: u32) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: Period::Months(n),
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets the interest rate to yearly/annual.
    pub const fn yearly(self) -> Self {
        Self {
            principal: self.principal,
            rate_percent: RatePercent::Yearly(self.rate_percent.get_rate_amount()),
            total_period: self.total_period,
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets the period of interest payments each year for n years.
    pub const fn years(self, n: u32) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: Period::Years(n),
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets start of the calculation year.
    pub const fn year(self, year: u32) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: self.total_period,
            interest_type: self.interest_type,
            year,
            month: self.month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Sets start of the calculation month by index, January = 1.
    pub const fn month(self, month: u32) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: self.total_period,
            interest_type: self.interest_type,
            year: self.year,
            month,
            day: self.day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Set start of the calculation day date.
    pub const fn day(self, day: u32) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: self.total_period,
            interest_type: self.interest_type,
            year: self.year,
            month: self.month,
            day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    /// Calculate the returns(total interests)
    pub fn returns(&self) -> Option<M> {
        match self.interest_type {
            InterestType::Fixed => {
                let fixed_ret = match (self.rate_percent, self.total_period) {
                    // r is yearly
                    (RatePercent::Yearly(_), Period::Years(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_yearly_rate(self.year)?)?
                        .checked_mul(Decimal::from_u32(t)?), // P * r * t
                    (RatePercent::Yearly(_), Period::Months(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_monthly_rate(self.month, self.year)?)?
                        .checked_mul(Decimal::from_u32(t)?), // P * (r/12) * t
                    (RatePercent::Yearly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut interest_total = dec!(0);
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    interest_total =
                                        interest_total.checked_add(self.principal.checked_mul(
                                            self.rate_percent.get_daily_rate(month.0, year.0)?,
                                        )?)?;
                                }
                            }
                        }
                        Some(interest_total)
                    } // P * (r/365) * t --> loop

                    // r is monthly
                    (RatePercent::Monthly(_), Period::Years(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_yearly_rate(self.year)?)?
                        .checked_mul(Decimal::from_u32(t)?), // P * (r*12) * t
                    (RatePercent::Monthly(_), Period::Months(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_monthly_rate(self.month, self.year)?)?
                        .checked_mul(Decimal::from_u32(t)?), // P * r * t
                    (RatePercent::Monthly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut total_interest = dec!(0);
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    total_interest =
                                        total_interest.checked_add(self.principal.checked_mul(
                                            self.rate_percent.get_daily_rate(month.0, year.0)?,
                                        )?)?;
                                }
                            }
                        }
                        Some(total_interest)
                    } // P * (r/30) * t  — loop

                    // r is daily
                    (RatePercent::Daily(_), Period::Years(t)) => {
                        let mut interest_total = dec!(0);
                        let mut current_year = self.year;
                        for _y in 0..t {
                            interest_total =
                                interest_total.checked_add(self.principal.checked_mul(
                                    self.rate_percent.get_yearly_rate(current_year)?,
                                )?)?;
                            current_year = current_year.checked_add(1)?;
                        }
                        Some(interest_total)
                    } // P * (r*365) * t --> loop
                    (RatePercent::Daily(_), Period::Months(t)) => {
                        let year_months = get_years_months(self.year, self.month, t)?;
                        let mut total_interest = dec!(0);
                        for (year, months) in year_months {
                            for month in months {
                                total_interest =
                                    total_interest.checked_add(self.principal.checked_mul(
                                        self.rate_percent.get_monthly_rate(month, year)?,
                                    )?)?;
                            }
                        }
                        Some(total_interest)
                    } // P * (r*30) * t   — loop
                    (RatePercent::Daily(_), Period::Days(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_daily_rate(self.month, self.year)?)?
                        .checked_mul(Decimal::from_u32(t)?), // P * r * t
                };

                M::new(fixed_ret?).ok()
            }

            InterestType::Compounding => {
                let compounding_ret = match (self.rate_percent, self.total_period) {
                    // r is yearly
                    (RatePercent::Yearly(_), Period::Years(t)) => {
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        let mut current_year = self.year;
                        for _y in 0..t {
                            let current_interest = current_principal
                                .checked_mul(self.rate_percent.get_yearly_rate(current_year)?)?;
                            total_interest = total_interest.checked_add(current_interest)?;
                            current_principal = self.principal.checked_add(total_interest)?;
                            current_year = current_year.checked_add(1)?;
                        }
                        Some(total_interest)
                    } // P * r * t
                    (RatePercent::Yearly(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        for year in years_months {
                            for month in year.1 {
                                let current_interest = current_principal.checked_mul(
                                    self.rate_percent.get_monthly_rate(month, year.0)?,
                                )?;
                                total_interest = total_interest.checked_add(current_interest)?;
                                current_principal = self.principal.checked_add(total_interest)?;
                            }
                        }
                        Some(total_interest)
                    } // P * (r/12) * t
                    (RatePercent::Yearly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    let current_interest = current_principal.checked_mul(
                                        self.rate_percent.get_daily_rate(month.0, year.0)?,
                                    )?;
                                    total_interest =
                                        total_interest.checked_add(current_interest)?;
                                    current_principal =
                                        self.principal.checked_add(total_interest)?;
                                }
                            }
                        }
                        Some(total_interest)
                    } // P * (r/365) * t --> loop

                    // r is monthly
                    (RatePercent::Monthly(_), Period::Years(t)) => {
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        let mut current_year = self.year;
                        for _y in 0..t {
                            let current_interest = current_principal
                                .checked_mul(self.rate_percent.get_yearly_rate(current_year)?)?;
                            total_interest = total_interest.checked_add(current_interest)?;
                            current_principal = self.principal.checked_add(total_interest)?;
                            current_year = current_year.checked_add(1)?;
                        }
                        Some(total_interest)
                    } // P * (r*12) * t
                    (RatePercent::Monthly(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        for year in years_months {
                            for month in year.1 {
                                let current_interest = current_principal.checked_mul(
                                    self.rate_percent.get_monthly_rate(month, year.0)?,
                                )?;
                                total_interest = total_interest.checked_add(current_interest)?;
                                current_principal = self.principal.checked_add(total_interest)?;
                            }
                        }
                        Some(total_interest)
                    } // P * r * t
                    (RatePercent::Monthly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    let current_interest = current_principal.checked_mul(
                                        self.rate_percent.get_daily_rate(month.0, year.0)?,
                                    )?;
                                    total_interest =
                                        total_interest.checked_add(current_interest)?;
                                    current_principal =
                                        self.principal.checked_add(total_interest)?;
                                }
                            }
                        }
                        Some(total_interest)
                    } // P * (r/30) * t  — loop

                    // r is daily
                    (RatePercent::Daily(_), Period::Years(t)) => {
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        let mut current_year = self.year;
                        for _y in 0..t {
                            let current_interest = current_principal
                                .checked_mul(self.rate_percent.get_yearly_rate(current_year)?)?;
                            total_interest = total_interest.checked_add(current_interest)?;
                            current_principal = self.principal.checked_add(total_interest)?;
                            current_year = current_year.checked_add(1)?;
                        }
                        Some(total_interest)
                    } // P * (r*365) * t --> loop
                    (RatePercent::Daily(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        for year in years_months {
                            for month in year.1 {
                                let current_interest = current_principal.checked_mul(
                                    self.rate_percent.get_monthly_rate(month, year.0)?,
                                )?;
                                total_interest = total_interest.checked_add(current_interest)?;
                                current_principal = self.principal.checked_add(total_interest)?;
                            }
                        }
                        Some(total_interest)
                    } // P * (r*30) * t   — loop
                    (RatePercent::Daily(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut total_interest = dec!(0);
                        let mut current_principal = self.principal;
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    let current_interest = current_principal.checked_mul(
                                        self.rate_percent.get_daily_rate(month.0, year.0)?,
                                    )?;
                                    total_interest =
                                        total_interest.checked_add(current_interest)?;
                                    current_principal =
                                        self.principal.checked_add(total_interest)?;
                                }
                            }
                        }
                        Some(total_interest)
                    } // P * r * t
                };

                M::new(compounding_ret?).ok()
            }
        }
    }

    /// Calculate the total of returns: Principal + Interests.
    pub fn total(&self) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    {
        M::new(self.principal.checked_add(self.returns()?.amount())?).ok()
    }
}
