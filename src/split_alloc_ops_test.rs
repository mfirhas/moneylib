use rust_decimal::prelude::FromPrimitive;

use crate::dec;
use crate::{BaseOps, Decimal};
use crate::{
    Money,
    iso::{BHD, JPY, USD},
    money,
};

#[cfg(feature = "raw_money")]
use crate::{BaseMoney, RawMoney, macros::raw};

struct SplitCase {
    money: Money<USD>,
    n: u32,
    expected: Option<(Money<USD>, Money<USD>)>, // (base, remainder)
}

#[test]
fn test_split() {
    let cases = vec![
        SplitCase {
            money: money!(USD, 10.00),
            n: 3,
            expected: Some((money!(USD, 3.33), money!(USD, 0.01))),
        },
        SplitCase {
            money: money!(USD, 10.00),
            n: 2,
            expected: Some((money!(USD, 5.00), money!(USD, 0.00))),
        },
        SplitCase {
            money: money!(USD, 0.01),
            n: 2,
            expected: Some((money!(USD, 0.00), money!(USD, 0.01))),
        },
        SplitCase {
            money: money!(USD, 0.00),
            n: 3,
            expected: Some((money!(USD, 0.00), money!(USD, 0.00))),
        },
        // n=0 is invalid
        SplitCase {
            money: money!(USD, 10.00),
            n: 0,
            expected: None,
        },
        // single part — all goes to base
        SplitCase {
            money: money!(USD, 10.00),
            n: 1,
            expected: Some((money!(USD, 10.00), money!(USD, 0.00))),
        },
        // indivisible remainder
        SplitCase {
            money: money!(USD, 10.01),
            n: 3,
            expected: Some((money!(USD, 3.33), money!(USD, 0.02))),
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let result = case.money.split(case.n);
        assert_eq!(
            result, case.expected,
            "{}. split({}, {})",
            i, case.money, case.n
        );
        if let Some((base, remainder)) = result {
            let total = (base * Decimal::from_u32(case.n).unwrap()) + remainder;
            assert_eq!(total, case.money);
        }
    }
}

struct SplitDistCase {
    money: Money<USD>,
    n: u32,
    expected: Option<Vec<Money<USD>>>,
}

#[test]
fn test_split_dist() {
    let cases = vec![
        SplitDistCase {
            money: money!(USD, 10.00),
            n: 3,
            expected: Some(vec![
                money!(USD, 3.34),
                money!(USD, 3.33),
                money!(USD, 3.33),
            ]),
        },
        SplitDistCase {
            money: money!(USD, 10.00),
            n: 2,
            expected: Some(vec![money!(USD, 5.00), money!(USD, 5.00)]),
        },
        SplitDistCase {
            money: money!(USD, 0.03),
            n: 3,
            expected: Some(vec![
                money!(USD, 0.01),
                money!(USD, 0.01),
                money!(USD, 0.01),
            ]),
        },
        // remainder distributed across first two parts
        SplitDistCase {
            money: money!(USD, 10.02),
            n: 3,
            expected: Some(vec![
                money!(USD, 3.34),
                money!(USD, 3.34),
                money!(USD, 3.34),
            ]),
        },
        SplitDistCase {
            money: money!(USD, 10.01),
            n: 3,
            expected: Some(vec![
                money!(USD, 3.34),
                money!(USD, 3.34),
                money!(USD, 3.33),
            ]),
        },
        // single part
        SplitDistCase {
            money: money!(USD, 10.00),
            n: 1,
            expected: Some(vec![money!(USD, 10.00)]),
        },
        // zero money
        SplitDistCase {
            money: money!(USD, 0.00),
            n: 3,
            expected: Some(vec![
                money!(USD, 0.00),
                money!(USD, 0.00),
                money!(USD, 0.00),
            ]),
        },
        // n=0 invalid
        SplitDistCase {
            money: money!(USD, 10.00),
            n: 0,
            expected: None,
        },
        // small indivisible amount
        SplitDistCase {
            money: money!(USD, 0.01),
            n: 2,
            expected: Some(vec![money!(USD, 0.01), money!(USD, 0.00)]),
        },
    ];

    for case in cases {
        let result = case.money.split_dist(case.n);
        assert_eq!(
            result, case.expected,
            "split_dist({}, {})",
            case.money, case.n
        );
        // invariant: parts always sum to money
        if let Some(parts) = &result {
            let sum: Money<USD> = parts.iter().sum();
            assert_eq!(
                sum, case.money,
                "sum invariant failed for split_dist({}, {})",
                case.money, case.n
            );
        }
    }
}

struct AllocateCase {
    money: Money<USD>,
    pcns: Vec<Decimal>,
    expected: Option<Vec<Money<USD>>>,
}

#[test]
fn test_allocate() {
    let cases = vec![
        // even split via percentages
        AllocateCase {
            money: money!(USD, 100.00),
            pcns: vec![dec!(50), dec!(50)],
            expected: Some(vec![money!(USD, 50.00), money!(USD, 50.00)]),
        },
        // uneven percentages
        AllocateCase {
            money: money!(USD, 100.00),
            pcns: vec![dec!(33), dec!(33), dec!(34)],
            expected: Some(vec![
                money!(USD, 33.00),
                money!(USD, 33.00),
                money!(USD, 34.00),
            ]),
        },
        // remainder distributed to first part
        AllocateCase {
            money: money!(USD, 100.00),
            pcns: vec![dec!(33.33), dec!(33.33), dec!(33.34)],
            expected: Some(vec![
                money!(USD, 33.33),
                money!(USD, 33.33),
                money!(USD, 33.34),
            ]),
        },
        // small amount
        AllocateCase {
            money: money!(USD, 0.03),
            pcns: vec![dec!(50), dec!(50)],
            expected: Some(vec![money!(USD, 0.02), money!(USD, 0.01)]),
        },
        // zero money
        AllocateCase {
            money: money!(USD, 0.00),
            pcns: vec![dec!(50), dec!(50)],
            expected: Some(vec![money!(USD, 0.00), money!(USD, 0.00)]),
        },
        // empty percentages
        AllocateCase {
            money: money!(USD, 100.00),
            pcns: vec![],
            expected: None,
        },
        // percentages don't sum to 100
        AllocateCase {
            money: money!(USD, 100.00),
            pcns: vec![dec!(50), dec!(40)],
            expected: None,
        },
    ];

    for case in cases {
        let result = case.money.allocate(&case.pcns);
        assert_eq!(
            result, case.expected,
            "allocate({}, {:?})",
            case.money, case.pcns
        );
        if let Some(parts) = &result {
            let sum: Money<USD> = parts.iter().sum();
            assert_eq!(
                sum, case.money,
                "sum invariant failed for allocate({}, {:?})",
                case.money, case.pcns
            );
        }
    }
}

struct AllocateByRatiosCase {
    money: Money<USD>,
    ratios: Vec<i32>,
    expected: Option<Vec<Money<USD>>>,
}

#[test]
fn test_allocate_by_ratios() {
    let cases = vec![
        // even ratios
        AllocateByRatiosCase {
            money: money!(USD, 10.00),
            ratios: vec![1, 1],
            expected: Some(vec![money!(USD, 5.00), money!(USD, 5.00)]),
        },
        // classic 1:2:1
        AllocateByRatiosCase {
            money: money!(USD, 10.00),
            ratios: vec![1, 2, 1],
            expected: Some(vec![
                money!(USD, 2.50),
                money!(USD, 5.00),
                money!(USD, 2.50),
            ]),
        },
        // remainder distributed to first part
        AllocateByRatiosCase {
            money: money!(USD, 10.01),
            ratios: vec![1, 2, 1],
            expected: Some(vec![
                money!(USD, 2.51),
                money!(USD, 5.00),
                money!(USD, 2.50),
            ]),
        },
        // single ratio
        AllocateByRatiosCase {
            money: money!(USD, 10.00),
            ratios: vec![1],
            expected: Some(vec![money!(USD, 10.00)]),
        },
        // zero ratio in slice (that part gets nothing)
        AllocateByRatiosCase {
            money: money!(USD, 10.00),
            ratios: vec![1, 0, 1],
            expected: Some(vec![
                money!(USD, 5.00),
                money!(USD, 0.00),
                money!(USD, 5.00),
            ]),
        },
        // all zero ratios — invalid
        AllocateByRatiosCase {
            money: money!(USD, 10.00),
            ratios: vec![0, 0],
            expected: None,
        },
        // empty ratios — invalid
        AllocateByRatiosCase {
            money: money!(USD, 10.00),
            ratios: vec![],
            expected: None,
        },
        // zero money
        AllocateByRatiosCase {
            money: money!(USD, 0.00),
            ratios: vec![1, 2, 1],
            expected: Some(vec![
                money!(USD, 0.00),
                money!(USD, 0.00),
                money!(USD, 0.00),
            ]),
        },
        // indivisible penny
        AllocateByRatiosCase {
            money: money!(USD, 0.01),
            ratios: vec![1, 1],
            expected: Some(vec![money!(USD, 0.01), money!(USD, 0.00)]),
        },
    ];

    for case in cases {
        let result = case.money.allocate_by_ratios(&case.ratios);
        assert_eq!(
            result, case.expected,
            "allocate_by_ratios({}, {:?})",
            case.money, case.ratios
        );
        if let Some(parts) = &result {
            let sum: Money<USD> = parts.iter().sum();
            assert_eq!(
                sum, case.money,
                "sum invariant failed for allocate_by_ratios({}, {:?})",
                case.money, case.ratios
            );
        }
    }
}

// ===========================================================================
// Additional tests: negative money, zero money, big money, JPY, BHD,
// many percentage/ratio variants, and RawMoney
// ===========================================================================

// ==================== split: zero money ====================

#[test]
fn test_split_zero_money() {
    let money = money!(USD, 0.00);
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal, money!(USD, 0.00));
    assert_eq!(remainder, money!(USD, 0.00));

    // n=1 also works on zero
    let (equal1, rem1) = money.split(1).unwrap();
    assert_eq!(equal1, money!(USD, 0.00));
    assert_eq!(rem1, money!(USD, 0.00));

    // n=0 on zero money is still invalid
    assert!(money.split(0).is_none());
}

