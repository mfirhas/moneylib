use crate::money_macros::dec;
use crate::{
    BHD, BaseMoney, BaseOps, CustomMoney, EUR, GBP, JPY, Money, RawMoney, RoundingStrategy, USD,
};
use std::str::FromStr;

// ==================== RawMoney::new() Tests ====================

#[test]
fn test_new_with_usd() {
    let raw = RawMoney::<USD>::new(dec!(100.50)).unwrap();
    assert_eq!(raw.amount(), dec!(100.50));
}

#[test]
fn test_new_preserves_precision() {
    // RawMoney should NOT round even though USD has 2 decimal places
    let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    assert_eq!(raw.amount(), dec!(100.567));
}

#[test]
fn test_new_with_zero_amount() {
    let raw = RawMoney::<EUR>::new(dec!(0)).unwrap();
    assert_eq!(raw.amount(), dec!(0));
}

#[test]
fn test_new_with_negative_amount() {
    let raw = RawMoney::<GBP>::new(dec!(-50.25)).unwrap();
    assert_eq!(raw.amount(), dec!(-50.25));
}

#[test]
fn test_new_with_many_decimal_places() {
    let raw = RawMoney::<USD>::new(dec!(123.123456789)).unwrap();
    assert_eq!(raw.amount(), dec!(123.123456789));
}

#[test]
fn test_new_jpy_no_rounding() {
    // JPY has 0 decimal places but RawMoney should NOT round
    let raw = RawMoney::<JPY>::new(dec!(100.567)).unwrap();
    assert_eq!(raw.amount(), dec!(100.567));
}

// ==================== RawMoney::from_decimal() Tests ====================

#[test]
fn test_from_decimal_no_rounding() {
    let raw = RawMoney::<USD>::from_decimal(dec!(123.309));
    assert_eq!(raw.amount(), dec!(123.309));
}

// ==================== RawMoney::from_minor() Tests ====================

#[test]
fn test_from_minor_usd() {
    let raw = RawMoney::<USD>::from_minor(12302).unwrap();
    assert_eq!(raw.amount(), dec!(123.02));
}

#[test]
fn test_from_minor_jpy() {
    let raw = RawMoney::<JPY>::from_minor(1000).unwrap();
    assert_eq!(raw.amount(), dec!(1000));
}

#[test]
fn test_from_minor_bhd() {
    let raw = RawMoney::<BHD>::from_minor(12345).unwrap();
    assert_eq!(raw.amount(), dec!(12.345));
}

#[test]
fn test_from_minor_negative() {
    let raw = RawMoney::<USD>::from_minor(-12345).unwrap();
    assert_eq!(raw.amount(), dec!(-123.45));
}

// ==================== RawMoney::finish() Tests ====================

#[test]
fn test_finish_rounds_to_currency_precision() {
    let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(100.57));
}

#[test]
fn test_finish_with_jpy() {
    let raw = RawMoney::<JPY>::new(dec!(100.567)).unwrap();
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(101));
}

#[test]
fn test_finish_with_bhd() {
    let raw = RawMoney::<BHD>::new(dec!(100.9999)).unwrap();
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(101.000));
}

#[test]
fn test_finish_already_rounded() {
    let raw = RawMoney::<USD>::new(dec!(100.50)).unwrap();
    let money = raw.finish();
    assert_eq!(money.amount(), dec!(100.50));
}

// ==================== Money::into_raw() Tests ====================

#[test]
fn test_money_into_raw() {
    let money = Money::<USD>::new(dec!(100.50)).unwrap();
    let raw = money.into_raw();
    assert_eq!(raw.amount(), dec!(100.50));
}

#[test]
fn test_round_trip_conversion() {
    let original = Money::<USD>::new(dec!(100.567)).unwrap();
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
    let raw1 = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(200.456)).unwrap();
    let result = raw1 + raw2;
    assert_eq!(result.amount(), dec!(300.579));
}

#[test]
fn test_subtraction_no_rounding() {
    let raw1 = RawMoney::<USD>::new(dec!(200.456)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = raw1 - raw2;
    assert_eq!(result.amount(), dec!(100.333));
}

#[test]
fn test_multiplication_no_rounding() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(2)).unwrap();
    let result = raw * raw2;
    assert_eq!(result.amount(), dec!(200.246));
}

