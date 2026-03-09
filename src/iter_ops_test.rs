use crate::iso::{JPY, USD};
use crate::macros::dec;
use crate::{BaseMoney, IterOps, Money};

// ==================== checked_sum Tests ====================

#[test]
fn test_checked_sum_basic() {
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
    ];
    assert_eq!(moneys.checked_sum().unwrap().amount(), dec!(60.00));
}

#[test]
fn test_checked_sum_single_element() {
    let moneys = vec![Money::<USD>::new(dec!(42.00)).unwrap()];
    assert_eq!(moneys.checked_sum().unwrap().amount(), dec!(42.00));
}

#[test]
fn test_checked_sum_empty_returns_zero() {
    let empty: Vec<Money<USD>> = vec![];
    // checked_sum on an empty collection returns Some(zero), not None
    assert_eq!(empty.checked_sum().unwrap().amount(), dec!(0));
}

#[test]
fn test_checked_sum_with_negatives() {
    let moneys = vec![
        Money::<USD>::new(dec!(100.00)).unwrap(),
        Money::<USD>::new(dec!(-30.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
    ];
    assert_eq!(moneys.checked_sum().unwrap().amount(), dec!(90.00));
}

// ==================== mean Tests ====================

#[test]
fn test_mean_basic() {
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
    ];
    assert_eq!(moneys.mean().unwrap().amount(), dec!(20.00));
}

#[test]
fn test_mean_single_element() {
    let moneys = vec![Money::<USD>::new(dec!(42.00)).unwrap()];
    assert_eq!(moneys.mean().unwrap().amount(), dec!(42.00));
}

#[test]
fn test_mean_empty_returns_none() {
    let empty: Vec<Money<USD>> = vec![];
    assert!(empty.mean().is_none());
}

#[test]
fn test_mean_rounds_to_minor_unit() {
    // Mean of 10 and 11 is 10.50, which is exact for USD (2 decimal places)
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(11.00)).unwrap(),
    ];
    assert_eq!(moneys.mean().unwrap().amount(), dec!(10.50));
}

#[test]
fn test_mean_rounds_to_minor_unit_jpy() {
    // JPY has 0 minor units; mean of 10 and 11 = 10.5 -> rounds to 10 (bankers rounding)
    let moneys = vec![
        Money::<JPY>::new(dec!(10)).unwrap(),
        Money::<JPY>::new(dec!(11)).unwrap(),
    ];
    assert_eq!(moneys.mean().unwrap().amount(), dec!(10));
}

#[test]
fn test_mean_with_negatives() {
    let moneys = vec![
        Money::<USD>::new(dec!(-10.00)).unwrap(),
        Money::<USD>::new(dec!(0.00)).unwrap(),
        Money::<USD>::new(dec!(10.00)).unwrap(),
    ];
    assert_eq!(moneys.mean().unwrap().amount(), dec!(0.00));
}

#[test]
fn test_mean_slice() {
    let moneys = [
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
    ];
    assert_eq!(moneys.mean().unwrap().amount(), dec!(20.00));
}

// ==================== median Tests ====================

#[test]
fn test_median_odd_count() {
    let moneys = vec![
        Money::<USD>::new(dec!(30.00)).unwrap(),
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
    ];
    assert_eq!(moneys.median().unwrap().amount(), dec!(20.00));
}

#[test]
fn test_median_even_count() {
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
        Money::<USD>::new(dec!(40.00)).unwrap(),
    ];
    // Median = (20.00 + 30.00) / 2 = 25.00
    assert_eq!(moneys.median().unwrap().amount(), dec!(25.00));
}

#[test]
fn test_median_single_element() {
    let moneys = vec![Money::<USD>::new(dec!(42.00)).unwrap()];
    assert_eq!(moneys.median().unwrap().amount(), dec!(42.00));
}

#[test]
fn test_median_two_elements() {
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
    ];
    assert_eq!(moneys.median().unwrap().amount(), dec!(15.00));
}

#[test]
fn test_median_empty_returns_none() {
    let empty: Vec<Money<USD>> = vec![];
    assert!(empty.median().is_none());
}

