use crate::{
    BaseMoney,
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
