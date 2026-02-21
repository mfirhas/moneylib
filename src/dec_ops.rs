use crate::Currency;

use crate::{BaseMoney, Decimal, Money};
use std::ops::{Add, Div, Mul, Sub};

// Money + Decimal = Money
impl<C> Add<Decimal> for Money<C>
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

// Money - Decimal = Money
impl<C> Sub<Decimal> for Money<C>
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

// Money * Decimal = Money
impl<C> Mul<Decimal> for Money<C>
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

// Money / Decimal = Money
impl<C> Div<Decimal> for Money<C>
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

// Decimal + Money = Money
impl<C> Add<Money<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = Money<C>;

    fn add(self, rhs: Money<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        Money::from_decimal(ret)
    }
}

// Decimal - Money = Money
impl<C> Sub<Money<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = Money<C>;

    fn sub(self, rhs: Money<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_sub(rhs.amount())
            .expect("subtraction operation overflow");

        Money::from_decimal(ret)
    }
}

// Decimal * Money = Money
impl<C> Mul<Money<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = Money<C>;

    fn mul(self, rhs: Money<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_mul(rhs.amount())
            .expect("multiplication operation overflow");

        Money::from_decimal(ret)
    }
}

// Decimal / Money = Money
impl<C> Div<Money<C>> for Decimal
where
    C: Currency + Clone,
{
    type Output = Money<C>;

    fn div(self, rhs: Money<C>) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_div(rhs.amount())
            .expect("division operation overflow");

        Money::from_decimal(ret)
    }
}