// ==================== split: negative money ====================

#[test]
fn test_split_negative_money() {
    // -10.00 / 3 = -3.33 remainder -0.01
    let money = money!(USD, -10.00);
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal, money!(USD, -3.33));
    assert_eq!(remainder, money!(USD, -0.01));
    let reconstructed = equal * Decimal::from_u32(3).unwrap() + remainder;
    assert_eq!(reconstructed, money);

    // -10.00 / 2 = -5.00 remainder 0.00
    let money2 = money!(USD, -10.00);
    let (equal2, remainder2) = money2.split(2).unwrap();
    assert_eq!(equal2, money!(USD, -5.00));
    assert_eq!(remainder2, money!(USD, 0.00));

    // smallest negative penny: -0.01 / 2 = 0.00 remainder -0.01
    let money3 = money!(USD, -0.01);
    let (equal3, remainder3) = money3.split(2).unwrap();
    assert_eq!(equal3, money!(USD, 0.00));
    assert_eq!(remainder3, money!(USD, -0.01));
    let reconstructed3 = equal3 * Decimal::from_u32(2).unwrap() + remainder3;
    assert_eq!(reconstructed3, money3);

    // n=0 on negative money is invalid
    assert!(money.split(0).is_none());
}

// ==================== split: big money ====================

#[test]
fn test_split_big_money() {
    // 1_000_000.00 / 3 = 333_333.33 remainder 0.01
    let money = money!(USD, 1_000_000.00);
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal, money!(USD, 333_333.33));
    assert_eq!(remainder, money!(USD, 0.01));
    let reconstructed = equal * Decimal::from_u32(3).unwrap() + remainder;
    assert_eq!(reconstructed, money);

    // 1_000_000.00 / 7 — verify invariant only
    let (equal7, rem7) = money.split(7).unwrap();
    let reconstructed7 = equal7 * Decimal::from_u32(7).unwrap() + rem7;
    assert_eq!(reconstructed7, money);

    // large amount, exact division: 1_000_000.00 / 4
    let (eq2, rem2) = money.split(4).unwrap();
    assert_eq!(eq2, money!(USD, 250_000.00));
    assert_eq!(rem2, money!(USD, 0.00));
}

