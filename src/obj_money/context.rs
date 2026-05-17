use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock, atomic::AtomicBool},
};

use crate::{Currency, MoneyError};

use currencylib::data::{entries, get as get_data};

static IS_RAW: AtomicBool = AtomicBool::new(false);

/// Global context containing runtime data for all currencies supported for runtime checking.
static CURRENCIES: LazyLock<RwLock<HashMap<&'static str, super::DynCurrency>>> =
    LazyLock::new(|| {
        RwLock::new(
            entries()
                .map(|(k, v)| {
                    (
                        k,
                        super::DynCurrency {
                            code: v.code,
                            symbol: v.symbol,
                            name: v.name,
                            numeric: v.numeric,
                            minor_unit: v.minor_unit,
                            minor_unit_symbol: v.minor_unit_symbol,
                            minor_unit_name: v.minor_unit_name,
                            thousand_separator: v.thousand_separator,
                            decimal_separator: v.decimal_separator,
                            origin: v.origin,
                            locale: v.locale,
                        },
                    )
                })
                .collect(),
        )
    });

pub struct Context;

impl Context {
    pub fn is_raw() -> bool {
        IS_RAW.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn set_raw(is_raw: bool) {
        IS_RAW.store(is_raw, std::sync::atomic::Ordering::Release);
    }

    pub fn register_currency<C: Currency>() -> Result<(), MoneyError> {
        let mut write = CURRENCIES.write().map_err(|_| {
            MoneyError::Other("failed getting lock to write into CURRENCIES".into())
        })?;

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

        write.insert(C::CODE, super::helpers::dyn_curr_from::<C>());

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

        write.insert(C::CODE, super::helpers::dyn_curr_from::<C>());

        Ok(())
    }

    pub fn get_currency(code: &str) -> Option<super::dyn_money::DynCurrency> {
        if let Ok(data) = CURRENCIES.read() {
            return data.get(code).copied();
        }

        None
    }

    pub fn get_currency_by_symbol(symbol: &str) -> Option<super::dyn_money::DynCurrency> {
        if let Ok(data) = CURRENCIES.read() {
            return data
                .iter()
                .find(|(_, curr)| curr.symbol == symbol)
                .map(|(_, &v)| v);
        }

        None
    }
}
