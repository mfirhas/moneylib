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
    /// # Formula
    /// FV = PV * (1 + (r * t))
    /// PV = FV / (1 + (r * t))
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
    fn interest_fixed<R>(&self, rate: R) -> Option<Self::InterestBuilder>
    where
        R: DecimalNumber;

    /// Calculate compounding-interest on loan.
    ///
    /// # Formula
    /// FV = PV * (1 + r)^t
    /// PV = FV / (1 + r)^t
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
            rate_days: Default::default(),
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
            rate_days: Default::default(),
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

    /// rate days for calculating interest rate
    rate_days: RateDays,

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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
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
            rate_days: self.rate_days,
            year: self.year,
            month: self.month,
            day,
            _output: PhantomData,
            _currency: PhantomData,
        }
    }

    pub const fn rate_days(self, rate_days: RateDays) -> Self {
        Self {
            principal: self.principal,
            rate_percent: self.rate_percent,
            total_period: self.total_period,
            interest_type: self.interest_type,
            rate_days,
            year: self.year,
            month: self.month,
            day: self.day,
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
                        .checked_mul(
                            self.rate_percent
                                .get_yearly_rate(self.rate_days, self.year)?,
                        )?
                        .checked_mul(Decimal::from_u32(t)?), // P * r * t
                    (RatePercent::Yearly(_), Period::Months(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_monthly_rate(
                            self.rate_days,
                            self.year,
                            self.month,
                        )?)?
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
                                            self.rate_percent.get_daily_rate(
                                                self.rate_days,
                                                year.0,
                                                month.0,
                                            )?,
                                        )?)?;
                                }
                            }
                        }
                        Some(interest_total)
                    } // P * (r/365) * t --> loop

                    // r is monthly
                    (RatePercent::Monthly(_), Period::Years(t)) => self
                        .principal
                        .checked_mul(
                            self.rate_percent
                                .get_yearly_rate(self.rate_days, self.year)?,
                        )?
                        .checked_mul(Decimal::from_u32(t)?), // P * (r*12) * t
                    (RatePercent::Monthly(_), Period::Months(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_monthly_rate(
                            self.rate_days,
                            self.year,
                            self.month,
                        )?)?
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
                                            self.rate_percent.get_daily_rate(
                                                self.rate_days,
                                                year.0,
                                                month.0,
                                            )?,
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
                            interest_total = interest_total.checked_add(
                                self.principal.checked_mul(
                                    self.rate_percent
                                        .get_yearly_rate(self.rate_days, current_year)?,
                                )?,
                            )?;
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
                                        self.rate_percent.get_monthly_rate(
                                            self.rate_days,
                                            year,
                                            month,
                                        )?,
                                    )?)?;
                            }
                        }
                        Some(total_interest)
                    } // P * (r*30) * t   — loop
                    (RatePercent::Daily(_), Period::Days(t)) => self
                        .principal
                        .checked_mul(self.rate_percent.get_daily_rate(
                            self.rate_days,
                            self.year,
                            self.month,
                        )?)?
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
                            let current_interest = current_principal.checked_mul(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
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
                                    self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?,
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
                                        self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?,
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
                            let current_interest = current_principal.checked_mul(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
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
                                    self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?,
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
                                        self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?,
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
                            let current_interest = current_principal.checked_mul(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
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
                                    self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?,
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
                                        self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?,
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

    /// Calculate the future value of a money: Principal + Interests.
    pub fn future_value(&self) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    {
        M::new(self.principal.checked_add(self.returns()?.amount())?).ok()
    }

    /// Calculate the present value of future goal.
    pub fn present_value(&self) -> Option<M>
    where
        M: BaseMoney<C> + BaseOps<C> + Amount<C>,
    {
        match self.interest_type {
            // PV = FV / (1 + (r * t))
            InterestType::Fixed => {
                let fixed_ret = match (self.rate_percent, self.total_period) {
                    (RatePercent::Yearly(_), Period::Years(t)) => {
                        let mut actual_r = dec!(0);
                        let mut current_year = self.year;
                        for _y in 0..t {
                            actual_r = actual_r.checked_add(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
                            current_year = current_year.checked_add(1)?;
                        }
                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Monthly(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut actual_r = dec!(0);
                        for year in years_months {
                            for month in year.1 {
                                actual_r =
                                    actual_r.checked_add(self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?)?;
                            }
                        }
                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Daily(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut actual_r = dec!(0);
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    actual_r =
                                        actual_r.checked_add(self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?)?;
                                }
                            }
                        }
                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }

                    (RatePercent::Yearly(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut actual_r = dec!(0);
                        for year in years_months {
                            for month in year.1 {
                                actual_r =
                                    actual_r.checked_add(self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?)?;
                            }
                        }
                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Yearly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;

                        let actual_r = {
                            let mut total_r = dec!(0);
                            for year in years_months_days {
                                for month in year.1 {
                                    for _day in month.1 {
                                        total_r = total_r.checked_add(
                                            self.rate_percent.get_daily_rate(
                                                self.rate_days,
                                                year.0,
                                                month.0,
                                            )?,
                                        )?;
                                    }
                                }
                            }
                            total_r
                        };

                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }

                    (RatePercent::Monthly(_), Period::Years(t)) => {
                        let mut actual_r = dec!(0);
                        let mut current_year = self.year;
                        for _y in 0..t {
                            actual_r = actual_r.checked_add(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
                            current_year = current_year.checked_add(1)?;
                        }
                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Monthly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;

                        let actual_r = {
                            let mut total_r = dec!(0);
                            for year in years_months_days {
                                for month in year.1 {
                                    for _day in month.1 {
                                        total_r = total_r.checked_add(
                                            self.rate_percent.get_daily_rate(
                                                self.rate_days,
                                                year.0,
                                                month.0,
                                            )?,
                                        )?;
                                    }
                                }
                            }
                            total_r
                        };

                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }

                    (RatePercent::Daily(_), Period::Years(t)) => {
                        let mut current_year = self.year;
                        let mut actual_r = dec!(0);
                        for _y in 0..t {
                            actual_r = actual_r.checked_add(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
                            current_year = current_year.checked_add(1)?;
                        }

                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Daily(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;

                        let mut actual_r = dec!(0);
                        for year in years_months {
                            for month in year.1 {
                                actual_r =
                                    actual_r.checked_add(self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?)?;
                            }
                        }

                        let divisor = dec!(1).checked_add(actual_r)?;

                        self.principal.checked_div(divisor)
                    }
                };

                M::new(fixed_ret?).ok()
            }

            // PV = FV / (1 + r)^t
            InterestType::Compounding => {
                let compound_ret = match (self.rate_percent, self.total_period) {
                    (RatePercent::Yearly(_), Period::Years(t)) => {
                        let mut current_year = self.year;
                        let mut divisor = dec!(1);
                        for _y in 0..t {
                            let d = dec!(1).checked_add(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
                            divisor = divisor.checked_mul(d)?;

                            current_year = current_year.checked_add(1)?;
                        }

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Monthly(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut divisor = dec!(1);
                        for year in years_months {
                            for month in year.1 {
                                let d =
                                    dec!(1).checked_add(self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?)?;
                                divisor = divisor.checked_mul(d)?;
                            }
                        }

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Daily(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut divisor = dec!(1);
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    let d =
                                        dec!(1).checked_add(self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?)?;
                                    divisor = divisor.checked_mul(d)?;
                                }
                            }
                        }

                        self.principal.checked_div(divisor)
                    }

                    (RatePercent::Yearly(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;
                        let mut divisor = dec!(1);
                        for year in years_months {
                            for month in year.1 {
                                let d =
                                    dec!(1).checked_add(self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?)?;
                                divisor = divisor.checked_mul(d)?;
                            }
                        }

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Yearly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut divisor = dec!(1);
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    let d =
                                        dec!(1).checked_add(self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?)?;
                                    divisor = divisor.checked_mul(d)?;
                                }
                            }
                        }

                        self.principal.checked_div(divisor)
                    }

                    (RatePercent::Monthly(_), Period::Years(t)) => {
                        let mut current_year = self.year;
                        let mut divisor = dec!(1);
                        for _y in 0..t {
                            let d = dec!(1).checked_add(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
                            divisor = divisor.checked_mul(d)?;

                            current_year = current_year.checked_add(1)?;
                        }

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Monthly(_), Period::Days(t)) => {
                        let years_months_days =
                            get_years_months_days(self.year, self.month, self.day, t)?;
                        let mut divisor = dec!(1);
                        for year in years_months_days {
                            for month in year.1 {
                                for _day in month.1 {
                                    let d =
                                        dec!(1).checked_add(self.rate_percent.get_daily_rate(
                                            self.rate_days,
                                            year.0,
                                            month.0,
                                        )?)?;
                                    divisor = divisor.checked_mul(d)?;
                                }
                            }
                        }

                        self.principal.checked_div(divisor)
                    }

                    (RatePercent::Daily(_), Period::Years(t)) => {
                        let mut current_year = self.year;
                        let mut divisor = dec!(1);
                        for _y in 0..t {
                            let d = dec!(1).checked_add(
                                self.rate_percent
                                    .get_yearly_rate(self.rate_days, current_year)?,
                            )?;
                            divisor = divisor.checked_mul(d)?;
                            current_year = current_year.checked_add(1)?;
                        }

                        self.principal.checked_div(divisor)
                    }
                    (RatePercent::Daily(_), Period::Months(t)) => {
                        let years_months = get_years_months(self.year, self.month, t)?;

                        let mut divisor = dec!(1);
                        for year in years_months {
                            for month in year.1 {
                                let d =
                                    dec!(1).checked_add(self.rate_percent.get_monthly_rate(
                                        self.rate_days,
                                        year.0,
                                        month,
                                    )?)?;
                                divisor = divisor.checked_mul(d)?;
                            }
                        }

                        self.principal.checked_div(divisor)
                    }
                };

                M::new(compound_ret?).ok()
            }
        }
    }

    /// Calculate amortized payment(PMT)
    /// PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
    /// where r is the period rate and n is the total number of periods.
    /// PMT is amortized against fixed-rate loan.
    pub fn payment(&self) -> Option<M> {
        match self.total_period {
            Period::Years(t) => {
                // c accumulates (1+r)ⁿ; r_period holds the last period rate.
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let mut current_year = self.year;
                for _ in 0..t {
                    let r = self
                        .rate_percent
                        .get_yearly_rate(self.rate_days, current_year)?;
                    r_period = r;
                    // (1+r)ⁿ
                    c = c.checked_mul(dec!(1).checked_add(r)?)?;
                    current_year = current_year.checked_add(1)?;
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                let ret = self.principal.checked_mul(d)?;

                M::new(ret).ok()
            }
            Period::Months(t) => {
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let years_months = get_years_months(self.year, self.month, t)?;
                for year in years_months {
                    for month in year.1.iter() {
                        let r =
                            self.rate_percent
                                .get_monthly_rate(self.rate_days, year.0, *month)?;
                        r_period = r;
                        // (1+r)ⁿ
                        c = c.checked_mul(dec!(1).checked_add(r)?)?;
                    }
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                let ret = self.principal.checked_mul(d)?;

                M::new(ret).ok()
            }
            Period::Days(t) => {
                let mut c = dec!(1);
                let mut r_period = dec!(0);
                let years_months_days = get_years_months_days(self.year, self.month, self.day, t)?;

                for year in years_months_days {
                    for month in year.1 {
                        for _day in month.1.iter() {
                            let r = self.rate_percent.get_daily_rate(
                                self.rate_days,
                                year.0,
                                month.0,
                            )?;
                            r_period = r;
                            // (1+r)ⁿ
                            c = c.checked_mul(dec!(1).checked_add(r)?)?;
                        }
                    }
                }

                // PMT = P × r × (1+r)ⁿ / [(1+r)ⁿ − 1]
                let c_minus_1 = c.checked_sub(dec!(1))?;
                let d = r_period.checked_mul(c)?.checked_div(c_minus_1)?;
                let ret = self.principal.checked_mul(d)?;

                M::new(ret).ok()
            }
        }
    }
}