// ==================== split: JPY (zero-decimal currency) ====================

#[test]
fn test_split_jpy_zero_decimal() {
    // 1000 JPY / 3 = 333 remainder 1
    let money = money!(JPY, 1000);
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal, money!(JPY, 333));
    assert_eq!(remainder, money!(JPY, 1));
    let reconstructed = equal * Decimal::from_u32(3).unwrap() + remainder;
    assert_eq!(reconstructed, money);

    // 10 JPY / 3 = 3 remainder 1
    let money2 = money!(JPY, 10);
    let (equal2, remainder2) = money2.split(3).unwrap();
    assert_eq!(equal2, money!(JPY, 3));
    assert_eq!(remainder2, money!(JPY, 1));

    // exact: 9 JPY / 3 = 3 remainder 0
    let money3 = money!(JPY, 9);
    let (equal3, remainder3) = money3.split(3).unwrap();
    assert_eq!(equal3, money!(JPY, 3));
    assert_eq!(remainder3, money!(JPY, 0));

    // zero JPY
    let money4 = money!(JPY, 0);
    let (equal4, remainder4) = money4.split(3).unwrap();
    assert_eq!(equal4, money!(JPY, 0));
    assert_eq!(remainder4, money!(JPY, 0));

    // negative JPY
    let money5 = money!(JPY, -1000);
    let (equal5, remainder5) = money5.split(3).unwrap();
    assert_eq!(equal5, money!(JPY, -333));
    assert_eq!(remainder5, money!(JPY, -1));
    let reconstructed5 = equal5 * Decimal::from_u32(3).unwrap() + remainder5;
    assert_eq!(reconstructed5, money5);
}

// ==================== split: BHD (three-decimal currency) ====================

#[test]
fn test_split_bhd_three_decimal() {
    // 10.000 BHD / 3 = 3.333 remainder 0.001
    let money = money!(BHD, 10.000);
    let (equal, remainder) = money.split(3).unwrap();
    assert_eq!(equal, money!(BHD, 3.333));
    assert_eq!(remainder, money!(BHD, 0.001));
    let reconstructed = equal * Decimal::from_u32(3).unwrap() + remainder;
    assert_eq!(reconstructed, money);

    // exact: 1.000 BHD / 4 = 0.250 remainder 0.000
    let money2 = money!(BHD, 1.000);
    let (equal2, remainder2) = money2.split(4).unwrap();
    assert_eq!(equal2, money!(BHD, 0.250));
    assert_eq!(remainder2, money!(BHD, 0.000));

    // negative BHD
    let money3 = money!(BHD, -10.000);
    let (equal3, remainder3) = money3.split(3).unwrap();
    assert_eq!(equal3, money!(BHD, -3.333));
    assert_eq!(remainder3, money!(BHD, -0.001));
    let reconstructed3 = equal3 * Decimal::from_u32(3).unwrap() + remainder3;
    assert_eq!(reconstructed3, money3);
}

