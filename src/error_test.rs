use crate::MoneyError;

#[test]
fn test_display_parse_str() {
    let error = MoneyError::ParseStr;
    let expected = "[MONEYLIB] failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator of `,` or `.`.";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_decimal_conversion() {
    let error = MoneyError::DecimalConversion;
    let expected = "[MONEYLIB] failed converting to/from Decimal";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_display_arithmetic_overflow() {
    let error = MoneyError::ArithmeticOverflow;
    let expected = "[MONEYLIB] arithmetic overflow";
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_error_trait_implementation() {
    // Test that MoneyError implements std::error::Error
    let error = MoneyError::ArithmeticOverflow;
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
        MoneyError::ParseStr,
        MoneyError::DecimalConversion,
        MoneyError::ArithmeticOverflow,
        MoneyError::CurrencyMismatch,
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
    let error = MoneyError::CurrencyMismatch;
    let _cloned = error.clone();
}
