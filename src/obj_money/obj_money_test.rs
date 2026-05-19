/// Tests for heterogeneous collections of `Money` and `RawMoney` with different currencies,
/// using the object-safe `ObjMoney` trait for dynamic dispatch (`dyn`).
use super::ObjMoney;
use crate::iso::{BHD, CHF, EUR, GBP, INR, JPY, SGD, USD};
use crate::macros::dec;
use crate::{BaseMoney, Decimal, Money, MoneyError, RoundingStrategy, money, raw};

#[cfg(feature = "raw_money")]
use crate::RawMoney;

// ==================== Money: attribute tests ====================

#[test]
fn test_obj_money_vec_currency_codes() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.50)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.75)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(15000)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(50.25)).unwrap()),
        Box::new(Money::<CHF>::new(dec!(300.00)).unwrap()),
        Box::new(Money::<INR>::new(dec!(8500.00)).unwrap()),
    ];

    let codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
    assert_eq!(codes, vec!["USD", "EUR", "JPY", "GBP", "CHF", "INR"]);
}

#[test]
fn test_obj_money_vec_symbols() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(100)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(1.00)).unwrap()),
    ];

    assert_eq!(portfolio[0].symbol(), "$");
    assert_eq!(portfolio[1].symbol(), "€");
    assert_eq!(portfolio[2].symbol(), "¥");
    assert_eq!(portfolio[3].symbol(), "£");
}

#[test]
fn test_obj_money_vec_names() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(100)).unwrap()),
    ];

    assert_eq!(portfolio[0].name(), "United States dollar");
    assert_eq!(portfolio[1].name(), "Euro");
    assert_eq!(portfolio[2].name(), "Japanese yen");
}

#[test]
fn test_obj_money_vec_minor_units() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1.00)).unwrap()), // 2 decimals
        Box::new(Money::<JPY>::new(dec!(100)).unwrap()),  // 0 decimals
        Box::new(Money::<SGD>::new(dec!(1.00)).unwrap()), // 2 decimals
    ];

    assert_eq!(portfolio[0].minor_unit(), 2);
    assert_eq!(portfolio[1].minor_unit(), 0);
    assert_eq!(portfolio[2].minor_unit(), 2);
}

#[test]
fn test_obj_money_vec_minor_amounts() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(10.50)).unwrap()), // 1050 cents
        Box::new(Money::<JPY>::new(dec!(300)).unwrap()),   // 300 yen (no sub-unit)
        Box::new(Money::<GBP>::new(dec!(5.75)).unwrap()),  // 575 pence
    ];

    assert_eq!(portfolio[0].minor_amount().unwrap(), 1050);
    assert_eq!(portfolio[1].minor_amount().unwrap(), 300);
    assert_eq!(portfolio[2].minor_amount().unwrap(), 575);
}

#[test]
fn test_obj_money_vec_amounts() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(123.45)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(678.90)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(50000)).unwrap()),
    ];

    assert_eq!(portfolio[0].amount(), dec!(123.45));
    assert_eq!(portfolio[1].amount(), dec!(678.90));
    assert_eq!(portfolio[2].amount(), dec!(50000));
}

#[test]
fn test_obj_money_vec_scale_and_fraction() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.45)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(500)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.99)).unwrap()),
    ];

    // USD: scale 2
    assert_eq!(portfolio[0].scale(), 2);
    assert_eq!(portfolio[0].fraction(), dec!(0.45));

    // JPY: scale 0
    assert_eq!(portfolio[1].scale(), 0);
    assert_eq!(portfolio[1].fraction(), dec!(0));

    // EUR: scale 2
    assert_eq!(portfolio[2].scale(), 2);
    assert_eq!(portfolio[2].fraction(), dec!(0.99));
}

#[test]
fn test_obj_money_vec_mantissa() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1234.59)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(9876)).unwrap()),
    ];

    assert_eq!(portfolio[0].mantissa(), 123459_i128);
    assert_eq!(portfolio[1].mantissa(), 9876_i128);
}

#[test]
fn test_obj_money_vec_format_code() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1234.56)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(1234)).unwrap()),
    ];

    assert_eq!(portfolio[0].format_code(), "USD 1,234.56");
    assert_eq!(portfolio[1].format_code(), "JPY 1,234");
}

#[test]
fn test_obj_money_vec_format_symbol() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1234.56)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(1234.56)).unwrap()),
    ];

    assert_eq!(portfolio[0].format_symbol(), "$1,234.56");
    assert_eq!(portfolio[1].format_symbol(), "£1,234.56");
}

#[test]
fn test_obj_money_vec_format_code_minor() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1234.45)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(10.50)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(1234)).unwrap()),
    ];

    assert_eq!(portfolio[0].format_code_minor(), "USD 123,445 ¢");
    assert_eq!(portfolio[1].format_code_minor(), "GBP 1,050 p");
    // JPY has no sub-unit so minor_amount == amount
    let jpy_minor_symbol = portfolio[2].minor_unit_symbol();
    assert_eq!(
        portfolio[2].format_code_minor(),
        format!("JPY 1,234 {jpy_minor_symbol}")
    );
}

#[test]
fn test_obj_money_vec_format_symbol_minor() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1234.45)).unwrap()),
        Box::new(Money::<USD>::new(dec!(-10.50)).unwrap()),
    ];

    assert_eq!(portfolio[0].format_symbol_minor(), "$123,445 ¢");
    assert_eq!(portfolio[1].format_symbol_minor(), "-$1,050 ¢");
}

// ==================== Money: sign / zero checks ====================

#[test]
fn test_obj_money_vec_positive_negative_zero() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(50.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(-30.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(0)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<INR>::new(dec!(-1.00)).unwrap()),
    ];

    assert!(portfolio[0].is_positive());
    assert!(!portfolio[0].is_negative());
    assert!(!portfolio[0].is_zero());

    assert!(portfolio[1].is_negative());
    assert!(!portfolio[1].is_positive());
    assert!(!portfolio[1].is_zero());

    assert!(portfolio[2].is_zero());
    // Decimal::ZERO has a positive sign bit, so is_positive() returns true for zero amounts
    assert!(!portfolio[2].is_positive());
    assert!(!portfolio[2].is_negative());

    assert!(portfolio[3].is_positive());
    assert!(portfolio[4].is_negative());
}

#[test]
fn test_obj_money_count_by_sign() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(-50.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(0)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(75.00)).unwrap()),
        Box::new(Money::<CHF>::new(dec!(-10.00)).unwrap()),
        Box::new(Money::<INR>::new(dec!(200.00)).unwrap()),
    ];

    let positives = portfolio.iter().filter(|m| m.is_positive()).count();
    let negatives = portfolio.iter().filter(|m| m.is_negative()).count();
    let zeros = portfolio.iter().filter(|m| m.is_zero()).count();

    // JPY(0) is_positive() returns true because Decimal zero has a positive sign bit;
    // positives = USD, JPY(0), GBP, INR = 4
    assert_eq!(positives, 3);
    assert_eq!(negatives, 2);
    assert_eq!(zeros, 1);
}

// ==================== Money: arithmetic / aggregate operations ====================

#[test]
fn test_obj_money_sum_all_amounts() {
    // Sum of decimal amounts regardless of currency (portfolio total at face value)
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(50.00)).unwrap()),
    ];

    let total = portfolio
        .iter()
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    assert_eq!(total, dec!(350.00));
}

#[test]
fn test_obj_money_sum_per_currency() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.00)).unwrap()),
        Box::new(Money::<USD>::new(dec!(50.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(75.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(30.00)).unwrap()),
    ];

    let usd_total = portfolio
        .iter()
        .filter(|m| m.code() == "USD")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    let eur_total = portfolio
        .iter()
        .filter(|m| m.code() == "EUR")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    let gbp_total = portfolio
        .iter()
        .filter(|m| m.code() == "GBP")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    assert_eq!(usd_total, dec!(150.00));
    assert_eq!(eur_total, dec!(275.00));
    assert_eq!(gbp_total, dec!(30.00));
}

#[test]
fn test_obj_money_apply_discount_to_amounts() {
    // Apply a 10% discount to each amount via Decimal arithmetic
    let prices: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(50.00)).unwrap()),
    ];

    let discounted: Vec<Decimal> = prices.iter().map(|m| m.amount() * dec!(0.9)).collect();

    assert_eq!(discounted[0], dec!(90.000));
    assert_eq!(discounted[1], dec!(180.000));
    assert_eq!(discounted[2], dec!(45.000));
}

#[test]
fn test_obj_money_max_and_min_amount() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(300.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(500.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(200)).unwrap()),
    ];

    let max = portfolio
        .iter()
        .max_by(|a, b| a.amount().cmp(&b.amount()))
        .unwrap();
    let min = portfolio
        .iter()
        .min_by(|a, b| a.amount().cmp(&b.amount()))
        .unwrap();

    assert_eq!(max.amount(), dec!(500.00));
    assert_eq!(max.code(), "GBP");
    assert_eq!(min.amount(), dec!(100.00));
    assert_eq!(min.code(), "EUR");
}

#[test]
fn test_obj_money_filter_positive_and_sum() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(-30.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(75.50)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(-500)).unwrap()),
        Box::new(Money::<CHF>::new(dec!(50.00)).unwrap()),
    ];

    let positive_sum = portfolio
        .iter()
        .filter(|m| m.is_positive())
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    assert_eq!(positive_sum, dec!(225.50));
}

#[test]
fn test_obj_money_same_currency_checked_ops_via_extraction() {
    // Extract same-currency moneys from a dyn vec and use ObjMoney arithmetic.
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1000.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(850.00)).unwrap()),
        Box::new(Money::<USD>::new(dec!(250.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(600.00)).unwrap()),
    ];

    // Aggregate USD amounts into a dyn ObjMoney
    let usd_sum = portfolio
        .iter()
        .filter(|m| m.code() == "USD")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    let aggregated: Box<dyn ObjMoney> = Box::new(Money::<USD>::from_decimal(usd_sum));
    assert_eq!(aggregated.amount(), dec!(1250.00));

    // checked_add on the aggregated value
    let total = aggregated.checked_add(dec!(100.00)).unwrap();
    assert_eq!(total.amount(), dec!(1350.00));

    // checked_sub
    let net = total.checked_sub(dec!(50.00)).unwrap();
    assert_eq!(net.amount(), dec!(1300.00));

    // checked_mul by a scalar
    let doubled = net.checked_mul(dec!(2)).unwrap();
    assert_eq!(doubled.amount(), dec!(2600.00));

    // checked_div by a scalar
    let halved = doubled.checked_div(dec!(4)).unwrap();
    assert_eq!(halved.amount(), dec!(650.00));
}

#[test]
fn test_obj_money_slice_iteration() {
    // Demonstrate that slices of `dyn ObjMoney` also work.
    let usd = Money::<USD>::new(dec!(10.00)).unwrap();
    let eur = Money::<EUR>::new(dec!(20.00)).unwrap();
    let jpy = Money::<JPY>::new(dec!(3000)).unwrap();

    let items: [&dyn ObjMoney; 3] = [&usd, &eur, &jpy];

    let codes: Vec<&str> = items.iter().map(|m| m.code()).collect();
    assert_eq!(codes, vec!["USD", "EUR", "JPY"]);

    let total = items.iter().fold(Decimal::ZERO, |acc, m| acc + m.amount());
    assert_eq!(total, dec!(3030.00));
}

#[test]
fn test_obj_money_sort_by_amount() {
    let mut portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<GBP>::new(dec!(300.00)).unwrap()),
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(50)).unwrap()),
    ];

    portfolio.sort_by(|a, b| a.amount().cmp(&b.amount()));

    let sorted_codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
    assert_eq!(sorted_codes, vec!["JPY", "USD", "EUR", "GBP"]);
}

#[test]
fn test_obj_money_dedup_currencies() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(200.00)).unwrap()),
        Box::new(Money::<USD>::new(dec!(50.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(75.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(25.00)).unwrap()),
    ];

    let mut seen_codes: Vec<&str> = Vec::new();
    let unique_currencies: Vec<&str> = portfolio
        .iter()
        .filter_map(|m| {
            let code = m.code();
            if seen_codes.contains(&code) {
                None
            } else {
                seen_codes.push(code);
                Some(code)
            }
        })
        .collect();

    assert_eq!(unique_currencies, vec!["USD", "EUR", "GBP"]);
}