// ==================== split: math invariant across many cases ====================

#[test]
fn test_split_math_invariant() {
    let amounts: &[Money<USD>] = &[
        money!(USD, 0.00),
        money!(USD, 0.01),
        money!(USD, 1.00),
        money!(USD, 100.00),
        money!(USD, 1_000_000.00),
        money!(USD, -0.01),
        money!(USD, -100.00),
        money!(USD, -1_000_000.00),
    ];
    let ns: &[u32] = &[1, 2, 3, 7, 100];
    for amount in amounts {
        for &n in ns {
            if let Some((equal, remainder)) = amount.split(n) {
                let total = equal * Decimal::from_u32(n).unwrap() + remainder;
                assert_eq!(
                    total, *amount,
                    "split invariant failed: split({}, {})",
                    amount, n
                );
            }
        }
    }
}

// ==================== split_dist: zero money ====================

#[test]
fn test_split_dist_zero_money_extra() {
    let money = money!(USD, 0.00);
    let parts = money.split_dist(1).unwrap();
    assert_eq!(parts, vec![money!(USD, 0.00)]);
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);
}

// ==================== split_dist: negative money ====================

#[test]
fn test_split_dist_negative_money() {
    // -10.00 / 3 with distribution -> [-3.34, -3.33, -3.33]
    let money = money!(USD, -10.00);
    let parts = money.split_dist(3).unwrap();
    assert_eq!(
        parts,
        vec![money!(USD, -3.34), money!(USD, -3.33), money!(USD, -3.33)]
    );
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // -10.01 / 3 -> [-3.34, -3.34, -3.33]
    let money2 = money!(USD, -10.01);
    let parts2 = money2.split_dist(3).unwrap();
    assert_eq!(
        parts2,
        vec![money!(USD, -3.34), money!(USD, -3.34), money!(USD, -3.33)]
    );
    let sum2: Money<USD> = parts2.iter().sum();
    assert_eq!(sum2, money2);

    // n=1 on negative: whole amount in one part
    let money3 = money!(USD, -99.99);
    let parts3 = money3.split_dist(1).unwrap();
    assert_eq!(parts3, vec![money!(USD, -99.99)]);

    // n=0 on negative -> None
    assert!(money.split_dist(0).is_none());
}

// ==================== split_dist: big money ====================

#[test]
fn test_split_dist_big_money() {
    let money = money!(USD, 1_000_000.00);

    // 1_000_000 / 3 with distribution
    let parts = money.split_dist(3).unwrap();
    assert_eq!(parts.len(), 3);
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // 1_000_000 / 7
    let parts7 = money.split_dist(7).unwrap();
    assert_eq!(parts7.len(), 7);
    let sum7: Money<USD> = parts7.iter().sum();
    assert_eq!(sum7, money);
}

// ==================== split_dist: JPY ====================

#[test]
fn test_split_dist_jpy() {
    // 10 JPY / 3 -> [4, 3, 3]
    let money = money!(JPY, 10);
    let parts = money.split_dist(3).unwrap();
    assert_eq!(parts, vec![money!(JPY, 4), money!(JPY, 3), money!(JPY, 3)]);
    let sum: Money<JPY> = parts.iter().sum();
    assert_eq!(sum, money);

    // 1000 JPY / 3 -> [334, 333, 333]
    let money2 = money!(JPY, 1000);
    let parts2 = money2.split_dist(3).unwrap();
    assert_eq!(
        parts2,
        vec![money!(JPY, 334), money!(JPY, 333), money!(JPY, 333)]
    );
    let sum2: Money<JPY> = parts2.iter().sum();
    assert_eq!(sum2, money2);

    // negative JPY: -10 / 3 -> [-4, -3, -3]
    let money3 = money!(JPY, -10);
    let parts3 = money3.split_dist(3).unwrap();
    assert_eq!(
        parts3,
        vec![money!(JPY, -4), money!(JPY, -3), money!(JPY, -3)]
    );
    let sum3: Money<JPY> = parts3.iter().sum();
    assert_eq!(sum3, money3);
}

// ==================== split_dist: math invariant ====================

