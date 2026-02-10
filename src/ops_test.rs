use crate::money_macros::dec;
use crate::{BaseMoney, Currency, Money, RoundingStrategy};

// ==================== Tests for Operations with Same Currency Code but Different Properties ====================

/// Test that operations succeed when currencies have same code but different minor_symbol
/// This tests the edge case where Currency::PartialEq only compares code, not all fields
#[test]
fn test_add_same_code_different_minor_symbol() {
    let mut currency1 = Currency::from_iso("USD").unwrap();
    let mut currency2 = Currency::from_iso("USD").unwrap();

    currency1.set_minor_symbol("cent");
    currency2.set_minor_symbol("penny");

    // Currencies are considered equal because they have same code
    assert_eq!(currency1, currency2);

    let money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(50.00));

    // Operation should succeed because currencies have same code
    let result = money1 + money2;
    assert_eq!(result.amount(), dec!(150.00));
    assert_eq!(result.currency(), currency1);
}

/// Test that operations succeed when currencies have same code but different rounding strategies
#[test]
fn test_add_same_code_different_rounding_strategy() {
    let mut currency1 = Currency::from_iso("USD").unwrap();
    let mut currency2 = Currency::from_iso("USD").unwrap();

    currency1.set_rounding_strategy(RoundingStrategy::HalfUp);
    currency2.set_rounding_strategy(RoundingStrategy::HalfDown);

    // Currencies are considered equal because they have same code
    assert_eq!(currency1, currency2);

    let money1 = Money::new(currency1, dec!(100.50));
    let money2 = Money::new(currency2, dec!(50.25));

    // Operation should succeed because currencies have same code
    let result = money1 + money2;
    assert_eq!(result.amount(), dec!(150.75));
}

/// Test subtraction with same code but different thousand separators
#[test]
fn test_sub_same_code_different_thousand_separator() {
    let mut currency1 = Currency::from_iso("EUR").unwrap();
    let mut currency2 = Currency::from_iso("EUR").unwrap();

    currency1.set_thousand_separator(",");
    currency2.set_thousand_separator(".");

    // Currencies are considered equal because they have same code
    assert_eq!(currency1, currency2);

    let money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(30.00));

    // Operation should succeed
    let result = money1 - money2;
    assert_eq!(result.amount(), dec!(70.00));
}

/// Test subtraction with same code but different decimal separators
#[test]
fn test_sub_same_code_different_decimal_separator() {
    let mut currency1 = Currency::from_iso("EUR").unwrap();
    let mut currency2 = Currency::from_iso("EUR").unwrap();

    currency1.set_decimal_separator(".");
    currency2.set_decimal_separator(",");

    // Currencies are considered equal because they have same code
    assert_eq!(currency1, currency2);

    let money1 = Money::new(currency1, dec!(200.50));
    let money2 = Money::new(currency2, dec!(100.25));

    // Operation should succeed
    let result = money1 - money2;
    assert_eq!(result.amount(), dec!(100.25));
}

/// Test multiplication with same code but different numeric codes
#[test]
fn test_mul_same_code_different_numeric_code() {
    let mut currency1 = Currency::from_iso("GBP").unwrap();
    let mut currency2 = Currency::from_iso("GBP").unwrap();

    currency1.set_numeric_code(826);
    currency2.set_numeric_code(999);

    // Currencies are considered equal because they have same code
    assert_eq!(currency1, currency2);

    let money1 = Money::new(currency1, dec!(10.00));
    let money2 = Money::new(currency2, dec!(5.00));

    // Operation should succeed
    let result = money1 * money2;
    assert_eq!(result.amount(), dec!(50.00));
}

/// Test division with same code but multiple different properties
#[test]
fn test_div_same_code_multiple_different_properties() {
    let mut currency1 = Currency::from_iso("JPY").unwrap();
    let mut currency2 = Currency::from_iso("JPY").unwrap();

    currency1.set_rounding_strategy(RoundingStrategy::HalfUp);
    currency1.set_thousand_separator(",");
    currency1.set_minor_symbol("sen");

    currency2.set_rounding_strategy(RoundingStrategy::Floor);
    currency2.set_thousand_separator(".");
    currency2.set_minor_symbol("rin");

    // Currencies are considered equal because they have same code
    assert_eq!(currency1, currency2);

    let money1 = Money::new(currency1, dec!(100));
    let money2 = Money::new(currency2, dec!(5));

    // Operation should succeed
    let result = money1 / money2;
    assert_eq!(result.amount(), dec!(20));
}

