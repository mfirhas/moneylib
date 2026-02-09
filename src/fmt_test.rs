use crate::Decimal;
use crate::fmt::{format, format_128_abs, format_decimal_abs};
use crate::{Currency, Money};
use crate::money_macros::dec;
use std::str::FromStr;

#[test]
fn test_format_with_thousands() {
    assert_eq!(format_128_abs(1000, ","), "1,000");
    assert_eq!(format_128_abs(100, ","), "100");
    assert_eq!(format_128_abs(100000, ","), "100,000");
    assert_eq!(format_128_abs(1, ","), "1");
    assert_eq!(format_128_abs(1000000, ","), "1,000,000");
    assert_eq!(format_128_abs(-1000, ","), "1,000");
    assert_eq!(format_128_abs(-100, ","), "100");
    assert_eq!(format_128_abs(0, ","), "0");
}

#[test]
fn test_format_decimal_with_thousands() {
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000").unwrap(), ",", ".", 0),
        "1,000"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("100").unwrap(), ",", ".", 0),
        "100"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("100000").unwrap(), ",", ".", 0),
        "100,000"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000.50").unwrap(), ",", ".", 0),
        "1,000.50"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1234567.89").unwrap(), ",", ".", 0),
        "1,234,567.89"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("-1000").unwrap(), ",", ".", 0),
        "1,000"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("-1000.25").unwrap(), ",", ".", 0),
        "1,000.25"
    );
}

// Tests for the main format function

#[test]
fn test_format_basic_code() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, "c a"), "USD 100.50");
    assert_eq!(format(money, "c"), "USD");
    assert_eq!(format(money, "a"), "100.50");
}

#[test]
fn test_format_basic_symbol() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, "sa"), "$100.50");
    assert_eq!(format(money, "s a"), "$ 100.50");
    assert_eq!(format(money, "s"), "$");
}

#[test]
fn test_format_negative_with_n_symbol() {
    let currency = Currency::from_iso("USD").unwrap();
    let negative = Money::new(currency, dec!(-50.00));
    
    // 'n' should display '-' for negative amounts
    assert_eq!(format(negative, "c na"), "USD -50.00");
    assert_eq!(format(negative, "nsa"), "-$50.00");
    assert_eq!(format(negative, "n c a"), "- USD 50.00");
    
    // 'n' should not display anything for positive amounts
    let positive = Money::new(currency, dec!(50.00));
    assert_eq!(format(positive, "c na"), "USD 50.00");
    assert_eq!(format(positive, "nsa"), "$50.00");
}

#[test]
fn test_format_minor_units() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // When 'm' is in format string, amount is shown in minor units
    // and 'm' itself displays the minor symbol
    assert_eq!(format(money, "c a m"), "USD 10,050 ¢");
    assert_eq!(format(money, "sa m"), "$10,050 ¢");
    assert_eq!(format(money, "m"), "¢");
}

#[test]
fn test_format_minor_units_in_minor_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1000.23));
    
    // When 'm' is present, amount is formatted as minor amount
    assert_eq!(format(money, "a m"), "100,023 ¢");
    assert_eq!(format(money, "c a m"), "USD 100,023 ¢");
}

#[test]
fn test_format_negative_minor_units() {
    let currency = Currency::from_iso("USD").unwrap();
    let negative = Money::new(currency, dec!(-100.23));
    
    assert_eq!(format(negative, "c na m"), "USD -10,023 ¢");
    assert_eq!(format(negative, "nsa m"), "-$10,023 ¢");
}

#[test]
fn test_format_zero_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let zero = Money::new(currency, dec!(0));
    
    assert_eq!(format(zero, "c a"), "USD 0.00");
    assert_eq!(format(zero, "sa"), "$0.00");
    assert_eq!(format(zero, "nsa"), "$0.00");
    assert_eq!(format(zero, "a m"), "0 ¢");
}

#[test]
fn test_format_very_large_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let large = Money::new(currency, dec!(1234567890.12));
    
    assert_eq!(format(large, "c a"), "USD 1,234,567,890.12");
    assert_eq!(format(large, "sa"), "$1,234,567,890.12");
}

#[test]
fn test_format_very_small_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let small = Money::new(currency, dec!(0.01));
    
    assert_eq!(format(small, "c a"), "USD 0.01");
    assert_eq!(format(small, "sa"), "$0.01");
    assert_eq!(format(small, "a m"), "1 ¢");
}

#[test]
fn test_format_escape_sequences() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // Escaping format symbols
    assert_eq!(format(money, "\\a"), "a");
    assert_eq!(format(money, "\\c"), "c");
    assert_eq!(format(money, "\\s"), "s");
    assert_eq!(format(money, "\\m"), "m");
    assert_eq!(format(money, "\\n"), "n");
    
    // Escaping backslash
    assert_eq!(format(money, "\\\\"), "\\");
    
    // Mixed escaping
    assert_eq!(format(money, "\\a a"), "a 100.50");
    assert_eq!(format(money, "c \\c a"), "USD c 100.50");
}