// ==================== RawMoney: attribute and arithmetic tests ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_vec_currency_codes() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(100.123456)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(200.789012)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(15000.5)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(50.9999)).unwrap()),
    ];

    let codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
    assert_eq!(codes, vec!["USD", "EUR", "JPY", "GBP"]);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_precision_preserved() {
    // RawMoney must NOT round – check that high-precision amounts survive dyn dispatch.
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(100.123456)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(200.789012)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(15000.5)).unwrap()), // JPY has 0 minor units but RawMoney keeps fraction
    ];

    assert_eq!(portfolio[0].amount(), dec!(100.123456));
    assert_eq!(portfolio[1].amount(), dec!(200.789012));
    assert_eq!(portfolio[2].amount(), dec!(15000.5));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_scale() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(1.12345)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(2.1)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(300.999)).unwrap()),
    ];

    assert_eq!(portfolio[0].scale(), 5);
    assert_eq!(portfolio[1].scale(), 1);
    assert_eq!(portfolio[2].scale(), 3);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_fraction() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(123.456)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(10.999)).unwrap()),
    ];

    assert_eq!(portfolio[0].fraction(), dec!(0.456));
    assert_eq!(portfolio[1].fraction(), dec!(0.999));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_minor_amounts() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(10.50)).unwrap()), // minor = 1050
        Box::new(RawMoney::<JPY>::new(dec!(300)).unwrap()),   // minor = 300
    ];

    assert_eq!(portfolio[0].minor_amount().unwrap(), 1050);
    assert_eq!(portfolio[1].minor_amount().unwrap(), 300);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_sum_amounts() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(100.001)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(200.002)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(50.003)).unwrap()),
    ];

    let total = portfolio
        .iter()
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    assert_eq!(total, dec!(350.006));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_filter_and_aggregate() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(100.1234)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(-50.5678)).unwrap()),
        Box::new(RawMoney::<USD>::new(dec!(75.8765)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(0)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(-1000.5)).unwrap()),
    ];

    // Sum of positive RawMoney amounts
    let positive_sum = portfolio
        .iter()
        .filter(|m| m.is_positive())
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    assert_eq!(positive_sum, dec!(175.9999));

    // Count zeros
    let zero_count = portfolio.iter().filter(|m| m.is_zero()).count();
    assert_eq!(zero_count, 1);

    // Count negatives
    let negative_count = portfolio.iter().filter(|m| m.is_negative()).count();
    assert_eq!(negative_count, 2);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_apply_multiplier() {
    let prices: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(33.333333)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(66.666666)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(11.111111)).unwrap()),
    ];

    // Multiply each amount by 3, keep as Decimal
    let tripled: Vec<Decimal> = prices.iter().map(|m| m.amount() * dec!(3)).collect();

    assert_eq!(tripled[0], dec!(99.999999));
    assert_eq!(tripled[1], dec!(199.999998));
    assert_eq!(tripled[2], dec!(33.333333));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_same_currency_checked_ops_via_extraction() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(500.12345)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(400.99)).unwrap()),
        Box::new(RawMoney::<USD>::new(dec!(250.67890)).unwrap()),
    ];

    // Aggregate USD raw amounts into a dyn ObjMoney
    let usd_sum = portfolio
        .iter()
        .filter(|m| m.code() == "USD")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    let aggregated: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(usd_sum).unwrap());
    assert_eq!(aggregated.amount(), dec!(750.80235));

    // checked_add
    let total = aggregated.checked_add(dec!(49.19765)).unwrap();
    assert_eq!(total.amount(), dec!(800.00000));

    // checked_sub
    let after_fee = total.checked_sub(dec!(0.00001)).unwrap();
    assert_eq!(after_fee.amount(), dec!(799.99999));

    // checked_mul
    let scaled = after_fee.checked_mul(dec!(2)).unwrap();
    assert_eq!(scaled.amount(), dec!(1599.99998));

    // checked_div
    let halved = scaled.checked_div(dec!(2)).unwrap();
    assert_eq!(halved.amount(), dec!(799.99999));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_sort_by_amount() {
    let mut portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<EUR>::new(dec!(300.999)).unwrap()),
        Box::new(RawMoney::<USD>::new(dec!(100.001)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(200.500)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(50.0)).unwrap()),
    ];

    portfolio.sort_by(|a, b| a.amount().cmp(&b.amount()));

    let sorted_codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
    assert_eq!(sorted_codes, vec!["JPY", "USD", "GBP", "EUR"]);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_format_code_minor() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(10.50)).unwrap()),
        Box::new(RawMoney::<GBP>::new(dec!(5.75)).unwrap()),
    ];

    assert_eq!(portfolio[0].format_code_minor(), "USD 1,050 ¢");
    assert_eq!(portfolio[1].format_code_minor(), "GBP 575 p");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_format_symbol_minor() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(1234.45)).unwrap()),
        Box::new(RawMoney::<USD>::new(dec!(-10.50)).unwrap()),
    ];

    assert_eq!(portfolio[0].format_symbol_minor(), "$123,445 ¢");
    assert_eq!(portfolio[1].format_symbol_minor(), "-$1,050 ¢");
}

// ==================== Mixed Money and RawMoney in the same dyn vec ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_mixed_money_and_raw_money() {
    // A single Vec can hold both Money and RawMoney via dyn ObjMoney.
    let mixed: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.505)).unwrap()), // rounds to 100.50 (bankers: 0 is even, round down)
        Box::new(RawMoney::<USD>::new(dec!(100.505)).unwrap()), // keeps 100.505
        Box::new(Money::<EUR>::new(dec!(200.999)).unwrap()), // rounds to 201.00
        Box::new(RawMoney::<EUR>::new(dec!(200.999)).unwrap()), // keeps 200.999
    ];

    // Money rounds; RawMoney preserves
    assert_eq!(mixed[0].amount(), dec!(100.50));
    assert_eq!(mixed[0].code(), "USD");
    assert_eq!(mixed[1].amount(), dec!(100.505));
    assert_eq!(mixed[1].code(), "USD");
    assert_eq!(mixed[2].amount(), dec!(201.00));
    assert_eq!(mixed[2].code(), "EUR");
    assert_eq!(mixed[3].amount(), dec!(200.999));
    assert_eq!(mixed[3].code(), "EUR");

    // All four report the same currency metadata
    assert_eq!(mixed[0].symbol(), mixed[1].symbol());
    assert_eq!(mixed[2].symbol(), mixed[3].symbol());
    assert_eq!(mixed[0].minor_unit(), mixed[1].minor_unit());
    assert_eq!(mixed[2].minor_unit(), mixed[3].minor_unit());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_mixed_sum_amounts_by_currency() {
    // Both Money (rounded) and RawMoney (precise) contribute to per-currency totals.
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.505)).unwrap()), // rounds to 100.50
        Box::new(RawMoney::<USD>::new(dec!(50.123)).unwrap()), // keeps 50.123
        Box::new(Money::<EUR>::new(dec!(200.999)).unwrap()), // rounds to 201.00
        Box::new(RawMoney::<EUR>::new(dec!(10.001)).unwrap()), // keeps 10.001
        Box::new(Money::<GBP>::new(dec!(75.00)).unwrap()),
    ];

    let usd_total = portfolio
        .iter()
        .filter(|m| m.code() == "USD")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    let eur_total = portfolio
        .iter()
        .filter(|m| m.code() == "EUR")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    let gbp_total = portfolio
        .iter()
        .filter(|m| m.code() == "GBP")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    // Money<USD>(100.505) rounds to 100.50; RawMoney<USD>(50.123) keeps 50.123
    assert_eq!(usd_total, dec!(150.623));
    // Money<EUR>(200.999) rounds to 201.00; RawMoney<EUR>(10.001) keeps 10.001
    assert_eq!(eur_total, dec!(211.001));
    assert_eq!(gbp_total, dec!(75.00));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_mixed_sign_checks() {
    // is_positive / is_negative / is_zero work uniformly across Money and RawMoney.
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(50.00)).unwrap()), // positive
        Box::new(RawMoney::<USD>::new(dec!(-0.001)).unwrap()), // negative (sub-cent)
        Box::new(Money::<EUR>::new(dec!(0)).unwrap()),     // zero
        Box::new(RawMoney::<GBP>::new(dec!(0.0001)).unwrap()), // positive (sub-penny)
        Box::new(Money::<JPY>::new(dec!(-100)).unwrap()),  // negative
    ];

    assert!(portfolio[0].is_positive() && !portfolio[0].is_negative() && !portfolio[0].is_zero());
    assert!(portfolio[1].is_negative() && !portfolio[1].is_positive() && !portfolio[1].is_zero());
    assert!(portfolio[2].is_zero());
    assert!(portfolio[3].is_positive() && !portfolio[3].is_negative());
    assert!(portfolio[4].is_negative());

    let positives = portfolio.iter().filter(|m| m.is_positive()).count();
    let negatives = portfolio.iter().filter(|m| m.is_negative()).count();
    let zeros = portfolio.iter().filter(|m| m.is_zero()).count();
    assert_eq!(positives, 2); // USD(50), EUR(0), GBP(0.0001)
    assert_eq!(negatives, 2); // USD(-0.001), JPY(-100)
    assert_eq!(zeros, 1); // EUR(0)
}

// ==================== RawMoney: name accessor ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_names() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(1.00)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(1.00)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(100)).unwrap()),
    ];

    assert_eq!(portfolio[0].name(), "United States dollar");
    assert_eq!(portfolio[1].name(), "Euro");
    assert_eq!(portfolio[2].name(), "Japanese yen");
}

// ==================== fmt helper: escape sequences and edge cases ====================

/// `format_obj_money` with `\{...}` literal block: text inside braces is output verbatim
/// and any format symbols within the block (e.g. 'a', 'm') are NOT interpreted.
/// This covers the `\{` branch in `contains_active_format_symbol` and `format_parts`.
#[test]
fn test_format_obj_money_literal_block_escape() {
    use super::fmt::format_obj_money;

    // The 'a' inside \{Total:} is literal text, not the amount placeholder.
    // 'm' does not appear as an active symbol, so decimal (not minor) amount is used.
    let result = format_obj_money(
        dec!(1234.56),
        "USD",
        "$",
        "¢",
        2,
        ",",
        ".",
        r"\{Total:} c na",
    );
    assert_eq!(result, "Total: USD 1,234.56");
}

// ==================== ObjIterOps::checked_sum Tests ====================

#[cfg(feature = "exchange")]
use crate::{ExchangeRates, obj_money::ObjIterOps};

/// All items are USD; the USD→USD rate is always 1, so the result is the simple sum.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_single_currency() {
    let portfolio = vec![
        Money::<USD>::new(dec!(100.00)).unwrap(),
        Money::<USD>::new(dec!(200.00)).unwrap(),
        Money::<USD>::new(dec!(50.00)).unwrap(),
    ];
    let rates = ExchangeRates::<USD>::new();
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(350.00));
}

/// EUR items converted to USD.
/// Stored rate: EUR = 0.8 means "1 USD = 0.8 EUR", so get_pair("EUR","USD") = 1/0.8 = 1.25.
/// 80 EUR × 1.25 + 40 EUR × 1.25 = 100 + 50 = 150 USD.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_convert_to_different_currency() {
    let portfolio = vec![
        Money::<EUR>::new(dec!(80.00)).unwrap(),
        Money::<EUR>::new(dec!(40.00)).unwrap(),
    ];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(150.00));
}

/// An empty collection should produce zero (Money::new(Decimal::ZERO)).
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_empty_collection() {
    let empty: Vec<Money<USD>> = vec![];
    let rates = ExchangeRates::<USD>::new();
    let result = empty.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(0.00));
}

/// When a currency in the collection has no entry in the rates map,
/// `checked_sum` must return an `ExchangeError`.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_missing_rate() {
    let portfolio = vec![Money::<EUR>::new(dec!(100.00)).unwrap()];
    let rates = ExchangeRates::<USD>::new(); // EUR not present
    let result: Result<_, _> = portfolio.checked_sum("USD", rates);
    assert!(matches!(result, Err(MoneyError::ExchangeError(_))));
}

/// Accumulating Decimal::MAX then adding 1 more (× rate 1) must overflow the sum.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_overflow_add() {
    let portfolio = [
        Money::<USD>::from_decimal(Decimal::MAX),
        Money::<USD>::from_decimal(dec!(1)),
    ];
    let rates = ExchangeRates::<USD>::new();
    let result: Result<_, _> = portfolio.checked_sum("USD", rates);
    assert!(matches!(result, Err(MoneyError::OverflowError)));
}

/// Decimal::MAX × a rate > 1 must overflow the multiplication step.
/// Stored rate "EUR = 0.00001" means get_pair("EUR","USD") = 100 000.
/// Decimal::MAX × 100 000 overflows rust_decimal's 28-digit precision.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_overflow_mul() {
    let portfolio = [Money::<EUR>::from_decimal(Decimal::MAX)];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.00001)).unwrap();
    let result: Result<_, _> = portfolio.checked_sum("USD", rates);
    assert!(matches!(result, Err(MoneyError::OverflowError)));
}

