use crate::EUR;
use crate::GBP;
use crate::JPY;
use crate::USD;

use crate::Decimal;
use crate::Money;
use crate::fmt::{format, format_128_abs, format_decimal_abs};
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
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, "c a"), "USD 100.50");
    assert_eq!(format(money, "c"), "USD");
    assert_eq!(format(money, "a"), "100.50");
}

#[test]
fn test_format_basic_symbol() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, "sa"), "$100.50");
    assert_eq!(format(money, "s a"), "$ 100.50");
    assert_eq!(format(money, "s"), "$");
}

#[test]
fn test_format_decimal_places() {
    let money = Money::<USD>::new(123.4_f64).unwrap();

    assert_eq!(format(money, "c na"), "USD 123.40");

    let money = Money::<USD>::new(12343.8_f64).unwrap();
    assert_eq!(format(money, "c na"), "USD 12,343.80");

    // test Money always rounded to its minor unit
    let money = Money::<USD>::new(12345.29678).unwrap();
    assert_eq!(format(money, "c na"), "USD 12,345.30");
}

#[test]
fn test_format_negative_with_n_symbol() {
    let negative = Money::<USD>::new(dec!(-50.00)).unwrap();

    // 'n' should display '-' for negative amounts
    assert_eq!(format(negative, "c na"), "USD -50.00");
    assert_eq!(format(negative, "nsa"), "-$50.00");
    assert_eq!(format(negative, "n c a"), "- USD 50.00");

    // 'n' should not display anything for positive amounts
    let positive = Money::<USD>::new(dec!(50.00)).unwrap();
    assert_eq!(format(positive, "c na"), "USD 50.00");
    assert_eq!(format(positive, "nsa"), "$50.00");
}

#[test]
fn test_format_minor_units() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    // When 'm' is in format string, amount is shown in minor units
    // and 'm' itself displays the minor symbol
    assert_eq!(format(money, "c a m"), "USD 10,050 ¢");
    assert_eq!(format(money, "sa m"), "$10,050 ¢");
    assert_eq!(format(money, "m"), "¢");
}

#[test]
fn test_format_minor_units_in_minor_amount() {
    let money = Money::<USD>::new(dec!(1000.23)).unwrap();

    // When 'm' is present, amount is formatted as minor amount
    assert_eq!(format(money, "a m"), "100,023 ¢");
    assert_eq!(format(money, "c a m"), "USD 100,023 ¢");
}

#[test]
fn test_format_negative_minor_units() {
    let negative = Money::<USD>::new(dec!(-100.23)).unwrap();

    assert_eq!(format(negative, "c na m"), "USD -10,023 ¢");
    assert_eq!(format(negative, "nsa m"), "-$10,023 ¢");
}

#[test]
fn test_format_zero_amount() {
    let zero = Money::<USD>::new(dec!(0)).unwrap();

    assert_eq!(format(zero, "c a"), "USD 0.00");
    assert_eq!(format(zero, "sa"), "$0.00");
    assert_eq!(format(zero, "nsa"), "$0.00");
    assert_eq!(format(zero, "a m"), "0 ¢");
}

#[test]
fn test_format_very_large_amount() {
    let large = Money::<USD>::new(dec!(1234567890.12)).unwrap();

    assert_eq!(format(large, "c a"), "USD 1,234,567,890.12");
    assert_eq!(format(large, "sa"), "$1,234,567,890.12");
}

#[test]
fn test_format_very_small_amount() {
    let small = Money::<USD>::new(dec!(0.01)).unwrap();

    assert_eq!(format(small, "c a"), "USD 0.01");
    assert_eq!(format(small, "sa"), "$0.01");
    assert_eq!(format(small, "a m"), "1 ¢");
}

#[test]
fn test_format_escape_sequences() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

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
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    // Backslash followed by non-format symbol should keep backslash
    assert_eq!(format(money, "\\x"), "\\x");
    assert_eq!(format(money, "\\1"), "\\1");

    // Trailing backslash should be kept
    assert_eq!(format(money, "a\\"), "100.50\\");
}

#[test]
fn test_format_all_symbols_together() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, "c s a m"), "USD $ 10,050 ¢");

    let negative = Money::<USD>::new(dec!(-100.50)).unwrap();
    assert_eq!(format(negative, "n c s a m"), "- USD $ 10,050 ¢");
}

#[test]
fn test_format_empty_string() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, ""), "");
}

#[test]
fn test_format_only_spaces() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, "   "), "   ");
}

#[test]
fn test_format_with_text_literals() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

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
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

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
    let money = Money::<USD>::new(dec!(100.50)).unwrap();
    let negative = Money::<USD>::new(dec!(-50.25)).unwrap();

    // Mix literal text with format symbols (using words without format symbols)
    assert_eq!(format(money, "Buy: sa"), "Buy: $100.50");
    assert_eq!(format(money, "V\\alue: a USD"), "Value: 100.50 USD");
    assert_eq!(format(money, "c = a"), "USD = 100.50");

    // Mix escaped format symbols with actual format symbols
    assert_eq!(format(money, "\\a=a, \\c=c"), "a=100.50, c=USD");
    assert_eq!(format(money, "\\s: s, v\\alue: a"), "s: $, value: 100.50");
    assert_eq!(
        format(money, "Code \\c: c, \\a: a"),
        "Code c: USD, a: 100.50"
    );

    // Complex mixing with multiple literal and format symbols
    assert_eq!(
        format(money, "Full: sa (\\code: c)"),
        "Full: $100.50 (code: USD)"
    );

    // Negative amounts with mixed literals
    assert_eq!(format(negative, "Debt: nsa"), "Debt: -$50.25");
    assert_eq!(format(negative, "Due: c na"), "Due: USD -50.25");

    // Mix all types: regular text, escaped symbols, and format symbols
    assert_eq!(format(money, "Order #123: sa"), "Order #123: $100.50");

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
    let money_eur = Money::<EUR>::new(dec!(50.99)).unwrap();
    assert_eq!(format(money_eur, "c a"), "EUR 50,99");
    assert_eq!(format(money_eur, "sa"), "€50,99");

    // GBP
    let money_gbp = Money::<GBP>::new(dec!(75.50)).unwrap();
    assert_eq!(format(money_gbp, "c a"), "GBP 75.50");
    assert_eq!(format(money_gbp, "sa"), "£75.50");

    // JPY (no minor units)
    let money_jpy = Money::<JPY>::new(dec!(1000)).unwrap();
    assert_eq!(format(money_jpy, "c a"), "JPY 1,000");
    assert_eq!(format(money_jpy, "sa"), "¥1,000");
}