/// Test AddAssign with same code but different properties
#[test]
fn test_add_assign_same_code_different_properties() {
    let mut currency1 = Currency::from_iso("USD").unwrap();
    let mut currency2 = Currency::from_iso("USD").unwrap();

    currency1.set_rounding_strategy(RoundingStrategy::HalfUp);
    currency2.set_rounding_strategy(RoundingStrategy::Floor);

    assert_eq!(currency1, currency2);

    let mut money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(50.00));

    money1 += money2;
    assert_eq!(money1.amount(), dec!(150.00));
}

/// Test SubAssign with same code but different properties
#[test]
fn test_sub_assign_same_code_different_properties() {
    let mut currency1 = Currency::from_iso("EUR").unwrap();
    let mut currency2 = Currency::from_iso("EUR").unwrap();

    currency1.set_thousand_separator(",");
    currency2.set_thousand_separator(".");

    assert_eq!(currency1, currency2);

    let mut money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(30.00));

    money1 -= money2;
    assert_eq!(money1.amount(), dec!(70.00));
}

/// Test MulAssign with same code but different properties
#[test]
fn test_mul_assign_same_code_different_properties() {
    let mut currency1 = Currency::from_iso("GBP").unwrap();
    let mut currency2 = Currency::from_iso("GBP").unwrap();

    currency1.set_decimal_separator(".");
    currency2.set_decimal_separator(",");

    assert_eq!(currency1, currency2);

    let mut money1 = Money::new(currency1, dec!(10.00));
    let money2 = Money::new(currency2, dec!(5.00));

    money1 *= money2;
    assert_eq!(money1.amount(), dec!(50.00));
}

/// Test DivAssign with same code but different properties
#[test]
fn test_div_assign_same_code_different_properties() {
    let mut currency1 = Currency::from_iso("JPY").unwrap();
    let mut currency2 = Currency::from_iso("JPY").unwrap();

    currency1.set_numeric_code(392);
    currency2.set_numeric_code(999);

    assert_eq!(currency1, currency2);

    let mut money1 = Money::new(currency1, dec!(100));
    let money2 = Money::new(currency2, dec!(5));

    money1 /= money2;
    assert_eq!(money1.amount(), dec!(20));
}

/// Test that the result takes properties from the left operand
#[test]
fn test_operation_result_uses_left_operand_properties() {
    let mut currency1 = Currency::from_iso("USD").unwrap();
    let mut currency2 = Currency::from_iso("USD").unwrap();

    currency1.set_rounding_strategy(RoundingStrategy::HalfUp);
    currency1.set_minor_symbol("cent");

    currency2.set_rounding_strategy(RoundingStrategy::Floor);
    currency2.set_minor_symbol("penny");

    let money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(50.00));

    let result = money1 + money2;

    // Result should have the same currency as the left operand
    assert_eq!(result.currency(), currency1);
    // This means result has rounding strategy and minor symbol from currency1
    assert_eq!(
        result.currency().rounding_strategy(),
        RoundingStrategy::HalfUp
    );
    assert_eq!(result.currency().minor_symbol(), "cent");
}

/// Test operations with extreme property differences but same code
#[test]
fn test_extreme_property_differences_same_code() {
    let mut currency1 = Currency::from_iso("EUR").unwrap();
    let mut currency2 = Currency::from_iso("EUR").unwrap();

    // Maximize differences in all mutable properties
    currency1.set_rounding_strategy(RoundingStrategy::HalfUp);
    currency1.set_thousand_separator(",");
    currency1.set_decimal_separator(".");
    currency1.set_minor_symbol("cent");
    currency1.set_numeric_code(978);

    currency2.set_rounding_strategy(RoundingStrategy::Floor);
    currency2.set_thousand_separator(".");
    currency2.set_decimal_separator(",");
    currency2.set_minor_symbol("centime");
    currency2.set_numeric_code(999);

    // Despite all these differences, they're still considered equal
    assert_eq!(currency1, currency2);

    // All operations should work
    let money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(50.00));

    let add_result = money1 + money2;
    assert_eq!(add_result.amount(), dec!(150.00));

    let money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(50.00));
    let sub_result = money1 - money2;
    assert_eq!(sub_result.amount(), dec!(50.00));

    let money1 = Money::new(currency1, dec!(10.00));
    let money2 = Money::new(currency2, dec!(5.00));
    let mul_result = money1 * money2;
    assert_eq!(mul_result.amount(), dec!(50.00));

    let money1 = Money::new(currency1, dec!(100.00));
    let money2 = Money::new(currency2, dec!(5.00));
    let div_result = money1 / money2;
    assert_eq!(div_result.amount(), dec!(20.00));
}
