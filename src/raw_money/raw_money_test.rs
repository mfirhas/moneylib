use crate::iso::{AUD, BDT, BHD, EUR, GBP, IDR, INR, JPY, SAR, SGD, USD};

use crate::macros::dec;
use crate::{BaseMoney, BaseOps, CustomMoney, Money, MoneyError, RawMoney, RoundingStrategy};
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

#[test]
fn test_raw_money_default() {
    let money = RawMoney::<IDR>::default();
    assert!(money.is_zero());
}

#[test]
fn test_raw_money_new_overflow() {
    let money = RawMoney::<IDR>::new(i128::MAX);
    assert!(money.is_err());

    let money = RawMoney::<IDR>::from_minor(i128::MAX);
    assert!(money.is_err());

    let money = RawMoney::<IDR>::new(i128::MAX);
    assert!(money.is_err());

    #[derive(Debug, Clone)]
    struct TooBig;
    impl crate::Currency for TooBig {
        const CODE: &'static str = "BIG";
        const SYMBOL: &'static str = "B";
        const NAME: &'static str = "Too Big";
        const NUMERIC: u16 = 69;
        const MINOR_UNIT: u16 = 98;
        const MINOR_UNIT_SYMBOL: &'static str = "*";
        const THOUSAND_SEPARATOR: &'static str = ",";
        const DECIMAL_SEPARATOR: &'static str = ".";
    }

    let money = RawMoney::<TooBig>::from_minor(123);
    assert!(money.is_err());

    let money = RawMoney::<EUR>::from_str(format!("EUR {}", i128::MAX.to_string()).as_str());
    assert!(money.is_err());

    let money =
        RawMoney::<EUR>::from_str_dot_thousands(format!("EUR {}", i128::MAX.to_string()).as_str());
    assert!(money.is_err());

    let money = RawMoney::<EUR>::from_symbol_comma_thousands(
        format!("€{}", i128::MAX.to_string()).as_str(),
    );
    assert!(money.is_err());

    let money =
        RawMoney::<EUR>::from_symbol_dot_thousands(format!("€{}", i128::MAX.to_string()).as_str());
    assert!(money.is_err());

    let money = RawMoney::<TooBig>::from_decimal(dec!(123.2348));
    let minor = money.minor_amount();
    assert!(minor.is_err());

    let money = RawMoney::<EUR>::from_decimal(crate::Decimal::MAX);
    let minor = money.minor_amount();
    assert!(minor.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.add(crate::Decimal::MAX);
    assert!(ret.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.sub(crate::Decimal::MIN);
    assert!(ret.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.mul(crate::Decimal::MAX);
    assert!(ret.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.div(dec!(0));
    assert!(ret.is_err());

    // --
    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.add(i128::MAX);
    assert!(ret.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.sub(i128::MAX);
    assert!(ret.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.mul(i128::MAX);
    assert!(ret.is_err());

    let money = RawMoney::<SGD>::from_decimal(dec!(123234));
    let ret = money.div(i128::MAX);
    assert!(ret.is_err());
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

#[test]
fn test_multiple_arithmetics() {
    let money1 = RawMoney::<USD>::from_decimal(dec!(12334.23444));
    let money2 = RawMoney::<USD>::from_decimal(dec!(234.9044));
    let money3 = RawMoney::<USD>::new(400).unwrap();

    let ret = (money1.add(money2).unwrap())
        .mul(dec!(1.2))
        .unwrap()
        .div(100_i128)
        .unwrap()
        .mul(money3)
        .unwrap();

    assert_eq!(ret.amount(), dec!(60331.86643200));
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

#[test]
fn test_from_str_dot_thousands_keep_precision() {
    let result = RawMoney::<EUR>::from_str_dot_thousands("EUR 1.234,578396").unwrap();
    assert_eq!(result.amount(), dec!(1_234.578396));
}

#[test]
fn test_from_str_dot_thousands_invalid_format() {
    let result = RawMoney::<EUR>::from_str_dot_thousands("EUR 1,234.578396");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoneyError::ParseStr);
}

#[test]
fn test_from_str_dot_thousands_invalid_format_2() {
    let result = RawMoney::<EUR>::from_str_dot_thousands("EUR 1234.578396");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MoneyError::ParseStr);
}

#[test]
fn test_parsing_negative_money_no_separator() {
    let money = RawMoney::<USD>::from_str("USD -1234567.8924").unwrap();
    assert_eq!(money.amount(), dec!(-1_234_567.8924));
}

#[test]
fn test_parsing_negative_money() {
    let money = RawMoney::<USD>::from_str("USD -1,234,567.8999").unwrap();
    assert_eq!(money.amount(), dec!(-1_234_567.8999));
}

#[test]
fn test_parsing_negative_dot_separator_money() {
    let money = RawMoney::<USD>::from_str_dot_thousands("USD -1.234.567,8942").unwrap();
    assert_eq!(money.amount(), dec!(-1_234_567.8942));
}

#[test]
fn test_parsing_all_raw() {
    //! from code comma thousands positive (NO rounding, keep full precision)
    let money: RawMoney<USD> = RawMoney::from_str("USD 12").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 12.2").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.2));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 12.23").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.23));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 12.239489").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.239489));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,234.3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,234.38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,269.34983").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_str("USD 1234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1234.3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1234.38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1269.34983").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,234,000").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,234,000.3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,234,111.38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234111.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD 1,269,899.34983").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from code comma thousands negative (NO rounding)
    let money: RawMoney<USD> = RawMoney::from_str("USD -12").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -12.2").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.2));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -12.23").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.23));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -12.239489").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.239489));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,234.3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,234.38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,269.34983").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_str("USD -1234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1234.3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1234.38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1269.34983").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,234,000").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,234,000.3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,234,111.38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234111.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<USD> = RawMoney::from_str("USD -1,269,899.34983").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from code dot thousands positive (EUR, NO rounding)
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 12").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 12,2").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.2));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 12,23").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.23));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 12,239489").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.239489));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.234,3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.234,38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.269,34983").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1234,3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1234,38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1269,34983").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.234.000").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.234.000,3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.234.111,38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234111.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR 1.269.899,34983").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from code dot thousands negative (EUR, NO rounding)
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -12").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -12,2").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.2));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -12,23").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.23));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -12,239489").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.239489));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.234,3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.234,38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.269,34983").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1234,3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1234,38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1269,34983").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.234.000").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.234.000,3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000.3));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.234.111,38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234111.38));
    println!("{} | {}", money, money.amount());
    let money: RawMoney<EUR> = RawMoney::from_str_dot_thousands("EUR -1.269.899,34983").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from symbol comma thousands positive
    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$12").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$12.2").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.2));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$12.23").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.23));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$12.239489").unwrap();
    assert!(money.is_positive());
    // USD: round to 2 decimal places using bankers rounding -> 12.24
    assert_eq!(money.amount(), dec!(12.239489));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,234.3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,234.38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,269.34983").unwrap();
    assert!(money.is_positive());
    // USD: round to 2 decimal places using bankers rounding -> 1269.35
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1234.3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1234.38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1269.34983").unwrap();
    assert!(money.is_positive());
    // USD: round to 2 decimal places using bankers rounding -> 1269.35
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,234,000").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,234,000.3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,234,111.38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234111.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("$1,269,899.34983").unwrap();
    assert!(money.is_positive());
    // USD: round to 2 decimal places using bankers rounding -> 1269899.35
    assert_eq!(money.amount(), dec!(1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from symbol comma thousands negative
    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$12").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$12.2").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.2));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$12.23").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.23));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$12.239489").unwrap();
    assert!(money.is_negative());
    // USD: round to 2 decimal places using bankers rounding -> -12.24
    assert_eq!(money.amount(), dec!(-12.239489));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,234.3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,234.38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,269.34983").unwrap();
    assert!(money.is_negative());
    // USD: round to 2 decimal places using bankers rounding -> -1269.35
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1234.3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1234.38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1269.34983").unwrap();
    assert!(money.is_negative());
    // USD: round to 2 decimal places using bankers rounding -> -1269.35
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,234,000").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,234,000.3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,234,111.38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234111.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<USD> = RawMoney::from_symbol_comma_thousands("-$1,269,899.34983").unwrap();
    assert!(money.is_negative());
    // USD: round to 2 decimal places using bankers rounding -> -1269899.35
    assert_eq!(money.amount(), dec!(-1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from symbol dot thousands positive
    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€12").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€12,2").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.2));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€12,23").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(12.23));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€12,239489").unwrap();
    assert!(money.is_positive());
    // EUR: round to 2 decimal places using bankers rounding -> 12.24
    assert_eq!(money.amount(), dec!(12.239489));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.234,3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.234,38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.269,34983").unwrap();
    assert!(money.is_positive());
    // EUR: round to 2 decimal places using bankers rounding -> 1269.35
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1234").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1234,3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1234,38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1269,34983").unwrap();
    assert!(money.is_positive());
    // EUR: round to 2 decimal places using bankers rounding -> 1269.35
    assert_eq!(money.amount(), dec!(1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.234.000").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.234.000,3").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234000.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.234.111,38").unwrap();
    assert!(money.is_positive());
    assert_eq!(money.amount(), dec!(1234111.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("€1.269.899,34983").unwrap();
    assert!(money.is_positive());
    // EUR: round to 2 decimal places using bankers rounding -> 1269899.35
    assert_eq!(money.amount(), dec!(1269899.34983));
    println!("{} | {}", money, money.amount());

    println!("----------------------------------------");

    // from symbol dot thousands negative
    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€12").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€12,2").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.2));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€12,23").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-12.23));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€12,239489").unwrap();
    assert!(money.is_negative());
    // EUR: round to 2 decimal places using bankers rounding -> -12.24
    assert_eq!(money.amount(), dec!(-12.239489));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.234,3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.234,38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.269,34983").unwrap();
    assert!(money.is_negative());
    // EUR: round to 2 decimal places using bankers rounding -> -1269.35
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1234").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1234,3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1234,38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1269,34983").unwrap();
    assert!(money.is_negative());
    // EUR: round to 2 decimal places using bankers rounding -> -1269.35
    assert_eq!(money.amount(), dec!(-1269.34983));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.234.000").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.234.000,3").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234000.3));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.234.111,38").unwrap();
    assert!(money.is_negative());
    assert_eq!(money.amount(), dec!(-1234111.38));
    println!("{} | {}", money, money.amount());

    let money: RawMoney<EUR> = RawMoney::from_symbol_dot_thousands("-€1.269.899,34983").unwrap();
    assert!(money.is_negative());
    // EUR: round to 2 decimal places using bankers rounding -> -1269899.35
    assert_eq!(money.amount(), dec!(-1269899.34983));
    println!("{} | {}", money, money.amount());
}

