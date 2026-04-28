mod fmt;

use crate::fmt::{CODE_FORMAT, CODE_FORMAT_MINOR, SYMBOL_FORMAT, SYMBOL_FORMAT_MINOR};
use fmt::format_obj_money;

use crate::macros::dec;
use crate::{Decimal, MoneyError};
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::ToPrimitive;

/// Object-safe trait enabling dynamic dispatch (`dyn`) over different-currency money types.
///
/// This trait exposes the read-only subset of [`crate::BaseMoney`] needed for heterogeneous
/// collections (e.g. `Vec<Box<dyn ObjMoney>>`) where the currency type `C` is erased at runtime.
///
/// # Why not `BaseMoney<C>` directly?
///
/// `BaseMoney<C>` cannot be used as a trait object for three reasons:
/// - It has a generic type parameter `C`, so `dyn BaseMoney<USD>` and `dyn BaseMoney<EUR>` are
///   different types and cannot be stored in the same collection.
/// - It has `Sized` as a supertrait, which makes it non-object-safe.
/// - Several methods return `Self` or take `impl Trait` arguments, both of which are
///   object-safety violations.
///
/// `ObjMoney` solves all three: no type parameter, no `Sized`/`Clone` supertraits, and every
/// method uses only concrete types (`Decimal`, `&str`, `String`, `bool`, etc.).
///
/// # Required methods
///
/// Implementors must provide the eight primitive accessors. All other methods have
/// default implementations derived from those primitives.
///
/// # Examples
///
/// ```
/// use moneylib::{Money, ObjMoney, Decimal, BaseMoney, macros::dec, iso::{USD, EUR, JPY}};
///
/// let portfolio: Vec<Box<dyn ObjMoney>> = vec![
///     Box::new(Money::<USD>::new(dec!(100.50)).unwrap()),
///     Box::new(Money::<EUR>::new(dec!(200.75)).unwrap()),
///     Box::new(Money::<JPY>::new(dec!(15000)).unwrap()),
/// ];
///
/// let codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
/// assert_eq!(codes, vec!["USD", "EUR", "JPY"]);
///
/// let total: Decimal = portfolio.iter().fold(Decimal::ZERO, |acc, m| acc + m.amount());
/// assert_eq!(total, dec!(15301.25));
/// ```
pub trait ObjMoney {
    // ---- Required: eight primitive accessors ----

    /// Returns the decimal amount of this money value.
    fn amount(&self) -> Decimal;

    /// Returns the ISO 4217 currency code (e.g. `"USD"`).
    fn code(&self) -> &str;

    /// Returns the currency symbol (e.g. `"$"`).
    fn symbol(&self) -> &str;

    /// Returns the full name of the currency (e.g. `"United States dollar"`).
    fn name(&self) -> &str;

    /// Returns the number of decimal places in the currency's minor unit (e.g. `2` for USD).
    fn minor_unit(&self) -> u16;

    /// Returns the thousands separator used by the currency's locale (e.g. `","` for USD).
    fn thousand_separator(&self) -> &str;

    /// Returns the decimal separator used by the currency's locale (e.g. `"."` for USD).
    fn decimal_separator(&self) -> &str;

    /// Returns the minor-unit symbol (e.g. `"¢"` for USD, `"minor"` when none is defined).
    fn minor_unit_symbol(&self) -> &str;

    // ---- Provided: derived from the required methods above ----

    /// Returns the money amount in its smallest unit (e.g. cents for USD, pence for GBP).
    ///
    /// # Errors
    ///
    /// Returns [`MoneyError::OverflowError`] if the computation overflows.
    #[inline]
    fn minor_amount(&self) -> Result<i128, MoneyError> {
        self.amount()
            .checked_mul(
                dec!(10)
                    .checked_powu(self.minor_unit().into())
                    .ok_or(MoneyError::OverflowError)?,
            )
            .ok_or(MoneyError::OverflowError)?
            .to_i128()
            .ok_or(MoneyError::OverflowError)
    }

    /// Returns `true` if the amount is zero.
    #[inline]
    fn is_zero(&self) -> bool {
        self.amount().is_zero()
    }

    /// Returns `true` if the amount is positive (or zero).
    #[inline]
    fn is_positive(&self) -> bool {
        self.amount().is_sign_positive()
    }

    /// Returns `true` if the amount is negative.
    #[inline]
    fn is_negative(&self) -> bool {
        self.amount().is_sign_negative()
    }

    /// Returns the scale (number of decimal places) of the stored amount.
    #[inline]
    fn scale(&self) -> u32 {
        self.amount().scale()
    }

    /// Returns the fractional part of the amount.
    #[inline]
    fn fraction(&self) -> Decimal {
        self.amount().fract()
    }

    /// Returns the mantissa (significand digits) of the amount.
    #[inline]
    fn mantissa(&self) -> i128 {
        self.amount().mantissa()
    }

    /// Formats money with currency code and locale separators (e.g. `"USD 1,234.56"`).
    fn format_code(&self) -> String {
        format_obj_money(
            self.amount(),
            self.code(),
            self.symbol(),
            self.minor_unit_symbol(),
            self.minor_unit(),
            self.thousand_separator(),
            self.decimal_separator(),
            CODE_FORMAT,
        )
    }

    /// Formats money with currency symbol and locale separators (e.g. `"$1,234.56"`).
    fn format_symbol(&self) -> String {
        format_obj_money(
            self.amount(),
            self.code(),
            self.symbol(),
            self.minor_unit_symbol(),
            self.minor_unit(),
            self.thousand_separator(),
            self.decimal_separator(),
            SYMBOL_FORMAT,
        )
    }

    /// Formats money with currency code in the smallest unit (e.g. `"USD 123,456 ¢"`).
    fn format_code_minor(&self) -> String {
        format_obj_money(
            self.amount(),
            self.code(),
            self.symbol(),
            self.minor_unit_symbol(),
            self.minor_unit(),
            self.thousand_separator(),
            self.decimal_separator(),
            CODE_FORMAT_MINOR,
        )
    }

    /// Formats money with currency symbol in the smallest unit (e.g. `"$123,456 ¢"`).
    fn format_symbol_minor(&self) -> String {
        format_obj_money(
            self.amount(),
            self.code(),
            self.symbol(),
            self.minor_unit_symbol(),
            self.minor_unit(),
            self.thousand_separator(),
            self.decimal_separator(),
            SYMBOL_FORMAT_MINOR,
        )
    }
}

// ---- Implementations for Money and RawMoney ----

mod money_impl;

#[cfg(feature = "raw_money")]
mod raw_money_impl;

#[cfg(test)]
mod obj_money_test;
