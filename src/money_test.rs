use crate::{
    BaseMoney, BaseOps, Currency, CustomMoney, Money, MoneyError, RoundingStrategy,
};
use rust_decimal_macros::dec;
use std::str::FromStr;

// ==================== Money::new() Tests ====================

#[test]
fn test_new_with_usd() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    assert_eq!(money.currency(), currency);
    assert_eq!(money.amount(), dec!(100.50));
}

#[test]
fn test_new_with_zero_amount() {
    let currency = Currency::from_iso("EUR").unwrap();
    let money = Money::new(currency, dec!(0));
    assert_eq!(money.amount(), dec!(0));
}

#[test]
fn test_new_with_negative_amount() {
    let currency = Currency::from_iso("GBP").unwrap();
    let money = Money::new(currency, dec!(-50.25));
    assert_eq!(money.amount(), dec!(-50.25));
}

#[test]
fn test_new_with_large_amount() {
    let currency = Currency::from_iso("JPY").unwrap();
    let money = Money::new(currency, dec!(999999999.99));
    assert_eq!(money.amount(), dec!(999999999.99));
}

// ==================== PartialEq Tests ====================

#[test]
fn test_partial_eq_same_currency_same_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.50));
    let money2 = Money::new(currency, dec!(100.50));
    assert_eq!(money1, money2);
}

#[test]
fn test_partial_eq_same_currency_different_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.50));
    let money2 = Money::new(currency, dec!(100.51));
    assert_ne!(money1, money2);
}

#[test]
fn test_partial_eq_different_currency_same_amount() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.50));
    let money2 = Money::new(eur, dec!(100.50));
    assert_ne!(money1, money2);
}

#[test]
fn test_partial_eq_different_currency_different_amount() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.50));
    let money2 = Money::new(eur, dec!(200.75));
    assert_ne!(money1, money2);
}

#[test]
fn test_partial_eq_negative_amounts() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(-100.50));
    let money2 = Money::new(currency, dec!(-100.50));
    assert_eq!(money1, money2);
}

// ==================== PartialOrd Tests ====================

#[test]
fn test_partial_ord_same_currency_less_than() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(200.00));
    assert!(money1 < money2);
}

#[test]
fn test_partial_ord_same_currency_greater_than() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(200.00));
    let money2 = Money::new(currency, dec!(100.00));
    assert!(money1 > money2);
}

#[test]
fn test_partial_ord_same_currency_equal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(100.00));
    assert!(money1 <= money2);
    assert!(money1 >= money2);
}

#[test]
fn test_partial_ord_different_currency_returns_none() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(100.00));
    assert_eq!(money1.partial_cmp(&money2), None);
}

#[test]
fn test_partial_ord_negative_amounts() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(-200.00));
    let money2 = Money::new(currency, dec!(-100.00));
    assert!(money1 < money2);
}

// ==================== FromStr Tests ====================

