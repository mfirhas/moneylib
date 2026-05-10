//--------- Ops

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::Currency;

use crate::{BaseMoney, Money};

/// Money + Money = Money
impl<C> Add for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        Self::from_decimal(ret)
    }
}

/// Money - Money = Money
impl<C> Sub for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(rhs.amount())
            .expect("subtraction operation overflow");

        Self::from_decimal(ret)
    }
}

/// Money += Money
impl<C> AddAssign for Money<C>
where
    C: Currency,
{
    fn add_assign(&mut self, other: Self) {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_add(other.amount())
            .expect("addition operation overflow");

        let ret = Self::from_decimal(ret);

        *self = ret
    }
}

/// Money -= Money
impl<C> SubAssign for Money<C>
where
    C: Currency,
{
    fn sub_assign(&mut self, other: Self) {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(other.amount())
            .expect("subtraction operation overflow");

        let ret = Self::from_decimal(ret);

        *self = ret
    }
}

impl<C> Neg for Money<C>
where
    C: Currency,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_decimal(-self.amount())
    }
}
