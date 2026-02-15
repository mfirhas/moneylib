use crate::money_macros::dec;
use crate::{
    BaseMoney, BaseOps, Currency, CustomMoney, Decimal, Money, MoneyError, RawMoney, RoundingStrategy,
};
use std::str::FromStr;

// ==================== RawMoney::new() Tests ====================

#[test]
fn test_new_with_usd() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(currency, dec!(100.50));
    assert_eq!(raw.currency(), currency);
    assert_eq!(raw.amount(), dec!(100.50));
}

#[test]
fn test_new_preserves_precision() {
    let currency = Currency::from_iso("USD").unwrap();
    // RawMoney should NOT round even though USD has 2 decimal places
    let raw = RawMoney::new(currency, dec!(100.567));
    assert_eq!(raw.amount(), dec!(100.567));
}

#[test]
fn test_new_with_zero_amount() {
    let currency = Currency::from_iso("EUR").unwrap();
    let raw = RawMoney::new(currency, dec!(0));
    assert_eq!(raw.amount(), dec!(0));
}

#[test]
fn test_new_with_negative_amount() {
    let currency = Currency::from_iso("GBP").unwrap();
    let raw = RawMoney::new(currency, dec!(-50.25));
    assert_eq!(raw.amount(), dec!(-50.25));
}

#[test]
fn test_new_with_many_decimal_places() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(currency, dec!(123.123456789));
    assert_eq!(raw.amount(), dec!(123.123456789));
}

// ==================== RawMoney::finish() Tests ====================

#[test]
fn test_finish_rounds_to_currency_precision() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(currency, dec!(100.567));
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(100.57));
}

#[test]
fn test_finish_with_jpy() {
    let jpy = Currency::from_iso("JPY").unwrap();
    let raw = RawMoney::new(jpy, dec!(100.567));
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(101));
}

#[test]
fn test_finish_with_bhd() {
    let bhd = Currency::from_iso("BHD").unwrap();
    let raw = RawMoney::new(bhd, dec!(100.9999));
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(101.000));
}

// ==================== Money::into_raw() Tests ====================

#[test]
fn test_money_into_raw() {
    let usd = Currency::from_iso("USD").unwrap();
    let money = Money::new(usd, dec!(100.50));
    let raw = money.into_raw();
    assert_eq!(raw.amount(), dec!(100.50));
    assert_eq!(raw.currency(), usd);
}

#[test]
fn test_round_trip_conversion() {
    let usd = Currency::from_iso("USD").unwrap();
    let original = Money::new(usd, dec!(100.567));
    // Money rounds to 100.57
    assert_eq!(original.amount(), dec!(100.57));
    
    let raw = original.into_raw();
    assert_eq!(raw.amount(), dec!(100.57));
    
    let back_to_money = raw.finish();
    assert_eq!(back_to_money.amount(), dec!(100.57));
}

// ==================== No Auto-Rounding Tests ====================

#[test]
fn test_addition_no_rounding() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(usd, dec!(100.123));
    let raw2 = RawMoney::new(usd, dec!(200.456));
    let result = raw1 + raw2;
    assert_eq!(result.amount(), dec!(300.579));
}

#[test]
fn test_subtraction_no_rounding() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(usd, dec!(200.456));
    let raw2 = RawMoney::new(usd, dec!(100.123));
    let result = raw1 - raw2;
    assert_eq!(result.amount(), dec!(100.333));
}

#[test]
fn test_multiplication_no_rounding() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let result = raw * dec!(2.5);
    assert_eq!(result.amount(), dec!(250.3075));
}

#[test]
fn test_division_no_rounding() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100));
    let result = raw / dec!(3);
    // Should preserve full precision, not round to 2 decimal places
    // Rust Decimal uses 28 decimal places of precision
    let expected = dec!(100) / dec!(3);
    assert_eq!(result.amount(), expected);
}

#[test]
fn test_division_preserves_precision() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(1));
    let result = raw / dec!(3);
    // RawMoney should preserve the repeating decimal
    let expected = dec!(1) / dec!(3);
    assert_eq!(result.amount(), expected);
}

// ==================== Arithmetic with Decimal Tests ====================

#[test]
fn test_add_decimal() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let result = raw + dec!(50.456);
    assert_eq!(result.amount(), dec!(150.579));
}

#[test]
fn test_sub_decimal() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let result = raw - dec!(50.456);
    assert_eq!(result.amount(), dec!(49.667));
}