#[test]
fn test_split_dist_math_invariant() {
    let amounts: &[Money<USD>] = &[
        money!(USD, 0.00),
        money!(USD, 0.01),
        money!(USD, 100.00),
        money!(USD, 1_000_000.00),
        money!(USD, -0.01),
        money!(USD, -100.00),
    ];
    let ns: &[u32] = &[1, 2, 3, 7];
    for amount in amounts {
        for &n in ns {
            if let Some(parts) = amount.split_dist(n) {
                assert_eq!(parts.len(), n as usize);
                let sum: Money<USD> = parts.iter().sum();
                assert_eq!(
                    sum, *amount,
                    "split_dist invariant failed: split_dist({}, {})",
                    amount, n
                );
            }
        }
    }
}

// ==================== allocate: zero money ====================

#[test]
fn test_allocate_zero_money_extra() {
    let money = money!(USD, 0.00);
    let parts = money
        .allocate(&[dec!(33.33), dec!(33.33), dec!(33.34)])
        .unwrap();
    assert_eq!(
        parts,
        vec![money!(USD, 0.00), money!(USD, 0.00), money!(USD, 0.00)]
    );
}

// ==================== allocate: negative money ====================

#[test]
fn test_allocate_negative_money() {
    // -100 by 50/50 -> [-50, -50]
    let money = money!(USD, -100.00);
    let parts = money.allocate(&[dec!(50), dec!(50)]).unwrap();
    assert_eq!(parts, vec![money!(USD, -50.00), money!(USD, -50.00)]);
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // -100 by 70/30 -> [-70, -30]
    let parts2 = money.allocate(&[dec!(70), dec!(30)]).unwrap();
    assert_eq!(parts2, vec![money!(USD, -70.00), money!(USD, -30.00)]);
    let sum2: Money<USD> = parts2.iter().sum();
    assert_eq!(sum2, money);

    // -0.01 by 50/50 — invariant check
    let small = money!(USD, -0.01);
    let parts3 = small.allocate(&[dec!(50), dec!(50)]).unwrap();
    let sum3: Money<USD> = parts3.iter().sum();
    assert_eq!(sum3, small);

    // percentages not summing to 100 -> None even for negative money
    assert!(money.allocate(&[dec!(50), dec!(40)]).is_none());
}

// ==================== allocate: big money ====================

#[test]
fn test_allocate_big_money() {
    let money = money!(USD, 1_000_000.00);

    // 70/30 split
    let parts = money.allocate(&[dec!(70), dec!(30)]).unwrap();
    assert_eq!(
        parts,
        vec![money!(USD, 700_000.00), money!(USD, 300_000.00)]
    );
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // 10/20/30/40
    let parts2 = money
        .allocate(&[dec!(10), dec!(20), dec!(30), dec!(40)])
        .unwrap();
    assert_eq!(
        parts2,
        vec![
            money!(USD, 100_000.00),
            money!(USD, 200_000.00),
            money!(USD, 300_000.00),
            money!(USD, 400_000.00),
        ]
    );
    let sum2: Money<USD> = parts2.iter().sum();
    assert_eq!(sum2, money);
}

// ==================== allocate: many percentage combinations ====================

#[test]
fn test_allocate_many_percentages() {
    let money = money!(USD, 100.00);

    // 70/30
    let parts = money.allocate(&[dec!(70), dec!(30)]).unwrap();
    assert_eq!(parts, vec![money!(USD, 70.00), money!(USD, 30.00)]);

    // 1/99
    let parts2 = money.allocate(&[dec!(1), dec!(99)]).unwrap();
    assert_eq!(parts2, vec![money!(USD, 1.00), money!(USD, 99.00)]);

    // 25/25/25/25
    let parts3 = money
        .allocate(&[dec!(25), dec!(25), dec!(25), dec!(25)])
        .unwrap();
    assert_eq!(
        parts3,
        vec![
            money!(USD, 25.00),
            money!(USD, 25.00),
            money!(USD, 25.00),
            money!(USD, 25.00),
        ]
    );

    // 10/20/30/40
    let parts4 = money
        .allocate(&[dec!(10), dec!(20), dec!(30), dec!(40)])
        .unwrap();
    assert_eq!(
        parts4,
        vec![
            money!(USD, 10.00),
            money!(USD, 20.00),
            money!(USD, 30.00),
            money!(USD, 40.00),
        ]
    );

    // decimal percentages with remainder distribution
    let parts5 = money
        .allocate(&[dec!(33.33), dec!(33.33), dec!(33.34)])
        .unwrap();
    assert_eq!(
        parts5,
        vec![money!(USD, 33.33), money!(USD, 33.33), money!(USD, 33.34)]
    );

    // single 100% slice
    let parts6 = money.allocate(&[dec!(100)]).unwrap();
    assert_eq!(parts6, vec![money!(USD, 100.00)]);

    // all variants must satisfy sum invariant
    for p in [&parts, &parts2, &parts3, &parts4, &parts5, &parts6] {
        let sum: Money<USD> = p.iter().sum();
        assert_eq!(sum, money);
    }
}

// ==================== allocate: JPY ====================

