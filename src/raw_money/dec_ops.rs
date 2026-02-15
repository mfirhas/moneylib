use crate::{BaseMoney, Decimal, RawMoney};
use std::ops::{Add, Div, Mul, Sub};

// RawMoney + Decimal = RawMoney (no auto-rounding)
impl Add<Decimal> for RawMoney {
    type Output = Self;

    fn add(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_add(rhs)
            .expect("addition operation overflow");

        Self::new(self.currency(), ret)
    }
}

// RawMoney - Decimal = RawMoney (no auto-rounding)
impl Sub<Decimal> for RawMoney {
    type Output = Self;

    fn sub(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(rhs)
            .expect("subtraction operation overflow");

        Self::new(self.currency(), ret)
    }
}

// RawMoney * Decimal = RawMoney (no auto-rounding)
impl Mul<Decimal> for RawMoney {
    type Output = Self;

    fn mul(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_mul(rhs)
            .expect("multiplication operation overflow");

        Self::new(self.currency(), ret)
    }
}

// RawMoney / Decimal = RawMoney (no auto-rounding)
impl Div<Decimal> for RawMoney {
    type Output = Self;

    fn div(self, rhs: Decimal) -> Self::Output {
        // WARN: PANIC!
        assert!(!rhs.is_zero(), "divisor must not be zero");

        let ret = self
            .amount()
            .checked_div(rhs)
            .expect("division operation overflow");

        Self::new(self.currency(), ret)
    }
}

// Decimal + RawMoney = RawMoney (no auto-rounding)
impl Add<RawMoney> for Decimal {
    type Output = RawMoney;

    fn add(self, rhs: RawMoney) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        RawMoney::new(rhs.currency(), ret)
    }
}

// Decimal - RawMoney = RawMoney (no auto-rounding)
impl Sub<RawMoney> for Decimal {
    type Output = RawMoney;

    fn sub(self, rhs: RawMoney) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_sub(rhs.amount())
            .expect("subtraction operation overflow");

        RawMoney::new(rhs.currency(), ret)
    }
}

// Decimal * RawMoney = RawMoney (no auto-rounding)
impl Mul<RawMoney> for Decimal {
    type Output = RawMoney;

    fn mul(self, rhs: RawMoney) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_mul(rhs.amount())
            .expect("multiplication operation overflow");

        RawMoney::new(rhs.currency(), ret)
    }
}

// Decimal / RawMoney = RawMoney (no auto-rounding)
impl Div<RawMoney> for Decimal {
    type Output = RawMoney;

    fn div(self, rhs: RawMoney) -> Self::Output {
        // WARN: PANIC!
        assert!(!rhs.is_zero(), "divisor must not be zero");

        let ret = self
            .checked_div(rhs.amount())
            .expect("division operation overflow");

        RawMoney::new(rhs.currency(), ret)
    }
}