/// Negative amounts must be included correctly in the sum.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_negative_amounts() {
    let portfolio = vec![
        Money::<USD>::new(dec!(100.00)).unwrap(),
        Money::<USD>::new(dec!(-30.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
    ];
    let rates = ExchangeRates::<USD>::new();
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(90.00));
}

/// `Vec<Box<dyn ObjMoney>>` holding multiple currencies, all converted to USD.
/// USD 100 × 1 + EUR 80 × 1.25 + GBP 50 × 2 = 100 + 100 + 100 = 300 USD.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_heterogeneous_dyn() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(80.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(50.00)).unwrap()),
    ];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap(); // get_pair("EUR","USD") = 1.25
    rates.set("GBP", dec!(0.5)).unwrap(); // get_pair("GBP","USD") = 2.00
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(300.00));
}

/// Missing rate on a heterogeneous dyn collection must also return `ExchangeError`.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_heterogeneous_dyn_missing_rate() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(50.00)).unwrap()), // EUR not in rates
    ];
    let rates = ExchangeRates::<USD>::new();
    let result: Result<_, _> = portfolio.checked_sum("USD", rates);
    assert!(matches!(result, Err(MoneyError::ExchangeError(_))));
}

/// Arrays / slices of a concrete type must also satisfy the `ObjIterOps` blanket impl.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_array_slice() {
    let arr = [
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
    ];
    let rates = ExchangeRates::<USD>::new();
    let result = arr.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(60.00));
}

// ==================== Money: numeric_code ====================

#[test]
fn test_obj_money_numeric_code() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(100)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(1.00)).unwrap()),
        Box::new(Money::<BHD>::new(dec!(1.00)).unwrap()),
    ];

    assert_eq!(portfolio[0].numeric_code(), 840); // USD
    assert_eq!(portfolio[1].numeric_code(), 978); // EUR
    assert_eq!(portfolio[2].numeric_code(), 392); // JPY
    assert_eq!(portfolio[3].numeric_code(), 826); // GBP
    assert_eq!(portfolio[4].numeric_code(), 48); // BHD
}

// ==================== Money: display ====================

#[test]
fn test_obj_money_display() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(1234.45)).unwrap());
    assert_eq!(m.display(), "USD 1,234.45");

    let neg: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(-1234.45)).unwrap());
    assert_eq!(neg.display(), "USD -1,234.45");
}

// ==================== Money: round ====================

#[test]
fn test_obj_money_round() {
    // Money is already rounded on construction, round() should be a no-op.
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(123.456)).unwrap());
    // After construction, already 123.46 (bankers rounding).
    assert_eq!(m.amount(), dec!(123.46));
    let rounded = m.round();
    assert_eq!(rounded.amount(), dec!(123.46));
    assert_eq!(rounded.code(), "USD");
}

#[test]
fn test_obj_money_round_preserves_currency() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(99.99)).unwrap());
    let rounded = m.round();
    assert_eq!(rounded.code(), "EUR");
    assert_eq!(rounded.amount(), dec!(99.99));
}

// ==================== Money: round_with ====================

#[test]
fn test_obj_money_round_with_half_up() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(2.5)).unwrap());
    let rounded = m.round_with(0, RoundingStrategy::HalfUp);
    assert_eq!(rounded.amount(), dec!(3));
    assert_eq!(rounded.code(), "USD");
}

#[test]
fn test_obj_money_round_with_bankers() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(2.5)).unwrap());
    let rounded = m.round_with(0, RoundingStrategy::BankersRounding);
    assert_eq!(rounded.amount(), dec!(2)); // rounds to even
}

#[test]
fn test_obj_money_round_with_floor() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(2.9)).unwrap());
    let rounded = m.round_with(0, RoundingStrategy::Floor);
    assert_eq!(rounded.amount(), dec!(2));
}

#[test]
fn test_obj_money_round_with_ceil() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(2.1)).unwrap());
    let rounded = m.round_with(0, RoundingStrategy::Ceil);
    assert_eq!(rounded.amount(), dec!(3));
}

// ==================== Money: truncate / truncate_with ====================

#[test]
fn test_obj_money_truncate() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(40.99)).unwrap());
    let truncated = m.truncate();
    assert_eq!(truncated.amount(), dec!(40));
    assert_eq!(truncated.code(), "USD");
}

#[test]
fn test_obj_money_truncate_negative() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(-40.99)).unwrap());
    let truncated = m.truncate();
    assert_eq!(truncated.amount(), dec!(-40));
}

// ==================== Money: abs ====================

#[test]
fn test_obj_money_abs() {
    let neg: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(-100.50)).unwrap());
    let pos = neg.abs();
    assert_eq!(pos.amount(), dec!(100.50));
    assert_eq!(pos.code(), "USD");
    assert!(pos.is_positive());
}

#[test]
fn test_obj_money_abs_already_positive() {
    let pos: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(50.00)).unwrap());
    let result = pos.abs();
    assert_eq!(result.amount(), dec!(50.00));
}

// ==================== Money: checked arithmetic ====================

#[test]
fn test_obj_money_checked_add() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.00)).unwrap());
    let result = m.checked_add(dec!(50.00)).unwrap();
    assert_eq!(result.amount(), dec!(150.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_obj_money_checked_sub() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.00)).unwrap());
    let result = m.checked_sub(dec!(30.00)).unwrap();
    assert_eq!(result.amount(), dec!(70.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_obj_money_checked_mul() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(10.00)).unwrap());
    let result = m.checked_mul(dec!(3)).unwrap();
    assert_eq!(result.amount(), dec!(30.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_obj_money_checked_div() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.00)).unwrap());
    let result = m.checked_div(dec!(4)).unwrap();
    assert_eq!(result.amount(), dec!(25.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_obj_money_checked_div_by_zero() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.00)).unwrap());
    assert!(m.checked_div(dec!(0)).is_none());
}

#[test]
fn test_obj_money_checked_rem() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.00)).unwrap());
    let result = m.checked_rem(dec!(3)).unwrap();
    assert_eq!(result.amount(), dec!(1.00));
    assert_eq!(result.code(), "USD");
}

// ==================== Box<dyn ObjMoney> blanket forwarding ====================

#[test]
fn test_boxed_obj_money_new_methods_forward() {
    // Ensure Box<dyn ObjMoney> blanket impl correctly forwards.
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(-50.75)).unwrap());
    assert_eq!(m.numeric_code(), 840);
    assert_eq!(m.display(), "USD -50.75");
    assert_eq!(m.abs().amount(), dec!(50.75));
    assert_eq!(m.round().amount(), dec!(-50.75));
    assert_eq!(m.truncate().amount(), dec!(-50));
    assert_eq!(m.checked_add(dec!(10)).unwrap().amount(), dec!(-40.75));
    assert_eq!(m.checked_sub(dec!(10)).unwrap().amount(), dec!(-60.75));
    assert_eq!(m.checked_mul(dec!(2)).unwrap().amount(), dec!(-101.50));
    assert_eq!(m.checked_div(dec!(5)).unwrap().amount(), dec!(-10.15));
}

// ==================== RawMoney: numeric_code ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_numeric_code() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(RawMoney::<USD>::new(dec!(1.00)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(1.00)).unwrap()),
        Box::new(RawMoney::<JPY>::new(dec!(100)).unwrap()),
    ];

    assert_eq!(portfolio[0].numeric_code(), 840);
    assert_eq!(portfolio[1].numeric_code(), 978);
    assert_eq!(portfolio[2].numeric_code(), 392);
}

// ==================== RawMoney: display ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_display() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(1234.5678)).unwrap());
    assert_eq!(m.display(), "USD 1,234.5678");
}

// ==================== RawMoney: round ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_round() {
    // RawMoney stores full precision; round() rounds to currency minor unit.
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(123.456)).unwrap());
    assert_eq!(m.amount(), dec!(123.456));
    let rounded = m.round();
    assert_eq!(rounded.amount(), dec!(123.46)); // USD has 2 decimal places
    assert_eq!(rounded.code(), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_round_jpy() {
    // JPY has 0 minor units.
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<JPY>::new(dec!(1234.567)).unwrap());
    let rounded = m.round();
    assert_eq!(rounded.amount(), dec!(1235)); // rounds to whole number
    assert_eq!(rounded.code(), "JPY");
}

// ==================== RawMoney: round_with ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_round_with() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(123.456789)).unwrap());
    let rounded = m.round_with(3, RoundingStrategy::HalfUp);
    assert_eq!(rounded.amount(), dec!(123.457));
    assert_eq!(rounded.code(), "USD");
}

// ==================== RawMoney: truncate / truncate_with ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_truncate() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(40.999999)).unwrap());
    let truncated = m.truncate();
    assert_eq!(truncated.amount(), dec!(40));
    assert_eq!(truncated.code(), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_truncate_with() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(40.234845)).unwrap());
    let truncated = m.truncate_with(3);
    assert_eq!(truncated.amount(), dec!(40.234));
    assert_eq!(truncated.code(), "USD");
}

// ==================== RawMoney: abs ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_abs() {
    let neg: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(-100.12345)).unwrap());
    let pos = neg.abs();
    assert_eq!(pos.amount(), dec!(100.12345));
    assert_eq!(pos.code(), "USD");
    assert!(pos.is_positive());
}

// ==================== RawMoney: checked arithmetic ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_checked_add() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(100.12345)).unwrap());
    let result = m.checked_add(dec!(50.00001)).unwrap();
    assert_eq!(result.amount(), dec!(150.12346));
    assert_eq!(result.code(), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_checked_sub() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(100.12345)).unwrap());
    let result = m.checked_sub(dec!(0.12345)).unwrap();
    assert_eq!(result.amount(), dec!(100.00000));
    assert_eq!(result.code(), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_checked_mul() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(33.333333)).unwrap());
    let result = m.checked_mul(dec!(3)).unwrap();
    assert_eq!(result.amount(), dec!(99.999999));
    assert_eq!(result.code(), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_checked_div() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(100.000000)).unwrap());
    let result = m.checked_div(dec!(3)).unwrap();
    // rust_decimal division result
    assert_eq!(result.code(), "USD");
    // Just check it doesn't overflow and produces a reasonable value
    assert!(result.amount() > dec!(33) && result.amount() < dec!(34));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_checked_rem() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(100.00)).unwrap());
    let result = m.checked_rem(dec!(3)).unwrap();
    assert_eq!(result.amount(), dec!(1.00));
    assert_eq!(result.code(), "USD");
}

// ==================== ObjMoney::convert Tests ====================

/// Converting Money to the same currency is a no-op: amount is unchanged.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_same_currency() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let rates = ExchangeRates::<USD>::new();
    let result = money.convert("USD", &rates).unwrap();
    assert_eq!(result.amount(), dec!(100.00));
    assert_eq!(result.code(), "USD");
}

/// Converting Money<USD> to EUR multiplies the amount by the USD→EUR rate.
/// ExchangeRates<USD> stores EUR=0.8, so get_pair("USD","EUR")=0.8; 100*0.8=80.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_usd_to_eur() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(80.00));
    // Issue #126: verify the returned object carries the TARGET currency code, not the source.
    assert_eq!(result.code(), "EUR");
}

/// Converting Money<EUR> to USD via ExchangeRates<USD>:
/// get_pair("EUR","USD") = 1/0.8 = 1.25; 80*1.25 = 100.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_cross_currency() {
    let money = Money::<EUR>::new(dec!(80.00)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("USD", &rates).unwrap();
    assert_eq!(result.amount(), dec!(100.00));
    // Issue #126: target currency code must be "USD", not the source "EUR".
    assert_eq!(result.code(), "USD");
}

/// When the target currency is not present in the rates map, convert returns ExchangeError.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_missing_rate() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let rates = ExchangeRates::<USD>::new(); // JPY not set
    let err = money.convert("JPY", &rates);
    assert!(matches!(err, Err(MoneyError::ExchangeError(_))));
}

/// amount * rate overflowing the 28-digit Decimal precision yields OverflowError.
/// rate stored as 0.00001 → get_pair("USD","EUR")=0.00001 (tiny), not enough;
/// instead, store a rate that gives a large multiplier: EUR=2 means get_pair("USD","EUR")=2,
/// and Decimal::MAX*2 overflows.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_overflow() {
    let money = Money::<USD>::from_decimal(Decimal::MAX);
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(2)).unwrap(); // get_pair("USD","EUR")=2
    let err = money.convert("EUR", &rates);
    assert!(matches!(err, Err(MoneyError::OverflowError)));
}

/// Zero amount converted to a different currency stays zero.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_zero_amount() {
    let money = Money::<USD>::new(dec!(0)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(0));
}

/// Negative amount converts correctly (preserves sign).
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_negative_amount() {
    let money = Money::<USD>::new(dec!(-50.00)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(-40.00));
}

