//--------- Ops for RawMoney (without automatic rounding)

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::{BaseMoney, Currency};

use super::RawMoney;

/// RawMoney + RawMoney = RawMoney (no auto-rounding)
impl<C> Add for RawMoney<C>
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

/// RawMoney - RawMoney = RawMoney (no auto-rounding)
impl<C> Sub for RawMoney<C>
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

/// RawMoney += RawMoney (no auto-rounding)
impl<C> AddAssign for RawMoney<C>
where
    C: Currency,
{
    fn add_assign(&mut self, other: Self) {
        let ret = self
            .amount()
            .checked_add(other.amount())
            .expect("addition operation overflow");

        *self = Self::from_decimal(ret);
    }
}

/// RawMoney -= RawMoney (no auto-rounding)
impl<C> SubAssign for RawMoney<C>
where
    C: Currency,
{
    fn sub_assign(&mut self, other: Self) {
        let ret = self
            .amount()
            .checked_sub(other.amount())
            .expect("subtraction operation overflow");

        *self = Self::from_decimal(ret);
    }
}

/// Negation: -RawMoney = RawMoney
impl<C> Neg for RawMoney<C>
where
    C: Currency,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_decimal(-self.amount())
    }
}