// ==================== from_symbol_comma_thousands Tests ====================

#[test]
fn test_from_symbol_comma_thousands_basic() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("$1,234.56").unwrap();
    assert_eq!(money.code(), "USD");
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn test_from_symbol_comma_thousands_no_thousands_separator() {
    let money = RawMoney::<AUD>::from_symbol_comma_thousands("$100.50").unwrap();
    assert_eq!(money.amount(), dec!(100.50));
}

#[test]
fn test_from_symbol_comma_thousands_integer_only() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("$100").unwrap();
    assert_eq!(money.amount(), dec!(100));
}

#[test]
fn test_from_symbol_comma_thousands_large_amount() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("$1,000,000.99").unwrap();
    assert_eq!(money.amount(), dec!(1000000.99));
}

#[test]
fn test_from_symbol_comma_thousands_zero() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("$0").unwrap();
    assert_eq!(money.amount(), dec!(0.00));
}

#[test]
fn test_from_symbol_comma_thousands_zero_point_zeros() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("$0.00").unwrap();
    assert_eq!(money.amount(), dec!(0.00));
}

#[test]
fn test_from_symbol_comma_thousands_negative() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("-$1,234.56").unwrap();
    assert_eq!(money.amount(), dec!(-1234.56));
}

#[test]
fn test_from_symbol_comma_thousands_with_whitespace() {
    let money = RawMoney::<USD>::from_symbol_comma_thousands("  $1,234.56  ").unwrap();
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn test_from_symbol_comma_thousands_currency_mismatch() {
    // EUR symbol (€) doesn't match USD parser ($), so it returns ParseStr
    let result = RawMoney::<EUR>::from_symbol_comma_thousands("$1,234.56");
    assert!(result.is_err());
}

#[test]
fn test_from_symbol_comma_thousands_invalid_empty() {
    assert!(RawMoney::<USD>::from_symbol_comma_thousands("").is_err());
}

#[test]
fn test_from_symbol_comma_thousands_invalid_format() {
    // dot-thousands / comma-decimal format is rejected
    assert!(RawMoney::<EUR>::from_symbol_comma_thousands("€1.234,56").is_err());
}

#[test]
fn test_from_symbol_comma_thousands_optional_separator() {
    let with_sep = RawMoney::<USD>::from_symbol_comma_thousands("$1,234.56").unwrap();
    let without_sep = RawMoney::<USD>::from_symbol_comma_thousands("$1234.56").unwrap();
    assert_eq!(with_sep.amount(), without_sep.amount());
    assert_eq!(with_sep.amount(), dec!(1234.56));
}

#[test]
fn test_from_symbol_comma_thousands_optional_separator_rounded() {
    let with_sep = RawMoney::<USD>::from_symbol_comma_thousands("$1,234.56988").unwrap();
    let without_sep = RawMoney::<USD>::from_symbol_comma_thousands("$1234.56672").unwrap();
    assert_ne!(with_sep.amount(), without_sep.amount());
    assert_eq!(with_sep.amount(), dec!(1_234.56988));
}

#[test]
fn test_from_symbol_comma_thousands_optional_separator_rounded_negative() {
    let with_sep = RawMoney::<USD>::from_symbol_comma_thousands("-$1,234.56988").unwrap();
    let without_sep = RawMoney::<USD>::from_symbol_comma_thousands("-$1234.56672").unwrap();
    assert_ne!(with_sep.amount(), without_sep.amount());
    assert_eq!(with_sep.amount(), dec!(-1_234.56988));
}

#[test]
fn test_from_symbol_dot_thousands_optional_separator_rounded_negative() {
    let with_sep = RawMoney::<USD>::from_symbol_dot_thousands("-$1.234,56988").unwrap();
    let without_sep = RawMoney::<USD>::from_symbol_dot_thousands("-$1234,56672").unwrap();
    assert_ne!(with_sep.amount(), without_sep.amount());
    assert_eq!(with_sep.amount(), dec!(-1_234.56988));
}

#[test]
fn test_from_symbol_comma_currency_mismatch() {
    let money = RawMoney::<SGD>::from_symbol_comma_thousands("$1,234.56");
    assert!(money.is_err());
}

#[test]
fn test_from_symbol_dot_currency_mismatch() {
    let money = RawMoney::<SGD>::from_symbol_comma_thousands("$1,234.56988");
    assert!(money.is_err());
}

// ==================== from_symbol_dot_thousands Tests ====================

#[test]
fn test_from_symbol_dot_thousands_basic() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("€1.234,56").unwrap();
    assert_eq!(money.code(), "EUR");
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn test_from_symbol_dot_thousands_no_thousands_separator() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("€100,50").unwrap();
    assert_eq!(money.amount(), dec!(100.50));
}