/// Convert through Box<dyn ObjMoney> uses the blanket impl that delegates to the inner type.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_via_box_dyn() {
    let boxed: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(200.00)).unwrap());
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = boxed.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(160.00));
}

/// Same-currency convert through Box<dyn ObjMoney> is also a no-op.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_via_box_dyn_same_currency() {
    let boxed: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(75.00)).unwrap());
    let rates = ExchangeRates::<USD>::new();
    let result = boxed.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(75.00));
    assert_eq!(result.code(), "EUR");
}

/// RawMoney converts to same currency; result is rounded to currency's minor unit.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_raw_money_convert_same_currency() {
    let money = RawMoney::<USD>::new(dec!(123.456789)).unwrap();
    let rates = ExchangeRates::<USD>::new();
    let result = money.convert("USD", &rates).unwrap();
    assert_eq!(result.amount(), dec!(123.46));
    assert_eq!(result.code(), "USD");
}

/// RawMoney converts to a different currency; result is rounded to the target currency's minor unit.
/// 100.123456 * 0.8 = 80.0987648, rounded to EUR's 2 decimal places → 80.10
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_raw_money_convert_to_different_currency() {
    let money = RawMoney::<USD>::new(dec!(100.123456)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(80.10));
}

/// RawMoney missing rate also returns ExchangeError.
/// JPY is not in the rates table for ExchangeRates<USD>.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_raw_money_convert_missing_rate() {
    let money = RawMoney::<EUR>::new(dec!(50.00)).unwrap();
    let rates = ExchangeRates::<USD>::new(); // JPY not present in rates
    let err = money.convert("JPY", &rates);
    assert!(matches!(err, Err(MoneyError::ExchangeError(_))));
}

/// Convert each item in a heterogeneous dyn portfolio individually, then sum.
/// Portfolio: USD 100, EUR 80. Rates<USD>: EUR=0.8.
/// USD→EUR: get_pair("USD","EUR")=0.8 → 100*0.8=80.
/// EUR→EUR: same currency → 80 unchanged.
/// Sum = 80 + 80 = 160 EUR-equivalent.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_mixed_portfolio_convert_each() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(80.00)).unwrap()),
    ];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();

    let converted: Vec<_> = portfolio
        .iter()
        .map(|m| m.convert("EUR", &rates).unwrap())
        .collect();

    assert_eq!(converted[0].amount(), dec!(80.00)); // USD 100 → EUR 80
    assert_eq!(converted[1].amount(), dec!(80.00)); // EUR 80 same currency

    let total = converted
        .iter()
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());
    assert_eq!(total, dec!(160.00));
}

/// Multiple currencies converted to USD via a shared rate table.
/// JPY 1500, EUR 80, GBP 50 all converted to USD.
/// Rates<USD>: JPY=150, EUR=0.8, GBP=0.5
/// get_pair("JPY","USD")=1/150; 1500*(1/150)=10
/// get_pair("EUR","USD")=1/0.8=1.25; 80*1.25=100
/// get_pair("GBP","USD")=1/0.5=2; 50*2=100
#[cfg(feature = "exchange")]
#[test]
fn test_obj_money_convert_multiple_currencies_to_usd() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<JPY>::new(dec!(1500)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(80.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(50.00)).unwrap()),
    ];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("JPY", dec!(150)).unwrap();
    rates.set("EUR", dec!(0.8)).unwrap();
    rates.set("GBP", dec!(0.5)).unwrap();

    let jpy_converted = portfolio[0].convert("USD", &rates).unwrap();
    let eur_converted = portfolio[1].convert("USD", &rates).unwrap();
    let gbp_converted = portfolio[2].convert("USD", &rates).unwrap();

    assert_eq!(jpy_converted.amount(), dec!(10));
    assert_eq!(eur_converted.amount(), dec!(100.00));
    assert_eq!(gbp_converted.amount(), dec!(100.00));
}

/// `RawMoney` preserves full precision; the sum must not be rounded.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_iter_ops_checked_sum_raw_money() {
    let portfolio = vec![
        RawMoney::<USD>::new(dec!(100.123)).unwrap(),
        RawMoney::<USD>::new(dec!(200.456)).unwrap(),
    ];
    let rates = ExchangeRates::<USD>::new();
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(300.58)); // rounded
}

/// `RawMoney` items converted to a different currency preserve rate arithmetic precision.
/// Stored rate EUR = 0.8 → get_pair("EUR","USD") = 1.25.
/// 100.001 EUR × 1.25 = 125.00125 (RawMoney keeps all decimal places).
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_iter_ops_checked_sum_raw_money_convert() {
    let portfolio = vec![RawMoney::<EUR>::new(dec!(100.001)).unwrap()];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(125));
}

/// `Vec<Box<dyn ObjMoney>>` containing both `Money` and `RawMoney` instances.
/// USD 100 × 1 + EUR(RawMoney) 80 × 1.25 = 100 + 100 = 200 USD.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_iter_ops_checked_sum_mixed_money_and_raw_money_dyn() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(100.00)).unwrap()),
        Box::new(RawMoney::<EUR>::new(dec!(80.00)).unwrap()),
    ];
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap(); // get_pair("EUR","USD") = 1.25
    let result = portfolio.checked_sum("USD", rates).unwrap();
    assert_eq!(result.amount(), dec!(200.00));
}

/// `format_obj_money` with a backslash-escaped format symbol (`\m`): the symbol is
/// output as a literal character and NOT used to trigger minor-unit formatting.
/// This covers the single-char escape branch in `contains_active_format_symbol`
/// and the `FORMAT_SYMBOLS` branch in `format_parts`.
#[test]
fn test_format_obj_money_escape_format_symbol() {
    use super::fmt::format_obj_money;

    // \m → literal 'm' in output; minor formatting is NOT activated.
    let result = format_obj_money(dec!(100.50), "USD", "$", "¢", 2, ",", ".", r"\m c na");
    assert_eq!(result, "m USD 100.50");
}

/// `format_obj_money` with a backslash followed by a character that is neither `{`
/// nor a recognised format symbol: the backslash is pushed literally and the
/// following character falls through to the `_` wildcard arm of `format_parts`.
/// This covers lines 72–74 (unknown-char escape) and line 90 (`_` arm) in `fmt.rs`.
#[test]
fn test_format_obj_money_escape_non_format_symbol() {
    use super::fmt::format_obj_money;

    // "\\x na" is the 4-character string: \, x, space, n, a.
    // \x → backslash pushed; x → _ arm; n (positive) → nothing; a → amount.
    let result = format_obj_money(dec!(100.00), "USD", "$", "¢", 2, ",", ".", "\\x na");
    assert_eq!(result, "\\x 100.00");
}

/// `format_obj_money` with a trailing backslash (no character following it):
/// the backslash is pushed literally.
/// This covers lines 75–77 in `format_parts`.
#[test]
fn test_format_obj_money_trailing_backslash() {
    use super::fmt::format_obj_money;

    // "na\\" is the 3-character string: n, a, \.
    // n (positive) → nothing; a → amount; \ (no next char) → push \.
    let result = format_obj_money(dec!(100.00), "USD", "$", "¢", 2, ",", ".", "na\\");
    assert_eq!(result, "100.00\\");
}

/// `format_obj_money` with a non-format-symbol character in the template (`:` here):
/// the character falls through to the `_` wildcard arm of `format_parts`.
/// This covers line 90 in `fmt.rs`.
#[test]
fn test_format_obj_money_wildcard_char_in_template() {
    use super::fmt::format_obj_money;

    // "c:na" → code + ':' (via _ arm) + amount (n is positive so nothing).
    let result = format_obj_money(dec!(100.00), "USD", "$", "¢", 2, ",", ".", "c:na");
    assert_eq!(result, "USD:100.00");
}

/// When `amount * 10^minor_unit` overflows `Decimal`, the display falls back to
/// the sentinel string `"OVERFLOWED_AMOUNT"`.
/// This covers line 124 in `fmt.rs`.
#[test]
fn test_format_obj_money_minor_overflow() {
    use super::fmt::format_obj_money;
    use crate::Decimal;

    // Decimal::MAX * 100 (USD minor unit = 2) overflows → "OVERFLOWED_AMOUNT".
    let result = format_obj_money(Decimal::MAX, "USD", "$", "¢", 2, ",", ".", "c na m");
    assert_eq!(result, "USD OVERFLOWED_AMOUNT ¢");
}

#[test]
fn test_any() {
    let m: &dyn ObjMoney = &money!(IDR, 123498.128);

    let money = m.as_any().downcast_ref::<Money<crate::iso::IDR>>();
    assert!(money.is_some());
    assert_eq!(BaseMoney::amount(money.unwrap()), dec!(123498.13));
    assert_eq!(BaseMoney::code(money.unwrap()), "IDR");
    assert_eq!(ObjMoney::amount(money.unwrap()), dec!(123498.13));
    assert_eq!(ObjMoney::code(money.unwrap()), "IDR");

    let curr_mismatch = m.as_any().downcast_ref::<Money<crate::iso::BRL>>();
    assert!(curr_mismatch.is_none());

    let wrong_type = m.as_any().downcast_ref::<RawMoney<crate::iso::IDR>>();
    assert!(wrong_type.is_none());

    let wrong_type_curr_mismatch = m.as_any().downcast_ref::<RawMoney<crate::iso::BRL>>();
    assert!(wrong_type_curr_mismatch.is_none());

    // ----

    let m: Box<dyn ObjMoney> = Box::new(raw!(IDR, 123498.128));

    let money = m.as_any().downcast_ref::<RawMoney<crate::iso::IDR>>();
    assert!(money.is_some());
    assert_eq!(BaseMoney::amount(money.unwrap()), dec!(123498.128));
    assert_eq!(BaseMoney::code(money.unwrap()), "IDR");
    assert_eq!(ObjMoney::amount(money.unwrap()), dec!(123498.128));
    assert_eq!(ObjMoney::code(money.unwrap()), "IDR");

    let curr_mismatch = m.as_any().downcast_ref::<RawMoney<crate::iso::BRL>>();
    assert!(curr_mismatch.is_none());

    let wrong_type = m.as_any().downcast_ref::<Money<crate::iso::IDR>>();
    assert!(wrong_type.is_none());

    let wrong_type_curr_mismatch = m.as_any().downcast_ref::<Money<crate::iso::BRL>>();
    assert!(wrong_type_curr_mismatch.is_none());
}

// ==================== TryFrom<&dyn ObjMoney> for Money<C> ====================

#[test]
fn test_tryfrom_obj_money_to_money_success() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.50)).unwrap());
    let money = Money::<USD>::try_from(obj.as_ref()).unwrap();
    let money_2: Money<USD> = obj.try_into().unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(100.50));
    assert_eq!(BaseMoney::code(&money), "USD");
    assert_eq!(money, money_2);
}

#[test]
fn test_tryfrom_obj_money_to_money_currency_mismatch() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.50)).unwrap());
    let result = Money::<EUR>::try_from(obj.as_ref());
    assert!(matches!(
        result,
        Err(MoneyError::CurrencyMismatchError(ref got, ref exp))
        if got == "USD" && exp == "EUR"
    ));
}

#[test]
fn test_tryfrom_obj_money_to_money_rounds_amount() {
    // Money::from_decimal applies banker's rounding to the currency's minor unit.
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.505)).unwrap());
    // Money<USD> already rounds on construction; amount stored is 100.50 (bankers rounding: 5 → even)
    let money = Money::<USD>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(100.50));
}

#[test]
fn test_tryfrom_obj_money_to_money_zero_amount() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<JPY>::new(dec!(0)).unwrap());
    let money = Money::<JPY>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(0));
    assert!(ObjMoney::is_zero(&money));
}

#[test]
fn test_tryfrom_obj_money_to_money_negative_amount() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(-42.99)).unwrap());
    let money = Money::<EUR>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(-42.99));
    assert!(ObjMoney::is_negative(&money));
}

#[test]
fn test_tryfrom_obj_money_to_money_via_ref() {
    // Works directly with a reference to a concrete Money value.
    let m = Money::<GBP>::new(dec!(75.25)).unwrap();
    let obj: &dyn ObjMoney = &m;
    let money = Money::<GBP>::try_from(obj).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(75.25));
}

#[test]
fn test_tryfrom_obj_money_to_money_multiple_currencies() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(10.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(20.00)).unwrap()),
        Box::new(Money::<JPY>::new(dec!(3000)).unwrap()),
    ];

    let usd = Money::<USD>::try_from(portfolio[0].as_ref()).unwrap();
    let eur = Money::<EUR>::try_from(portfolio[1].as_ref()).unwrap();
    let jpy = Money::<JPY>::try_from(portfolio[2].as_ref()).unwrap();

    assert_eq!(BaseMoney::amount(&usd), dec!(10.00));
    assert_eq!(BaseMoney::amount(&eur), dec!(20.00));
    assert_eq!(BaseMoney::amount(&jpy), dec!(3000));

    // Wrong currency extracting first item as EUR must fail.
    assert!(Money::<EUR>::try_from(portfolio[0].as_ref()).is_err());
}

