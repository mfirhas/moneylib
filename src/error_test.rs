use crate::MoneyError;

#[test]
fn test_parse_str_error_display() {
    let err = MoneyError::ParseStrError("bad input".to_string().into());
    assert!(err.to_string().contains("[MONEYLIB]"));
    assert!(err.to_string().contains("bad input"));
}

#[test]
fn test_overflow_error_display() {
    let err = MoneyError::OverflowError;
    assert_eq!(err.to_string(), "[MONEYLIB] got overflowed");
}

#[test]
fn test_currency_mismatch_error_display() {
    let err = MoneyError::CurrencyMismatchError("EUR".to_string(), "USD".to_string());
    assert_eq!(
        err.to_string(),
        "[MONEYLIB] currency mismatch: got EUR, expected USD"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_parse_locale_error_display() {
    let err = MoneyError::ParseLocale("invalid locale".to_string().into());
    assert!(err.to_string().contains("[MONEYLIB]"));
    assert!(err.to_string().contains("invalid locale"));
}

#[cfg(feature = "exchange")]
#[test]
fn test_exchange_error_display() {
    let err = MoneyError::ExchangeError("rate not found".to_string().into());
    assert!(err.to_string().contains("[MONEYLIB]"));
    assert!(err.to_string().contains("rate not found"));
}