#[test]
fn test_from_str_usd_comma_separator() {
    let money = Money::from_str("USD 1,234.56").unwrap();
    assert_eq!(money.currency().code(), "USD");
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn test_from_str_eur_dot_separator() {
    let money = Money::from_str("EUR 1.234,56").unwrap();
    assert_eq!(money.currency().code(), "EUR");
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn test_from_str_simple_amount() {
    let money = Money::from_str("USD 100.50").unwrap();
    assert_eq!(money.currency().code(), "USD");
    assert_eq!(money.amount(), dec!(100.50));
}

#[test]
fn test_from_str_large_amount_with_commas() {
    let money = Money::from_str("USD 1,000,000.99").unwrap();
    assert_eq!(money.amount(), dec!(1000000.99));
}

#[test]
fn test_from_str_large_amount_with_dots() {
    let money = Money::from_str("EUR 1.000.000,99").unwrap();
    assert_eq!(money.amount(), dec!(1000000.99));
}

#[test]
fn test_from_str_zero_amount() {
    let money = Money::from_str("USD 0.00").unwrap();
    assert_eq!(money.amount(), dec!(0.00));
}

#[test]
fn test_from_str_zero_amount_with_dec_0() {
    let money = Money::from_str("USD 0.00").unwrap();
    assert_eq!(money.amount(), dec!(0));
}

#[test]
fn test_from_str_zero_amount_variations() {
    // Test 0.00 money compared with dec!(0)
    let money1 = Money::from_str("USD 0.00").unwrap();
    assert_eq!(money1.amount(), dec!(0));
    
    // Test dec!(0) compared with 0.00
    let currency = Currency::from_iso("USD").unwrap();
    let money2 = Money::new(currency, dec!(0));
    assert_eq!(money2.amount(), dec!(0.00));
    
    // Test with more zeros after decimal point
    let money3 = Money::from_str("USD 0.000").unwrap();
    assert_eq!(money3.amount(), dec!(0.000));
    
    let money4 = Money::from_str("USD 0.0000").unwrap();
    assert_eq!(money4.amount(), dec!(0.0000));
    
    let money5 = Money::from_str("USD 0.00000").unwrap();
    assert_eq!(money5.amount(), dec!(0.00000));
    
    // All should be equal
    assert_eq!(money1.amount(), money2.amount());
    assert_eq!(money2.amount(), money3.amount());
    assert_eq!(money3.amount(), money4.amount());
    assert_eq!(money4.amount(), money5.amount());
}

#[test]
fn test_from_str_with_whitespace() {
    let money = Money::from_str("  USD 100.50  ").unwrap();
    assert_eq!(money.amount(), dec!(100.50));
}

#[test]
fn test_from_str_rounding_to_minor_unit() {
    let money = Money::from_str("USD 100.999").unwrap();
    // Should round to 2 decimal places for USD
    assert_eq!(money.amount(), dec!(101.00));
}

#[test]
fn test_from_str_invalid_no_space() {
    let result = Money::from_str("USD100.50");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ParseStr));
}

#[test]
fn test_from_str_invalid_currency() {
    let result = Money::from_str("XYZ 100.50");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::InvalidCurrency));
}

#[test]
fn test_from_str_invalid_amount() {
    let result = Money::from_str("USD abc");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ParseStr));
}

#[test]
fn test_from_str_empty_string() {
    let result = Money::from_str("");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ParseStr));
}

#[test]
fn test_from_str_only_currency() {
    let result = Money::from_str("USD");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ParseStr));
}

#[test]
fn test_from_str_only_amount() {
    let result = Money::from_str("100.50");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ParseStr));
}

#[test]
fn test_from_str_too_many_parts() {
    let result = Money::from_str("USD 100.50 extra");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ParseStr));
}

#[test]
fn test_from_str_no_decimal_separator() {
    // This actually parses successfully if it matches the thousands separator regex
    let result = Money::from_str("USD 100.0");
    assert!(result.is_ok());
    if let Ok(money) = result {
        assert_eq!(money.amount(), dec!(100.0));
    }
}

// ==================== Display Tests ====================

#[test]
fn test_display_format() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1234.56));
    let display_str = format!("{}", money);
    assert_eq!(display_str, "USD 1,234.56");
}

#[test]
fn test_display_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-1234.56));
    let display_str = format!("{}", money);
    assert_eq!(display_str, "USD -1,234.56");
}

#[test]
fn test_display_zero() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0.00));
    let display_str = format!("{}", money);
    assert_eq!(display_str, "USD 0.00");
}

// ==================== BaseMoney Trait Tests ====================

#[test]
fn test_base_money_currency() {
    let currency = Currency::from_iso("EUR").unwrap();
    let money = Money::new(currency, dec!(100.50));
    assert_eq!(money.currency(), currency);
}

#[test]
fn test_base_money_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.45));
    assert_eq!(money.amount(), dec!(123.45));
}

#[test]
fn test_base_money_round() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.456));
    let rounded = money.round();
    assert_eq!(rounded.amount(), dec!(123.46));
}

#[test]
fn test_base_money_round_jpy_no_minor_unit() {
    let currency = Currency::from_iso("JPY").unwrap();
    let money = Money::new(currency, dec!(123.56));
    let rounded = money.round();
    assert_eq!(rounded.amount(), dec!(124));
}

#[test]
fn test_base_money_name() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert_eq!(money.name(), "United States dollar");
}

#[test]
fn test_base_money_symbol() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert_eq!(money.symbol(), "$");
}

