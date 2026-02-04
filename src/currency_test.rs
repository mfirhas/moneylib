#[cfg(test)]
mod tests {
    use super::super::Currency;
    use crate::{Country, MoneyError};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::str::FromStr;

    // Test Currency::from_iso() with valid ISO code
    #[test]
    fn test_from_iso_valid_usd() {
        let currency = Currency::from_iso("USD").unwrap();
        assert_eq!(currency.code(), "USD");
        assert_eq!(currency.symbol(), "$");
        assert_eq!(currency.name(), "United States dollar");
        assert_eq!(currency.minor_unit(), 2);
        assert_eq!(currency.numeric_code(), 840);
    }

    #[test]
    fn test_from_iso_valid_eur() {
        let currency = Currency::from_iso("EUR").unwrap();
        assert_eq!(currency.code(), "EUR");
        assert_eq!(currency.symbol(), "€");
        assert_eq!(currency.name(), "Euro");
        assert_eq!(currency.minor_unit(), 2);
    }

    #[test]
    fn test_from_iso_valid_jpy() {
        let currency = Currency::from_iso("JPY").unwrap();
        assert_eq!(currency.code(), "JPY");
        assert_eq!(currency.symbol(), "¥");
        assert_eq!(currency.name(), "Japanese yen");
        assert_eq!(currency.minor_unit(), 0); // JPY has no minor unit
    }

    #[test]
    fn test_from_iso_valid_gbp() {
        let currency = Currency::from_iso("GBP").unwrap();
        assert_eq!(currency.code(), "GBP");
        assert_eq!(currency.symbol(), "£");
        assert_eq!(currency.name(), "Pound sterling");
        assert_eq!(currency.minor_unit(), 2);
    }

    #[test]
    fn test_from_iso_lowercase() {
        let currency = Currency::from_iso("usd").unwrap();
        assert_eq!(currency.code(), "USD");
    }

    #[test]
    fn test_from_iso_mixed_case() {
        let currency = Currency::from_iso("UsD").unwrap();
        assert_eq!(currency.code(), "USD");
    }

