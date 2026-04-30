/// Tests for heterogeneous collections of `Money` and `RawMoney` with different currencies,
/// using the object-safe `ObjMoney` trait for dynamic dispatch (`dyn`).
use super::ObjMoney;
use crate::iso::{CHF, EUR, GBP, INR, JPY, SGD, USD};
use crate::macros::dec;
use crate::{BaseMoney, BaseOps, Decimal, Money, money, raw};

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
    // EUR(0) counts as zero; Decimal zero has a positive sign bit so is_positive() is also true for it
    assert_eq!(positives, 3); // USD(50), EUR(0), GBP(0.0001)
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
use crate::{ExchangeRates, MoneyError, ObjIterOps};

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
    let result: Money<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(350.00));
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
    let result: Money<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(150.00));
}

/// An empty collection should produce zero (Money::new(Decimal::ZERO)).
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_empty_collection() {
    let empty: Vec<Money<USD>> = vec![];
    let rates = ExchangeRates::<USD>::new();
    let result: Money<USD> = empty.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(0.00));
}

/// When a currency in the collection has no entry in the rates map,
/// `checked_sum` must return an `ExchangeError`.
#[cfg(feature = "exchange")]
#[test]
fn test_obj_iter_ops_checked_sum_missing_rate() {
    let portfolio = vec![Money::<EUR>::new(dec!(100.00)).unwrap()];
    let rates = ExchangeRates::<USD>::new(); // EUR not present
    let result: Result<Money<USD>, _> = portfolio.checked_sum(rates);
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
    let result: Result<Money<USD>, _> = portfolio.checked_sum(rates);
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
    let result: Result<Money<USD>, _> = portfolio.checked_sum(rates);
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
    let result: Money<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(90.00));
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
    let result: Money<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(300.00));
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
    let result: Result<Money<USD>, _> = portfolio.checked_sum(rates);
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
    let result: Money<USD> = arr.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(60.00));
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
fn test_obj_money_convert_to_eur() {
    let money = Money::<USD>::new(dec!(100.00)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(80.00));
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

/// RawMoney converts to same currency without changing amount.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_raw_money_convert_same_currency() {
    let money = RawMoney::<USD>::new(dec!(123.456789)).unwrap();
    let rates = ExchangeRates::<USD>::new();
    let result = money.convert("USD", &rates).unwrap();
    assert_eq!(result.amount(), dec!(123.456789));
    assert_eq!(result.code(), "USD");
}

/// RawMoney preserves full precision after conversion (no rounding).
/// 100.123456 * 0.8 = 80.0987648
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_raw_money_convert_to_different_currency() {
    let money = RawMoney::<USD>::new(dec!(100.123456)).unwrap();
    let mut rates = ExchangeRates::<USD>::new();
    rates.set("EUR", dec!(0.8)).unwrap();
    let result = money.convert("EUR", &rates).unwrap();
    assert_eq!(result.amount(), dec!(80.0987648));
}

/// RawMoney missing rate also returns ExchangeError.
#[cfg(all(feature = "exchange", feature = "raw_money"))]
#[test]
fn test_obj_raw_money_convert_missing_rate() {
    let money = RawMoney::<EUR>::new(dec!(50.00)).unwrap();
    let rates = ExchangeRates::<USD>::new(); // EUR not present (EUR→USD would need EUR in rates)
    // get_pair("EUR","JPY") → None (neither EUR nor JPY in a fresh USD-based rates)
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
/// get_pair("JPY","USD")=1/150 ≈ 0.00666...; 1500*(1/150)=10
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
    let result: RawMoney<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(300.579));
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
    let result: RawMoney<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(125.00125));
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
    let result: Money<USD> = portfolio.checked_sum(rates).unwrap();
    assert_eq!(BaseMoney::amount(&result), dec!(200.00));
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

// end of obj_money_test.rs