#[test]
fn test_base_money_code() {
    let currency = Currency::from_iso("EUR").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert_eq!(money.code(), "EUR");
}

#[test]
fn test_base_money_numeric_code() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert_eq!(money.numeric_code(), 840);
}

#[test]
fn test_base_money_minor_unit() {
    let usd_currency = Currency::from_iso("USD").unwrap();
    let usd_money = Money::new(usd_currency, dec!(100.00));
    assert_eq!(usd_money.minor_unit(), 2);

    let jpy_currency = Currency::from_iso("JPY").unwrap();
    let jpy_money = Money::new(jpy_currency, dec!(100));
    assert_eq!(jpy_money.minor_unit(), 0);
}

#[test]
fn test_base_money_minor_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.45));
    assert_eq!(money.minor_amount().unwrap(), 12345);
}

#[test]
fn test_base_money_minor_amount_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-123.45));
    assert_eq!(money.minor_amount().unwrap(), -12345);
}

#[test]
fn test_base_money_minor_amount_jpy() {
    let currency = Currency::from_iso("JPY").unwrap();
    let money = Money::new(currency, dec!(123));
    assert_eq!(money.minor_amount().unwrap(), 123);
}

#[test]
fn test_base_money_thousand_separator() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert_eq!(money.thousand_separator(), ",");
}

#[test]
fn test_base_money_decimal_separator() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert_eq!(money.decimal_separator(), ".");
}

#[test]
fn test_base_money_is_zero_true() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0));
    assert!(money.is_zero());
}

#[test]
fn test_base_money_is_zero_false() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0.01));
    assert!(!money.is_zero());
}

#[test]
fn test_base_money_is_positive() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    assert!(money.is_positive());
}

#[test]
fn test_base_money_is_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-100.00));
    assert!(money.is_negative());
}

#[test]
fn test_base_money_format_code() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1234.56));
    assert_eq!(money.format_code(), "USD 1,234.56");
}

#[test]
fn test_base_money_format_code_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-1234.56));
    assert_eq!(money.format_code(), "USD -1,234.56");
}

#[test]
fn test_base_money_format_symbol() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1234.56));
    assert_eq!(money.format_symbol(), "$1,234.56");
}

#[test]
fn test_base_money_format_code_minor() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1234.56));
    let formatted = money.format_code_minor().unwrap();
    assert!(formatted.contains("USD"));
    assert!(formatted.contains("123,456"));
}

#[test]
fn test_base_money_format_code_minor_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-1234.56));
    let formatted = money.format_code_minor().unwrap();
    assert!(formatted.contains("USD"));
    assert!(formatted.contains("-123,456"));
}

#[test]
fn test_base_money_format_symbol_minor() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(1234.56));
    let formatted = money.format_symbol_minor().unwrap();
    assert!(formatted.contains("$"));
    assert!(formatted.contains("123,456"));
}

#[test]
fn test_base_money_display() {
    let currency = Currency::from_iso("EUR").unwrap();
    let money = Money::new(currency, dec!(1234.56));
    assert_eq!(money.display(), "EUR 1,234.56");
}

#[test]
fn test_base_money_countries() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let countries = money.countries();
    assert!(countries.is_some());
}

// ==================== BaseOps Trait Tests ====================

#[test]
fn test_base_ops_abs_positive() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.50));
    let abs_money = money.abs();
    assert_eq!(abs_money.amount(), dec!(100.50));
}

#[test]
fn test_base_ops_abs_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-100.50));
    let abs_money = money.abs();
    assert_eq!(abs_money.amount(), dec!(100.50));
}

#[test]
fn test_base_ops_abs_zero() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0));
    let abs_money = money.abs();
    assert_eq!(abs_money.amount(), dec!(0));
}

#[test]
fn test_base_ops_min() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(200.00));
    let min_money = money1.min(money2);
    assert_eq!(min_money.amount(), dec!(100.00));
}

#[test]
fn test_base_ops_min_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(-100.00));
    let money2 = Money::new(currency, dec!(200.00));
    let min_money = money1.min(money2);
    assert_eq!(min_money.amount(), dec!(-100.00));
}