#[test]
fn test_from_symbol_dot_thousands_integer_only() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("€100").unwrap();
    assert_eq!(money.amount(), dec!(100));
}

#[test]
fn test_from_symbol_dot_thousands_large_amount() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("€1.000.000,99").unwrap();
    assert_eq!(money.amount(), dec!(1000000.99));
}

#[test]
fn test_from_symbol_dot_thousands_zero() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("€0,00").unwrap();
    assert_eq!(money.amount(), dec!(0.00));
}

#[test]
fn test_from_symbol_dot_thousands_negative() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("-€1.234,56").unwrap();
    assert_eq!(money.amount(), dec!(-1234.56));
}

#[test]
fn test_from_symbol_dot_thousands_with_whitespace() {
    let money = RawMoney::<EUR>::from_symbol_dot_thousands("  €1.234,56  ").unwrap();
    assert_eq!(money.amount(), dec!(1234.56));
}

#[test]
fn test_from_symbol_dot_thousands_currency_mismatch() {
    // USD symbol ($) doesn't match EUR parser (€), so it returns ParseStr
    let result = RawMoney::<USD>::from_symbol_dot_thousands("€1.234,56");
    assert!(result.is_err());
}

#[test]
fn test_from_symbol_dot_thousands_invalid_empty() {
    assert!(RawMoney::<EUR>::from_symbol_dot_thousands("").is_err());
}