    #[test]
    fn test_from_iso_invalid_code() {
        let result = Currency::from_iso("XYZ");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::InvalidCurrency));
    }

    #[test]
    fn test_from_iso_invalid_empty() {
        let result = Currency::from_iso("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::InvalidCurrency));
    }

    #[test]
    fn test_from_iso_invalid_too_short() {
        let result = Currency::from_iso("US");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::InvalidCurrency));
    }

    #[test]
    fn test_from_iso_invalid_too_long() {
        let result = Currency::from_iso("USDD");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::InvalidCurrency));
    }

    // Test Currency::new()
    #[test]
    fn test_new_valid() {
        let currency = Currency::new("TST", "$", "Test Currency", 2).unwrap();
        assert_eq!(currency.code(), "TST");
        assert_eq!(currency.symbol(), "$");
        assert_eq!(currency.name(), "Test Currency");
        assert_eq!(currency.minor_unit(), 2);
        assert_eq!(currency.thousand_separator(), ",");
        assert_eq!(currency.decimal_separator(), ".");
        assert_eq!(currency.minor_symbol(), "minor");
        assert_eq!(currency.numeric_code(), 0); // Default
    }

    #[test]
    fn test_new_with_zero_minor_unit() {
        let currency = Currency::new("TST", "$", "Test Currency", 0).unwrap();
        assert_eq!(currency.minor_unit(), 0);
    }

    #[test]
    fn test_new_with_large_minor_unit() {
        let currency = Currency::new("TST", "$", "Test Currency", 8).unwrap();
        assert_eq!(currency.minor_unit(), 8);
    }

    #[test]
    fn test_new_empty_code() {
        let result = Currency::new("", "$", "Test Currency", 2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::NewCurrency));
    }

    #[test]
    fn test_new_empty_symbol() {
        let result = Currency::new("TST", "", "Test Currency", 2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::NewCurrency));
    }

    #[test]
    fn test_new_empty_name() {
        let result = Currency::new("TST", "$", "", 2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::NewCurrency));
    }

    #[test]
    fn test_new_all_empty() {
        let result = Currency::new("", "", "", 2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::NewCurrency));
    }

    // Test setter methods
    #[test]
    fn test_set_thousand_separator() {
        let mut currency = Currency::from_iso("USD").unwrap();
        assert_eq!(currency.thousand_separator(), ",");
        currency.set_thousand_separator(".");
        assert_eq!(currency.thousand_separator(), ".");
    }

    #[test]
    fn test_set_decimal_separator() {
        let mut currency = Currency::from_iso("USD").unwrap();
        assert_eq!(currency.decimal_separator(), ".");
        currency.set_decimal_separator(",");
        assert_eq!(currency.decimal_separator(), ",");
    }

    #[test]
    fn test_set_minor_symbol() {
        let mut currency = Currency::from_iso("USD").unwrap();
        currency.set_minor_symbol("cents");
        assert_eq!(currency.minor_symbol(), "cents");
    }

    #[test]
    fn test_set_numeric_code() {
        let mut currency = Currency::new("TST", "$", "Test", 2).unwrap();
        assert_eq!(currency.numeric_code(), 0);
        currency.set_numeric_code(999);
        assert_eq!(currency.numeric_code(), 999);
    }

    #[test]
    fn test_set_countries() {
        let mut currency = Currency::new("TST", "$", "Test", 2).unwrap();
        let countries = &[Country::US, Country::GB];
        currency.set_countries(countries);
        let result_countries = currency.countries();
        assert_eq!(result_countries.len(), 2);
        assert!(result_countries.contains(&Country::US));
        assert!(result_countries.contains(&Country::GB));
    }

    // Test getter methods
    #[test]
    fn test_getters() {
        let currency = Currency::from_iso("EUR").unwrap();
        assert_eq!(currency.code(), "EUR");
        assert_eq!(currency.symbol(), "€");
        assert_eq!(currency.name(), "Euro");
        assert_eq!(currency.minor_unit(), 2);
        assert!(currency.numeric_code() > 0);
        assert_eq!(currency.thousand_separator(), ",");
        assert_eq!(currency.decimal_separator(), ".");
        assert!(!currency.minor_symbol().is_empty());
    }

    // Test FromStr trait
    #[test]
    fn test_from_str_valid() {
        let currency = Currency::from_str("USD").unwrap();
        assert_eq!(currency.code(), "USD");
    }

    #[test]
    fn test_from_str_invalid() {
        let result = Currency::from_str("INVALID");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MoneyError::InvalidCurrency));
    }

    // Test PartialEq
    #[test]
    fn test_partial_eq_same() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = Currency::from_iso("USD").unwrap();
        assert_eq!(currency1, currency2);
    }

    #[test]
    fn test_partial_eq_different() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = Currency::from_iso("EUR").unwrap();
        assert_ne!(currency1, currency2);
    }

    #[test]
    fn test_partial_eq_same_code_different_settings() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let mut currency2 = Currency::from_iso("USD").unwrap();
        currency2.set_thousand_separator(".");
        // Should still be equal because comparison is based on code only
        assert_eq!(currency1, currency2);
    }

    // Test PartialOrd
    #[test]
    fn test_partial_ord() {
        let usd = Currency::from_iso("USD").unwrap();
        let eur = Currency::from_iso("EUR").unwrap();
        let gbp = Currency::from_iso("GBP").unwrap();

        // Test ordering based on code alphabetically
        assert!(eur < gbp);
        assert!(gbp < usd);
        assert!(eur < usd);
    }

    #[test]
    fn test_partial_ord_same() {
        let usd1 = Currency::from_iso("USD").unwrap();
        let usd2 = Currency::from_iso("USD").unwrap();
        assert!(usd1 <= usd2);
        assert!(usd1 >= usd2);
    }

    // Test Hash trait
    #[test]
    fn test_hash_same_currency() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = Currency::from_iso("USD").unwrap();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        currency1.hash(&mut hasher1);
        currency2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_different_currency() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = Currency::from_iso("EUR").unwrap();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        currency1.hash(&mut hasher1);
        currency2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // Test countries() method
    #[test]
    fn test_countries_for_usd() {
        let currency = Currency::from_iso("USD").unwrap();
        let countries = currency.countries();
        assert!(!countries.is_empty());
        assert!(countries.contains(&Country::US));
    }

    #[test]
    fn test_countries_custom() {
        let mut currency = Currency::new("TST", "$", "Test", 2).unwrap();
        let custom_countries = &[Country::US, Country::CA];
        currency.set_countries(custom_countries);
        let countries = currency.countries();
        assert_eq!(countries.len(), 2);
    }

    // Test Default trait
    #[test]
    fn test_default() {
        let currency = Currency::default();
        assert_eq!(currency.code(), "");
        assert_eq!(currency.symbol(), "");
        assert_eq!(currency.name(), "");
        assert_eq!(currency.minor_unit(), 0);
        assert_eq!(currency.numeric_code(), 0);
    }

    // Test Clone trait
    #[test]
    fn test_clone() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = currency1.clone();
        assert_eq!(currency1, currency2);
        assert_eq!(currency1.code(), currency2.code());
        assert_eq!(currency1.symbol(), currency2.symbol());
    }

    // Test Copy trait
    #[test]
    fn test_copy() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = currency1; // Copy happens here
        // Both should still be usable
        assert_eq!(currency1.code(), "USD");
        assert_eq!(currency2.code(), "USD");
    }

    // Test Debug trait
    #[test]
    fn test_debug() {
        let currency = Currency::from_iso("USD").unwrap();
        let debug_str = format!("{:?}", currency);
        assert!(debug_str.contains("Currency"));
    }

    // Test multiple currencies
    #[test]
    fn test_multiple_currencies() {
        let currencies = vec!["USD", "EUR", "GBP", "JPY", "CHF"];
        for code in currencies {
            let currency = Currency::from_iso(code).unwrap();
            assert_eq!(currency.code(), code);
            assert!(!currency.symbol().is_empty());
            assert!(!currency.name().is_empty());
        }
    }

    // Edge cases and special scenarios
    #[test]
    fn test_currency_with_no_minor_unit() {
        let currency = Currency::from_iso("JPY").unwrap();
        assert_eq!(currency.minor_unit(), 0);
    }

    #[test]
    fn test_currency_with_three_decimal_places() {
        let currency = Currency::from_iso("BHD").unwrap(); // Bahraini Dinar
        assert_eq!(currency.minor_unit(), 3);
    }

    #[test]
    fn test_separator_consistency() {
        let currency = Currency::from_iso("USD").unwrap();
        assert_eq!(currency.thousand_separator(), ",");
        assert_eq!(currency.decimal_separator(), ".");
        assert_ne!(currency.thousand_separator(), currency.decimal_separator());
    }

    #[test]
    fn test_numeric_code_positive() {
        let currency = Currency::from_iso("USD").unwrap();
        assert!(currency.numeric_code() > 0);
    }

    // Test immutability of from_iso
    #[test]
    fn test_from_iso_creates_new_instance() {
        let currency1 = Currency::from_iso("USD").unwrap();
        let currency2 = Currency::from_iso("USD").unwrap();
        // They should be equal but independent instances
        assert_eq!(currency1, currency2);
    }

    // Test case sensitivity handling
    #[test]
    fn test_case_insensitive_iso_code() {
        let upper = Currency::from_iso("USD").unwrap();
        let lower = Currency::from_iso("usd").unwrap();
        let mixed = Currency::from_iso("UsD").unwrap();
        
        assert_eq!(upper, lower);
        assert_eq!(upper, mixed);
        assert_eq!(lower, mixed);
    }

    // Test that setters don't affect equality
    #[test]
    fn test_equality_ignores_separators() {
        let mut currency1 = Currency::from_iso("USD").unwrap();
        let mut currency2 = Currency::from_iso("USD").unwrap();
        
        currency1.set_thousand_separator(" ");
        currency2.set_thousand_separator(".");
        
        // Should still be equal as equality is based on code only
        assert_eq!(currency1, currency2);
    }

    // Test ordering with modified currencies
    #[test]
    fn test_ordering_ignores_modifications() {
        let mut currency1 = Currency::from_iso("EUR").unwrap();
        let currency2 = Currency::from_iso("USD").unwrap();
        
        currency1.set_thousand_separator(" ");
        currency1.set_minor_symbol("euro cents");
        
        // Ordering should still be based on code
        assert!(currency1 < currency2);
    }

    // Test that custom currency can have custom settings
    #[test]
    fn test_custom_currency_with_all_setters() {
        let mut currency = Currency::new("XXX", "X", "Custom", 4).unwrap();
        currency.set_thousand_separator(" ");
        currency.set_decimal_separator(",");
        currency.set_minor_symbol("parts");
        currency.set_numeric_code(999);
        
        assert_eq!(currency.code(), "XXX");
        assert_eq!(currency.symbol(), "X");
        assert_eq!(currency.name(), "Custom");
        assert_eq!(currency.minor_unit(), 4);
        assert_eq!(currency.thousand_separator(), " ");
        assert_eq!(currency.decimal_separator(), ",");
        assert_eq!(currency.minor_symbol(), "parts");
        assert_eq!(currency.numeric_code(), 999);
    }
}