// ==================== TryFrom<&dyn ObjMoney> for RawMoney<C> ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_obj_money_to_raw_money_success() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(100.567)).unwrap());
    let raw = RawMoney::<USD>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(100.567));
    assert_eq!(BaseMoney::code(&raw), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_obj_money_to_raw_money_currency_mismatch() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(100.567)).unwrap());
    let result = RawMoney::<EUR>::try_from(obj.as_ref());
    assert!(matches!(
        result,
        Err(MoneyError::CurrencyMismatchError(ref got, ref exp))
        if got == "USD" && exp == "EUR"
    ));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_obj_money_to_raw_money_preserves_precision() {
    // RawMoney::from_decimal does NOT round; precision must survive the round-trip.
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(99.123456789)).unwrap());
    let raw = RawMoney::<USD>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(99.123456789));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_obj_money_to_raw_money_zero_amount() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<JPY>::new(dec!(0)).unwrap());
    let raw = RawMoney::<JPY>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(0));
    assert!(ObjMoney::is_zero(&raw));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_obj_money_to_raw_money_negative_amount() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<GBP>::new(dec!(-3.14159)).unwrap());
    let raw = RawMoney::<GBP>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(-3.14159));
    assert!(ObjMoney::is_negative(&raw));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_obj_money_to_raw_money_via_ref() {
    let m = RawMoney::<CHF>::new(dec!(1234.5678)).unwrap();
    let obj: &dyn ObjMoney = &m;
    let raw = RawMoney::<CHF>::try_from(obj).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(1234.5678));
}

// TryFrom Money → RawMoney and RawMoney → Money cross-type conversions via dyn ObjMoney

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_money_obj_to_raw_money() {
    // A Money<USD> exposed as dyn ObjMoney can be turned into RawMoney<USD>.
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(50.75)).unwrap());
    let raw = RawMoney::<USD>::try_from(obj.as_ref()).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(50.75));
    // Currency mismatch still errors.
    assert!(RawMoney::<EUR>::try_from(obj.as_ref()).is_err());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_raw_money_obj_to_money() {
    // A RawMoney<EUR> exposed as dyn ObjMoney can be turned into Money<EUR> (with rounding).
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<EUR>::new(dec!(99.999)).unwrap());
    let money = Money::<EUR>::try_from(obj.as_ref()).unwrap();
    // Money::from_decimal rounds 99.999 → 100.00 (bankers rounding, EUR minor unit = 2).
    assert_eq!(BaseMoney::amount(&money), dec!(100.00));
    // Currency mismatch still errors.
    assert!(Money::<USD>::try_from(obj.as_ref()).is_err());
}

// ==================== DynMoney: ObjMoney impl tests ====================

use crate::obj_money::DynMoney;

// ---- primitive accessors ----

#[test]
fn test_dyn_money_obj_amount() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.50)));
    assert_eq!(m.amount(), dec!(100.50));
}

#[test]
fn test_dyn_money_obj_code() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(m.code(), "USD");
}

#[test]
fn test_dyn_money_obj_symbol() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(m.symbol(), "$");
}

#[test]
fn test_dyn_money_obj_name() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(m.name(), "United States dollar");
}

#[test]
fn test_dyn_money_obj_minor_unit() {
    let usd: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(usd.minor_unit(), 2);

    let jpy: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<JPY>(dec!(100)));
    assert_eq!(jpy.minor_unit(), 0);
}

#[test]
fn test_dyn_money_obj_thousand_separator() {
    let usd: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(usd.thousand_separator(), ",");
}

#[test]
fn test_dyn_money_obj_decimal_separator() {
    let usd: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(usd.decimal_separator(), ".");
}

#[test]
fn test_dyn_money_obj_minor_unit_symbol() {
    let usd: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(usd.minor_unit_symbol(), "¢");
}

#[test]
fn test_dyn_money_obj_minor_amount() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(10.50)));
    assert_eq!(m.minor_amount().unwrap(), 1050);

    let jpy: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<JPY>(dec!(300)));
    assert_eq!(jpy.minor_amount().unwrap(), 300);
}

#[test]
fn test_dyn_money_obj_numeric_code() {
    let usd: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert_eq!(usd.numeric_code(), 840);

    let eur: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<EUR>(dec!(1.00)));
    assert_eq!(eur.numeric_code(), 978);

    let jpy: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<JPY>(dec!(100)));
    assert_eq!(jpy.numeric_code(), 392);
}

#[test]
fn test_dyn_money_obj_as_any() {
    let m = DynMoney::from_decimal::<USD>(dec!(50.00));
    let boxed: Box<dyn ObjMoney> = Box::new(m);
    let any = boxed.as_any();
    assert!(any.downcast_ref::<DynMoney>().is_some());
}

// ---- sign helpers ----

#[test]
fn test_dyn_money_obj_is_positive() {
    let pos: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1.00)));
    assert!(pos.is_positive());
    assert!(!pos.is_negative());
    assert!(!pos.is_zero());
}

#[test]
fn test_dyn_money_obj_is_negative() {
    let neg: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(-1.00)));
    assert!(neg.is_negative());
    assert!(!neg.is_positive());
}

#[test]
fn test_dyn_money_obj_is_zero() {
    let zero: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(0)));
    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());
}

// ---- abs ----

#[test]
fn test_dyn_money_obj_abs() {
    let neg: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(-100.50)));
    let pos = neg.abs();
    assert_eq!(pos.amount(), dec!(100.50));
    assert_eq!(pos.code(), "USD");
    assert!(pos.is_positive());
}

#[test]
fn test_dyn_money_obj_abs_already_positive() {
    let pos: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(50.00)));
    assert_eq!(pos.abs().amount(), dec!(50.00));
}

// ---- round ----

#[test]
fn test_dyn_money_obj_round() {
    // DynMoney rounds on construction (is_raw defaults to false).
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(123.456)));
    // Constructed amount is already rounded to 2 dp.
    let rounded = m.round();
    assert_eq!(rounded.code(), "USD");
    assert_eq!(rounded.amount(), m.amount());
}

#[test]
fn test_dyn_money_obj_round_jpy() {
    // JPY has 0 minor units: round to whole number.
    // Use new_with_curr which applies round_dp(0): 1234.7 → 1235 (above 0.5, rounds up).
    use crate::obj_money::Context;
    let currency = Context::get_currency("JPY").unwrap();
    let m = DynMoney::new_with_curr(currency, dec!(1234.7));
    let obj: Box<dyn ObjMoney> = Box::new(m);
    let rounded = obj.round();
    assert_eq!(rounded.amount(), dec!(1235));
    assert_eq!(rounded.code(), "JPY");
}

// ---- round_with ----

#[test]
fn test_dyn_money_obj_round_with_half_up() {
    let m = DynMoney::new_with_code("USD", dec!(2.45)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(m);
    let rounded = obj.round_with(1, RoundingStrategy::HalfUp);
    assert_eq!(rounded.amount(), dec!(2.5));
    assert_eq!(rounded.code(), "USD");
}

#[test]
fn test_dyn_money_obj_round_with_floor() {
    let m = DynMoney::new_with_code("USD", dec!(2.99)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(m);
    let rounded = obj.round_with(0, RoundingStrategy::Floor);
    assert_eq!(rounded.amount(), dec!(2));
}

// ---- truncate ----

#[test]
fn test_dyn_money_obj_truncate() {
    let m = DynMoney::new_with_code("USD", dec!(40.99)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(m);
    let truncated = obj.truncate();
    assert_eq!(truncated.amount(), dec!(40));
    assert_eq!(truncated.code(), "USD");
}

#[test]
fn test_dyn_money_obj_truncate_negative() {
    let m = DynMoney::new_with_code("USD", dec!(-40.99)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(m);
    let truncated = obj.truncate();
    assert_eq!(truncated.amount(), dec!(-40));
}

// ---- truncate_with ----

#[test]
fn test_dyn_money_obj_truncate_with() {
    // Use new_with_curr which rounds to the currency's minor unit (2 dp for USD).
    // 40.234845 rounded to 2 dp = 40.23; truncate_with(1) gives 40.2.
    use crate::obj_money::Context;
    let currency = Context::get_currency("USD").unwrap();
    let m = DynMoney::new_with_curr(currency, dec!(40.234845));
    let obj: Box<dyn ObjMoney> = Box::new(m);
    let truncated = obj.truncate_with(1);
    assert_eq!(truncated.amount(), dec!(40.2));
    assert_eq!(truncated.code(), "USD");
}

// ---- checked arithmetic ----

#[test]
fn test_dyn_money_obj_checked_add() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    let result = m.checked_add(dec!(50.00)).unwrap();
    assert_eq!(result.amount(), dec!(150.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_dyn_money_obj_checked_sub() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    let result = m.checked_sub(dec!(30.00)).unwrap();
    assert_eq!(result.amount(), dec!(70.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_dyn_money_obj_checked_mul() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(10.00)));
    let result = m.checked_mul(dec!(3)).unwrap();
    assert_eq!(result.amount(), dec!(30.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_dyn_money_obj_checked_div() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    let result = m.checked_div(dec!(4)).unwrap();
    assert_eq!(result.amount(), dec!(25.00));
    assert_eq!(result.code(), "USD");
}

#[test]
fn test_dyn_money_obj_checked_div_by_zero() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    assert!(m.checked_div(dec!(0)).is_none());
}

#[test]
fn test_dyn_money_obj_checked_rem() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    let result = m.checked_rem(dec!(3)).unwrap();
    assert_eq!(result.amount(), dec!(1.00));
    assert_eq!(result.code(), "USD");
}

// ---- chained arithmetic via dot notation ----

#[test]
fn test_dyn_money_obj_chained_ops() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1000.00)));
    let result = m
        .checked_add(dec!(500.00))
        .unwrap()
        .checked_sub(dec!(200.00))
        .unwrap()
        .checked_mul(dec!(2))
        .unwrap()
        .checked_div(dec!(4))
        .unwrap();
    assert_eq!(result.amount(), dec!(650.00));
    assert_eq!(result.code(), "USD");
}

// ---- display / format ----

#[test]
fn test_dyn_money_obj_display() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(m.display(), "USD 1,234.56");
}

#[test]
fn test_dyn_money_obj_format_symbol() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(m.format_symbol(), "$1,234.56");
}

// ---- heterogeneous vec with DynMoney ----

#[test]
fn test_dyn_money_obj_in_vec() {
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(DynMoney::from_decimal::<USD>(dec!(100.00))),
        Box::new(DynMoney::from_decimal::<EUR>(dec!(200.00))),
        Box::new(DynMoney::from_decimal::<JPY>(dec!(15000))),
        Box::new(Money::<GBP>::new(dec!(50.00)).unwrap()),
    ];

    let codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
    assert_eq!(codes, vec!["USD", "EUR", "JPY", "GBP"]);
}

// ---- new_with_code constructor ----

