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

/// Global runtime context for `obj_money`.
///
/// `Context` manages two pieces of process-wide state:
///
/// 1. **Raw mode** (`IS_RAW` flag) — when `true`, [`DynMoney`](super::DynMoney) constructors skip
///    rounding so amounts are stored with full precision (mirrors [`RawMoney`](crate::RawMoney)).
/// 2. **Currency registry** (`CURRENCIES` map) — a `HashMap` pre-seeded with every currency from
///    [`currencylib`](currencylib). Custom currencies can be added via
///    [`register_currency`](Self::register_currency).
///
/// All methods are `pub` and operate on the static singletons, so no instance is needed.
pub struct Context;

impl Context {
    /// Returns `true` when raw (unrounded) mode is active.
    ///
    /// When raw mode is enabled, [`DynMoney`](super::DynMoney) constructors preserve the full
    /// decimal precision of the supplied amount without rounding to the currency's `minor_unit`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::Context;
    ///
    /// // Default state is not raw
    /// assert!(!Context::is_raw());
    /// ```
    pub fn is_raw() -> bool {
        IS_RAW.load(std::sync::atomic::Ordering::Acquire)
    }

    /// Enables or disables raw (unrounded) mode.
    ///
    /// Setting this to `true` causes subsequent [`DynMoney`](super::DynMoney) constructions to
    /// skip rounding to the currency's `minor_unit`. Setting it back to `false` restores normal
    /// rounding behaviour.
    ///
    /// **Note:** `IS_RAW` is a process-wide [`AtomicBool`](std::sync::atomic::AtomicBool).
    /// If tests mutate this flag they must run serially to prevent race conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::{Context, DynMoney, ObjMoney};
    /// use moneylib::{macros::dec, iso::USD};
    ///
    /// Context::set_raw(true);
    /// let m = DynMoney::from_decimal::<USD>(dec!(1.23456));
    /// assert_eq!(m.amount(), dec!(1.23456)); // full precision kept
    /// Context::set_raw(false); // restore default
    /// ```
    pub fn set_raw(is_raw: bool) {
        IS_RAW.store(is_raw, std::sync::atomic::Ordering::Release);
    }

    /// Registers a new custom [`Currency`] type in the global registry.
    ///
    /// The currency is identified by `C::CODE`. Attempting to register a code that already exists
    /// returns an error.
    ///
    /// # Errors
    ///
    /// - [`MoneyError::ObjMoneyError`] if the currency code is already registered or if the
    ///   internal `RwLock` is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{obj_money::Context, Currency};
    ///
    /// // All ISO currencies are pre-registered, so re-registering one fails:
    /// use moneylib::iso::USD;
    /// assert!(Context::register_currency::<USD>().is_err());
    /// ```
    pub fn register_currency<C: Currency>() -> Result<(), MoneyError> {
        let mut write = CURRENCIES.write().map_err(|_| {
            MoneyError::ObjMoneyError("failed getting lock to write into CURRENCIES".into())
        })?;

        if write.contains_key(&C::CODE) {
            return Err(MoneyError::ObjMoneyError(
                format!(
                    "Currency with code {} already exist: {:?}",
                    C::CODE,
                    get_data(C::CODE)
                )
                .into(),
            ));
        }

        write.insert(C::CODE, super::DynCurrency::from_curr::<C>());

        Ok(())
    }

    /// Returns `true` if the given currency `code` is present in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::Context;
    ///
    /// assert!(Context::is_currency_exist("USD"));
    /// assert!(!Context::is_currency_exist("XYZ"));
    /// ```
    pub fn is_currency_exist(code: &str) -> bool {
        if let Ok(data) = CURRENCIES.read()
            && data.contains_key(code)
        {
            return true;
        }

        false
    }

    /// Updates an existing currency entry in the registry with the current compile-time data of
    /// `C`.
    ///
    /// `code` must equal `C::CODE`; otherwise a [`MoneyError::CurrencyMismatchError`] is returned.
    ///
    /// # Errors
    ///
    /// - [`MoneyError::CurrencyMismatchError`] if `code != C::CODE`.
    /// - [`MoneyError::ObjMoneyError`] if the internal `RwLock` is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{obj_money::Context, iso::USD};
    ///
    /// // Refreshing USD with its own code succeeds
    /// assert!(Context::set_currency::<USD>("USD").is_ok());
    ///
    /// // Mismatched code returns an error
    /// assert!(Context::set_currency::<USD>("EUR").is_err());
    /// ```
    pub fn set_currency<C: Currency>(code: &str) -> Result<(), MoneyError> {
        if code != C::CODE {
            return Err(MoneyError::CurrencyMismatchError(
                C::CODE.into(),
                code.into(),
            ));
        }

        let mut write = CURRENCIES.write().map_err(|_| {
            MoneyError::ObjMoneyError("failed getting lock to set into CURRENCIES".into())
        })?;

        write.insert(C::CODE, super::DynCurrency::from_curr::<C>());

        Ok(())
    }

    /// Retrieves a [`DynCurrency`](super::dyn_money::DynCurrency) from the registry by ISO 4217
    /// `code`.
    ///
    /// Returns `None` if the code is not registered or if the `RwLock` is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::Context;
    ///
    /// let dc = Context::get_currency("USD").unwrap();
    /// assert_eq!(dc.code(), "USD");
    ///
    /// assert!(Context::get_currency("XYZ").is_none());
    /// ```
    pub fn get_currency(code: &str) -> Option<super::dyn_money::DynCurrency> {
        if let Ok(data) = CURRENCIES.read() {
            return data.get(code).copied();
        }

        None
    }

    /// Retrieves a [`DynCurrency`](super::dyn_money::DynCurrency) from the registry by its
    /// `symbol` (e.g. `"$"`, `"€"`).
    ///
    /// Returns `None` if no currency with that symbol is registered, or if the `RwLock` is
    /// poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::obj_money::Context;
    ///
    /// // € is the unique symbol for EUR
    /// let dc = Context::get_currency_by_symbol("€").unwrap();
    /// assert_eq!(dc.code(), "EUR");
    ///
    /// assert!(Context::get_currency_by_symbol("??").is_none());
    /// ```
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