#[test]
fn test_division_no_rounding() {
    let raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(3)).unwrap();
    let result = raw / raw2;
    let expected = dec!(100) / dec!(3);
    assert_eq!(result.amount(), expected);
}

// ==================== Decimal Operations Tests ====================

#[test]
fn test_add_decimal() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = raw + dec!(50.456);
    assert_eq!(result.amount(), dec!(150.579));
}

#[test]
fn test_sub_decimal() {
    let raw = RawMoney::<USD>::new(dec!(100.456)).unwrap();
    let result = raw - dec!(50.123);
    assert_eq!(result.amount(), dec!(50.333));
}

#[test]
fn test_mul_decimal() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = raw * dec!(1.5);
    assert_eq!(result.amount(), dec!(150.1845));
}

#[test]
fn test_div_decimal() {
    let raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    let result = raw / dec!(4);
    assert_eq!(result.amount(), dec!(25));
}

// ==================== Decimal Operations (reversed) Tests ====================

#[test]
fn test_decimal_add_raw_money() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = dec!(50.456) + raw;
    assert_eq!(result.amount(), dec!(150.579));
}

#[test]
fn test_decimal_sub_raw_money() {
    let raw = RawMoney::<USD>::new(dec!(50.123)).unwrap();
    let result = dec!(100.456) - raw;
    assert_eq!(result.amount(), dec!(50.333));
}

#[test]
fn test_decimal_mul_raw_money() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = dec!(1.5) * raw;
    assert_eq!(result.amount(), dec!(150.1845));
}

#[test]
fn test_decimal_div_raw_money() {
    let raw = RawMoney::<USD>::new(dec!(4)).unwrap();
    let result = dec!(100) / raw;
    assert_eq!(result.amount(), dec!(25));
}

// ==================== Assignment Operations Tests ====================

#[test]
fn test_add_assign() {
    let mut raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    raw += RawMoney::<USD>::new(dec!(50.456)).unwrap();
    assert_eq!(raw.amount(), dec!(150.579));
}

#[test]
fn test_sub_assign() {
    let mut raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    raw -= RawMoney::<USD>::new(dec!(50.456)).unwrap();
    assert_eq!(raw.amount(), dec!(49.667));
}

#[test]
fn test_mul_assign() {
    let mut raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    raw *= RawMoney::<USD>::new(dec!(2)).unwrap();
    assert_eq!(raw.amount(), dec!(200.246));
}

#[test]
fn test_div_assign() {
    let mut raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    raw /= RawMoney::<USD>::new(dec!(4)).unwrap();
    assert_eq!(raw.amount(), dec!(25));
}

// ==================== Negation Tests ====================

#[test]
fn test_negation() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let neg = -raw;
    assert_eq!(neg.amount(), dec!(-100.123));
}

#[test]
fn test_double_negation() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let double_neg = -(-raw);
    assert_eq!(double_neg.amount(), dec!(100.123));
}

// ==================== PartialEq Tests ====================

#[test]
fn test_partial_eq_same_amount() {
    let raw1 = RawMoney::<USD>::new(dec!(100.50)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(100.50)).unwrap();
    assert_eq!(raw1, raw2);
}

#[test]
fn test_partial_eq_different_amount() {
    let raw1 = RawMoney::<USD>::new(dec!(100.50)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(100.51)).unwrap();
    assert_ne!(raw1, raw2);
}

#[test]
fn test_partial_eq_with_precision() {
    let raw1 = RawMoney::<USD>::new(dec!(100.123456789)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(100.123456789)).unwrap();
    assert_eq!(raw1, raw2);
}

// ==================== PartialOrd Tests ====================

#[test]
fn test_partial_ord_less_than() {
    let raw1 = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(200.00)).unwrap();
    assert!(raw1 < raw2);
}