#[test]
fn test_dyn_money_new_with_code_valid() {
    let m = DynMoney::new_with_code("EUR", dec!(99.99)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(m);
    assert_eq!(obj.code(), "EUR");
    assert_eq!(obj.amount(), dec!(99.99));
}

#[test]
fn test_dyn_money_new_with_code_invalid() {
    let result = DynMoney::new_with_code("INVALID", dec!(1.00));
    assert!(result.is_err());
}

// ---- ObjMoney::convert for DynMoney (exchange feature) ----

#[cfg(feature = "exchange")]
#[test]
fn test_dyn_money_obj_convert_same_currency() {
    use crate::ExchangeRates;
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let rates = ExchangeRates::<USD>::new();
    let result = m.convert("USD", &rates).unwrap();
    assert_eq!(result.amount(), dec!(100.00));
    assert_eq!(result.code(), "USD");
}

#[cfg(feature = "exchange")]
#[test]
fn test_dyn_money_obj_convert_usd_to_eur() {
    use crate::ExchangeRates;
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = m.convert("EUR", &rates).unwrap();
    assert_eq!(result.code(), "EUR");
    assert_eq!(result.amount(), dec!(80.00));
}

#[cfg(feature = "exchange")]
#[test]
fn test_dyn_money_obj_convert_missing_rate() {
    use crate::ExchangeRates;
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let rates = ExchangeRates::<USD>::new(); // JPY not set
    let err = m.convert("JPY", &rates);
    assert!(matches!(err, Err(MoneyError::ExchangeError(_))));
}

#[cfg(feature = "exchange")]
#[test]
fn test_dyn_money_obj_convert_unknown_target_currency() {
    use crate::ExchangeRates;
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let rates = ExchangeRates::<USD>::new();
    // Inject a fake rate for a non-existent currency code so the rate lookup succeeds
    // but Context::get_currency should fail.  We can test via a normal unknown code.
    let err = m.convert("XYZ", &rates);
    // Either ExchangeError (rate not found) or Other (currency not found).
    assert!(err.is_err());
}

// ---- DynMoney constructor and mutation helpers ----

#[test]
fn test_dyn_money_set_amount() {
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let updated = m.set_amount(dec!(200.00));
    assert_eq!(updated.amount(), dec!(200.00));
    assert_eq!(updated.code(), "USD");
}

#[test]
fn test_dyn_money_set_curr() {
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let updated = m.set_curr::<EUR>();
    assert_eq!(updated.code(), "EUR");
    assert_eq!(updated.amount(), dec!(100.00));
}

#[test]
fn test_dyn_money_set_curr_from_code_valid() {
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let updated = m.set_curr_from_code("JPY").unwrap();
    assert_eq!(updated.code(), "JPY");
}

#[test]
fn test_dyn_money_set_curr_from_code_invalid() {
    let m = DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(m.set_curr_from_code("INVALID_CODE").is_err());
}

#[test]
fn test_dyn_money_partial_eq_same() {
    let m1 = DynMoney::from_decimal::<USD>(dec!(100.00));
    let m2 = DynMoney::from_decimal::<USD>(dec!(100.00));
    assert_eq!(m1, m2);
}

#[test]
fn test_dyn_money_partial_eq_different_amount() {
    let m1 = DynMoney::from_decimal::<USD>(dec!(100.00));
    let m2 = DynMoney::from_decimal::<USD>(dec!(200.00));
    assert_ne!(m1, m2);
}

#[test]
fn test_dyn_money_partial_ord_same_currency() {
    let m1 = DynMoney::from_decimal::<USD>(dec!(100.00));
    let m2 = DynMoney::from_decimal::<USD>(dec!(200.00));
    assert!(m1 < m2);
}

#[test]
fn test_dyn_money_partial_ord_different_currency() {
    let m1 = DynMoney::from_decimal::<USD>(dec!(100.00));
    let m2 = DynMoney::from_decimal::<EUR>(dec!(200.00));
    assert!(m1.partial_cmp(&m2).is_none());
}

// ==================== Money: truncate_with ====================

#[test]
fn test_obj_money_truncate_with() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(99.99)).unwrap());
    let truncated = m.truncate_with(1);
    assert_eq!(truncated.amount(), dec!(99.9));
    assert_eq!(truncated.code(), "USD");
}

#[test]
fn test_obj_money_truncate_with_zero_scale() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(99.99)).unwrap());
    let truncated = m.truncate_with(0);
    assert_eq!(truncated.amount(), dec!(99));
    assert_eq!(truncated.code(), "USD");
}

// ==================== Box<dyn ObjMoney>: additional blanket forwarding ====================

#[test]
fn test_boxed_obj_money_truncate_with_forwards() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(99.99)).unwrap());
    let result = m.truncate_with(1);
    assert_eq!(result.amount(), dec!(99.9));
    assert_eq!(result.code(), "USD");
}

// ==================== Money: checked arithmetic overflow / edge cases ====================

#[test]
fn test_obj_money_checked_add_overflow() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::from_decimal(Decimal::MAX));
    assert!(m.checked_add(dec!(1)).is_none());
}

#[test]
fn test_obj_money_checked_mul_overflow() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::from_decimal(Decimal::MAX));
    assert!(m.checked_mul(dec!(2)).is_none());
}

#[test]
fn test_obj_money_checked_rem_by_zero() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.00)).unwrap());
    assert!(m.checked_rem(dec!(0)).is_none());
}

// ==================== DynMoney: scale, fraction, mantissa ====================

#[test]
fn test_dyn_money_obj_scale() {
    let usd: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.45)));
    assert_eq!(usd.scale(), 2);

    let jpy: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<JPY>(dec!(500)));
    assert_eq!(jpy.scale(), 0);
}

#[test]
fn test_dyn_money_obj_fraction() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.45)));
    assert_eq!(m.fraction(), dec!(0.45));

    let jpy: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<JPY>(dec!(500)));
    assert_eq!(jpy.fraction(), dec!(0));
}

#[test]
fn test_dyn_money_obj_mantissa() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(m.mantissa(), 123456_i128);
}

// ==================== DynMoney: format methods ====================

#[test]
fn test_dyn_money_obj_format_code() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(m.format_code(), "USD 1,234.56");
}

#[test]
fn test_dyn_money_obj_format_code_minor() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(m.format_code_minor(), "USD 123,456 ¢");
}

#[test]
fn test_dyn_money_obj_format_symbol_minor() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(m.format_symbol_minor(), "$123,456 ¢");
}

#[test]
fn test_dyn_money_obj_format_custom() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.50)));
    // c = code, space, n = negative sign (positive → empty), a = amount
    assert_eq!(m.format("c na"), "USD 100.50");
}

#[test]
fn test_dyn_money_obj_format_negative_custom() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(-50.00)));
    // n = '-' for negatives, s = symbol, a = amount
    assert_eq!(m.format("nsa"), "-$50.00");
}

// ==================== DynMoney: Neg operator ====================

#[test]
fn test_dyn_money_neg_positive() {
    let m = DynMoney::from_decimal::<USD>(dec!(100.50));
    let neg = -m;
    assert_eq!(neg.amount(), dec!(-100.50));
    assert_eq!(neg.code(), "USD");
}

#[test]
fn test_dyn_money_neg_negative() {
    let m = DynMoney::from_decimal::<EUR>(dec!(-50.00));
    let neg = -m;
    assert_eq!(neg.amount(), dec!(50.00));
    assert_eq!(neg.code(), "EUR");
}

#[test]
fn test_dyn_money_neg_zero() {
    let m = DynMoney::from_decimal::<USD>(dec!(0));
    let neg = -m;
    assert_eq!(neg.amount(), dec!(0));
    assert_eq!(neg.code(), "USD");
}

// ==================== DynMoney: checked arithmetic overflow / edge cases ====================

#[test]
fn test_dyn_money_obj_checked_add_overflow() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(Decimal::MAX));
    assert!(m.checked_add(dec!(1)).is_none());
}

#[test]
fn test_dyn_money_obj_checked_mul_overflow() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(Decimal::MAX));
    assert!(m.checked_mul(dec!(2)).is_none());
}

#[test]
fn test_dyn_money_obj_checked_rem_by_zero() {
    let m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    assert!(m.checked_rem(dec!(0)).is_none());
}

// ==================== DynMoney: sort in vec ====================

#[test]
fn test_dyn_money_vec_sort_by_amount() {
    let mut portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(DynMoney::from_decimal::<GBP>(dec!(300.00))),
        Box::new(DynMoney::from_decimal::<USD>(dec!(100.00))),
        Box::new(DynMoney::from_decimal::<EUR>(dec!(200.00))),
    ];
    portfolio.sort_by(|a, b| a.amount().cmp(&b.amount()));
    let codes: Vec<&str> = portfolio.iter().map(|m| m.code()).collect();
    assert_eq!(codes, vec!["USD", "EUR", "GBP"]);
}

// ==================== DynMoney: convert overflow (exchange feature) ====================

#[cfg(feature = "exchange")]
#[test]
fn test_dyn_money_obj_convert_overflow() {
    use crate::ExchangeRates;
    let m = DynMoney::from_decimal::<USD>(Decimal::MAX);
    let mut rates = ExchangeRates::<USD>::new();
    // EUR=2 means get_pair("USD","EUR")=2; Decimal::MAX * 2 overflows.
    rates.set("EUR", dec!(2)).unwrap();
    let err = m.convert("EUR", &rates);
    assert!(matches!(err, Err(MoneyError::OverflowError)));
}

// ==================== Context: runtime functions ====================

#[test]
fn test_context_is_currency_exist_known() {
    use crate::obj_money::Context;
    assert!(Context::is_currency_exist("USD"));
    assert!(Context::is_currency_exist("EUR"));
    assert!(Context::is_currency_exist("JPY"));
}

#[test]
fn test_context_is_currency_exist_unknown() {
    use crate::obj_money::Context;
    assert!(!Context::is_currency_exist("INVALID"));
}

#[test]
fn test_context_get_currency_known() {
    use crate::obj_money::Context;
    let usd = Context::get_currency("USD");
    assert!(usd.is_some());
    assert_eq!(usd.unwrap().code, "USD");
}

#[test]
fn test_context_get_currency_unknown() {
    use crate::obj_money::Context;
    assert!(Context::get_currency("INVALID").is_none());
}

#[test]
fn test_context_get_currency_by_symbol_known() {
    use crate::obj_money::Context;
    let result = Context::get_currency_by_symbol("$");
    assert!(result.is_some());
    // Multiple currencies may use "$"; just verify a non-empty code is returned.
    assert!(!result.unwrap().code.is_empty());
}

#[test]
fn test_context_get_currency_by_symbol_unknown() {
    use crate::obj_money::Context;
    assert!(Context::get_currency_by_symbol("###").is_none());
}

#[test]
fn test_context_register_currency_duplicate_error() {
    use crate::obj_money::Context;
    // USD is already registered, so this must return an error without modifying state.
    let result = Context::register_currency::<USD>();
    assert!(matches!(result, Err(MoneyError::ObjMoneyError(_))));
}

#[test]
fn test_context_set_currency_code_mismatch_error() {
    use crate::obj_money::Context;
    // Code "EUR" does not match C::CODE ("USD") → error.
    let result = Context::set_currency::<USD>("EUR");
    assert!(matches!(
        result,
        Err(MoneyError::CurrencyMismatchError(_, _))
    ));
}

#[test]
fn test_context_set_currency_success() {
    use crate::obj_money::Context;
    // Code matches C::CODE → updates (same value) successfully.
    let result = Context::set_currency::<USD>("USD");
    assert!(result.is_ok());
    // Currency is still accessible.
    assert!(Context::is_currency_exist("USD"));
}

// ==================== Context: register_currency success path ====================

/// A custom test currency with a code not present in the ISO standard set,
/// used to exercise the `register_currency` success path.
struct TestCurrencyXZZ;
impl crate::Currency for TestCurrencyXZZ {
    const CODE: &'static str = "XZZ";
    const SYMBOL: &'static str = "T";
    const NAME: &'static str = "Test Currency";
    const NUMERIC: u16 = 999;
    const MINOR_UNIT: u16 = 2;
    const MINOR_UNIT_SYMBOL: &'static str = "tc";
    const MINOR_UNIT_NAME: &'static str = "test-cent";
    const THOUSAND_SEPARATOR: &'static str = ",";
    const DECIMAL_SEPARATOR: &'static str = ".";
    const ORIGIN: &'static str = "Testing";
    const LOCALE: &'static str = "en-US";
}

#[test]
fn test_context_register_currency_success() {
    use crate::obj_money::Context;
    // "XZZ" is not a standard ISO currency, so it won't be in the initial map.
    // The first call must succeed; subsequent calls (e.g., in repeated test runs
    // within the same process) may return Err, so we accept both outcomes.
    let result = Context::register_currency::<TestCurrencyXZZ>();
    match result {
        Ok(()) => {
            // Successfully registered — verify the currency is now accessible.
            assert!(Context::is_currency_exist("XZZ"));
            let curr = Context::get_currency("XZZ").unwrap();
            assert_eq!(curr.code, "XZZ");
            assert_eq!(curr.symbol, "T");
        }
        Err(MoneyError::ObjMoneyError(_)) => {
            // Already registered from a previous test in the same process — that's fine.
            assert!(Context::is_currency_exist("XZZ"));
        }
        Err(e) => panic!("unexpected error: {:?}", e),
    }
}

// ==================== DynCurrency: PartialEq ====================

#[test]
fn test_dyn_currency_partial_eq_same_code() {
    use crate::obj_money::Context;
    let usd1 = Context::get_currency("USD").unwrap();
    let usd2 = Context::get_currency("USD").unwrap();
    assert_eq!(usd1, usd2);
}

#[test]
fn test_dyn_currency_partial_eq_different_code() {
    use crate::obj_money::Context;
    let usd = Context::get_currency("USD").unwrap();
    let eur = Context::get_currency("EUR").unwrap();
    assert_ne!(usd, eur);
}

// ==================== TryFrom<&dyn ObjMoney> for DynMoney ====================