#[test]
fn test_allocate_jpy() {
    // 100 JPY by 70/30 -> [70, 30]
    let money = money!(JPY, 100);
    let parts = money.allocate(&[dec!(70), dec!(30)]).unwrap();
    assert_eq!(parts, vec![money!(JPY, 70), money!(JPY, 30)]);
    let sum: Money<JPY> = parts.iter().sum();
    assert_eq!(sum, money);

    // 10 JPY by 33/33/34 — remainder distribution
    let money2 = money!(JPY, 10);
    let parts2 = money2.allocate(&[dec!(33), dec!(33), dec!(34)]).unwrap();
    let sum2: Money<JPY> = parts2.iter().sum();
    assert_eq!(sum2, money2);

    // negative: -100 JPY by 50/50 -> [-50, -50]
    let money3 = money!(JPY, -100);
    let parts3 = money3.allocate(&[dec!(50), dec!(50)]).unwrap();
    assert_eq!(parts3, vec![money!(JPY, -50), money!(JPY, -50)]);
    let sum3: Money<JPY> = parts3.iter().sum();
    assert_eq!(sum3, money3);
}

// ==================== allocate: BHD ====================

#[test]
fn test_allocate_bhd() {
    // 1.000 BHD by 50/50 -> [0.500, 0.500]
    let money = money!(BHD, 1.000);
    let parts = money.allocate(&[dec!(50), dec!(50)]).unwrap();
    assert_eq!(parts, vec![money!(BHD, 0.500), money!(BHD, 0.500)]);
    let sum: Money<BHD> = parts.iter().sum();
    assert_eq!(sum, money);

    // 1.000 BHD by 33.33/33.33/33.34 — invariant
    let parts2 = money
        .allocate(&[dec!(33.33), dec!(33.33), dec!(33.34)])
        .unwrap();
    let sum2: Money<BHD> = parts2.iter().sum();
    assert_eq!(sum2, money);
}

// ==================== allocate: math invariant ====================

#[test]
fn test_allocate_math_invariant() {
    let amounts: &[Money<USD>] = &[
        money!(USD, 0.00),
        money!(USD, 0.01),
        money!(USD, 100.00),
        money!(USD, 1_000_000.00),
        money!(USD, -100.00),
        money!(USD, -0.01),
    ];
    let pcn_slices: &[&[Decimal]] = &[
        &[dec!(50), dec!(50)],
        &[dec!(70), dec!(30)],
        &[dec!(10), dec!(20), dec!(30), dec!(40)],
        &[dec!(33.33), dec!(33.33), dec!(33.34)],
        &[dec!(1), dec!(99)],
        &[dec!(100)],
    ];
    for amount in amounts {
        for pcns in pcn_slices {
            if let Some(parts) = amount.allocate(*pcns) {
                let sum: Money<USD> = parts.iter().sum();
                assert_eq!(
                    sum, *amount,
                    "allocate invariant failed: allocate({}, {:?})",
                    amount, pcns
                );
            }
        }
    }
}

// ==================== allocate_by_ratios: zero money ====================

#[test]
fn test_allocate_by_ratios_zero_money_extra() {
    let money = money!(USD, 0.00);
    let parts = money.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(
        parts,
        vec![money!(USD, 0.00), money!(USD, 0.00), money!(USD, 0.00)]
    );
}

// ==================== allocate_by_ratios: negative money ====================

#[test]
fn test_allocate_by_ratios_negative_money() {
    // -100.00 by 1:1 -> [-50.00, -50.00]
    let money = money!(USD, -100.00);
    let parts = money.allocate_by_ratios(&[1, 1]).unwrap();
    assert_eq!(parts, vec![money!(USD, -50.00), money!(USD, -50.00)]);
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // -100.00 by 1:2:1 -> [-25.00, -50.00, -25.00]
    let parts2 = money.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(
        parts2,
        vec![
            money!(USD, -25.00),
            money!(USD, -50.00),
            money!(USD, -25.00),
        ]
    );
    let sum2: Money<USD> = parts2.iter().sum();
    assert_eq!(sum2, money);

    // -10.01 by 1:2:1 — invariant check
    let money3 = money!(USD, -10.01);
    let parts3 = money3.allocate_by_ratios(&[1, 2, 1]).unwrap();
    let sum3: Money<USD> = parts3.iter().sum();
    assert_eq!(sum3, money3);

    // all-zero ratios still invalid
    assert!(money.allocate_by_ratios(&[0, 0]).is_none());
}

// ==================== allocate_by_ratios: big money ====================

#[test]
fn test_allocate_by_ratios_big_money() {
    let money = money!(USD, 1_000_000.00);

    // 1:1 -> [500_000, 500_000]
    let parts = money.allocate_by_ratios(&[1, 1]).unwrap();
    assert_eq!(
        parts,
        vec![money!(USD, 500_000.00), money!(USD, 500_000.00)]
    );
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // 1:2:1 -> [250_000, 500_000, 250_000]
    let parts2 = money.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(
        parts2,
        vec![
            money!(USD, 250_000.00),
            money!(USD, 500_000.00),
            money!(USD, 250_000.00),
        ]
    );
    let sum2: Money<USD> = parts2.iter().sum();
    assert_eq!(sum2, money);

    // 7-way — invariant only
    let parts7 = money.allocate_by_ratios(&[1, 1, 1, 1, 1, 1, 1]).unwrap();
    assert_eq!(parts7.len(), 7);
    let sum7: Money<USD> = parts7.iter().sum();
    assert_eq!(sum7, money);
}