#[test]
fn test_partial_ord_greater_than() {
    let raw1 = RawMoney::<USD>::new(dec!(200.00)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    assert!(raw1 > raw2);
}

#[test]
fn test_partial_ord_equal() {
    let raw1 = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let raw2 = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    assert!(raw1 <= raw2);
    assert!(raw1 >= raw2);
}

// ==================== BaseOps Method Tests ====================

#[test]
fn test_abs_positive() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let abs_raw = raw.abs();
    assert_eq!(abs_raw.amount(), dec!(100.123));
}

#[test]
fn test_abs_negative() {
    let raw = RawMoney::<USD>::new(dec!(-100.123)).unwrap();
    let abs_raw = raw.abs();
    assert_eq!(abs_raw.amount(), dec!(100.123));
}

#[test]
fn test_add_checked() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = raw.add(dec!(50.456)).unwrap();
    assert_eq!(result.amount(), dec!(150.579));
}

#[test]
fn test_sub_checked() {
    let raw = RawMoney::<USD>::new(dec!(100.456)).unwrap();
    let result = raw.sub(dec!(50.123)).unwrap();
    assert_eq!(result.amount(), dec!(50.333));
}

#[test]
fn test_mul_checked() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    let result = raw.mul(dec!(1.5)).unwrap();
    assert_eq!(result.amount(), dec!(150.1845));
}

#[test]
fn test_div_checked() {
    let raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    let result = raw.div(dec!(4)).unwrap();
    assert_eq!(result.amount(), dec!(25));
}

// ==================== Round Tests ====================

#[test]
fn test_round_explicit() {
    let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    let rounded = raw.round();
    assert_eq!(rounded.amount(), dec!(100.57));
}

#[test]
fn test_round_returns_raw_money() {
    let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    // round() returns RawMoney, can continue operations without rounding
    let rounded: RawMoney<USD> = raw.round();
    let continued = rounded + dec!(0.001);
    assert_eq!(continued.amount(), dec!(100.571));
}

#[test]
fn test_round_with_custom_strategy_ceil() {
    let raw = RawMoney::<USD>::new(dec!(100.564)).unwrap();
    let rounded = raw.round_with(2, RoundingStrategy::Ceil);
    assert_eq!(rounded.amount(), dec!(100.57));
}

#[test]
fn test_round_with_floor() {
    let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
    let rounded = raw.round_with(2, RoundingStrategy::Floor);
    assert_eq!(rounded.amount(), dec!(100.56));
}

#[test]
fn test_round_with_half_up() {
    let raw = RawMoney::<USD>::new(dec!(100.565)).unwrap();
    let rounded = raw.round_with(2, RoundingStrategy::HalfUp);
    assert_eq!(rounded.amount(), dec!(100.57));
}

// ==================== BaseMoney Trait Method Tests ====================

#[test]
fn test_code() {
    let raw = RawMoney::<EUR>::new(dec!(100.50)).unwrap();
    assert_eq!(raw.code(), "EUR");
}

#[test]
fn test_name() {
    let raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    assert_eq!(raw.name(), "United States dollar");
}

#[test]
fn test_symbol() {
    let raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    assert_eq!(raw.symbol(), "$");
}

#[test]
fn test_minor_unit() {
    let raw = RawMoney::<USD>::new(dec!(100)).unwrap();
    assert_eq!(raw.minor_unit(), 2);

    let raw_jpy = RawMoney::<JPY>::new(dec!(100)).unwrap();
    assert_eq!(raw_jpy.minor_unit(), 0);
}

#[test]
fn test_is_positive() {
    let raw = RawMoney::<USD>::new(dec!(100.123)).unwrap();
    assert!(raw.is_positive());
}

#[test]
fn test_is_negative() {
    let raw = RawMoney::<USD>::new(dec!(-100.123)).unwrap();
    assert!(raw.is_negative());
}

#[test]
fn test_is_zero() {
    let raw = RawMoney::<USD>::new(dec!(0)).unwrap();
    assert!(raw.is_zero());
}

// ==================== Display Tests ====================

#[test]
fn test_display_format() {
    let raw = RawMoney::<USD>::from_decimal(dec!(1234.567));
    let formatted = format!("{}", raw);
    assert_eq!(formatted, "USD 1,234.567");
}

