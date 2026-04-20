/// Tests for heterogeneous collections of `Money` and `RawMoney` with different currencies,
/// using the object-safe `ObjMoney` trait for dynamic dispatch (`dyn`).
use super::ObjMoney;
use crate::iso::{CHF, EUR, GBP, INR, JPY, SGD, USD};
use crate::macros::dec;
use crate::{BaseMoney, BaseOps, Decimal, Money};

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
    assert!(portfolio[2].is_positive());
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
    assert_eq!(positives, 4);
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
    // Extract same-currency moneys from a dyn vec and use typed arithmetic.
    let portfolio: Vec<Box<dyn ObjMoney>> = vec![
        Box::new(Money::<USD>::new(dec!(1000.00)).unwrap()),
        Box::new(Money::<EUR>::new(dec!(850.00)).unwrap()),
        Box::new(Money::<USD>::new(dec!(250.00)).unwrap()),
        Box::new(Money::<GBP>::new(dec!(600.00)).unwrap()),
    ];

    // Aggregate USD amounts into a typed Money<USD> via Decimal
    let usd_sum = portfolio
        .iter()
        .filter(|m| m.code() == "USD")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    let aggregated = Money::<USD>::from_decimal(usd_sum);
    assert_eq!(BaseMoney::amount(&aggregated), dec!(1250.00));

    // checked_add on the aggregated value
    let bonus = Money::<USD>::new(dec!(100.00)).unwrap();
    let total = aggregated.checked_add(bonus).unwrap();
    assert_eq!(BaseMoney::amount(&total), dec!(1350.00));

    // checked_sub
    let fee = Money::<USD>::new(dec!(50.00)).unwrap();
    let net = total.checked_sub(fee).unwrap();
    assert_eq!(BaseMoney::amount(&net), dec!(1300.00));

    // checked_mul by a scalar
    let doubled = net.checked_mul(dec!(2)).unwrap();
    assert_eq!(BaseMoney::amount(&doubled), dec!(2600.00));

    // checked_div by a scalar
    let halved = doubled.checked_div(dec!(4)).unwrap();
    assert_eq!(BaseMoney::amount(&halved), dec!(650.00));
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

    // Aggregate USD raw amounts
    let usd_sum = portfolio
        .iter()
        .filter(|m| m.code() == "USD")
        .fold(Decimal::ZERO, |acc, m| acc + m.amount());

    let aggregated = RawMoney::<USD>::new(usd_sum).unwrap();
    assert_eq!(BaseMoney::amount(&aggregated), dec!(750.80235));

    // checked_add
    let extra = RawMoney::<USD>::new(dec!(49.19765)).unwrap();
    let total = aggregated.checked_add(extra).unwrap();
    assert_eq!(BaseMoney::amount(&total), dec!(800.00000));

    // checked_sub
    let fee = RawMoney::<USD>::new(dec!(0.00001)).unwrap();
    let after_fee = total.checked_sub(fee).unwrap();
    assert_eq!(BaseMoney::amount(&after_fee), dec!(799.99999));

    // checked_mul
    let scaled = after_fee.checked_mul(dec!(2)).unwrap();
    assert_eq!(BaseMoney::amount(&scaled), dec!(1599.99998));

    // checked_div
    let halved = scaled.checked_div(dec!(2)).unwrap();
    assert_eq!(BaseMoney::amount(&halved), dec!(799.99999));
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

// end of obj_money_test.rs