#[test]
fn test_from_symbol_dot_thousands_invalid_format() {
    // comma-thousands / dot-decimal format is rejected
    assert!(RawMoney::<USD>::from_symbol_dot_thousands("$1,234.56").is_err());
}

#[test]
fn test_from_symbol_dot_thousands_optional_separator() {
    let with_sep = RawMoney::<EUR>::from_symbol_dot_thousands("€1.234,56").unwrap();
    let without_sep = RawMoney::<EUR>::from_symbol_dot_thousands("€1234,56").unwrap();
    assert_eq!(with_sep.amount(), without_sep.amount());
    assert_eq!(with_sep.amount(), dec!(1234.56));
}

#[test]
fn test_from_symbol_dot_thousands_optional_separator_rounded() {
    let with_sep = RawMoney::<EUR>::from_symbol_dot_thousands("€1.234,56988").unwrap();
    let without_sep = RawMoney::<EUR>::from_symbol_dot_thousands("€1234,56672").unwrap();
    assert_ne!(with_sep.amount(), without_sep.amount());
    assert_eq!(with_sep.amount(), dec!(1_234.56988));
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

#[test]
fn test_format_with_separator() {
    let money = RawMoney::<USD>::from_decimal(dec!(93009.446688));
    let ret = money.format_with_separator("c na", "*", "#");
    assert_eq!(ret, "USD 93*009#446688");

    let money = RawMoney::<EUR>::from_decimal(dec!(93009.446688));
    let ret = money.format_with_separator("s na", " ", ",");
    assert_eq!(ret, "€ 93 009,446688");
}

// ==================== format_locale_amount() Tests ====================

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_en_us() {
    let money = RawMoney::<USD>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("en-US", "c na");
    assert_eq!(result.unwrap(), "USD 1,234.56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_language_only() {
    // Language code only (no region): should fall back to Latin numbering
    let money = RawMoney::<USD>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("en", "c na");
    assert_eq!(result.unwrap(), "USD 1,234.56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_fr_fr() {
    // French locale: narrow no-break space (U+202F) thousands separator, comma decimal
    let money = RawMoney::<EUR>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("fr-FR", "c na");
    assert_eq!(result.unwrap(), "EUR 1\u{202f}234,56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_ar_sa() {
    // Arabic (Saudi Arabia): Arabic-Indic numerals
    let money = RawMoney::<USD>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("ar-SA", "c na");
    assert_eq!(
        result.unwrap(),
        "USD \u{0661}\u{066C}\u{0662}\u{0663}\u{0664}\u{066B}\u{0665}\u{0666}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_ar_sa_sar_symbol() {
    // Saudi riyal (SAR) with Arabic locale and currency symbol (ر.س)
    let money = RawMoney::<SAR>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("ar-SA", "s na");
    assert_eq!(
        result.unwrap(),
        "\u{0631}.\u{0633} \u{0661}\u{066C}\u{0662}\u{0663}\u{0664}\u{066B}\u{0665}\u{0666}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_id_id() {
    // Indonesian locale: dot thousands separator, comma decimal (European style)
    let money = RawMoney::<USD>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("id-ID", "c na");
    assert_eq!(result.unwrap(), "USD 1.234,56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_bcp47_extension() {
    // BCP 47 extension: zh-CN with hanidec numbering system
    let money = RawMoney::<USD>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("zh-CN-u-nu-hanidec", "c na");
    assert_eq!(
        result.unwrap(),
        "USD \u{4e00},\u{4e8c}\u{4e09}\u{56db}.\u{4e94}\u{516d}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_negative() {
    // Negative amount: 'n' in format_str controls display of the negative sign
    let money = RawMoney::<USD>::new(dec!(-1234.56)).unwrap();
    let result = money.format_locale_amount("en-US", "c na");
    assert_eq!(result.unwrap(), "USD -1,234.56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_invalid_locale() {
    let money = RawMoney::<USD>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("!!!invalid", "c na");
    assert_eq!(result.unwrap_err(), MoneyError::ParseLocale);
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_hi_in_latin() {
    // Indian locale (hi-IN) with Latin numerals (default for hi-IN)
    let money = RawMoney::<INR>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("hi-IN", "c na");
    assert_eq!(result.unwrap(), "INR 1,234.56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_hi_in_latin_symbol() {
    // Indian locale (hi-IN) with rupee symbol (₹) and Latin numerals
    let money = RawMoney::<INR>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("hi-IN", "s na");
    assert_eq!(result.unwrap(), "\u{20B9} 1,234.56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_hi_in_devanagari_symbol_inr_grouping() {
    let money = RawMoney::<INR>::new(dec!(123408.569)).unwrap();
    let result = money.format_locale_amount("hi-IN", "s na");
    assert_eq!(result.unwrap(), "\u{20B9} 1,23,408.569");

    let money = RawMoney::<INR>::new(dec!(123408.569)).unwrap();
    let result = money.format_locale_amount("hi-IN-u-nu-deva", "s na");
    assert_eq!(result.unwrap(), "\u{20B9} १,२३,४०८.५६९");

    let money = RawMoney::<INR>::new(dec!(1234012.56)).unwrap();
    let result = money.format_locale_amount("hi-IN", "s na");
    assert_eq!(result.unwrap(), "\u{20B9} 12,34,012.56");

    let money = RawMoney::<INR>::new(dec!(-1234012.52498)).unwrap();
    let result = money.format_locale_amount("hi-IN-u-nu-deva", "s na");
    assert_eq!(result.unwrap(), "\u{20B9} -१२,३४,०१२.५२४९८");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_hi_in_devanagari() {
    // Indian locale with Devanagari numerals via BCP 47 extension (nu-deva)
    let money = RawMoney::<INR>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("hi-IN-u-nu-deva", "c na");
    assert_eq!(
        result.unwrap(),
        "INR \u{0967},\u{0968}\u{0969}\u{096A}.\u{096B}\u{096C}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_hi_in_devanagari_symbol() {
    // Indian locale with rupee symbol (₹) and Devanagari numerals
    let money = RawMoney::<INR>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("hi-IN-u-nu-deva", "s na");
    assert_eq!(
        result.unwrap(),
        "\u{20B9} \u{0967},\u{0968}\u{0969}\u{096A}.\u{096B}\u{096C}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_bn_bd_bengali() {
    // Bengali locale (bn-BD) with Bengali numerals (default)
    let money = RawMoney::<BDT>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("bn-BD", "c na");
    assert_eq!(
        result.unwrap(),
        "BDT \u{09E7},\u{09E8}\u{09E9}\u{09EA}.\u{09EB}\u{09EC}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_bn_bd_bengali_symbol() {
    // Bengali locale (bn-BD) with taka symbol (৳) and Bengali numerals
    let money = RawMoney::<BDT>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("bn-BD", "s na");
    assert_eq!(
        result.unwrap(),
        "\u{09F3} \u{09E7},\u{09E8}\u{09E9}\u{09EA}.\u{09EB}\u{09EC}"
    );
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_bn_bd_latin() {
    // Bengali locale with Latin numerals via BCP 47 extension (nu-latn)
    let money = RawMoney::<BDT>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("bn-BD-u-nu-latn", "c na");
    assert_eq!(result.unwrap(), "BDT 1,234.56");
}

#[cfg(feature = "locale")]
#[test]
fn test_format_locale_amount_bn_bd_latin_symbol() {
    // Bengali locale with taka symbol (৳) and Latin numerals
    let money = RawMoney::<BDT>::new(dec!(1234.56)).unwrap();
    let result = money.format_locale_amount("bn-BD-u-nu-latn", "s na");
    assert_eq!(result.unwrap(), "\u{09F3} 1,234.56");
}
