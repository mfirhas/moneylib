//--------- Ops for RawMoney (without automatic rounding)

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{BaseMoney, RawMoney};

/// RawMoney + RawMoney = RawMoney (no auto-rounding)
impl Add for RawMoney {
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

        Self::new(self.currency(), ret)
    }
}

/// RawMoney - RawMoney = RawMoney (no auto-rounding)
impl Sub for RawMoney {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            rhs.currency(),
            "currency mismatch for subtraction operation"
        );

        // WARN: PANIC!
        let ret = self
            .amount()
            .checked_sub(rhs.amount())
            .expect("subtraction operation overflow");

        Self::new(self.currency(), ret)
    }
}

/// RawMoney * RawMoney = RawMoney (no auto-rounding)
impl Mul for RawMoney {
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

        Self::new(self.currency(), ret)
    }
}

/// RawMoney / RawMoney = RawMoney (no auto-rounding)
impl Div for RawMoney {
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

        Self::new(self.currency(), ret)
    }
}

/// RawMoney += RawMoney (no auto-rounding)
impl AddAssign for RawMoney {
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

        let ret = RawMoney::new(self.currency(), ret);

        *self = ret
    }
}

/// RawMoney -= RawMoney (no auto-rounding)
impl SubAssign for RawMoney {
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

        let ret = RawMoney::new(self.currency(), ret);

        *self = ret
    }
}

/// RawMoney *= RawMoney (no auto-rounding)
impl MulAssign for RawMoney {
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

        let ret = RawMoney::new(self.currency(), ret);

        *self = ret
    }
}

/// RawMoney /= RawMoney (no auto-rounding)
impl DivAssign for RawMoney {
    fn div_assign(&mut self, other: Self) {
        // WARN: PANIC!
        assert_eq!(
            self.currency(),
            other.currency(),
            "currency mismatch for div assign operation"
        );

        assert!(!other.is_zero(), "divisor must not be zero");

        let ret = self
            .amount()
            .checked_div(other.amount())
            .expect("division operation overflow");

        let ret = RawMoney::new(self.currency(), ret);

        *self = ret
    }
}

/// Negation: -RawMoney = RawMoney
impl Neg for RawMoney {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.currency(), -self.amount())
    }
}