#[test]
fn test_base_ops_max() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(200.00));
    let max_money = money1.max(money2);
    assert_eq!(max_money.amount(), dec!(200.00));
}

#[test]
fn test_base_ops_max_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(-100.00));
    let money2 = Money::new(currency, dec!(200.00));
    let max_money = money1.max(money2);
    assert_eq!(max_money.amount(), dec!(200.00));
}

#[test]
fn test_base_ops_clamp_within_range() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(150.00));
    let clamped = money.clamp(dec!(100.00), dec!(200.00));
    assert_eq!(clamped.amount(), dec!(150.00));
}

#[test]
fn test_base_ops_clamp_below_range() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(50.00));
    let clamped = money.clamp(dec!(100.00), dec!(200.00));
    assert_eq!(clamped.amount(), dec!(100.00));
}

#[test]
fn test_base_ops_clamp_above_range() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(250.00));
    let clamped = money.clamp(dec!(100.00), dec!(200.00));
    assert_eq!(clamped.amount(), dec!(200.00));
}

#[test]
fn test_base_ops_add_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.add(dec!(50.00)).unwrap();
    assert_eq!(result.amount(), dec!(150.00));
}

#[test]
fn test_base_ops_add_decimal_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.add(dec!(-50.00)).unwrap();
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_base_ops_add_decimal_rounds() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.add(dec!(0.005)).unwrap();
    // Banker's rounding: 0.005 rounds to 0.00 (rounds to nearest even)
    assert_eq!(result.amount(), dec!(100.00));
}

#[test]
fn test_base_ops_sub_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.sub(dec!(50.00)).unwrap();
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_base_ops_sub_decimal_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.sub(dec!(-50.00)).unwrap();
    assert_eq!(result.amount(), dec!(150.00));
}

#[test]
fn test_base_ops_sub_decimal_rounds() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.sub(dec!(0.005)).unwrap();
    assert_eq!(result.amount(), dec!(100.00));
}

#[test]
fn test_base_ops_mul_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.mul(dec!(2.5)).unwrap();
    assert_eq!(result.amount(), dec!(250.00));
}

#[test]
fn test_base_ops_mul_decimal_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.mul(dec!(-2.0)).unwrap();
    assert_eq!(result.amount(), dec!(-200.00));
}

#[test]
fn test_base_ops_mul_decimal_rounds() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.mul(dec!(1.005)).unwrap();
    assert_eq!(result.amount(), dec!(100.50));
}

#[test]
fn test_base_ops_div_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.div(dec!(2.0)).unwrap();
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_base_ops_div_decimal_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.div(dec!(-2.0)).unwrap();
    assert_eq!(result.amount(), dec!(-50.00));
}

#[test]
fn test_base_ops_div_decimal_rounds() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.div(dec!(3.0)).unwrap();
    assert_eq!(result.amount(), dec!(33.33));
}

#[test]
fn test_base_ops_div_decimal_zero_error() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.div(dec!(0));
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MoneyError::ArithmeticOverflow));
}

// ==================== CustomMoney Trait Tests ====================

#[test]
fn test_custom_money_set_thousand_separator() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money = Money::new(currency, dec!(1234.56));
    money.set_thousand_separator(".");
    assert_eq!(money.thousand_separator(), ".");
}

#[test]
fn test_custom_money_set_decimal_separator() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money = Money::new(currency, dec!(1234.56));
    money.set_decimal_separator(",");
    assert_eq!(money.decimal_separator(), ",");
}

#[test]
fn test_custom_money_round_with_bankers_rounding() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.456));
    let rounded = money.round_with(2, RoundingStrategy::BankersRounding);
    assert_eq!(rounded.amount(), dec!(123.46));
}

#[test]
fn test_custom_money_round_with_half_up() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.445));
    let rounded = money.round_with(2, RoundingStrategy::HalfUp);
    assert_eq!(rounded.amount(), dec!(123.45));
}

#[test]
fn test_custom_money_round_with_half_down() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.445));
    let rounded = money.round_with(2, RoundingStrategy::HalfDown);
    assert_eq!(rounded.amount(), dec!(123.44));
}

