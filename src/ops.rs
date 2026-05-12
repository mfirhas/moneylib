//--------- Ops

use crate::Money;

/// Implements all standard arithmetic operator overloads for a money type.
///
/// Generates `Add`, `Sub`, `AddAssign`, `SubAssign`, `Neg`, `Add<Decimal>`,
/// `Sub<Decimal>`, `Mul<Decimal>`, `Div<Decimal>`, `Add<$T<C>> for Decimal`,
/// `Mul<$T<C>> for Decimal`, and `Rem<Decimal>` impls for `$T<C>` where
/// `C: Currency`.
///
/// This is an internal code-generation macro. It is exported only to allow
/// use across modules within this crate (e.g. for `RawMoney`). Do not call
/// it from external crates.
///
/// # Usage (crate-internal)
///
/// ```ignore
/// // `ignore` because invoking the macro a second time for `Money` would
/// // produce duplicate impl errors; the actual call lives in src/ops.rs.
/// impl_money_ops!(Money);
/// impl_money_ops!(RawMoney);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! impl_money_ops {
    ($T:ident) => {
        /// M + M = M
        impl<C> ::std::ops::Add for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_add($crate::BaseMoney::amount(&rhs))
                    .expect("addition operation overflow");
                <Self as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// M - M = M
        impl<C> ::std::ops::Sub for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_sub($crate::BaseMoney::amount(&rhs))
                    .expect("subtraction operation overflow");
                <Self as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// M += M
        impl<C> ::std::ops::AddAssign for $T<C>
        where
            C: $crate::Currency,
        {
            fn add_assign(&mut self, other: Self) {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(self)
                    .checked_add($crate::BaseMoney::amount(&other))
                    .expect("addition operation overflow");
                *self = <Self as $crate::BaseMoney<C>>::from_decimal(ret);
            }
        }

        /// M -= M
        impl<C> ::std::ops::SubAssign for $T<C>
        where
            C: $crate::Currency,
        {
            fn sub_assign(&mut self, other: Self) {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(self)
                    .checked_sub($crate::BaseMoney::amount(&other))
                    .expect("subtraction operation overflow");
                *self = <Self as $crate::BaseMoney<C>>::from_decimal(ret);
            }
        }

        /// -M = M
        impl<C> ::std::ops::Neg for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                <Self as $crate::BaseMoney<C>>::from_decimal(-$crate::BaseMoney::amount(&self))
            }
        }

        /// M + d = M
        impl<C> ::std::ops::Add<$crate::Decimal> for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn add(self, rhs: $crate::Decimal) -> Self::Output {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_add(rhs)
                    .expect("addition operation overflow");
                <Self as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// M - d = M
        impl<C> ::std::ops::Sub<$crate::Decimal> for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn sub(self, rhs: $crate::Decimal) -> Self::Output {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_sub(rhs)
                    .expect("subtraction operation overflow");
                <Self as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// M * d = M
        impl<C> ::std::ops::Mul<$crate::Decimal> for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn mul(self, rhs: $crate::Decimal) -> Self::Output {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_mul(rhs)
                    .expect("multiplication operation overflow");
                <Self as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// M / d = M
        impl<C> ::std::ops::Div<$crate::Decimal> for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = Self;

            fn div(self, rhs: $crate::Decimal) -> Self::Output {
                // WARN: PANIC!
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_div(rhs)
                    .expect("division operation overflow");
                <Self as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// d + M = M
        impl<C> ::std::ops::Add<$T<C>> for $crate::Decimal
        where
            C: $crate::Currency,
        {
            type Output = $T<C>;

            fn add(self, rhs: $T<C>) -> Self::Output {
                // WARN: PANIC!
                let ret = self
                    .checked_add($crate::BaseMoney::amount(&rhs))
                    .expect("addition operation overflow");
                <$T<C> as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// d * M = M
        impl<C> ::std::ops::Mul<$T<C>> for $crate::Decimal
        where
            C: $crate::Currency,
        {
            type Output = $T<C>;

            fn mul(self, rhs: $T<C>) -> Self::Output {
                // WARN: PANIC!
                let ret = self
                    .checked_mul($crate::BaseMoney::amount(&rhs))
                    .expect("multiplication operation overflow");
                <$T<C> as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }

        /// M % d = M
        impl<C> ::std::ops::Rem<$crate::Decimal> for $T<C>
        where
            C: $crate::Currency,
        {
            type Output = $T<C>;

            fn rem(self, rhs: $crate::Decimal) -> Self::Output {
                let ret = $crate::BaseMoney::amount(&self)
                    .checked_rem(rhs)
                    .expect("remainder operation failed");
                <$T<C> as $crate::BaseMoney<C>>::from_decimal(ret)
            }
        }
    };
}

impl_money_ops!(Money);
