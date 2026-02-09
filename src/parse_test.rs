use crate::parse::{parse_comma_thousands_separator, parse_dot_thousands_separator};

// Tests for parse_comma_thousands_separator

#[test]
fn test_comma_simple_amount() {
    let result = parse_comma_thousands_separator("USD 100.50");
    assert_eq!(result, Some(("USD", "100.50".to_string())));
}

#[test]
fn test_comma_simple_integer() {
    let result = parse_comma_thousands_separator("USD 100");
    assert_eq!(result, Some(("USD", "100".to_string())));
}

#[test]
fn test_comma_with_thousands() {
    let result = parse_comma_thousands_separator("USD 1,234.56");
    assert_eq!(result, Some(("USD", "1234.56".to_string())));
}

#[test]
fn test_comma_large_amount() {
    let result = parse_comma_thousands_separator("USD 1,000,000.99");
    assert_eq!(result, Some(("USD", "1000000.99".to_string())));
}

#[test]
fn test_comma_no_decimal() {
    let result = parse_comma_thousands_separator("USD 1,234");
    assert_eq!(result, Some(("USD", "1234".to_string())));
}

#[test]
fn test_comma_lowercase_currency() {
    let result = parse_comma_thousands_separator("usd 100.50");
    assert_eq!(result, Some(("usd", "100.50".to_string())));
}

#[test]
fn test_comma_mixed_case_currency() {
    let result = parse_comma_thousands_separator("UsD 100.50");
    assert_eq!(result, Some(("UsD", "100.50".to_string())));
}