#[test]
fn test_custom_money_round_with_ceil() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.441));
    let rounded = money.round_with(2, RoundingStrategy::Ceil);
    assert_eq!(rounded.amount(), dec!(123.45));
}

#[test]
fn test_custom_money_round_with_floor() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(123.449));
    let rounded = money.round_with(2, RoundingStrategy::Floor);
    assert_eq!(rounded.amount(), dec!(123.44));
}

// ==================== Operator Tests (Money + Money) ====================

#[test]
fn test_add_money_to_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(50.00));
    let result = money1 + money2;
    assert_eq!(result.amount(), dec!(150.00));
}

#[test]
fn test_add_money_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(-50.00));
    let result = money1 + money2;
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
#[should_panic(expected = "currency mismatch for addition operation")]
fn test_add_money_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    let _ = money1 + money2;
}

#[test]
fn test_sub_money_from_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(50.00));
    let result = money1 - money2;
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_sub_money_negative_result() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(50.00));
    let money2 = Money::new(currency, dec!(100.00));
    let result = money1 - money2;
    assert_eq!(result.amount(), dec!(-50.00));
}

#[test]
#[should_panic(expected = "currency mismatch for substraction operation")]
fn test_sub_money_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    let _ = money1 - money2;
}

#[test]
fn test_mul_money_by_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(10.00));
    let money2 = Money::new(currency, dec!(5.00));
    let result = money1 * money2;
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_mul_money_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(10.00));
    let money2 = Money::new(currency, dec!(-5.00));
    let result = money1 * money2;
    assert_eq!(result.amount(), dec!(-50.00));
}

#[test]
#[should_panic(expected = "currency mismatch for multiplication operation")]
fn test_mul_money_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    let _ = money1 * money2;
}

#[test]
fn test_div_money_by_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(5.00));
    let result = money1 / money2;
    assert_eq!(result.amount(), dec!(20.00));
}

#[test]
fn test_div_money_negative() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(-5.00));
    let result = money1 / money2;
    assert_eq!(result.amount(), dec!(-20.00));
}

#[test]
#[should_panic(expected = "divisor must not be zero")]
fn test_div_money_by_zero_panic() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(0));
    let _ = money1 / money2;
}

#[test]
#[should_panic(expected = "currency mismatch for division operation")]
fn test_div_money_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    let _ = money1 / money2;
}

// ==================== Operator Tests (Money + Decimal) ====================

#[test]
fn test_add_decimal_to_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money + dec!(50.00);
    assert_eq!(result.amount(), dec!(150.00));
}

#[test]
fn test_sub_decimal_from_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money - dec!(50.00);
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_mul_money_by_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money * dec!(2.5);
    assert_eq!(result.amount(), dec!(250.00));
}

#[test]
fn test_div_money_by_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money / dec!(2.0);
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
#[should_panic(expected = "divisor must not be zero")]
fn test_div_money_by_decimal_zero_panic() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let _ = money / dec!(0);
}

// ==================== Operator Tests (Decimal + Money) ====================

#[test]
fn test_add_money_to_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = dec!(50.00) + money;
    assert_eq!(result.amount(), dec!(150.00));
}

#[test]
fn test_sub_money_from_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(50.00));
    let result = dec!(100.00) - money;
    assert_eq!(result.amount(), dec!(50.00));
}

#[test]
fn test_mul_decimal_by_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = dec!(2.5) * money;
    assert_eq!(result.amount(), dec!(250.00));
}

#[test]
fn test_div_decimal_by_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(5.00));
    let result = dec!(100.00) / money;
    assert_eq!(result.amount(), dec!(20.00));
}

#[test]
#[should_panic(expected = "divisor must not be zero")]
fn test_div_decimal_by_money_zero_panic() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0));
    let _ = dec!(100.00) / money;
}

// ==================== Assignment Operator Tests ====================

#[test]
fn test_add_assign_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(50.00));
    money1 += money2;
    assert_eq!(money1.amount(), dec!(150.00));
}

#[test]
#[should_panic(expected = "currency mismatch for add assign operation")]
fn test_add_assign_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let mut money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    money1 += money2;
}

#[test]
fn test_sub_assign_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(50.00));
    money1 -= money2;
    assert_eq!(money1.amount(), dec!(50.00));
}