#[test]
fn test_display_with_many_decimals() {
    let raw = RawMoney::<USD>::from_decimal(dec!(100.123456789));
    let formatted = format!("{}", raw);
    assert_eq!(formatted, "USD 100.123456789");
}

#[test]
fn test_display_negative() {
    let raw = RawMoney::<USD>::from_decimal(dec!(-1234.56));
    let formatted = format!("{}", raw);
    assert_eq!(formatted, "USD -1,234.56");
}

// ==================== FromStr Tests ====================

#[test]
fn test_from_str_simple() {
    let raw = RawMoney::<USD>::from_str("USD 100.50").unwrap();
    assert_eq!(raw.amount(), dec!(100.50));
    assert_eq!(raw.code(), "USD");
}

#[test]
fn test_from_str_with_thousands() {
    let raw = RawMoney::<USD>::from_str("USD 1,234.56").unwrap();
    assert_eq!(raw.amount(), dec!(1234.56));
}

#[test]
fn test_from_str_many_decimals() {
    let raw = RawMoney::<USD>::from_str("USD 100.123456789").unwrap();
    assert_eq!(raw.amount(), dec!(100.123456789));
}

#[test]
fn test_from_str_invalid_format() {
    let result = RawMoney::<USD>::from_str("invalid");
    assert!(result.is_err());
}

#[test]
fn test_from_str_currency_mismatch() {
    let result = RawMoney::<USD>::from_str("EUR 100.00");
    assert!(result.is_err());
}

#[test]
fn test_from_str_dot_thousands() {
    let raw = RawMoney::<EUR>::from_str_dot_thousands("EUR 1.234,56").unwrap();
    assert_eq!(raw.amount(), dec!(1234.56));
    assert_eq!(raw.code(), "EUR");
}

#[test]
fn test_from_str_dot_thousands_currency_mismatch() {
    let result = RawMoney::<USD>::from_str_dot_thousands("EUR 1.234,56");
    assert!(result.is_err());
}

// ==================== Real-world Use Case Tests ====================

#[test]
fn test_precise_calculation_workflow() {
    // Start with Money (rounded)
    let money = Money::<USD>::new(dec!(100)).unwrap();

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
    // Item price
    let price = Money::<USD>::new(dec!(19.99)).unwrap();

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
    // Total amount
    let total = Money::<USD>::new(dec!(100)).unwrap();
    let raw_total = total.into_raw();

    // Split into percentages: 33.33%, 33.33%, 33.34%
    let part1 = (raw_total * dec!(0.3333)).finish();
    let part2 = (raw_total * dec!(0.3333)).finish();
    let part3 = (raw_total * dec!(0.3334)).finish();

    // Each part is rounded
    assert_eq!(part1.amount(), dec!(33.33));
    assert_eq!(part2.amount(), dec!(33.33));
    assert_eq!(part3.amount(), dec!(33.34));

    // Sum should equal original
    let sum = part1 + part2 + part3;
    assert_eq!(sum.amount(), dec!(100.00));
}

#[test]
fn test_compound_interest_calculation() {
    // Principal: $1000
    let principal = Money::<USD>::new(dec!(1000)).unwrap();
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
    // USD rounds to 2 decimal places, 1157.625 rounds to 1157.62 with banker's rounding
    assert_eq!(final_amount.amount(), dec!(1157.62));
}

// ==================== Minor Amount Tests ====================

#[test]
fn test_minor_amount_exact() {
    let raw = RawMoney::<USD>::new(dec!(123.45)).unwrap();
    assert_eq!(raw.minor_amount().unwrap(), 12345_i128);
}

#[test]
fn test_minor_amount_rounds_for_integer() {
    // minor_amount() needs to convert to integer which requires rounding
    let raw = RawMoney::<USD>::new(dec!(123.238533)).unwrap();
    // 123.238533 * 100 = 12323.8533 -> banker's round = 12324
    assert_eq!(raw.minor_amount().unwrap(), 12324_i128);
}

#[test]
fn test_minor_amount_jpy() {
    let raw = RawMoney::<JPY>::new(dec!(1000)).unwrap();
    assert_eq!(raw.minor_amount().unwrap(), 1000_i128);
}