#[test]
fn test_mul_decimal() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let result = raw * dec!(1.5);
    assert_eq!(result.amount(), dec!(150.1845));
}

#[test]
fn test_div_decimal() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100));
    let result = raw / dec!(4);
    assert_eq!(result.amount(), dec!(25));
}

// ==================== Decimal Operations (reversed) Tests ====================

#[test]
fn test_decimal_add_raw_money() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let result = dec!(50.456) + raw;
    assert_eq!(result.amount(), dec!(150.579));
}

#[test]
fn test_decimal_sub_raw_money() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(50.123));
    let result = dec!(100.456) - raw;
    assert_eq!(result.amount(), dec!(50.333));
}

#[test]
fn test_decimal_mul_raw_money() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let result = dec!(1.5) * raw;
    assert_eq!(result.amount(), dec!(150.1845));
}

#[test]
fn test_decimal_div_raw_money() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(4));
    let result = dec!(100) / raw;
    assert_eq!(result.amount(), dec!(25));
}

// ==================== Assignment Operations Tests ====================

#[test]
fn test_add_assign() {
    let usd = Currency::from_iso("USD").unwrap();
    let mut raw = RawMoney::new(usd, dec!(100.123));
    raw += RawMoney::new(usd, dec!(50.456));
    assert_eq!(raw.amount(), dec!(150.579));
}

#[test]
fn test_sub_assign() {
    let usd = Currency::from_iso("USD").unwrap();
    let mut raw = RawMoney::new(usd, dec!(100.123));
    raw -= RawMoney::new(usd, dec!(50.456));
    assert_eq!(raw.amount(), dec!(49.667));
}

#[test]
fn test_mul_assign() {
    let usd = Currency::from_iso("USD").unwrap();
    let mut raw = RawMoney::new(usd, dec!(100.123));
    raw *= RawMoney::new(usd, dec!(2));
    assert_eq!(raw.amount(), dec!(200.246));
}

#[test]
fn test_div_assign() {
    let usd = Currency::from_iso("USD").unwrap();
    let mut raw = RawMoney::new(usd, dec!(100));
    raw /= RawMoney::new(usd, dec!(4));
    assert_eq!(raw.amount(), dec!(25));
}

// ==================== Negation Tests ====================

#[test]
fn test_negation() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let neg = -raw;
    assert_eq!(neg.amount(), dec!(-100.123));
}

#[test]
fn test_double_negation() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let double_neg = -(-raw);
    assert_eq!(double_neg.amount(), dec!(100.123));
}

// ==================== PartialEq Tests ====================

#[test]
fn test_partial_eq_same_currency_same_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(currency, dec!(100.50));
    let raw2 = RawMoney::new(currency, dec!(100.50));
    assert_eq!(raw1, raw2);
}

#[test]
fn test_partial_eq_same_currency_different_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(currency, dec!(100.50));
    let raw2 = RawMoney::new(currency, dec!(100.51));
    assert_ne!(raw1, raw2);
}

#[test]
fn test_partial_eq_different_currency_same_amount() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let raw1 = RawMoney::new(usd, dec!(100.50));
    let raw2 = RawMoney::new(eur, dec!(100.50));
    assert_ne!(raw1, raw2);
}

#[test]
fn test_partial_eq_with_precision() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(currency, dec!(100.123456789));
    let raw2 = RawMoney::new(currency, dec!(100.123456789));
    assert_eq!(raw1, raw2);
}

// ==================== PartialOrd Tests ====================

#[test]
fn test_partial_ord_same_currency_less_than() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(currency, dec!(100.00));
    let raw2 = RawMoney::new(currency, dec!(200.00));
    assert!(raw1 < raw2);
}

#[test]
fn test_partial_ord_same_currency_greater_than() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(currency, dec!(200.00));
    let raw2 = RawMoney::new(currency, dec!(100.00));
    assert!(raw1 > raw2);
}

#[test]
fn test_partial_ord_different_currency() {
    let usd = Currency::from_iso("USD").unwrap();
    let eur = Currency::from_iso("EUR").unwrap();
    let raw1 = RawMoney::new(usd, dec!(100.00));
    let raw2 = RawMoney::new(eur, dec!(200.00));
    assert!(raw1.partial_cmp(&raw2).is_none());
}

// ==================== BaseOps Tests ====================

#[test]
fn test_abs_positive() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    let abs_raw = raw.abs();
    assert_eq!(abs_raw.amount(), dec!(100.123));
}