#[test]
fn test_format_backslash_without_format_symbol() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // Backslash followed by non-format symbol should keep backslash
    assert_eq!(format(money, "\\x"), "\\x");
    assert_eq!(format(money, "\\1"), "\\1");
    
    // Trailing backslash should be kept
    assert_eq!(format(money, "a\\"), "100.50\\");
}

#[test]
fn test_format_all_symbols_together() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, "c s a m"), "USD $ 10,050 ¢");
    
    let negative = Money::new(currency, dec!(-100.50));
    assert_eq!(format(negative, "n c s a m"), "- USD $ 10,050 ¢");
}

#[test]
fn test_format_empty_string() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, ""), "");
}

#[test]
fn test_format_only_spaces() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, "   "), "   ");
}

#[test]
fn test_format_with_text_literals() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // Escaping format symbols allows literal display
    assert_eq!(format(money, "\\code: c"), "code: USD");
    assert_eq!(format(money, "\\sy\\mbol: s"), "symbol: $");
    assert_eq!(format(money, "a USD"), "100.50 USD");
    
    // Complex example with multiple escapes
    assert_eq!(format(money, "\\c=c, \\s=s, \\a=a"), "c=USD, s=$, a=100.50");
}

#[test]
fn test_format_literal_characters_including_format_symbols() {
    // Test case 1: We can insert literal characters (1 or many), even if it includes the format symbols
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // Single literal format symbol characters
    assert_eq!(format(money, "\\a"), "a");
    assert_eq!(format(money, "\\c"), "c");
    assert_eq!(format(money, "\\s"), "s");
    assert_eq!(format(money, "\\m"), "m");
    assert_eq!(format(money, "\\n"), "n");
    
    // Multiple consecutive literal format symbols
    assert_eq!(format(money, "\\a\\c\\s\\m\\n"), "acsmn");
    
    // Literal format symbols with other literal text (no accidental format symbols)
    assert_eq!(format(money, "word: "), "word: ");
    assert_eq!(format(money, "text \\a\\nd \\more"), "text and more");
    assert_eq!(format(money, "letter \\c here"), "letter c here");
    
    // All format symbols as literals
    assert_eq!(
        format(money, "Five: \\a, \\c, \\s, \\m, \\n"),
        "Five: a, c, s, m, n"
    );
}

#[test]
fn test_format_mix_literals_with_format_symbols() {
    // Test case 2: We can mix literal characters with format symbols
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    let negative = Money::new(currency, dec!(-50.25));
    
    // Mix literal text with format symbols (using words without format symbols)
    assert_eq!(format(money, "Buy: sa"), "Buy: $100.50");
    assert_eq!(format(money, "V\\alue: a USD"), "Value: 100.50 USD");
    assert_eq!(format(money, "c = a"), "USD = 100.50");
    
    // Mix escaped format symbols with actual format symbols
    assert_eq!(format(money, "\\a=a, \\c=c"), "a=100.50, c=USD");
    assert_eq!(format(money, "\\s: s, v\\alue: a"), "s: $, value: 100.50");
    assert_eq!(format(money, "Code \\c: c, \\a: a"), "Code c: USD, a: 100.50");
    
    // Complex mixing with multiple literal and format symbols
    assert_eq!(
        format(money, "Full: sa (\\code: c)"),
        "Full: $100.50 (code: USD)"
    );
    
    // Negative amounts with mixed literals
    assert_eq!(format(negative, "Debt: nsa"), "Debt: -$50.25");
    assert_eq!(format(negative, "Due: c na"), "Due: USD -50.25");
    
    // Mix all types: regular text, escaped symbols, and format symbols
    assert_eq!(
        format(money, "Order #123: sa"),
        "Order #123: $100.50"
    );
    
    // Complex real-world example
    // Note: When 'm' is present in the format string, ALL 'a' symbols display the minor amount
    assert_eq!(
        format(money, "Full: c a (\\mi\\nor u\\nit\\s: a m)"),
        "Full: USD 10,050 (minor units: 10,050 ¢)"
    );
}

