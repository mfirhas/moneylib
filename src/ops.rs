//--------- Ops

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{BaseMoney, Money};

/// Money + Money = Money
impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            rhs.currency(),
            "currency mismatch for addition operation"
        );

        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_add(rhs.amount())
            .expect("addition operation overflow");

        let ret = Self::new(self.currency(), ret).round();

        ret
    }
}

/// Money - Money = Money
impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            rhs.currency(),
            "currency mismatch for substraction operation"
        );

        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(rhs.amount())
            .expect("substraction operation overflow");

        let ret = Self::new(self.currency(), ret).round();

        ret
    }
}

/// Money * Money = Money
impl Mul for Money {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            rhs.currency(),
            "currency mismatch for multiplication operation"
        );

        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_mul(rhs.amount())
            .expect("multiplication operation overflow");

        let ret = Self::new(self.currency(), ret).round();

        ret
    }
}

/// Money / Money = Money
impl Div for Money {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            rhs.currency(),
            "currency mismatch for division operation"
        );

        assert!(!rhs.is_zero(), "divisor must not be zero");

        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_div(rhs.amount())
            .expect("division operation overflow");

        let ret = Self::new(self.currency(), ret).round();

        ret
    }
}

/// Money += Money
impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            other.currency(),
            "currency mismatch for add assign operation"
        );

        let ret = self
            .amount()
            .checked_add(other.amount())
            .expect("addition operation overflow");

        let ret = Money::new(self.currency(), ret).round();

        *self = ret
    }
}

/// Money -= Money
impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            other.currency(),
            "currency mismatch for sub assign operation"
        );

        let ret = self
            .amount()
            .checked_sub(other.amount())
            .expect("subtraction operation overflow");

        let ret = Money::new(self.currency(), ret).round();

        *self = ret
    }
}

/// Money *= Money
impl MulAssign for Money {
    fn mul_assign(&mut self, other: Self) {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            other.currency(),
            "currency mismatch for mul assign operation"
        );

        let ret = self
            .amount()
            .checked_mul(other.amount())
            .expect("multiplication operation overflow");

        let ret = Money::new(self.currency(), ret).round();

        *self = ret
    }
}

/// Money /= Money
impl DivAssign for Money {
    fn div_assign(&mut self, other: Self) {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            other.currency(),
            "currency mismatch for div assign operation"
        );

        // WARN: PANIC!
        assert!(!other.is_zero(), "divisor must not be zero");

        let ret = self
            .amount()
            .checked_div(other.amount())
            .expect("division operation failed");

        let ret = Money::new(self.currency(), ret).round();

        *self = ret
    }
}

impl Neg for Money {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.currency(), -self.amount())
    }
}
