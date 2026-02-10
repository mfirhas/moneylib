use crate::MoneyError;

#[test]
fn test_display_new_currency() {
    let error = MoneyError::NewCurrency;
    let expected = "[MONEYLIB] new currency must have code, symbol, name, and minor unit atleast, and not already existed in ISO 4217";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_exists_in_iso() {
    let error = MoneyError::ExistsInISO;
    let expected = "[MONEYLIB] this currency is already existed in ISO 4217 list, use Currency::from_iso to create ISO 4217 currency";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_parse_str() {
    let error = MoneyError::ParseStr;
    let expected = "[MONEYLIB] failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator of `,` or `.`.";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_invalid_currency() {
    let error = MoneyError::InvalidCurrency;
    let expected = "[MONEYLIB] invalid currency, please use currencies supported by ISO 4217";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_division_by_zero() {
    let error = MoneyError::DivisionByZero;
    let expected = "[MONEYLIB] cannot divide by zero";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_decimal_to_integer() {
    let error = MoneyError::DecimalToInteger;
    let expected = "[MONEYLIB] failed converting Decimal to integer types";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_arithmetic_overflow() {
    let error = MoneyError::ArithmeticOverflow;
    let expected = "[MONEYLIB] Arithmetic overflow";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_error_trait_implementation() {
    // Test that MoneyError implements std::error::Error
    let error = MoneyError::DivisionByZero;
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_display_format_with_formatter() {
    // Test that Display format works with formatter
    let error = MoneyError::ParseStr;
    let formatted = format!("{}", error);
    assert!(formatted.starts_with("[MONEYLIB]"));
}

#[test]
fn test_all_errors_have_prefix() {
    // Verify all error messages start with the expected prefix
    let errors = vec![
        MoneyError::NewCurrency,
        MoneyError::ExistsInISO,
        MoneyError::ParseStr,
        MoneyError::InvalidCurrency,
        MoneyError::DivisionByZero,
        MoneyError::DecimalToInteger,
        MoneyError::ArithmeticOverflow,
        MoneyError::MoneyAmount("money amount error".into()),
        MoneyError::NewMoney("new money error".into()),
    ];

    for error in errors {
        let message = error.to_string();
        assert!(
            message.starts_with("[MONEYLIB]"),
            "Error message should start with [MONEYLIB]: {}",
            message
        );
    }
}

#[test]
fn test_error_is_clone() {
    // Test that MoneyError is Clone
    let error = MoneyError::DivisionByZero;
    let _cloned = error.clone();
}

#[test]
fn test_error_debug_format() {
    // Test Debug implementation
    let error = MoneyError::NewCurrency;
    let debug_str = format!("{:?}", error);
    assert_eq!(debug_str, "NewCurrency");
}
