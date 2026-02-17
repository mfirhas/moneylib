//--------- Ops

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::Currency;

use crate::{BaseMoney, Money};

/// Money + Money = Money
impl<C> Add for Money<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        Self::new(ret).expect("failed doing addition")
    }
}

/// Money - Money = Money
impl<C> Sub for Money<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(rhs.amount())
            .expect("substraction operation overflow");

        Self::new(ret).expect("failed doing substraction")
    }
}

/// Money * Money = Money
impl<C> Mul for Money<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_mul(rhs.amount())
            .expect("multiplication operation overflow");

        Self::new(ret).expect("failed doing multiplication")
    }
}

/// Money / Money = Money
impl<C> Div for Money<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_div(rhs.amount())
            .expect("division operation overflow");

        Self::new(ret).expect("failed doing division")
    }
}

/// Money += Money
impl<C> AddAssign for Money<C>
where
    C: Currency + Clone,
{
    fn add_assign(&mut self, other: Self) {
        let ret = self
            .amount()
            .checked_add(other.amount())
            .expect("addition operation overflow");

        let ret = Money::new(ret).expect("failed add assign");

        *self = ret
    }
}

/// Money -= Money
impl<C> SubAssign for Money<C>
where
    C: Currency + Clone,
{
    fn sub_assign(&mut self, other: Self) {
        let ret = self
            .amount()
            .checked_sub(other.amount())
            .expect("subtraction operation overflow");

        let ret = Money::new(ret).expect("failed sub assign");

        *self = ret
    }
}

/// Money *= Money
impl<C> MulAssign for Money<C>
where
    C: Currency + Clone,
{
    fn mul_assign(&mut self, other: Self) {
        let ret = self
            .amount()
            .checked_mul(other.amount())
            .expect("multiplication operation overflow");

        let ret = Money::new(ret).expect("failed mul assign");

        *self = ret
    }
}

/// Money /= Money
impl<C> DivAssign for Money<C>
where
    C: Currency + Clone,
{
    fn div_assign(&mut self, other: Self) {
        let ret = self
            .amount()
            .checked_div(other.amount())
            .expect("division operation failed");

        let ret = Money::new(ret).expect("failed div assign");

        *self = ret
    }
}

impl<C> Neg for Money<C>
where
    C: Currency + Clone,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_decimal(-self.amount())
    }
}