#[test]
fn test_abs_negative() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(-100.123));
    let abs_raw = raw.abs();
    assert_eq!(abs_raw.amount(), dec!(100.123));
}

#[test]
fn test_min() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(usd, dec!(100.123));
    let raw2 = RawMoney::new(usd, dec!(50.456));
    let min_raw = raw1.min(raw2);
    assert_eq!(min_raw.amount(), dec!(50.456));
}

#[test]
fn test_max() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(usd, dec!(100.123));
    let raw2 = RawMoney::new(usd, dec!(50.456));
    let max_raw = raw1.max(raw2);
    assert_eq!(max_raw.amount(), dec!(100.123));
}

#[test]
fn test_clamp() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(150.123));
    let clamped = raw.clamp(dec!(50), dec!(100));
    assert_eq!(clamped.amount(), dec!(100));
}

// ==================== Round Tests ====================

#[test]
fn test_round_explicit() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.567));
    let rounded = raw.round();
    assert_eq!(rounded.amount(), dec!(100.57));
}

#[test]
fn test_round_with_custom_strategy() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.564));
    let rounded = raw.round_with(2, RoundingStrategy::Ceil);
    assert_eq!(rounded.amount(), dec!(100.57));
}

#[test]
fn test_round_with_floor() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.567));
    let rounded = raw.round_with(2, RoundingStrategy::Floor);
    assert_eq!(rounded.amount(), dec!(100.56));
}

// ==================== BaseMoney Trait Methods Tests ====================

#[test]
fn test_base_money_currency() {
    let currency = Currency::from_iso("EUR").unwrap();
    let raw = RawMoney::new(currency, dec!(100.50));
    assert_eq!(raw.currency(), currency);
}

#[test]
fn test_base_money_amount() {
    let currency = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(currency, dec!(123.456789));
    assert_eq!(raw.amount(), dec!(123.456789));
}

#[test]
fn test_is_positive() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123));
    assert!(raw.is_positive());
}

#[test]
fn test_is_negative() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(-100.123));
    assert!(raw.is_negative());
}

#[test]
fn test_is_zero() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(0));
    assert!(raw.is_zero());
}

// ==================== Display Tests ====================

#[test]
fn test_display_format() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(1234.567));
    let formatted = format!("{}", raw);
    assert_eq!(formatted, "USD 1,234.567");
}

#[test]
fn test_display_with_many_decimals() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::new(usd, dec!(100.123456789));
    let formatted = format!("{}", raw);
    assert_eq!(formatted, "USD 100.123456789");
}

// ==================== FromStr Tests ====================

#[test]
fn test_from_str_simple() {
    let raw = RawMoney::from_str("USD 100.50").unwrap();
    assert_eq!(raw.amount(), dec!(100.50));
    assert_eq!(raw.code(), "USD");
}

#[test]
fn test_from_str_with_thousands() {
    let raw = RawMoney::from_str("USD 1,234.56").unwrap();
    assert_eq!(raw.amount(), dec!(1234.56));
}

#[test]
fn test_from_str_many_decimals() {
    let raw = RawMoney::from_str("USD 100.123456789").unwrap();
    assert_eq!(raw.amount(), dec!(100.123456789));
}

#[test]
fn test_from_str_invalid_format() {
    let result = RawMoney::from_str("invalid");
    assert!(result.is_err());
}

// ==================== from_amount Tests ====================

#[test]
fn test_from_amount_raw_money() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw1 = RawMoney::new(usd, dec!(100.567));
    let raw2 = RawMoney::from_amount(usd, raw1).unwrap();
    assert_eq!(raw2.amount(), dec!(100.567));
}

#[test]
fn test_from_amount_from_money() {
    let usd = Currency::from_iso("USD").unwrap();
    // Money rounds to 100.57, so raw should have that value
    let money = Money::new(usd, dec!(100.567));
    let raw = RawMoney::new(usd, money.amount());
    assert_eq!(raw.amount(), dec!(100.57));
}

// ==================== from_minor_amount Tests ====================

#[test]
fn test_from_minor_amount_usd() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::from_minor_amount(usd, 12345).unwrap();
    assert_eq!(raw.amount(), dec!(123.45));
}

#[test]
fn test_from_minor_amount_jpy() {
    let jpy = Currency::from_iso("JPY").unwrap();
    let raw = RawMoney::from_minor_amount(jpy, 1000).unwrap();
    assert_eq!(raw.amount(), dec!(1000));
}