#[test]
fn test_format_different_currencies() {
    // EUR
    let eur = Currency::from_iso("EUR").unwrap();
    let money_eur = Money::new(eur, dec!(50.99));
    assert_eq!(format(money_eur, "c a"), "EUR 50.99");
    assert_eq!(format(money_eur, "sa"), "€50.99");
    
    // GBP
    let gbp = Currency::from_iso("GBP").unwrap();
    let money_gbp = Money::new(gbp, dec!(75.50));
    assert_eq!(format(money_gbp, "c a"), "GBP 75.50");
    assert_eq!(format(money_gbp, "sa"), "£75.50");
    
    // JPY (no minor units)
    let jpy = Currency::from_iso("JPY").unwrap();
    let money_jpy = Money::new(jpy, dec!(1000));
    assert_eq!(format(money_jpy, "c a"), "JPY 1,000");
    assert_eq!(format(money_jpy, "sa"), "¥1,000");
}

#[test]
fn test_format_with_custom_separators() {
    // Create currency with different separators
    let mut currency = Currency::from_iso("USD").unwrap();
    currency.set_thousand_separator(".");
    currency.set_decimal_separator(",");
    
    let money = Money::new(currency, dec!(1234.56));
    
    assert_eq!(format(money, "a"), "1.234,56");
    assert_eq!(format(money, "sa"), "$1.234,56");
}

#[test]
fn test_format_rounding() {
    let currency = Currency::from_iso("USD").unwrap();
    
    // USD has 2 decimal places, so should round
    let money = Money::new(currency, dec!(100.123));
    assert_eq!(format(money, "a"), "100.12");
    
    let money2 = Money::new(currency, dec!(100.126));
    assert_eq!(format(money2, "a"), "100.13");
}

#[test]
fn test_format_multiple_escapes() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // "\\" escapes to "\", then "a" is treated as format symbol
    assert_eq!(format(money, "\\\\a"), "\\100.50");
    // "\\\" escapes to "\", then "\a" escapes to "a"
    assert_eq!(format(money, "\\\\\\a"), "\\a");
    // "\\\\" -> "\" escaped, then "\" escaped -> "\\"
    assert_eq!(format(money, "\\\\\\\\"), "\\\\");
}

#[test]
fn test_format_special_characters() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, "a!"), "100.50!");
    assert_eq!(format(money, "a@b#c$"), "100.50@b#USD$");
    assert_eq!(format(money, "(a)"), "(100.50)");
    assert_eq!(format(money, "[c]"), "[USD]");
}

#[test]
fn test_format_numeric_characters() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    assert_eq!(format(money, "1a2"), "1100.502");
    // Escape format symbols in text to display them literally
    assert_eq!(format(money, "Pri\\ce: a USD"), "Price: 100.50 USD");
}

#[test]
fn test_format_case_sensitivity() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    
    // Format symbols are case-sensitive (lowercase only)
    assert_eq!(format(money, "A"), "A");
    assert_eq!(format(money, "C"), "C");
    assert_eq!(format(money, "S"), "S");
    assert_eq!(format(money, "M"), "M");
    assert_eq!(format(money, "N"), "N");
}

#[test]
fn test_format_boundary_values() {
    let currency = Currency::from_iso("USD").unwrap();
    
    // Maximum positive decimal
    let max_money = Money::new(currency, Decimal::MAX);
    let result = format(max_money, "c a");
    assert!(result.starts_with("USD "));
    
    // Minimum negative decimal (close to negative max)
    let min_money = Money::new(currency, Decimal::MIN);
    let result = format(min_money, "nsa");
    assert!(result.starts_with("-$"));
}

#[test]
fn test_format_one_unit() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1));
    
    assert_eq!(format(money, "a"), "1.00");
    assert_eq!(format(money, "sa"), "$1.00");
    assert_eq!(format(money, "c a"), "USD 1.00");
}

#[test]
fn test_format_negative_zero() {
    let currency = Currency::from_iso("USD").unwrap();
    // Decimal can represent negative zero
    let neg_zero = Money::new(currency, dec!(-0));
    
    // Should not show negative sign for zero
    let result = format(neg_zero, "nsa");
    assert!(!result.contains('-') && result == "$0.00");
}

#[test]
fn test_format_decimal_abs_with_minor_unit() {
    // Test that when fractional part is None and minor_unit > 0, zeros are appended
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000").unwrap(), ",", ".", 2),
        "1,000.00"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("100").unwrap(), ",", ".", 2),
        "100.00"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("50").unwrap(), ",", ".", 3),
        "50.000"
    );
    
    // Test that when fractional part exists, it's preserved
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000.50").unwrap(), ",", ".", 2),
        "1,000.50"
    );
    
    // Test with minor_unit = 0 (no zeros appended)
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000").unwrap(), ",", ".", 0),
        "1,000"
    );
    
    // Test that existing fractional parts shorter than minor_unit are preserved as-is
    // Note: In practice, Money objects are rounded to the currency's minor_unit,
    // so this scenario represents the actual usage where the Decimal has already
    // been rounded to the correct precision by rust_decimal's round_dp_with_strategy
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000.5").unwrap(), ",", ".", 3),
        "1,000.5"
    );
}
