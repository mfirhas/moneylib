use crate::MoneyError;
use std::error::Error;

const ERROR_PREFIX: &str = "[MONEYLIB]";

// ==================== MoneyError::OverflowError Tests ====================

#[test]
fn test_overflow_error_display_contains_prefix() {
    let err = MoneyError::OverflowError;
    assert!(err.to_string().contains(ERROR_PREFIX));
}

#[test]
fn test_overflow_error_display_contains_overflowed() {
    let err = MoneyError::OverflowError;
    assert!(err.to_string().contains("overflowed"));
}

#[test]
fn test_overflow_error_display_message() {
    let err = MoneyError::OverflowError;
    assert_eq!(err.to_string(), "[MONEYLIB] got overflowed");
}

#[test]
fn test_overflow_error_debug() {
    let err = MoneyError::OverflowError;
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("OverflowError"));
}

#[test]
fn test_overflow_error_implements_error_trait() {
    let err = MoneyError::OverflowError;
    let _: &dyn Error = &err;
}

#[test]
fn test_overflow_error_source_is_none() {
    let err = MoneyError::OverflowError;
    assert!(err.source().is_none());
}

// ==================== MoneyError::ParseStrError Tests ====================

#[test]
fn test_parse_str_error_display_contains_prefix() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("invalid decimal")));
    assert!(err.to_string().contains(ERROR_PREFIX));
}

#[test]
fn test_parse_str_error_display_contains_parsing() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("invalid decimal")));
    assert!(err.to_string().contains("parsing error"));
}

#[test]
fn test_parse_str_error_display_contains_inner_message() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("invalid decimal")));
    assert!(err.to_string().contains("invalid decimal"));
}

#[test]
fn test_parse_str_error_display_full_format() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("bad input")));
    assert!(err.to_string().starts_with("[MONEYLIB] parsing error:"));
    assert!(err.to_string().contains("bad input"));
}

#[test]
fn test_parse_str_error_debug() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("bad input")));
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("ParseStrError"));
}

#[test]
fn test_parse_str_error_implements_error_trait() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("bad input")));
    let _: &dyn Error = &err;
}

#[test]
fn test_parse_str_error_source_is_some() {
    let err = MoneyError::ParseStrError(Box::new(std::io::Error::other("bad input")));
    assert!(err.source().is_some());
}

#[test]
fn test_parse_str_error_from_string_conversion() {
    let inner: Box<dyn Error + Send + Sync + 'static> = "string parse error".to_string().into();
    let err = MoneyError::ParseStrError(inner);
    assert!(err.to_string().contains("string parse error"));
}

// ==================== MoneyError::CurrencyMismatchError Tests ====================

#[test]
fn test_currency_mismatch_error_display_contains_prefix() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert!(err.to_string().contains(ERROR_PREFIX));
}

#[test]
fn test_currency_mismatch_error_display_contains_mismatch() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert!(err.to_string().contains("currency mismatch"));
}

#[test]
fn test_currency_mismatch_error_display_contains_got() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert!(err.to_string().contains("EUR"));
}

#[test]
fn test_currency_mismatch_error_display_contains_expected() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert!(err.to_string().contains("USD"));
}

#[test]
fn test_currency_mismatch_error_display_full_format() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert_eq!(
        err.to_string(),
        "[MONEYLIB] currency mismatch: got EUR, expected USD"
    );
}

#[test]
fn test_currency_mismatch_error_display_different_currencies() {
    let err = MoneyError::CurrencyMismatchError("JPY".to_string(), "GBP".to_string());
    assert_eq!(
        err.to_string(),
        "[MONEYLIB] currency mismatch: got JPY, expected GBP"
    );
}

#[test]
fn test_currency_mismatch_error_debug() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("CurrencyMismatchError"));
    assert!(debug_str.contains("EUR"));
    assert!(debug_str.contains("USD"));
}

#[test]
fn test_currency_mismatch_error_implements_error_trait() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    let _: &dyn Error = &err;
}

#[test]
fn test_currency_mismatch_error_source_is_none() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert!(err.source().is_none());
}

// ==================== MoneyError::ParseLocale Tests (feature = "locale") ====================

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_display_contains_prefix() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("invalid locale")));
    assert!(err.to_string().contains(ERROR_PREFIX));
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_display_contains_parsing_locale() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("invalid locale")));
    assert!(err.to_string().contains("error parsing locale"));
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_display_contains_inner_message() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("bad locale format")));
    assert!(err.to_string().contains("bad locale format"));
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_display_full_format() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("unknown locale")));
    assert!(
        err.to_string()
            .starts_with("[MONEYLIB] error parsing locale:")
    );
    assert!(err.to_string().contains("unknown locale"));
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_debug() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("invalid locale")));
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("ParseLocale"));
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_implements_error_trait() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("invalid locale")));
    let _: &dyn Error = &err;
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_source_is_some() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("invalid locale")));
    assert!(err.source().is_some());
}

// ==================== MoneyError::ExchangeError Tests (feature = "exchange") ====================

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_display_contains_prefix() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    assert!(err.to_string().contains(ERROR_PREFIX));
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_display_contains_exchange_error() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    assert!(err.to_string().contains("exchange error"));
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_display_contains_inner_message() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    assert!(err.to_string().contains("rate not found"));
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_display_full_format() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    assert!(err.to_string().starts_with("[MONEYLIB] exchange error:"));
    assert!(err.to_string().contains("rate not found"));
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_debug() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("ExchangeError"));
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_implements_error_trait() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    let _: &dyn Error = &err;
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_source_is_some() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("rate not found")));
    assert!(err.source().is_some());
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_from_string_conversion() {
    let inner: Box<dyn Error + Send + Sync + 'static> = "conversion overflow".to_string().into();
    let err = MoneyError::ExchangeError(inner);
    assert!(err.to_string().contains("conversion overflow"));
}

// ==================== Error prefix constant tests ====================

#[test]
fn test_all_variants_contain_prefix() {
    let errors: Vec<String> = vec![
        MoneyError::OverflowError.to_string(),
        MoneyError::ParseStrError(Box::new(std::io::Error::other("e"))).to_string(),
        MoneyError::CurrencyMismatchError("A".to_string(), "B".to_string()).to_string(),
    ];
    for msg in errors {
        assert!(
            msg.starts_with(ERROR_PREFIX),
            "Expected '{msg}' to start with '{ERROR_PREFIX}'"
        );
    }
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_variant_contains_prefix() {
    let err = MoneyError::ExchangeError(Box::new(std::io::Error::other("e")));
    assert!(err.to_string().starts_with(ERROR_PREFIX));
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_variant_contains_prefix() {
    let err = MoneyError::ParseLocale(Box::new(std::io::Error::other("e")));
    assert!(err.to_string().starts_with(ERROR_PREFIX));
}