#[test]
fn test_format_rounding() {
    // USD has 2 decimal places, so should round
    let money = Money::<USD>::new(dec!(100.123)).unwrap();
    assert_eq!(format(money, "a"), "100.12");

    let money2 = Money::<USD>::new(dec!(100.126)).unwrap();
    assert_eq!(format(money2, "a"), "100.13");
}

#[test]
fn test_format_multiple_escapes() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    // "\\" escapes to "\", then "a" is treated as format symbol
    assert_eq!(format(money, "\\\\a"), "\\100.50");
    // "\\\" escapes to "\", then "\a" escapes to "a"
    assert_eq!(format(money, "\\\\\\a"), "\\a");
    // "\\\\" -> "\" escaped, then "\" escaped -> "\\"
    assert_eq!(format(money, "\\\\\\\\"), "\\\\");
}

#[test]
fn test_format_special_characters() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, "a!"), "100.50!");
    assert_eq!(format(money, "a@b#c$"), "100.50@b#USD$");
    assert_eq!(format(money, "(a)"), "(100.50)");
    assert_eq!(format(money, "[c]"), "[USD]");
}

#[test]
fn test_format_numeric_characters() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    assert_eq!(format(money, "1a2"), "1100.502");
    // Escape format symbols in text to display them literally
    assert_eq!(format(money, "Pri\\ce: a USD"), "Price: 100.50 USD");
}

#[test]
fn test_format_case_sensitivity() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();

    // Format symbols are case-sensitive (lowercase only)
    assert_eq!(format(money, "A"), "A");
    assert_eq!(format(money, "C"), "C");
    assert_eq!(format(money, "S"), "S");
    assert_eq!(format(money, "M"), "M");
    assert_eq!(format(money, "N"), "N");
}

#[test]
fn test_format_boundary_values() {
    // Maximum positive decimal
    let max_money = Money::<USD>::new(Decimal::MAX).unwrap();
    let result = format(max_money, "c a");
    assert!(result.starts_with("USD "));

    // Minimum negative decimal (close to negative max)
    let min_money = Money::<USD>::new(Decimal::MIN).unwrap();
    let result = format(min_money, "nsa");
    assert!(result.starts_with("-$"));
}

#[test]
fn test_format_one_unit() {
    let money = Money::<USD>::new(dec!(1)).unwrap();

    assert_eq!(format(money, "a"), "1.00");
    assert_eq!(format(money, "sa"), "$1.00");
    assert_eq!(format(money, "c a"), "USD 1.00");
}

#[test]
fn test_format_negative_zero() {
    // Decimal can represent negative zero
    let neg_zero = Money::<USD>::new(dec!(-0)).unwrap();

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

    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000.5").unwrap(), ",", ".", 3),
        "1,000.500"
    );
}

#[test]
fn test_format_minor_amount_overflow() {
    // Test that when minor_amount() overflows, we get "OVERFLOWED_AMOUNT"
    // Use Decimal::MAX with USD (2 decimal places)
    // Decimal::MAX * 100 will overflow i128
    let money = Money::<USD>::new(Decimal::MAX).unwrap();

    // When using 'm' in format string, it tries to convert to minor amount
    // which will overflow and return the error string
    assert_eq!(format(money, "a m"), "OVERFLOWED_AMOUNT ¢");
    assert_eq!(format(money, "c a m"), "USD OVERFLOWED_AMOUNT ¢");
}

#[test]
fn test_format_negative_symbol_with_positive_amount() {
    // Explicit test to ensure line 99 (NEGATIVE_FORMAT_SYMBOL match arm) is covered
    // when used with a positive amount (where the body doesn't execute)
    let positive = Money::<USD>::new(dec!(100.00)).unwrap();

    // 'n' should not add anything for positive amounts
    assert_eq!(format(positive, "n"), "");
    assert_eq!(format(positive, "na"), "100.00");
    assert_eq!(format(positive, "nnn"), "");

    // Also test with negative amount to ensure both branches are covered
    let negative = Money::<USD>::new(dec!(-100.00)).unwrap();
    assert_eq!(format(negative, "n"), "-");
    assert_eq!(format(negative, "na"), "-100.00");
}

#[test]
fn test_format_escape_all_format_symbols_explicitly() {
    // Explicit test to ensure line 86 (continue after escape) is covered
    // for all format symbols
    let money = Money::<USD>::new(dec!(100.00)).unwrap();

    // Test escaping each format symbol individually to ensure continue is hit
    assert_eq!(format(money, "\\a"), "a");
    assert_eq!(format(money, "\\c"), "c");
    assert_eq!(format(money, "\\s"), "s");
    assert_eq!(format(money, "\\m"), "m");
    assert_eq!(format(money, "\\n"), "n");
    assert_eq!(format(money, "\\\\"), "\\");
}