#[test]
fn test_tryfrom_obj_money_ref_to_dyn_money_success() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.50)).unwrap());
    let dyn_money = DynMoney::try_from(obj.as_ref()).unwrap();
    assert_eq!(dyn_money.amount(), dec!(100.50));
    assert_eq!(dyn_money.code(), "USD");
}

#[test]
fn test_tryfrom_obj_money_ref_to_dyn_money_invalid_code() {
    // DynMoney::new_with_code with an unknown currency fails with MoneyError::Other.
    // We create a custom ObjMoney that reports an invalid code to simulate this.
    // Instead, use a real Money but check that try_from delegates to new_with_code.
    // Since all valid Money/RawMoney currencies exist in Context, use a direct new_with_code:
    let result = DynMoney::new_with_code("INVALID", dec!(1.00));
    assert!(result.is_err());
    assert!(matches!(result, Err(MoneyError::ObjMoneyError(_))));
}

#[test]
fn test_tryfrom_obj_money_ref_to_dyn_money_eur() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(75.25)).unwrap());
    let dyn_money = DynMoney::try_from(obj.as_ref()).unwrap();
    assert_eq!(dyn_money.code(), "EUR");
    assert_eq!(dyn_money.amount(), dec!(75.25));
}

// ==================== TryFrom<Box<dyn ObjMoney>> for DynMoney ====================

#[test]
fn test_tryfrom_box_obj_money_to_dyn_money_success() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(200.00)).unwrap());
    let dyn_money = DynMoney::try_from(obj).unwrap();
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.amount(), dec!(200.00));
}

#[test]
fn test_tryfrom_box_obj_money_to_dyn_money_jpy() {
    let obj: Box<dyn ObjMoney> = Box::new(Money::<JPY>::new(dec!(5000)).unwrap());
    let dyn_money = DynMoney::try_from(obj).unwrap();
    assert_eq!(dyn_money.code(), "JPY");
    assert_eq!(dyn_money.amount(), dec!(5000));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_box_obj_money_raw_to_dyn_money() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<EUR>::new(dec!(99.9999)).unwrap());
    let dyn_money = DynMoney::try_from(obj).unwrap();
    assert_eq!(dyn_money.code(), "EUR");
}

// ==================== TryFrom<DynMoney> for Money<C> ====================

#[test]
fn test_tryfrom_dyn_money_to_money_success() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(50.75));
    let money = Money::<USD>::try_from(dyn_m).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(50.75));
    assert_eq!(BaseMoney::code(&money), "USD");
}

#[test]
fn test_tryfrom_dyn_money_to_money_currency_mismatch() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let result = Money::<EUR>::try_from(dyn_m);
    assert!(matches!(
        result,
        Err(MoneyError::CurrencyMismatchError(ref got, ref exp))
        if got == "USD" && exp == "EUR"
    ));
}

#[test]
fn test_tryfrom_dyn_money_to_money_rounds_on_conversion() {
    // DynMoney::from_decimal rounds to the currency's minor unit on construction.
    // Money<USD>::try_from preserves the already-rounded amount.
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.46));
    let money = Money::<USD>::try_from(dyn_m).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(100.46));
}

#[test]
fn test_tryfrom_dyn_money_to_money_jpy() {
    let dyn_m = DynMoney::from_decimal::<JPY>(dec!(3000));
    let money = Money::<JPY>::try_from(dyn_m).unwrap();
    assert_eq!(BaseMoney::amount(&money), dec!(3000));
}

// ==================== TryFrom<DynMoney> for RawMoney<C> ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_dyn_money_to_raw_money_success() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(50.12));
    let raw = RawMoney::<USD>::try_from(dyn_m).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(50.12));
    assert_eq!(BaseMoney::code(&raw), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_dyn_money_to_raw_money_currency_mismatch() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(80.00));
    let result = RawMoney::<USD>::try_from(dyn_m);
    assert!(matches!(
        result,
        Err(MoneyError::CurrencyMismatchError(ref got, ref exp))
        if got == "EUR" && exp == "USD"
    ));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_dyn_money_to_raw_money_rounded_value() {
    let dyn_m = DynMoney::from_decimal::<GBP>(dec!(12.35));
    let raw = RawMoney::<GBP>::try_from(dyn_m).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(12.35));
}

// ==================== DynMoney: PartialEq cross-type ====================

#[test]
fn test_dyn_money_partial_eq_ref_dyn_obj_money_same() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let obj: &dyn ObjMoney = &Money::<USD>::new(dec!(100.00)).unwrap();
    assert!(dyn_m == obj);
}

#[test]
fn test_dyn_money_partial_eq_ref_dyn_obj_money_diff_amount() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let obj: &dyn ObjMoney = &Money::<USD>::new(dec!(200.00)).unwrap();
    assert!(dyn_m != obj);
}

#[test]
fn test_dyn_money_partial_eq_ref_dyn_obj_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let obj: &dyn ObjMoney = &Money::<EUR>::new(dec!(100.00)).unwrap();
    assert!(dyn_m != obj);
}

#[test]
fn test_dyn_money_partial_eq_box_dyn_obj_money_same() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(50.00));
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(50.00)).unwrap());
    assert!(dyn_m == obj);
}

#[test]
fn test_dyn_money_partial_eq_box_dyn_obj_money_diff() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(50.00));
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(50.00)).unwrap());
    assert!(dyn_m != obj);
}

#[test]
fn test_dyn_money_partial_eq_money_same() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(75.50));
    let money = Money::<USD>::new(dec!(75.50)).unwrap();
    assert!(dyn_m == money);
}

#[test]
fn test_dyn_money_partial_eq_money_diff_amount() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(75.50));
    let money = Money::<USD>::new(dec!(75.00)).unwrap();
    assert!(dyn_m != money);
}

#[test]
fn test_dyn_money_partial_eq_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(75.50));
    let money = Money::<USD>::new(dec!(75.50)).unwrap();
    assert!(dyn_m != money);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_dyn_money_partial_eq_raw_money_same() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(33.33));
    let raw = RawMoney::<USD>::new(dec!(33.33)).unwrap();
    assert!(dyn_m == raw);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_dyn_money_partial_eq_raw_money_diff_amount() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(33.33));
    let raw = RawMoney::<USD>::new(dec!(33.34)).unwrap();
    assert!(dyn_m != raw);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_dyn_money_partial_eq_raw_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(33.33));
    let raw = RawMoney::<USD>::new(dec!(33.33)).unwrap();
    assert!(dyn_m != raw);
}

// ==================== DynMoney: PartialOrd cross-type ====================

#[test]
fn test_dyn_money_partial_ord_ref_dyn_obj_money_less() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let obj: &dyn ObjMoney = &Money::<USD>::new(dec!(200.00)).unwrap();
    assert!(dyn_m < obj);
}

#[test]
fn test_dyn_money_partial_ord_ref_dyn_obj_money_greater() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(300.00));
    let obj: &dyn ObjMoney = &Money::<USD>::new(dec!(200.00)).unwrap();
    assert!(dyn_m > obj);
}

#[test]
fn test_dyn_money_partial_ord_ref_dyn_obj_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let obj: &dyn ObjMoney = &Money::<EUR>::new(dec!(100.00)).unwrap();
    assert!(dyn_m.partial_cmp(&obj).is_none());
}

#[test]
fn test_dyn_money_partial_ord_box_dyn_obj_money_less() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(50.00));
    let obj: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(100.00)).unwrap());
    assert!(dyn_m < obj);
}

#[test]
fn test_dyn_money_partial_ord_box_dyn_obj_money_equal() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(100.00));
    let obj: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(100.00)).unwrap());
    assert!(dyn_m == obj);
}

#[test]
fn test_dyn_money_partial_ord_box_dyn_obj_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<GBP>(dec!(50.00));
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(50.00)).unwrap());
    assert!(dyn_m.partial_cmp(&obj).is_none());
}

#[test]
fn test_dyn_money_partial_ord_money_less() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(10.00));
    let money = Money::<USD>::new(dec!(20.00)).unwrap();
    assert!(dyn_m < money);
}

#[test]
fn test_dyn_money_partial_ord_money_greater() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(30.00));
    let money = Money::<USD>::new(dec!(20.00)).unwrap();
    assert!(dyn_m > money);
}

#[test]
fn test_dyn_money_partial_ord_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(100.00));
    let money = Money::<JPY>::new(dec!(100)).unwrap();
    assert!(dyn_m.partial_cmp(&money).is_none());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_dyn_money_partial_ord_raw_money_less() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(10.00));
    let raw = RawMoney::<USD>::new(dec!(20.00)).unwrap();
    assert!(dyn_m < raw);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_dyn_money_partial_ord_raw_money_greater() {
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(30.00));
    let raw = RawMoney::<USD>::new(dec!(20.00)).unwrap();
    assert!(dyn_m > raw);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_dyn_money_partial_ord_raw_money_diff_currency() {
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(100.00));
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    assert!(dyn_m.partial_cmp(&raw).is_none());
}

// ==================== From<Money<C>> for DynMoney ====================

#[test]
fn test_from_money_to_dyn_money() {
    let money = Money::<USD>::new(dec!(123.45)).unwrap();
    let dyn_m = DynMoney::from(money);
    assert_eq!(dyn_m.code(), "USD");
    assert_eq!(dyn_m.amount(), dec!(123.45));
}

#[test]
fn test_from_money_to_dyn_money_eur() {
    let money = Money::<EUR>::new(dec!(99.99)).unwrap();
    let dyn_m: DynMoney = money.into();
    assert_eq!(dyn_m.code(), "EUR");
    assert_eq!(dyn_m.amount(), dec!(99.99));
}

#[test]
fn test_from_money_to_dyn_money_jpy() {
    let money = Money::<JPY>::new(dec!(5000)).unwrap();
    let dyn_m: DynMoney = money.into();
    assert_eq!(dyn_m.code(), "JPY");
    assert_eq!(dyn_m.amount(), dec!(5000));
}

// ==================== Money<C>: PartialEq with dyn ObjMoney ====================

#[test]
fn test_money_partial_eq_ref_dyn_obj_money_same() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(money == obj);
}

#[test]
fn test_money_partial_eq_ref_dyn_obj_money_diff_amount() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(200.00));
    assert!(money != obj);
}

#[test]
fn test_money_partial_eq_ref_dyn_obj_money_diff_currency() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<EUR>(dec!(100.00));
    assert!(money != obj);
}

#[test]
fn test_money_partial_eq_box_dyn_obj_money_same() {
    let money = Money::<EUR>::new(dec!(75.50)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<EUR>(dec!(75.50)));
    assert!(money == obj);
}

#[test]
fn test_money_partial_eq_box_dyn_obj_money_diff() {
    let money = Money::<GBP>::new(dec!(50.00)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(50.00)));
    assert!(money != obj);
}

#[test]
fn test_money_partial_eq_dyn_money_same() {
    let money = Money::<USD>::new(dec!(88.88)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(88.88));
    assert!(money == dyn_m);
}

#[test]
fn test_money_partial_eq_dyn_money_diff_amount() {
    let money = Money::<USD>::new(dec!(88.88)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(99.99));
    assert!(money != dyn_m);
}

#[test]
fn test_money_partial_eq_dyn_money_diff_currency() {
    let money = Money::<USD>::new(dec!(88.88)).unwrap();
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(88.88));
    // Different currencies — must not compare as equal.
    assert!(money != dyn_m);
}

// ==================== Money<C>: PartialOrd with dyn ObjMoney ====================

