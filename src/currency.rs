use std::str::FromStr;

use crate::Country;

use crate::{
    MoneyError, MoneyResult,
    base::{COMMA_SEPARATOR, DOT_SEPARATOR},
};
use std::hash::{Hash, Hasher};

const DEFAULT_MINOR_UNIT_SYMBOL: &'static str = "minor";

#[derive(Debug, Clone, Copy, Eq)]
pub struct Currency {
    code: &'static str,
    symbol: &'static str,
    name: &'static str,
    minor_unit: u16,
    numeric_code: i32,
    thousand_separator: &'static str,
    decimal_separator: &'static str,
    minor_symbol: &'static str,

    countries: Option<&'static [Country]>,
}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.code().partial_cmp(other.code())
    }
}

impl FromStr for Currency {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_iso(s)
    }
}

impl Hash for Currency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code().hash(state);
    }
}

impl Currency {
    pub fn from_iso(iso_code: &str) -> MoneyResult<Currency> {
        let iso_code = iso_code.to_ascii_uppercase();
        let ret =
            iso_currency::Currency::from_code(&iso_code).ok_or(MoneyError::InvalidCurrency)?;
        let currency = Currency {
            code: ret.code(),
            symbol: ret.symbol().symbol,
            name: ret.name(),
            minor_unit: ret.exponent().unwrap_or_default(),
            numeric_code: ret.numeric() as i32,
            thousand_separator: COMMA_SEPARATOR,
            decimal_separator: DOT_SEPARATOR,
            minor_symbol: ret
                .symbol()
                .subunit_symbol
                .unwrap_or(DEFAULT_MINOR_UNIT_SYMBOL),

            countries: None,
        };

        Ok(currency)
    }

    /// Create new currency.
    ///
    /// It fails if the currency is already existed in ISO 4217 list.
    ///
    /// Use `Currency::from_iso` instead to create ISO 4217 currency.
    pub fn new(
        code: &'static str,
        symbol: &'static str,
        name: &'static str,
        minor_unit: u16,
    ) -> MoneyResult<Currency> {
        if !code.is_empty() {
            let uppercase_code = code.to_ascii_uppercase();
            if iso_currency::Currency::from_code(&uppercase_code).is_some() {
                return Err(MoneyError::ExistsInISO);
            }
        }
        if code.is_empty() || symbol.is_empty() || name.is_empty() {
            return Err(MoneyError::NewCurrency);
        }

        Ok(Currency {
            code,
            symbol,
            name,
            minor_unit,
            thousand_separator: COMMA_SEPARATOR,
            decimal_separator: DOT_SEPARATOR,
            minor_symbol: DEFAULT_MINOR_UNIT_SYMBOL,
            numeric_code: 0,
            countries: None,
        })
    }

    pub fn set_thousand_separator(&mut self, separator: &'static str) {
        self.thousand_separator = separator;
    }

    pub fn set_decimal_separator(&mut self, separator: &'static str) {
        self.decimal_separator = separator;
    }

    pub fn set_minor_symbol(&mut self, minor_symbol: &'static str) {
        self.minor_symbol = minor_symbol;
    }

    pub fn set_numeric_code(&mut self, numeric_code: i32) {
        self.numeric_code = numeric_code;
    }

    pub fn set_countries(&mut self, countries: &'static [Country]) {
        self.countries = Some(countries);
    }

    #[inline]
    pub fn code(&self) -> &'static str {
        self.code
    }

    #[inline]
    pub fn symbol(&self) -> &'static str {
        self.symbol
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn minor_unit(&self) -> u16 {
        self.minor_unit
    }

    #[inline]
    pub fn numeric_code(&self) -> i32 {
        self.numeric_code
    }

    #[inline]
    pub fn thousand_separator(&self) -> &'static str {
        self.thousand_separator
    }

    #[inline]
    pub fn decimal_separator(&self) -> &'static str {
        self.decimal_separator
    }

    #[inline]
    pub fn minor_symbol(&self) -> &'static str {
        self.minor_symbol
    }

    pub fn countries(&self) -> Option<Vec<Country>> {
        self.countries
            .map(|c| c.into())
            .or_else(|| iso_currency::Currency::from_code(self.code()).map(|curr| curr.used_by()))
    }
}
