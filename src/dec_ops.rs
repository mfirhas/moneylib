use crate::{BaseMoney, Decimal, Money};
use std::ops::{Add, Div, Mul, Sub};

// Money + Decimal = Money
impl Add<Decimal> for Money {
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

// Money - Decimal = Money
impl Sub<Decimal> for Money {
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

// Money * Decimal = Money
impl Mul<Decimal> for Money {
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

// Money / Decimal = Money
impl Div<Decimal> for Money {
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

// Decimal + Money = Money
impl Add<Money> for Decimal {
    type Output = Money;

    fn add(self, rhs: Money) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        Money::new(rhs.currency(), ret)
    }
}

// Decimal - Money = Money
impl Sub<Money> for Decimal {
    type Output = Money;

    fn sub(self, rhs: Money) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_sub(rhs.amount())
            .expect("subtraction operation overflow");

        Money::new(rhs.currency(), ret)
    }
}

// Decimal * Money = Money
impl Mul<Money> for Decimal {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        // WARN: PANIC!
        let ret = self
            .checked_mul(rhs.amount())
            .expect("multiplication operation overflow");

        Money::new(rhs.currency(), ret)
    }
}

// Decimal / Money = Money
impl Div<Money> for Decimal {
    type Output = Money;

    fn div(self, rhs: Money) -> Self::Output {
        // WARN: PANIC!
        assert!(!rhs.is_zero(), "divisor must not be zero");

        let ret = self
            .checked_div(rhs.amount())
            .expect("division operation overflow");

        Money::new(rhs.currency(), ret)
    }
}