// ==================== allocate_by_ratios: many ratio combinations ====================

#[test]
fn test_allocate_by_ratios_many_ratios() {
    let money = money!(USD, 10.00);

    // 3:7
    let parts = money.allocate_by_ratios(&[3, 7]).unwrap();
    assert_eq!(parts, vec![money!(USD, 3.00), money!(USD, 7.00)]);

    // 1:9
    let parts2 = money.allocate_by_ratios(&[1, 9]).unwrap();
    assert_eq!(parts2, vec![money!(USD, 1.00), money!(USD, 9.00)]);

    // 100:200:300 (proportional to 1:2:3)
    let parts3 = money.allocate_by_ratios(&[100, 200, 300]).unwrap();
    let sum3: Money<USD> = parts3.iter().sum();
    assert_eq!(sum3, money);

    // 1:3
    let parts4 = money.allocate_by_ratios(&[1, 3]).unwrap();
    assert_eq!(parts4, vec![money!(USD, 2.50), money!(USD, 7.50)]);

    for p in [&parts, &parts2, &parts3, &parts4] {
        let sum: Money<USD> = p.iter().sum();
        assert_eq!(sum, money);
    }

    // 0.10 by 1:2:3:4
    let small = money!(USD, 0.10);
    let parts5 = small.allocate_by_ratios(&[1, 2, 3, 4]).unwrap();
    assert_eq!(
        parts5,
        vec![
            money!(USD, 0.01),
            money!(USD, 0.02),
            money!(USD, 0.03),
            money!(USD, 0.04),
        ]
    );
    let sum5: Money<USD> = parts5.iter().sum();
    assert_eq!(sum5, small);
}

// ==================== allocate_by_ratios: decimal ratios ====================

#[test]
fn test_allocate_by_ratios_decimal_ratios() {
    // 1.5 : 2.5 : 1.0  (total = 5.0)
    let money = money!(USD, 10.00);
    let parts = money
        .allocate_by_ratios(&[dec!(1.5), dec!(2.5), dec!(1.0)])
        .unwrap();
    assert_eq!(
        parts,
        vec![money!(USD, 3.00), money!(USD, 5.00), money!(USD, 2.00)]
    );
    let sum: Money<USD> = parts.iter().sum();
    assert_eq!(sum, money);

    // 0.1 : 0.9 (same as 1:9)
    let parts2 = money.allocate_by_ratios(&[dec!(0.1), dec!(0.9)]).unwrap();
    assert_eq!(parts2, vec![money!(USD, 1.00), money!(USD, 9.00)]);
    let sum2: Money<USD> = parts2.iter().sum();
    assert_eq!(sum2, money);
}

// ==================== allocate_by_ratios: JPY ====================

#[test]
fn test_allocate_by_ratios_jpy() {
    // 100 JPY by 1:2:1 -> [25, 50, 25]
    let money = money!(JPY, 100);
    let parts = money.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(
        parts,
        vec![money!(JPY, 25), money!(JPY, 50), money!(JPY, 25)]
    );
    let sum: Money<JPY> = parts.iter().sum();
    assert_eq!(sum, money);

    // 10 JPY by 1:2:1 — remainder distribution, invariant check
    let money2 = money!(JPY, 10);
    let parts2 = money2.allocate_by_ratios(&[1, 2, 1]).unwrap();
    let sum2: Money<JPY> = parts2.iter().sum();
    assert_eq!(sum2, money2);

    // negative: -100 JPY by 1:1 -> [-50, -50]
    let money3 = money!(JPY, -100);
    let parts3 = money3.allocate_by_ratios(&[1, 1]).unwrap();
    assert_eq!(parts3, vec![money!(JPY, -50), money!(JPY, -50)]);
    let sum3: Money<JPY> = parts3.iter().sum();
    assert_eq!(sum3, money3);
}

// ==================== allocate_by_ratios: math invariant ====================

#[test]
fn test_allocate_by_ratios_math_invariant() {
    let amounts: &[Money<USD>] = &[
        money!(USD, 0.00),
        money!(USD, 0.01),
        money!(USD, 100.00),
        money!(USD, 1_000_000.00),
        money!(USD, -100.00),
        money!(USD, -0.01),
    ];
    let ratio_slices: &[&[i32]] = &[
        &[1, 1],
        &[1, 2, 1],
        &[3, 7],
        &[1, 2, 3, 4],
        &[100, 200, 300],
    ];
    for amount in amounts {
        for ratios in ratio_slices {
            if let Some(parts) = amount.allocate_by_ratios(*ratios) {
                let sum: Money<USD> = parts.iter().sum();
                assert_eq!(
                    sum, *amount,
                    "allocate_by_ratios invariant failed: ({}, {:?})",
                    amount, ratios
                );
            }
        }
    }
}