#[test]
#[should_panic(expected = "currency mismatch for sub assign operation")]
fn test_sub_assign_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let mut money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    money1 -= money2;
}

#[test]
fn test_mul_assign_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money1 = Money::new(currency, dec!(10.00));
    let money2 = Money::new(currency, dec!(5.00));
    money1 *= money2;
    assert_eq!(money1.amount(), dec!(50.00));
}

#[test]
#[should_panic(expected = "currency mismatch for mul assign operation")]
fn test_mul_assign_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let mut money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    money1 *= money2;
}

#[test]
fn test_div_assign_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(5.00));
    money1 /= money2;
    assert_eq!(money1.amount(), dec!(20.00));
}

#[test]
#[should_panic(expected = "divisor must not be zero")]
fn test_div_assign_zero_panic() {
    let currency = Currency::from_iso("USD").unwrap();
    let mut money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(0));
    money1 /= money2;
}

#[test]
#[should_panic(expected = "currency mismatch for div assign operation")]
fn test_div_assign_different_currencies_panic() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let mut money1 = Money::new(usd, dec!(100.00));
    let money2 = Money::new(eur, dec!(50.00));
    money1 /= money2;
}

// ==================== Negation Operator Tests ====================

#[test]
fn test_neg_positive_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let negated = -money;
    assert_eq!(negated.amount(), dec!(-100.00));
}

#[test]
fn test_neg_negative_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-100.00));
    let negated = -money;
    assert_eq!(negated.amount(), dec!(100.00));
}

#[test]
fn test_neg_zero_money() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0));
    let negated = -money;
    assert_eq!(negated.amount(), dec!(0));
}

// ==================== Clone and Copy Tests ====================

#[test]
fn test_clone() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = money1.clone();
    assert_eq!(money1, money2);
    assert_eq!(money1.amount(), money2.amount());
    assert_eq!(money1.currency(), money2.currency());
}

#[test]
fn test_copy() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = money1; // Copy happens here
    // Both should still be usable
    assert_eq!(money1.amount(), dec!(100.00));
    assert_eq!(money2.amount(), dec!(100.00));
}

// ==================== Debug Tests ====================

#[test]
fn test_debug() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let debug_str = format!("{:?}", money);
    assert!(debug_str.contains("Money"));
}

// ==================== Edge Cases and Complex Scenarios ====================

#[test]
fn test_very_large_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(999999999999999.99));
    assert_eq!(money.amount(), dec!(999999999999999.99));
}

#[test]
fn test_very_small_decimal() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0.001));
    let rounded = money.round();
    assert_eq!(rounded.amount(), dec!(0.00));
}

#[test]
fn test_chain_operations() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(50.00));
    let money3 = Money::new(currency, dec!(25.00));
    let result = money1 + money2 - money3;
    assert_eq!(result.amount(), dec!(125.00));
}

#[test]
fn test_complex_calculation() {
    let currency = Currency::from_iso("USD").unwrap();
    let base = Money::new(currency, dec!(100.00));
    let tax_rate = dec!(1.15);
    let discount = dec!(10.00);
    let result = (base * tax_rate) - discount;
    assert_eq!(result.amount(), dec!(105.00));
}

#[test]
fn test_jpy_no_decimal_places() {
    let currency = Currency::from_iso("JPY").unwrap();
    let money = Money::new(currency, dec!(100.50));
    let rounded = money.round();
    // Banker's rounding: 100.50 rounds to 100 (rounds to nearest even)
    assert_eq!(rounded.amount(), dec!(100));
}

#[test]
fn test_bhd_three_decimal_places() {
    let currency = Currency::from_iso("BHD").unwrap();
    let money = Money::new(currency, dec!(100.1234));
    let rounded = money.round();
    assert_eq!(rounded.amount(), dec!(100.123));
}

#[test]
fn test_format_with_different_separators() {
    let currency = Currency::from_iso("EUR").unwrap();
    let mut money = Money::new(currency, dec!(1234.56));
    money.set_thousand_separator(".");
    money.set_decimal_separator(",");
    let formatted = money.format_code();
    assert!(formatted.contains("1.234"));
    assert!(formatted.contains("56"));
}

