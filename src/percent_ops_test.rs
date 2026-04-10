use crate::{
    BaseMoney, Decimal,
    macros::{dec, money, raw},
    percent_ops::PercentOps,
};

#[test]
fn test_percent_ops() {
    let money = money!(USD, 100);
    let tips = money.percent(15).unwrap();
    assert_eq!(tips.amount(), dec!(15));

    let money = raw!(USD, 100.3434);
    let tips = money.percent(15).unwrap();
    assert_eq!(tips.amount(), dec!(15.051510));

    let money = money!(USD, 100);
    let tips = money.percent(i128::MAX);
    assert!(tips.is_none());

    let money = money!(USD, 100);
    let tips = money.percent(crate::Decimal::MAX);
    assert!(tips.is_none());

    let money = money!(USD, 100);
    let after_tax = money.percent_add(50).unwrap();
    assert_eq!(after_tax.amount(), dec!(150));

    let money = raw!(USD, 100.3434);
    let after_tax = money.percent_add(40).unwrap();
    assert_eq!(after_tax.amount(), dec!(140.480760));

    let money = money!(USD, 100);
    let after_discount = money.percent_sub(50).unwrap();
    assert_eq!(after_discount.amount(), dec!(50));

    let money = raw!(USD, 100.3434);
    let after_discount = money.percent_sub(40).unwrap();
    assert_eq!(after_discount.amount(), dec!(60.206040));

    let profit = money!(USD, 200);
    let revenue = money!(USD, 300);
    let margin = profit.percent_of(revenue).unwrap();
    assert_eq!(margin.amount(), dec!(67));

    let profit = raw!(USD, 200.4004);
    let revenue = raw!(USD, 300.123123);
    let margin = profit.percent_of(revenue).unwrap();
    assert_eq!(margin.amount(), dec!(66.772729137567984056996501400));

    let profit = money!(USD, 200);
    let margin = profit.percent_of(i128::MAX);
    assert!(margin.is_none());

    let profit = money!(USD, 200);
    let margin = profit.percent_of(0);
    assert!(margin.is_none());

    // fixed additions
    let base = money!(USD, 1_000_000);
    let insurance_percent = 5;
    let transportation_percent = 10;
    let royalty = 5;
    let ret = base
        .percent_adds_fixed([insurance_percent, transportation_percent, royalty])
        .unwrap();
    assert_eq!(ret.amount(), dec!(1_200_000));

    let base = money!(USD, 1_000_000);
    let insurance_percent = 5;
    let transportation_percent = 10;
    let royalty = i128::MAX;
    let ret = base.percent_adds_fixed([insurance_percent, transportation_percent, royalty]);
    assert!(ret.is_none());

    // compounding additions
    let base = money!(USD, 1_000_000);
    let insurance_percent = 5;
    let transportation_percent = 10;
    let royalty = 5;
    let ret = base
        .percent_adds_compound([insurance_percent, transportation_percent, royalty])
        .unwrap();
    assert_eq!(ret.amount(), dec!(1_212_750));

    let base = money!(USD, 1_000_000);
    let insurance_percent = 5;
    let transportation_percent = 10;
    let royalty = i128::MAX;
    let ret = base.percent_adds_compound([insurance_percent, transportation_percent, royalty]);
    assert!(ret.is_none());

    // sequence reduction
    let gross = money!(IDR, 50_000_000);
    let pph21 = 20;
    let bpjs = 10;
    let tapera = 5;
    let ret = gross.percent_subs_sequence([pph21, bpjs, tapera]).unwrap();
    assert_eq!(ret.amount(), dec!(34_200_000));

    let gross = money!(IDR, 50_000_000);
    let pph21 = 20;
    let bpjs = 10;
    let tapera = i128::MAX;
    let ret = gross.percent_subs_sequence([pph21, bpjs, tapera]);
    assert!(ret.is_none());
}

// Tests for None paths via `percent` failure (Decimal::MAX overflow).
// These cover the `?` operators on `percent(?)?.` in each compound/sequence
// function, which were not reachable through the i128::MAX tests above
// (those only cover the `get_decimal()?` None path, not the `percent(?)?` path).

#[test]
fn test_percent_add_none_via_overflow() {
    let money = money!(USD, 100);
    let ret = money.percent_add(crate::Decimal::MAX);
    assert!(ret.is_none());
}

