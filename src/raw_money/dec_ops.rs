use crate::{BaseMoney, Currency, Decimal};
use std::ops::{Add, Div, Mul, Sub};

use super::RawMoney;

// RawMoney + Decimal = RawMoney (no auto-rounding)
impl<C> Add<Decimal> for RawMoney<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn add(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_add(rhs)
            .expect("addition operation overflow");

        Self::from_decimal(ret)
    }
}

// RawMoney - Decimal = RawMoney (no auto-rounding)
impl<C> Sub<Decimal> for RawMoney<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn sub(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(rhs)
            .expect("subtraction operation overflow");

        Self::from_decimal(ret)
    }
}

// RawMoney * Decimal = RawMoney (no auto-rounding)
impl<C> Mul<Decimal> for RawMoney<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn mul(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_mul(rhs)
            .expect("multiplication operation overflow");

        Self::from_decimal(ret)
    }
}

// RawMoney / Decimal = RawMoney (no auto-rounding)
impl<C> Div<Decimal> for RawMoney<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn div(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_div(rhs)
            .expect("division operation overflow");

        Self::from_decimal(ret)
    }
}

// Decimal + RawMoney = RawMoney (no auto-rounding)
impl<C> Add<RawMoney<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = RawMoney<C>;

    fn add(self, rhs: RawMoney<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        RawMoney::from_decimal(ret)
    }
}

// Decimal - RawMoney = RawMoney (no auto-rounding)
impl<C> Sub<RawMoney<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = RawMoney<C>;

    fn sub(self, rhs: RawMoney<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_sub(rhs.amount())
            .expect("subtraction operation overflow");

        RawMoney::from_decimal(ret)
    }
}

// Decimal * RawMoney = RawMoney (no auto-rounding)
impl<C> Mul<RawMoney<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = RawMoney<C>;

    fn mul(self, rhs: RawMoney<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_mul(rhs.amount())
            .expect("multiplication operation overflow");

        RawMoney::from_decimal(ret)
    }
}

// Decimal / RawMoney = RawMoney (no auto-rounding)
impl<C> Div<RawMoney<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = RawMoney<C>;

    fn div(self, rhs: RawMoney<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_div(rhs.amount())
            .expect("division operation overflow");

        RawMoney::from_decimal(ret)
    }
}