#[test]
fn test_parse_and_format_roundtrip() {
    let original_str = "USD 1,234.56";
    let money = Money::from_str(original_str).unwrap();
    let formatted = money.format_code();
    assert_eq!(formatted, original_str);
}

#[test]
fn test_equality_after_rounding() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.004));
    let money2 = Money::new(currency, dec!(100.005));
    assert_ne!(money1, money2);
    let rounded1 = money1.round();
    let rounded2 = money2.round();
    assert_eq!(rounded1, rounded2); // Both should round to 100.00
}

#[test]
fn test_minor_amount_with_three_decimal_currency() {
    let currency = Currency::from_iso("BHD").unwrap();
    let money = Money::new(currency, dec!(100.123));
    assert_eq!(money.minor_amount().unwrap(), 100123);
}

#[test]
fn test_multiple_operations_maintain_precision() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(100.00));
    let result = money.mul(dec!(1.1)).unwrap();
    let result = result.div(dec!(1.1)).unwrap();
    assert_eq!(result.amount(), dec!(100.00));
}

#[test]
fn test_zero_amount_operations() {
    let currency = Currency::from_iso("USD").unwrap();
    let zero = Money::new(currency, dec!(0));
    let hundred = Money::new(currency, dec!(100.00));
    
    let result = zero + hundred;
    assert_eq!(result.amount(), dec!(100.00));
    
    let result = hundred - hundred;
    assert_eq!(result.amount(), dec!(0));
    
    let result = zero * hundred;
    assert_eq!(result.amount(), dec!(0));
}

#[test]
fn test_negative_operations() {
    let currency = Currency::from_iso("USD").unwrap();
    let negative = Money::new(currency, dec!(-50.00));
    let positive = Money::new(currency, dec!(100.00));
    
    let result = negative + positive;
    assert_eq!(result.amount(), dec!(50.00));
    
    let result = positive + negative;
    assert_eq!(result.amount(), dec!(50.00));
    
    let result = negative - positive;
    assert_eq!(result.amount(), dec!(-150.00));
}

#[test]
fn test_abs_doesnt_change_original() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(-100.00));
    let abs_money = money.abs();
    assert_eq!(money.amount(), dec!(-100.00));
    assert_eq!(abs_money.amount(), dec!(100.00));
}

#[test]
fn test_min_max_with_equal_values() {
    let currency = Currency::from_iso("USD").unwrap();
    let money1 = Money::new(currency, dec!(100.00));
    let money2 = Money::new(currency, dec!(100.00));
    
    let min_result = money1.min(money2);
    assert_eq!(min_result.amount(), dec!(100.00));
    
    let max_result = money1.max(money2);
    assert_eq!(max_result.amount(), dec!(100.00));
}

#[test]
fn test_clamp_at_boundaries() {
    let currency = Currency::from_iso("USD").unwrap();
    
    let money_at_min = Money::new(currency, dec!(100.00));
    let clamped = money_at_min.clamp(dec!(100.00), dec!(200.00));
    assert_eq!(clamped.amount(), dec!(100.00));
    
    let money_at_max = Money::new(currency, dec!(200.00));
    let clamped = money_at_max.clamp(dec!(100.00), dec!(200.00));
    assert_eq!(clamped.amount(), dec!(200.00));
}

#[test]
fn test_multiple_separators_in_parsing() {
    let money = Money::from_str("USD 1,234,567.89").unwrap();
    assert_eq!(money.amount(), dec!(1234567.89));
    
    let money = Money::from_str("EUR 1.234.567,89").unwrap();
    assert_eq!(money.amount(), dec!(1234567.89));
}

#[test]
fn test_format_preserves_precision() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0.01));
    let formatted = money.format_code();
    assert!(formatted.contains("0.01"));
}

#[test]
fn test_is_zero_with_very_small_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0.0001));
    // This is NOT zero, even though it's very small
    assert!(!money.is_zero());
}

#[test]
fn test_is_positive_zero() {
    let currency = Currency::from_iso("USD").unwrap();
    let money = Money::new(currency, dec!(0));
    // Zero is considered positive in Decimal
    assert!(money.is_positive());
    assert!(!money.is_negative());
}