#[test]
fn test_from_minor_amount_bhd() {
    let bhd = Currency::from_iso("BHD").unwrap();
    let raw = RawMoney::from_minor_amount(bhd, 12345).unwrap();
    assert_eq!(raw.amount(), dec!(12.345));
}

#[test]
fn test_from_minor_amount_negative() {
    let usd = Currency::from_iso("USD").unwrap();
    let raw = RawMoney::from_minor_amount(usd, -12345).unwrap();
    assert_eq!(raw.amount(), dec!(-123.45));
}

// ==================== CustomMoney Tests ====================

#[test]
fn test_set_thousand_separator() {
    let usd = Currency::from_iso("USD").unwrap();
    let mut raw = RawMoney::new(usd, dec!(1234.567));
    raw.set_thousand_separator(".");
    assert_eq!(raw.thousand_separator(), ".");
}

#[test]
fn test_set_decimal_separator() {
    let usd = Currency::from_iso("USD").unwrap();
    let mut raw = RawMoney::new(usd, dec!(1234.567));
    raw.set_decimal_separator(",");
    assert_eq!(raw.decimal_separator(), ",");
}

// ==================== Real-world Use Case Tests ====================

#[test]
fn test_precise_calculation_workflow() {
    let usd = Currency::from_iso("USD").unwrap();
    
    // Start with Money (rounded)
    let money = Money::new(usd, dec!(100));
    
    // Convert to RawMoney for precise calculations
    let raw = money.into_raw();
    
    // Divide by 3 (preserves precision)
    let divided = raw / dec!(3);
    let expected_divided = dec!(100) / dec!(3);
    assert_eq!(divided.amount(), expected_divided);
    
    // Multiply by 3 (should get back close to original)
    let multiplied = divided * dec!(3);
    let expected_multiplied = expected_divided * dec!(3);
    assert_eq!(multiplied.amount(), expected_multiplied);
    
    // Convert back to Money (rounds)
    let final_money = multiplied.finish();
    assert_eq!(final_money.amount(), dec!(100.00));
}

#[test]
fn test_tax_calculation_precision() {
    let usd = Currency::from_iso("USD").unwrap();
    
    // Item price
    let price = Money::new(usd, dec!(19.99));
    
    // Convert to raw for tax calculation
    let raw_price = price.into_raw();
    
    // Apply 8.875% tax (NYC tax rate)
    let tax_rate = dec!(0.08875);
    let tax = raw_price * tax_rate;
    
    // Tax should preserve precision
    assert_eq!(tax.amount(), dec!(1.7741125));
    
    // Total with tax
    let total_raw = raw_price + tax;
    
    // Convert to Money for final display (rounds)
    let total = total_raw.finish();
    assert_eq!(total.amount(), dec!(21.76));
}

#[test]
fn test_percentage_split_workflow() {
    let usd = Currency::from_iso("USD").unwrap();
    
    // Total amount
    let total = Money::new(usd, dec!(100));
    let raw_total = total.into_raw();
    
    // Split into percentages: 33.33%, 33.33%, 33.34%
    let part1 = (raw_total * dec!(0.3333)).finish();
    let part2 = (raw_total * dec!(0.3333)).finish();
    let part3 = (raw_total * dec!(0.3334)).finish();
    
    // Each part is rounded
    assert_eq!(part1.amount(), dec!(33.33));
    assert_eq!(part2.amount(), dec!(33.33));
    assert_eq!(part3.amount(), dec!(33.34));
    
    // Sum should equal original (after rounding)
    let sum = part1 + part2 + part3;
    assert_eq!(sum.amount(), dec!(100.00));
}

#[test]
fn test_compound_interest_calculation() {
    let usd = Currency::from_iso("USD").unwrap();
    
    // Principal: $1000
    let principal = Money::new(usd, dec!(1000));
    let mut raw_amount = principal.into_raw();
    
    // Interest rate: 5% per year
    let rate = dec!(1.05);
    
    // Compound for 3 years
    for _ in 0..3 {
        raw_amount = raw_amount * rate;
    }
    
    // Final amount should be 1000 * 1.05^3 = 1157.625
    assert_eq!(raw_amount.amount(), dec!(1157.625));
    
    // Convert to Money for display
    let final_amount = raw_amount.finish();
    // USD rounds to 2 decimal places, and 1157.625 rounds to 1157.62 with banker's rounding (round to even)
    assert_eq!(final_amount.amount(), dec!(1157.62));
}
