use crate::raw_money::RawMoney;
use crate::{BaseMoney, Currency, Decimal};

impl<C: Currency> super::ObjMoney for RawMoney<C> {
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
    fn name(&self) -> &str {
        C::NAME
    }
    #[inline]
    fn minor_unit(&self) -> u16 {
        C::MINOR_UNIT
    }
    #[inline]
    fn thousand_separator(&self) -> &str {
        C::THOUSAND_SEPARATOR
    }
    #[inline]
    fn decimal_separator(&self) -> &str {
        C::DECIMAL_SEPARATOR
    }
    #[inline]
    fn minor_unit_symbol(&self) -> &str {
        C::MINOR_UNIT_SYMBOL
    }

    #[inline]
    fn minor_amount(&self) -> Result<i128, crate::MoneyError> {
        BaseMoney::minor_amount(self)
    }
}
