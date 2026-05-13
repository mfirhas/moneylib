use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock, atomic::AtomicBool},
};

use crate::{Currency, MoneyError};

use currencylib::data::{Data, entries, get as get_data};

static OBJ_MONEY_IS_RAW: AtomicBool = AtomicBool::new(false);

pub fn is_raw() -> bool {
    OBJ_MONEY_IS_RAW.load(std::sync::atomic::Ordering::Acquire)
}

pub fn set_raw(is_raw: bool) {
    OBJ_MONEY_IS_RAW.store(is_raw, std::sync::atomic::Ordering::Release);
}

pub(super) fn amount<C: crate::Currency>(amount: crate::Decimal) -> crate::Decimal {
    if super::context::is_raw() {
        amount
    } else {
        amount.round_dp(C::MINOR_UNIT.into())
    }
}

/// Global context containing runtime data for all currencies supported for runtime checking.
static CURRENCIES: LazyLock<RwLock<HashMap<&'static str, Data>>> =
    LazyLock::new(|| RwLock::new(entries().collect()));

pub fn register_currency<C: Currency>() -> Result<(), MoneyError> {
    let mut write = CURRENCIES
        .write()
        .map_err(|_| MoneyError::Other("failed getting lock to write into CURRENCIES".into()))?;

    if write.contains_key(&C::CODE) {
        return Err(MoneyError::Other(
            format!(
                "Currency with code {} already exist: {:?}",
                C::CODE,
                get_data(C::CODE)
            )
            .into(),
        ));
    }

    write.insert(&C::CODE, into_currency_data::<C>());

    Ok(())
}

pub fn is_currency_exist(code: &str) -> bool {
    if let Ok(data) = CURRENCIES.read()
        && data.contains_key(code)
    {
        return true;
    }

    false
}

pub fn set_currency<C: Currency>(code: &str) -> Result<(), MoneyError> {
    if code != C::CODE {
        return Err(MoneyError::CurrencyMismatchError(
            C::CODE.into(),
            code.into(),
        ));
    }

    let mut write = CURRENCIES
        .write()
        .map_err(|_| MoneyError::Other("failed getting lock to set into CURRENCIES".into()))?;

    write.insert(&C::CODE, into_currency_data::<C>());

    Ok(())
}

pub fn get_currency(code: &str) -> Option<super::dyn_money::DynCurrency> {
    if let Ok(data) = CURRENCIES.read() {
        return Some(super::dyn_money::DynCurrency(data.get(code).copied()?));
    }

    None
}

pub(super) fn into_currency_data<C: Currency>() -> Data {
    Data {
        code: C::CODE,
        symbol: C::SYMBOL,
        name: C::NAME,
        numeric: C::NUMERIC,
        minor_unit: C::MINOR_UNIT,
        minor_unit_symbol: C::MINOR_UNIT_SYMBOL,
        minor_unit_name: C::MINOR_UNIT_NAME,
        thousand_separator: C::THOUSAND_SEPARATOR,
        decimal_separator: C::DECIMAL_SEPARATOR,
        origin: C::ORIGIN,
        locale: C::LOCALE,
    }
}