// ===========================================================================
// RawMoney additional tests
// ===========================================================================

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_split_negative() {
    // -100 / 3: parts are negative, invariant holds
    let money = raw!(USD, -100);
    let (equal, remainder) = money.split(3).unwrap();
    assert!(equal.is_negative());
    let n = Decimal::from_u32(3).unwrap();
    let reconstructed = equal * n + remainder;
    assert_eq!(reconstructed.amount(), money.amount());

    // -10.005 / 3 = -3.335 exactly (full RawMoney precision)
    let money2 = raw!(USD, -10.005);
    let (equal2, remainder2) = money2.split(3).unwrap();
    assert_eq!(equal2.amount(), dec!(-3.335));
    assert!(remainder2.is_zero());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_split_big_money() {
    // 1_000_000 / 7 — invariant
    let money = raw!(USD, 1_000_000);
    let (equal, remainder) = money.split(7).unwrap();
    let n = Decimal::from_u32(7).unwrap();
    let reconstructed = equal * n + remainder;
    assert_eq!(reconstructed.amount(), money.amount());

    // exact: 1_000_000 / 4 = 250_000
    let (equal2, rem2) = money.split(4).unwrap();
    assert_eq!(equal2.amount(), dec!(250000));
    assert!(rem2.is_zero());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_split_zero_money() {
    let money = raw!(USD, 0);
    let (equal, remainder) = money.split(3).unwrap();
    assert!(equal.is_zero());
    assert!(remainder.is_zero());

    assert!(money.split(0).is_none());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_split_dist_negative() {
    let money = raw!(USD, -10);
    let parts = money.split_dist(3).unwrap();
    assert_eq!(parts.len(), 3);
    for p in &parts {
        assert!(p.is_negative() || p.is_zero());
    }
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_split_dist_big_money() {
    // Use a value that divides evenly so full-precision RawMoney sums back exactly
    let money = raw!(USD, 1_000_000);
    let parts = money.split_dist(4).unwrap();
    assert_eq!(parts.len(), 4);
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_allocate_negative() {
    let money = raw!(USD, -100);
    let parts = money.allocate(&[dec!(50), dec!(50)]).unwrap();
    assert_eq!(parts.len(), 2);
    for p in &parts {
        assert!(p.is_negative() || p.is_zero());
    }
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());

    // percentages not summing to 100 -> None even for negative
    assert!(money.allocate(&[dec!(50), dec!(40)]).is_none());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_allocate_big_money() {
    let money = raw!(USD, 1_000_000);
    let parts = money.allocate(&[dec!(70), dec!(30)]).unwrap();
    assert_eq!(parts[0].amount(), dec!(700000));
    assert_eq!(parts[1].amount(), dec!(300000));
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_allocate_by_ratios_negative() {
    let money = raw!(USD, -100);
    let parts = money.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(parts.len(), 3);
    for p in &parts {
        assert!(p.is_negative() || p.is_zero());
    }
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_allocate_by_ratios_big_money() {
    let money = raw!(USD, 1_000_000);
    let parts = money.allocate_by_ratios(&[1, 2, 1]).unwrap();
    assert_eq!(parts[0].amount(), dec!(250000));
    assert_eq!(parts[1].amount(), dec!(500000));
    assert_eq!(parts[2].amount(), dec!(250000));
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_full_precision_allocate_by_ratios() {
    // With RawMoney, irrational-looking splits get full decimal precision
    let money = raw!(USD, 10.005);
    let parts = money.allocate_by_ratios(&[1, 1, 1]).unwrap();
    assert_eq!(parts.len(), 3);
    let sum: RawMoney<USD> = parts.iter().sum();
    assert_eq!(sum.amount(), money.amount());
}

#[cfg(feature = "raw_money")]
#[test]
fn test_raw_allocate_math_invariant() {
    let amounts: &[RawMoney<USD>] = &[
        raw!(USD, 0),
        raw!(USD, 0.001),
        raw!(USD, 100),
        raw!(USD, 1_000_000),
        raw!(USD, -100),
        raw!(USD, -0.001),
    ];
    let pcn_slices: &[&[Decimal]] = &[
        &[dec!(50), dec!(50)],
        &[dec!(70), dec!(30)],
        &[dec!(33.33), dec!(33.33), dec!(33.34)],
        &[dec!(100)],
    ];
    for amount in amounts {
        for pcns in pcn_slices {
            if let Some(parts) = amount.allocate(*pcns) {
                let sum: RawMoney<USD> = parts.iter().sum();
                assert_eq!(
                    sum.amount(),
                    amount.amount(),
                    "raw allocate invariant failed: ({}, {:?})",
                    amount,
                    pcns
                );
            }
        }
    }
}
