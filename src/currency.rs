use std::str::FromStr;

use crate::{Country, RoundingStrategy};

use crate::{
    MoneyError, MoneyResult,
    base::{COMMA_SEPARATOR, DOT_SEPARATOR},
};
use std::hash::{Hash, Hasher};

const DEFAULT_MINOR_UNIT_SYMBOL: &str = "minor";

/// A currency type representing monetary currencies.
///
/// `Currency` supports both ISO 4217 standard currencies and custom currencies.
/// It includes metadata such as the currency code, symbol, name, minor unit (decimal places),
/// and formatting options like thousand and decimal separators.
///
/// # ISO 4217 Currencies
///
/// Use [`Currency::from_iso`] to create standard currencies:
///
/// ```
/// use moneylib::Currency;
///
/// let usd = Currency::from_iso("USD").unwrap();
/// assert_eq!(usd.code(), "USD");
/// assert_eq!(usd.symbol(), "$");
/// assert_eq!(usd.minor_unit(), 2); // 2 decimal places
/// ```
///
/// # Custom Currencies
///
/// Use [`Currency::new`] to create custom currencies not in ISO 4217:
///
/// ```
/// use moneylib::Currency;
///
/// let custom = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
/// assert_eq!(custom.code(), "BTC");
/// assert_eq!(custom.symbol(), "₿");
/// assert_eq!(custom.minor_unit(), 8); // 8 decimal places
/// ```
///
/// # Customization
///
/// Currencies can be customized with different separators and rounding strategies:
///
/// ```
/// use moneylib::{Currency, RoundingStrategy};
///
/// let mut currency = Currency::from_iso("EUR").unwrap();
/// currency.set_thousand_separator(".");
/// currency.set_decimal_separator(",");
/// currency.set_rounding_strategy(RoundingStrategy::Ceil);
/// ```
///
/// # Equality and Ordering
///
/// Currency equality and ordering are based solely on the currency code,
/// ignoring formatting settings:
///
/// ```
/// use moneylib::Currency;
///
/// let mut usd1 = Currency::from_iso("USD").unwrap();
/// let mut usd2 = Currency::from_iso("USD").unwrap();
///
/// usd1.set_thousand_separator(".");
/// usd2.set_thousand_separator(",");
///
/// assert_eq!(usd1, usd2); // Still equal despite different separators
/// ```
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

    rounding_strategy: RoundingStrategy,

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

    /// Parse ISO 4217 currencies alphabetical code from string.
    ///
    /// Case insensitive.
    /// E.g. "USD", or "usd" or "uSd"
    ///
    /// Calls `Currency::from_iso` inside.
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
    /// Creates a currency from an ISO 4217 currency code.
    ///
    /// This method parses ISO 4217 alphabetical currency codes (case-insensitive)
    /// and returns a `Currency` with standard metadata from the ISO 4217 specification.
    ///
    /// # Arguments
    ///
    /// * `iso_code` - A 3-letter ISO 4217 currency code (e.g., "USD", "EUR", "JPY")
    ///
    /// # Returns
    ///
    /// * `Ok(Currency)` - Successfully parsed currency with ISO 4217 metadata
    /// * `Err(MoneyError::InvalidCurrency)` - Invalid or unknown currency code
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// // Create a US Dollar currency
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.code(), "USD");
    /// assert_eq!(usd.symbol(), "$");
    /// assert_eq!(usd.name(), "United States dollar");
    /// assert_eq!(usd.minor_unit(), 2);
    ///
    /// // Case insensitive
    /// let eur = Currency::from_iso("eur").unwrap();
    /// assert_eq!(eur.code(), "EUR");
    ///
    /// // Japanese Yen has no minor units
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// assert_eq!(jpy.minor_unit(), 0);
    ///
    /// // Invalid currency code returns error
    /// assert!(Currency::from_iso("XYZ").is_err());
    /// ```
    pub fn from_iso(iso_code: &str) -> MoneyResult<Currency> {
        let iso_code = iso_code.trim().to_ascii_uppercase();
        let ret =
            iso_currency_lib::Currency::from_code(&iso_code).ok_or(MoneyError::InvalidCurrency)?;
        let currency = Currency {
            code: ret.code(),
            symbol: ret.symbol().symbol,
            name: ret.name(),
            minor_unit: ret.exponent().unwrap_or_default(),
            numeric_code: ret.numeric().into(),
            thousand_separator: COMMA_SEPARATOR,
            decimal_separator: DOT_SEPARATOR,
            minor_symbol: ret
                .symbol()
                .subunit_symbol
                .unwrap_or(DEFAULT_MINOR_UNIT_SYMBOL),

            rounding_strategy: Default::default(),

            countries: None,
        };

        Ok(currency)
    }

    /// Creates a custom currency not defined in ISO 4217.
    ///
    /// This method allows you to create currencies that are not in the ISO 4217 standard,
    /// such as cryptocurrencies, loyalty points, or fictional currencies.
    ///
    /// # Arguments
    ///
    /// * `code` - Currency code (must not exist in ISO 4217, cannot be empty)
    /// * `symbol` - Currency symbol (cannot be empty)
    /// * `name` - Currency name (cannot be empty)
    /// * `minor_unit` - Number of decimal places (0 for no fractional units)
    ///
    /// # Returns
    ///
    /// * `Ok(Currency)` - Successfully created custom currency
    /// * `Err(MoneyError::ExistsInISO)` - Code already exists in ISO 4217
    /// * `Err(MoneyError::NewCurrency)` - Invalid parameters (empty code, symbol, or name)
    ///
    /// # Default Values
    ///
    /// Custom currencies are created with these defaults:
    /// - Thousand separator: ","
    /// - Decimal separator: "."
    /// - Minor symbol: "minor"
    /// - Numeric code: 0
    /// - Rounding strategy: BankersRounding
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// // Create a custom cryptocurrency
    /// let btc = Currency::new("XBT", "₿", "Bitcoin", 8).unwrap();
    /// assert_eq!(btc.code(), "XBT");
    /// assert_eq!(btc.symbol(), "₿");
    /// assert_eq!(btc.minor_unit(), 8);
    ///
    /// // Create a zero-decimal currency
    /// let gold = Currency::new("GOLD", "Au", "Gold Ounce", 0).unwrap();
    /// assert_eq!(gold.minor_unit(), 0);
    ///
    /// // Cannot use ISO 4217 codes
    /// let result = Currency::new("USD", "$", "My Dollar", 2);
    /// assert!(result.is_err()); // USD exists in ISO 4217
    ///
    /// // Cannot use empty values
    /// let result = Currency::new("", "$", "Empty", 2);
    /// assert!(result.is_err());
    /// ```
    pub fn new(
        code: &'static str,
        symbol: &'static str,
        name: &'static str,
        minor_unit: u16,
    ) -> MoneyResult<Currency> {
        let code = code.trim();
        if !code.is_empty() {
            let uppercase_code = code.to_ascii_uppercase();
            if iso_currency_lib::Currency::from_code(&uppercase_code).is_some() {
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

            rounding_strategy: Default::default(),

            countries: None,
        })
    }

    /// Sets the thousand separator for formatting.
    ///
    /// The thousand separator is used when formatting large amounts.
    /// Common values are "," (comma) for English locales and "." (dot) for European locales.
    ///
    /// # Arguments
    ///
    /// * `separator` - The thousand separator string
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let mut eur = Currency::from_iso("EUR").unwrap();
    /// eur.set_thousand_separator(".");
    /// assert_eq!(eur.thousand_separator(), ".");
    /// ```
    #[inline]
    pub fn set_thousand_separator(&mut self, separator: &'static str) {
        self.thousand_separator = separator;
    }

    /// Sets the decimal separator for formatting.
    ///
    /// The decimal separator is used when formatting fractional amounts.
    /// Common values are "." (dot) for English locales and "," (comma) for European locales.
    ///
    /// # Arguments
    ///
    /// * `separator` - The decimal separator string
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let mut eur = Currency::from_iso("EUR").unwrap();
    /// eur.set_decimal_separator(",");
    /// assert_eq!(eur.decimal_separator(), ",");
    /// ```
    #[inline]
    pub fn set_decimal_separator(&mut self, separator: &'static str) {
        self.decimal_separator = separator;
    }

    /// Sets the symbol for the minor unit (smallest denomination).
    ///
    /// The minor symbol represents the currency's smallest unit, such as "cent" for USD
    /// or "penny" for GBP. This is used in formatting when displaying amounts in minor units.
    ///
    /// # Arguments
    ///
    /// * `minor_symbol` - The minor unit symbol string
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let mut usd = Currency::from_iso("USD").unwrap();
    /// usd.set_minor_symbol("¢");
    /// assert_eq!(usd.minor_symbol(), "¢");
    ///
    /// let mut gbp = Currency::from_iso("GBP").unwrap();
    /// gbp.set_minor_symbol("p");
    /// assert_eq!(gbp.minor_symbol(), "p");
    /// ```
    #[inline]
    pub fn set_minor_symbol(&mut self, minor_symbol: &'static str) {
        self.minor_symbol = minor_symbol;
    }

    /// Sets the ISO 4217 numeric code.
    ///
    /// This is primarily useful for custom currencies where you want to assign
    /// a numeric identifier. ISO 4217 currencies have their numeric codes set automatically.
    ///
    /// # Arguments
    ///
    /// * `numeric_code` - The numeric code (typically a 3-digit number for ISO currencies)
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let mut custom = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
    /// custom.set_numeric_code(999);
    /// assert_eq!(custom.numeric_code(), 999);
    /// ```
    #[inline]
    pub fn set_numeric_code(&mut self, numeric_code: i32) {
        self.numeric_code = numeric_code;
    }

    /// Sets the rounding strategy for this currency.
    ///
    /// The rounding strategy determines how decimal values are rounded when performing
    /// monetary calculations. This is particularly important for currencies with limited
    /// decimal places.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The rounding strategy to use
    ///
    /// # Available Strategies
    ///
    /// - `BankersRounding` (default): Round to nearest even (reduces bias)
    /// - `HalfUp`: Round half values up
    /// - `HalfDown`: Round half values down
    /// - `Ceil`: Always round up
    /// - `Floor`: Always round down
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Currency, RoundingStrategy};
    ///
    /// let mut usd = Currency::from_iso("USD").unwrap();
    ///
    /// // Use banker's rounding (default)
    /// usd.set_rounding_strategy(RoundingStrategy::BankersRounding);
    ///
    /// // Always round up
    /// usd.set_rounding_strategy(RoundingStrategy::Ceil);
    ///
    /// // Always round half values up
    /// usd.set_rounding_strategy(RoundingStrategy::HalfUp);
    /// ```
    #[inline]
    pub fn set_rounding_strategy(&mut self, strategy: RoundingStrategy) {
        self.rounding_strategy = strategy
    }

    /// Sets the countries where this currency is used.
    ///
    /// This is typically used for custom currencies to specify their geographic usage.
    /// ISO 4217 currencies automatically have country information from the standard.
    ///
    /// # Arguments
    ///
    /// * `countries` - A static slice of `Country` values
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Currency, Country};
    ///
    /// let mut custom = Currency::new("REG", "R", "Regional Currency", 2).unwrap();
    /// let countries = &[Country::US, Country::CA];
    /// custom.set_countries(countries);
    /// ```
    #[inline]
    pub fn set_countries(&mut self, countries: &'static [Country]) {
        self.countries = Some(countries);
    }

    /// Returns the currency code.
    ///
    /// The code is typically a 3-letter ISO 4217 alphabetical code for standard currencies,
    /// or a custom code for non-ISO currencies.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.code(), "USD");
    /// ```
    #[inline]
    pub fn code(&self) -> &'static str {
        self.code
    }

    /// Returns the currency symbol.
    ///
    /// The symbol is typically a single character or short string representing the currency,
    /// such as "$" for USD, "€" for EUR, or "¥" for JPY.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.symbol(), "$");
    ///
    /// let eur = Currency::from_iso("EUR").unwrap();
    /// assert_eq!(eur.symbol(), "€");
    /// ```
    #[inline]
    pub fn symbol(&self) -> &'static str {
        self.symbol
    }

    /// Returns the currency name.
    ///
    /// The name is the full descriptive name of the currency,
    /// such as "United States dollar" for USD or "Euro" for EUR.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.name(), "United States dollar");
    ///
    /// let custom = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
    /// assert_eq!(custom.name(), "Bitcoin");
    /// ```
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the number of decimal places for the minor unit.
    ///
    /// The minor unit represents the number of decimal places used by this currency.
    /// For example, USD has 2 (cents), JPY has 0 (no fractional units), and BTC might have 8 (satoshis).
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.minor_unit(), 2); // 2 decimal places (cents)
    ///
    /// let jpy = Currency::from_iso("JPY").unwrap();
    /// assert_eq!(jpy.minor_unit(), 0); // No fractional units
    ///
    /// let btc = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
    /// assert_eq!(btc.minor_unit(), 8); // 8 decimal places
    /// ```
    #[inline]
    pub fn minor_unit(&self) -> u16 {
        self.minor_unit
    }

    /// Returns the ISO 4217 numeric code.
    ///
    /// The numeric code is a 3-digit number assigned to currencies in the ISO 4217 standard.
    /// For example, USD is 840, EUR is 978. Custom currencies default to 0 unless set.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.numeric_code(), 840);
    ///
    /// let eur = Currency::from_iso("EUR").unwrap();
    /// assert_eq!(eur.numeric_code(), 978);
    ///
    /// let custom = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
    /// assert_eq!(custom.numeric_code(), 0); // Default for custom currencies
    /// ```
    #[inline]
    pub fn numeric_code(&self) -> i32 {
        self.numeric_code
    }

    /// Returns the thousand separator used in formatting.
    ///
    /// The default is "," (comma), but this can be customized for different locales.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.thousand_separator(), ","); // Default
    ///
    /// let mut eur = Currency::from_iso("EUR").unwrap();
    /// eur.set_thousand_separator(".");
    /// assert_eq!(eur.thousand_separator(), ".");
    /// ```
    #[inline]
    pub fn thousand_separator(&self) -> &'static str {
        self.thousand_separator
    }

    /// Returns the decimal separator used in formatting.
    ///
    /// The default is "." (dot), but this can be customized for different locales.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// assert_eq!(usd.decimal_separator(), "."); // Default
    ///
    /// let mut eur = Currency::from_iso("EUR").unwrap();
    /// eur.set_decimal_separator(",");
    /// assert_eq!(eur.decimal_separator(), ",");
    /// ```
    #[inline]
    pub fn decimal_separator(&self) -> &'static str {
        self.decimal_separator
    }

    /// Returns the symbol for the minor unit.
    ///
    /// The minor symbol represents the smallest denomination of the currency.
    /// For example, "¢" for US cents, "p" for British pence. The default is "minor".
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// // Default minor symbol from ISO 4217, or "minor" if not specified
    ///
    /// let mut custom = Currency::new("PTS", "P", "Points", 2).unwrap();
    /// assert_eq!(custom.minor_symbol(), "minor"); // Default
    /// custom.set_minor_symbol("pt");
    /// assert_eq!(custom.minor_symbol(), "pt");
    /// ```
    #[inline]
    pub fn minor_symbol(&self) -> &'static str {
        self.minor_symbol
    }

    /// Returns the rounding strategy for this currency.
    ///
    /// The rounding strategy determines how decimal values are rounded during
    /// monetary calculations. The default is `BankersRounding`.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Currency, RoundingStrategy};
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// // Default is BankersRounding
    ///
    /// let mut custom = Currency::new("TST", "$", "Test", 2).unwrap();
    /// custom.set_rounding_strategy(RoundingStrategy::Ceil);
    /// assert_eq!(custom.rounding_strategy(), RoundingStrategy::Ceil);
    /// ```
    #[inline]
    pub fn rounding_strategy(&self) -> RoundingStrategy {
        self.rounding_strategy
    }

    /// Returns the list of countries where this currency is used.
    ///
    /// For ISO 4217 currencies, this returns the countries from the ISO standard.
    /// For custom currencies, this returns the countries set via [`set_countries`](Self::set_countries),
    /// or `None` if not set.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::Currency;
    ///
    /// let usd = Currency::from_iso("USD").unwrap();
    /// let countries = usd.countries();
    /// // Returns countries that use USD according to ISO 4217
    ///
    /// let custom = Currency::new("TST", "$", "Test", 2).unwrap();
    /// assert_eq!(custom.countries(), None); // No countries set by default
    /// ```
    pub fn countries(&self) -> Option<Vec<Country>> {
        self.countries.map(|c| c.into()).or_else(|| {
            iso_currency_lib::Currency::from_code(self.code()).map(|curr| curr.used_by())
        })
    }
}