#[test]
fn test_comma_invalid_currency_too_short() {
    let result = parse_comma_thousands_separator("US 100.50");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_currency_too_long() {
    let result = parse_comma_thousands_separator("USDA 100.50");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_currency_with_digits() {
    let result = parse_comma_thousands_separator("US1 100.50");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_no_space() {
    let result = parse_comma_thousands_separator("USD100.50");
    assert_eq!(result, None);
}

#[test]
fn test_comma_multiple_spaces() {
    // split_whitespace handles multiple spaces gracefully
    let result = parse_comma_thousands_separator("USD  100.50");
    assert_eq!(result, Some(("USD", "100.50".to_string())));
}

#[test]
fn test_comma_invalid_only_currency() {
    let result = parse_comma_thousands_separator("USD");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_only_amount() {
    let result = parse_comma_thousands_separator("100.50");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_empty() {
    let result = parse_comma_thousands_separator("");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_multiple_decimals() {
    let result = parse_comma_thousands_separator("USD 100.50.25");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_comma_after_decimal() {
    let result = parse_comma_thousands_separator("USD 100.50,25");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_wrong_grouping() {
    let result = parse_comma_thousands_separator("USD 12,34.56");
    assert_eq!(result, None);
}

#[test]
fn test_comma_invalid_ends_with_comma() {
    let result = parse_comma_thousands_separator("USD 1,234,");
    assert_eq!(result, None);
}

#[test]
fn test_comma_valid_first_group_one_digit() {
    let result = parse_comma_thousands_separator("USD 1,234");
    assert_eq!(result, Some(("USD", "1234".to_string())));
}

#[test]
fn test_comma_valid_first_group_two_digits() {
    let result = parse_comma_thousands_separator("USD 12,345");
    assert_eq!(result, Some(("USD", "12345".to_string())));
}

#[test]
fn test_comma_valid_first_group_three_digits() {
    let result = parse_comma_thousands_separator("USD 123,456");
    assert_eq!(result, Some(("USD", "123456".to_string())));
}

#[test]
fn test_comma_invalid_first_group_four_digits() {
    let result = parse_comma_thousands_separator("USD 1234,567");
    assert_eq!(result, None);
}

#[test]
fn test_comma_zero_amount() {
    let result = parse_comma_thousands_separator("USD 0");
    assert_eq!(result, Some(("USD", "0".to_string())));
}

#[test]
fn test_comma_zero_with_decimal() {
    let result = parse_comma_thousands_separator("USD 0.00");
    assert_eq!(result, Some(("USD", "0.00".to_string())));
}

#[test]
fn test_comma_amount_no_thousands_separator() {
    let result = parse_comma_thousands_separator("USD 123456.78");
    assert_eq!(result, Some(("USD", "123456.78".to_string())));
}

#[test]
fn test_comma_invalid_letter_in_amount() {
    let result = parse_comma_thousands_separator("USD 1,2a4.56");
    assert_eq!(result, None);
}

// Tests for parse_dot_thousands_separator

#[test]
fn test_dot_simple_amount() {
    let result = parse_dot_thousands_separator("EUR 100,50");
    assert_eq!(result, Some(("EUR", "100.50".to_string())));
}

#[test]
fn test_dot_simple_integer() {
    let result = parse_dot_thousands_separator("EUR 100");
    assert_eq!(result, Some(("EUR", "100".to_string())));
}

#[test]
fn test_dot_with_thousands() {
    let result = parse_dot_thousands_separator("EUR 1.234,56");
    assert_eq!(result, Some(("EUR", "1234.56".to_string())));
}

#[test]
fn test_dot_large_amount() {
    let result = parse_dot_thousands_separator("EUR 1.000.000,99");
    assert_eq!(result, Some(("EUR", "1000000.99".to_string())));
}

#[test]
fn test_dot_no_decimal() {
    let result = parse_dot_thousands_separator("EUR 1.234");
    assert_eq!(result, Some(("EUR", "1234".to_string())));
}

#[test]
fn test_dot_lowercase_currency() {
    let result = parse_dot_thousands_separator("eur 100,50");
    assert_eq!(result, Some(("eur", "100.50".to_string())));
}

#[test]
fn test_dot_mixed_case_currency() {
    let result = parse_dot_thousands_separator("EuR 100,50");
    assert_eq!(result, Some(("EuR", "100.50".to_string())));
}

#[test]
fn test_dot_invalid_currency_too_short() {
    let result = parse_dot_thousands_separator("EU 100,50");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_currency_too_long() {
    let result = parse_dot_thousands_separator("EURO 100,50");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_currency_with_digits() {
    let result = parse_dot_thousands_separator("EU1 100,50");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_no_space() {
    let result = parse_dot_thousands_separator("EUR100,50");
    assert_eq!(result, None);
}

#[test]
fn test_dot_multiple_spaces() {
    // split_whitespace handles multiple spaces gracefully
    let result = parse_dot_thousands_separator("EUR  100,50");
    assert_eq!(result, Some(("EUR", "100.50".to_string())));
}

#[test]
fn test_dot_invalid_only_currency() {
    let result = parse_dot_thousands_separator("EUR");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_only_amount() {
    let result = parse_dot_thousands_separator("100,50");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_empty() {
    let result = parse_dot_thousands_separator("");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_multiple_decimals() {
    let result = parse_dot_thousands_separator("EUR 100,50,25");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_dot_after_decimal() {
    let result = parse_dot_thousands_separator("EUR 100,50.25");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_wrong_grouping() {
    let result = parse_dot_thousands_separator("EUR 12.34,56");
    assert_eq!(result, None);
}

#[test]
fn test_dot_invalid_ends_with_dot() {
    let result = parse_dot_thousands_separator("EUR 1.234.");
    assert_eq!(result, None);
}

#[test]
fn test_dot_valid_first_group_one_digit() {
    let result = parse_dot_thousands_separator("EUR 1.234");
    assert_eq!(result, Some(("EUR", "1234".to_string())));
}

#[test]
fn test_dot_valid_first_group_two_digits() {
    let result = parse_dot_thousands_separator("EUR 12.345");
    assert_eq!(result, Some(("EUR", "12345".to_string())));
}

#[test]
fn test_dot_valid_first_group_three_digits() {
    let result = parse_dot_thousands_separator("EUR 123.456");
    assert_eq!(result, Some(("EUR", "123456".to_string())));
}

#[test]
fn test_dot_invalid_first_group_four_digits() {
    let result = parse_dot_thousands_separator("EUR 1234.567");
    assert_eq!(result, None);
}

#[test]
fn test_dot_zero_amount() {
    let result = parse_dot_thousands_separator("EUR 0");
    assert_eq!(result, Some(("EUR", "0".to_string())));
}

#[test]
fn test_dot_zero_with_decimal() {
    let result = parse_dot_thousands_separator("EUR 0,00");
    assert_eq!(result, Some(("EUR", "0.00".to_string())));
}

#[test]
fn test_dot_amount_no_thousands_separator() {
    let result = parse_dot_thousands_separator("EUR 123456,78");
    assert_eq!(result, Some(("EUR", "123456.78".to_string())));
}

#[test]
fn test_dot_invalid_letter_in_amount() {
    let result = parse_dot_thousands_separator("EUR 1.2a4,56");
    assert_eq!(result, None);
}

// Edge case tests that should work for both formats

#[test]
fn test_comma_very_large_number() {
    let result = parse_comma_thousands_separator("USD 999,999,999.99");
    assert_eq!(result, Some(("USD", "999999999.99".to_string())));
}

#[test]
fn test_dot_very_large_number() {
    let result = parse_dot_thousands_separator("EUR 999.999.999,99");
    assert_eq!(result, Some(("EUR", "999999999.99".to_string())));
}

#[test]
fn test_comma_one_digit_amount() {
    let result = parse_comma_thousands_separator("USD 5");
    assert_eq!(result, Some(("USD", "5".to_string())));
}

#[test]
fn test_dot_one_digit_amount() {
    let result = parse_dot_thousands_separator("EUR 5");
    assert_eq!(result, Some(("EUR", "5".to_string())));
}

#[test]
fn test_comma_trailing_zeros() {
    let result = parse_comma_thousands_separator("USD 100.00");
    assert_eq!(result, Some(("USD", "100.00".to_string())));
}

#[test]
fn test_dot_trailing_zeros() {
    let result = parse_dot_thousands_separator("EUR 100,00");
    assert_eq!(result, Some(("EUR", "100.00".to_string())));
}

#[test]
fn test_comma_many_decimal_places() {
    let result = parse_comma_thousands_separator("USD 123.456789");
    assert_eq!(result, Some(("USD", "123.456789".to_string())));
}

#[test]
fn test_dot_many_decimal_places() {
    let result = parse_dot_thousands_separator("EUR 123,456789");
    assert_eq!(result, Some(("EUR", "123.456789".to_string())));
}
