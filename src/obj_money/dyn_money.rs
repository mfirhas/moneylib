use super::obj_money::ObjMoney;
use crate::{BaseMoney, Currency, Decimal, Money};

#[cfg(feature = "raw_money")]
use crate::RawMoney;

/// Trait for mixed money types.
pub trait DynMoney {
    /// Money's amount.
    fn amount(&self) -> Decimal;

    /// Currency code.
    fn code(&self) -> &str;

    /// Currency symbol.
    fn symbol(&self) -> &str;

    /// Currency minor unit.
    fn minor_unit(&self) -> u16;
}

impl<const IS_RAW: bool> DynMoney for ObjMoney<IS_RAW> {
    #[inline]
    fn amount(&self) -> Decimal {
        self.amount()
    }

    #[inline]
    fn code(&self) -> &str {
        self.code()
    }

    #[inline]
    fn symbol(&self) -> &str {
        self.symbol()
    }

    #[inline]
    fn minor_unit(&self) -> u16 {
        self.minor_unit()
    }
}

impl<C: Currency> DynMoney for Money<C> {
    #[inline]
    fn amount(&self) -> Decimal {
        BaseMoney::amount(self)
    }

    #[inline]
    fn code(&self) -> &str {
        C::CODE
    }

    #[inline]
    fn symbol(&self) -> &str {
        C::SYMBOL
    }

    #[inline]
    fn minor_unit(&self) -> u16 {
        C::MINOR_UNIT
    }
}

#[cfg(feature = "raw_money")]
impl<C: Currency> DynMoney for RawMoney<C> {
    #[inline]
    fn amount(&self) -> Decimal {
        BaseMoney::amount(self)
    }

    #[inline]
    fn code(&self) -> &str {
        C::CODE
    }

    #[inline]
    fn symbol(&self) -> &str {
        C::SYMBOL
    }

    #[inline]
    fn minor_unit(&self) -> u16 {
        C::MINOR_UNIT
    }
}