#[test]
fn test_median_unsorted_input() {
    // Input is unsorted; the implementation must sort internally
    let moneys = vec![
        Money::<USD>::new(dec!(50.00)).unwrap(),
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(40.00)).unwrap(),
    ];
    assert_eq!(moneys.median().unwrap().amount(), dec!(30.00));
}

#[test]
fn test_median_rounds_to_minor_unit_jpy() {
    // JPY: median of 10 and 11 = 10.5 -> rounds to 10 (bankers rounding)
    let moneys = vec![
        Money::<JPY>::new(dec!(10)).unwrap(),
        Money::<JPY>::new(dec!(11)).unwrap(),
    ];
    assert_eq!(moneys.median().unwrap().amount(), dec!(10));
}

// ==================== mode Tests ====================

#[test]
fn test_mode_basic() {
    // 10.00 appears twice, others once – mode is [10.00]
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
    ];
    let result = moneys.mode().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].amount(), dec!(10.00));
}

#[test]
fn test_mode_single_element() {
    // Single element → Some(vec![element])
    let moneys = vec![Money::<USD>::new(dec!(42.00)).unwrap()];
    let result = moneys.mode().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].amount(), dec!(42.00));
}

#[test]
fn test_mode_empty_returns_none() {
    let empty: Vec<Money<USD>> = vec![];
    assert!(empty.mode().is_none());
}

#[test]
fn test_mode_all_same() {
    // All elements are equal → Some(vec![that element])
    let moneys = vec![
        Money::<USD>::new(dec!(5.00)).unwrap(),
        Money::<USD>::new(dec!(5.00)).unwrap(),
        Money::<USD>::new(dec!(5.00)).unwrap(),
    ];
    let result = moneys.mode().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].amount(), dec!(5.00));
}

#[test]
fn test_mode_all_distinct_returns_none() {
    // All distinct, each appears once → uniform frequency → None
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(30.00)).unwrap(),
    ];
    assert!(moneys.mode().is_none());
}

#[test]
fn test_mode_all_equal_occurrences_returns_none() {
    // [1,1,2,2,3,3] – each value appears twice, no dominant group → None
    let moneys = vec![
        Money::<USD>::new(dec!(1.00)).unwrap(),
        Money::<USD>::new(dec!(1.00)).unwrap(),
        Money::<USD>::new(dec!(2.00)).unwrap(),
        Money::<USD>::new(dec!(2.00)).unwrap(),
        Money::<USD>::new(dec!(3.00)).unwrap(),
        Money::<USD>::new(dec!(3.00)).unwrap(),
    ];
    assert!(moneys.mode().is_none());
}

#[test]
fn test_mode_multimodal_returns_modes() {
    // [1,1,1,2,2,3,3,3] – 1 and 3 each appear 3 times, 2 appears twice
    // → Some([1.00, 3.00]) in first-occurrence order
    let moneys = vec![
        Money::<USD>::new(dec!(1.00)).unwrap(),
        Money::<USD>::new(dec!(1.00)).unwrap(),
        Money::<USD>::new(dec!(1.00)).unwrap(),
        Money::<USD>::new(dec!(2.00)).unwrap(),
        Money::<USD>::new(dec!(2.00)).unwrap(),
        Money::<USD>::new(dec!(3.00)).unwrap(),
        Money::<USD>::new(dec!(3.00)).unwrap(),
        Money::<USD>::new(dec!(3.00)).unwrap(),
    ];
    let mut result: Vec<_> = moneys.mode().unwrap();
    result.sort_by(|a, b| a.amount().cmp(&b.amount()));
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].amount(), dec!(1.00));
    assert_eq!(result[1].amount(), dec!(3.00));
}

#[test]
fn test_mode_highest_frequency_wins() {
    // 20.00 appears three times, 10.00 appears twice → mode is [20.00]
    let moneys = vec![
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
        Money::<USD>::new(dec!(10.00)).unwrap(),
    ];
    let result = moneys.mode().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].amount(), dec!(20.00));
}

#[test]
fn test_mode_slice() {
    let moneys = [
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(10.00)).unwrap(),
        Money::<USD>::new(dec!(20.00)).unwrap(),
    ];
    let result = moneys.mode().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].amount(), dec!(10.00));
}