#[test]
fn test_percent_sub_none_via_overflow() {
    let money = money!(USD, 100);
    let ret = money.percent_sub(crate::Decimal::MAX);
    assert!(ret.is_none());
}

#[test]
fn test_percent_adds_fixed_none_via_percent_overflow() {
    // Decimal::MAX as the only percent: get_decimal() succeeds but
    // self.percent(Decimal::MAX) overflows → the `?.amount()` `?` fires.
    let money = money!(USD, 100);
    let ret = money.percent_adds_fixed([crate::Decimal::MAX]);
    assert!(ret.is_none());
}

#[test]
fn test_percent_adds_compound_none_via_percent_overflow() {
    // Same: current.percent(Decimal::MAX) overflows → the inner `?` fires.
    let money = money!(USD, 100);
    let ret = money.percent_adds_compound([crate::Decimal::MAX]);
    assert!(ret.is_none());
}

#[test]
fn test_percent_subs_sequence_none_via_percent_overflow() {
    // Same: current.percent(Decimal::MAX) overflows → the inner `?` fires.
    let money = money!(USD, 100);
    let ret = money.percent_subs_sequence([crate::Decimal::MAX]);
    assert!(ret.is_none());
}

// Test for the `checked_mul(dec!(100))` None path in `percent_of`.
// If self / rhs is large enough, multiplying by 100 overflows Decimal::MAX.
// With profit = 1 and revenue = 10^-27 the quotient is 10^27;
// 10^27 × 100 = 10^29 > Decimal::MAX ≈ 7.92 × 10^28 → overflow → None.
#[cfg(feature = "raw_money")]
#[test]
fn test_percent_of_none_via_checked_mul_overflow() {
    use crate::RawMoney;
    use crate::iso::USD;
    let profit = raw!(USD, 1);
    // 1e-27 = Decimal::new(1, 27)
    let tiny_revenue = RawMoney::<USD>::from_decimal(Decimal::new(1, 27));
    let margin = profit.percent_of(tiny_revenue);
    assert!(margin.is_none());
}

// Tests for the final `?` on checked_add in percent_adds_fixed and
// percent_adds_compound loops, covering the None path when accumulating
// the result overflows Decimal::MAX.
//
// Using JPY (0dp) with a value near Decimal::MAX and a small percentage
// so that percent() itself succeeds, but the accumulated sum exceeds
// Decimal::MAX and checked_add returns None.

#[test]
fn test_percent_adds_fixed_none_via_checked_add_overflow() {
    use std::str::FromStr;
    // 7.9e28 ≈ Decimal::MAX; 0.5% of it ≈ 3.95e26;
    // 7.9e28 + 3.95e26 ≈ 7.9395e28 > Decimal::MAX → checked_add returns None.
    let money = crate::Money::<crate::iso::JPY>::from_decimal(
        Decimal::from_str("79000000000000000000000000000").unwrap(),
    );
    let ret = money.percent_adds_fixed([dec!(0.5)]);
    assert!(ret.is_none());
}

#[test]
fn test_percent_adds_compound_none_via_checked_add_overflow() {
    use std::str::FromStr;
    let money = crate::Money::<crate::iso::JPY>::from_decimal(
        Decimal::from_str("79000000000000000000000000000").unwrap(),
    );
    let ret = money.percent_adds_compound([dec!(0.5)]);
    assert!(ret.is_none());
}

// Test for the `checked_sub(current.percent(?)?)?.` None path in
// `percent_subs_sequence`.  Starting from -Decimal::MAX, subtracting -1%
// produces a positive value; then result - positive_value goes below
// -Decimal::MAX and `checked_sub` returns None.
#[cfg(feature = "raw_money")]
#[test]
fn test_percent_subs_sequence_none_via_checked_sub_overflow() {
    use crate::RawMoney;
    use crate::iso::USD;
    // -Decimal::MAX as RawMoney. Subtracting -1% gives a positive amount
    // equal to Decimal::MAX/100, and result - that_value = -Decimal::MAX - Decimal::MAX/100
    // which is below -Decimal::MAX → checked_sub returns None.
    let money = RawMoney::<USD>::from_decimal(-crate::Decimal::MAX);
    let ret = money.percent_subs_sequence([-1i32]);
    assert!(ret.is_none());
}