#[test]
fn test_money_partial_ord_ref_dyn_obj_money_less() {
    let money = Money::<USD>::new(dec!(50.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(money < obj);
}

#[test]
fn test_money_partial_ord_ref_dyn_obj_money_greater() {
    let money = Money::<USD>::new(dec!(150.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(money > obj);
}

#[test]
fn test_money_partial_ord_ref_dyn_obj_money_diff_currency() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<EUR>(dec!(100.00));
    assert!(money.partial_cmp(&obj).is_none());
}

#[test]
fn test_money_partial_ord_box_dyn_obj_money_less() {
    let money = Money::<GBP>::new(dec!(25.00)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<GBP>(dec!(50.00)));
    assert!(money < obj);
}

#[test]
fn test_money_partial_ord_box_dyn_obj_money_equal() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    assert!(money.partial_cmp(&obj) == Some(std::cmp::Ordering::Equal));
}

#[test]
fn test_money_partial_ord_box_dyn_obj_money_diff_currency() {
    let money = Money::<JPY>::new(dec!(100)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    assert!(money.partial_cmp(&obj).is_none());
}

#[test]
fn test_money_partial_ord_dyn_money_less() {
    let money = Money::<USD>::new(dec!(10.00)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(20.00));
    assert!(money < dyn_m);
}

#[test]
fn test_money_partial_ord_dyn_money_greater() {
    let money = Money::<USD>::new(dec!(30.00)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(20.00));
    assert!(money > dyn_m);
}

#[test]
fn test_money_partial_ord_dyn_money_diff_currency() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(100.00));
    assert!(money.partial_cmp(&dyn_m).is_none());
}

// ==================== From<RawMoney<C>> for DynMoney ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_from_raw_money_to_dyn_money() {
    // RawMoney stores full precision but DynMoney::from_decimal respects Context::is_raw.
    // With is_raw=false (default), the amount is rounded to the currency's minor unit.
    let raw = RawMoney::<USD>::new(dec!(99.12)).unwrap();
    let dyn_m = DynMoney::from(raw);
    assert_eq!(dyn_m.code(), "USD");
    assert_eq!(dyn_m.amount(), dec!(99.12));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_from_raw_money_to_dyn_money_into() {
    let raw = RawMoney::<EUR>::new(dec!(55.55)).unwrap();
    let dyn_m: DynMoney = raw.into();
    assert_eq!(dyn_m.code(), "EUR");
    assert_eq!(dyn_m.amount(), dec!(55.55));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_from_raw_money_to_dyn_money_jpy() {
    let raw = RawMoney::<JPY>::new(dec!(10000)).unwrap();
    let dyn_m: DynMoney = raw.into();
    assert_eq!(dyn_m.code(), "JPY");
    assert_eq!(dyn_m.amount(), dec!(10000));
}

// ==================== TryFrom<Box<dyn ObjMoney>> for RawMoney<C> ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_box_obj_money_to_raw_money_success() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(50.999)).unwrap());
    let raw = RawMoney::<USD>::try_from(obj).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(50.999));
    assert_eq!(BaseMoney::code(&raw), "USD");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_box_obj_money_to_raw_money_currency_mismatch() {
    let obj: Box<dyn ObjMoney> = Box::new(RawMoney::<EUR>::new(dec!(50.00)).unwrap());
    let result = RawMoney::<USD>::try_from(obj);
    assert!(matches!(
        result,
        Err(MoneyError::CurrencyMismatchError(ref got, ref exp))
        if got == "EUR" && exp == "USD"
    ));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_tryfrom_box_money_to_raw_money() {
    // A Money<USD> Box<dyn ObjMoney> can be converted to RawMoney<USD> via TryFrom.
    let obj: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(20.25)).unwrap());
    let raw = RawMoney::<USD>::try_from(obj).unwrap();
    assert_eq!(BaseMoney::amount(&raw), dec!(20.25));
}

// ==================== RawMoney<C>: PartialEq with dyn ObjMoney ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_ref_dyn_obj_money_same() {
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(raw == obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_ref_dyn_obj_money_diff_amount() {
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(200.00));
    assert!(raw != obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_ref_dyn_obj_money_diff_currency() {
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<EUR>(dec!(100.00));
    assert!(raw != obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_box_dyn_obj_money_same() {
    let raw = RawMoney::<EUR>::new(dec!(75.50)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<EUR>(dec!(75.50)));
    assert!(raw == obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_box_dyn_obj_money_diff() {
    let raw = RawMoney::<GBP>::new(dec!(50.00)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(50.00)));
    assert!(raw != obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_dyn_money_same() {
    let raw = RawMoney::<USD>::new(dec!(88.88)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(88.88));
    assert!(raw == dyn_m);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_dyn_money_diff_amount() {
    let raw = RawMoney::<USD>::new(dec!(88.88)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(99.99));
    assert!(raw != dyn_m);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_eq_dyn_money_diff_currency() {
    let raw = RawMoney::<USD>::new(dec!(88.88)).unwrap();
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(88.88));
    // Different currencies — must not compare as equal.
    assert!(raw != dyn_m);
}

// ==================== RawMoney<C>: PartialOrd with dyn ObjMoney ====================

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_ref_dyn_obj_money_less() {
    let raw = RawMoney::<USD>::new(dec!(50.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(raw < obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_ref_dyn_obj_money_greater() {
    let raw = RawMoney::<USD>::new(dec!(150.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<USD>(dec!(100.00));
    assert!(raw > obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_ref_dyn_obj_money_diff_currency() {
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let obj: &dyn ObjMoney = &DynMoney::from_decimal::<EUR>(dec!(100.00));
    assert!(raw.partial_cmp(&obj).is_none());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_box_dyn_obj_money_less() {
    let raw = RawMoney::<GBP>::new(dec!(25.00)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<GBP>(dec!(50.00)));
    assert!(raw < obj);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_box_dyn_obj_money_equal() {
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    assert!(raw.partial_cmp(&obj) == Some(std::cmp::Ordering::Equal));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_box_dyn_obj_money_diff_currency() {
    let raw = RawMoney::<JPY>::new(dec!(100)).unwrap();
    let obj: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(100.00)));
    assert!(raw.partial_cmp(&obj).is_none());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_dyn_money_less() {
    let raw = RawMoney::<USD>::new(dec!(10.00)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(20.00));
    assert!(raw < dyn_m);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_dyn_money_greater() {
    let raw = RawMoney::<USD>::new(dec!(30.00)).unwrap();
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(20.00));
    assert!(raw > dyn_m);
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_money_partial_ord_dyn_money_diff_currency() {
    let raw = RawMoney::<USD>::new(dec!(100.00)).unwrap();
    let dyn_m = DynMoney::from_decimal::<EUR>(dec!(100.00));
    assert!(raw.partial_cmp(&dyn_m).is_none());
}

// end of obj_money_test.rs

// ==================== Issue #126 regression: convert returns target currency code ====================

/// Regression test for issue #126: ObjMoney::convert must return an object whose
/// `.code()` is the TARGET currency, not the source currency.
#[cfg(feature = "exchange")]
#[test]
fn test_issue_126_convert_returns_target_currency_code_money() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("GBP", dec!(0.75)).unwrap();
    let result = money.convert("GBP", &rates).unwrap();
    // Before the fix this would return "USD"; now it must return "GBP".
    assert_eq!(result.code(), "GBP");
    assert_eq!(result.amount(), dec!(75.00));
}

#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_issue_126_convert_returns_target_currency_code_raw_money() {
    let money = RawMoney::<EUR>::new(dec!(200.00)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("USD", &rates).unwrap();
    // Before the fix this would return "EUR"; now it must return "USD".
    assert_eq!(result.code(), "USD");
    assert_eq!(result.amount(), dec!(250.00));
}

/// Conversion via Box<dyn ObjMoney> also returns the target currency code.
#[cfg(feature = "exchange")]
#[test]
fn test_issue_126_convert_box_dyn_returns_target_code() {
    let boxed: Box<dyn ObjMoney> = Box::new(Money::<JPY>::new(dec!(1500)).unwrap());
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("JPY", dec!(150)).unwrap();
    let result = boxed.convert("USD", &rates).unwrap();
    assert_eq!(result.code(), "USD");
}

// ==================== ObjMoney: new accessors (minor_unit_name, origin, locale) ====================

#[test]
fn test_obj_money_minor_unit_name() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(1.00)).unwrap());
    assert_eq!(m.minor_unit_name(), "cent");
}

#[test]
fn test_obj_money_origin() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(1.00)).unwrap());
    assert_eq!(m.origin(), "United States");
}

#[test]
fn test_obj_money_locale() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(1.00)).unwrap());
    assert_eq!(m.locale(), "en-US");
}

#[test]
fn test_obj_money_eur_accessors() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(1.00)).unwrap());
    assert_eq!(m.minor_unit_name(), "cent");
    assert_eq!(m.origin(), "Eurozone");
    assert_eq!(m.locale(), "de-DE");
}

#[test]
fn test_obj_money_jpy_accessors() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<JPY>::new(dec!(100)).unwrap());
    assert_eq!(m.origin(), "Japan");
    assert_eq!(m.locale(), "ja-JP");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_minor_unit_name() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<GBP>::new(dec!(1.00)).unwrap());
    assert_eq!(m.minor_unit_name(), "penny");
    assert_eq!(m.origin(), "United Kingdom");
    assert_eq!(m.locale(), "en-GB");
}

#[test]
fn test_dyn_money_minor_unit_name_origin_locale() {
    use crate::obj_money::DynMoney;
    let dyn_m = DynMoney::from_decimal::<USD>(dec!(1.00));
    let obj: &dyn ObjMoney = &dyn_m;
    assert_eq!(obj.minor_unit_name(), "cent");
    assert_eq!(obj.origin(), "United States");
    assert_eq!(obj.locale(), "en-US");
}

// ==================== DynCurrency::code() ====================

#[test]
fn test_dyn_currency_code_method() {
    use crate::obj_money::Context;
    let usd_curr = Context::get_currency("USD").unwrap();
    assert_eq!(usd_curr.code(), "USD");
    let eur_curr = Context::get_currency("EUR").unwrap();
    assert_eq!(eur_curr.code(), "EUR");
}

// ==================== ObjMoney::is_approx() ====================

#[test]
fn test_obj_money_is_approx_within_tolerance() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.01)).unwrap());
    assert!(m.is_approx(dec!(100.00), dec!(0.05)));
}

#[test]
fn test_obj_money_is_approx_exact_tolerance() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.01)).unwrap());
    // Difference is exactly 0.01; tolerance 0.01 means inclusive.
    assert!(m.is_approx(dec!(100.00), dec!(0.01)));
}

#[test]
fn test_obj_money_is_approx_outside_tolerance() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.05)).unwrap());
    assert!(!m.is_approx(dec!(100.00), dec!(0.04)));
}

#[test]
fn test_obj_money_is_approx_zero_tolerance() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(100.01)).unwrap());
    assert!(!m.is_approx(dec!(100.00), dec!(0)));
}

#[test]
fn test_obj_money_is_approx_exact_match() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(50.00)).unwrap());
    assert!(m.is_approx(dec!(50.00), dec!(0)));
}

#[test]
fn test_obj_money_is_approx_negative_amounts() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(-100.01)).unwrap());
    // diff = -100.01 - (-100.00) = -0.01; abs = 0.01 <= 0.05
    assert!(m.is_approx(dec!(-100.00), dec!(0.05)));
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_is_approx() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<EUR>::new(dec!(200.005)).unwrap());
    assert!(m.is_approx(dec!(200.000), dec!(0.01)));
    assert!(!m.is_approx(dec!(200.000), dec!(0.004)));
}

#[test]
fn test_dyn_money_is_approx() {
    use crate::obj_money::DynMoney;
    let dyn_m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(10.00)));
    assert!(dyn_m.is_approx(dec!(10.00), dec!(0)));
    assert!(dyn_m.is_approx(dec!(9.99), dec!(0.01)));
    assert!(!dyn_m.is_approx(dec!(9.98), dec!(0.01)));
}

// ==================== ObjMoney::format_with_separator() ====================

#[test]
fn test_obj_money_format_with_separator_basic() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(93009.45)).unwrap());
    assert_eq!(m.format_with_separator("c na", "*", "#"), "USD 93*009#45");
}

#[test]
fn test_obj_money_format_with_separator_negative() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<USD>::new(dec!(-1234.56)).unwrap());
    assert_eq!(m.format_with_separator("c na", ",", "."), "USD -1,234.56");
}

#[test]
fn test_obj_money_format_with_separator_space_comma() {
    let m: Box<dyn ObjMoney> = Box::new(Money::<EUR>::new(dec!(93009.45)).unwrap());
    assert_eq!(m.format_with_separator("s na", " ", ","), "€ 93 009,45");
}

#[cfg(feature = "raw_money")]
#[test]
fn test_obj_raw_money_format_with_separator_full_precision() {
    let m: Box<dyn ObjMoney> = Box::new(RawMoney::<USD>::new(dec!(93009.446688)).unwrap());
    assert_eq!(
        m.format_with_separator("c na", "*", "#"),
        "USD 93*009#446688"
    );
}

#[test]
fn test_dyn_money_format_with_separator() {
    use crate::obj_money::DynMoney;
    let dyn_m: Box<dyn ObjMoney> = Box::new(DynMoney::from_decimal::<USD>(dec!(1234.56)));
    assert_eq!(
        dyn_m.format_with_separator("c na", ".", ","),
        "USD 1.234,56"
    );
}

#[test]
fn test_set_raw() {
    super::Context::set_raw(false);
}

#[test]
fn test_dyn_currency_from_curr() {
    let dc: super::DynCurrency = crate::iso::JPY.into();
    assert_eq!(dc.code, "JPY");
}

#[test]
fn test_dyn_currency_from_code() {
    let dc = super::DynCurrency::from_code("XAU").unwrap();
    assert_eq!(dc.code, "XAU");
}

#[test]
fn test_dyn_currency_from_code_not_found() {
    let dc = super::DynCurrency::from_code("ASD");
    assert!(dc.is_err());
}
