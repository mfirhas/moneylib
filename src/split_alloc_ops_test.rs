use rust_decimal::prelude::FromPrimitive;

use crate::dec;
use crate::{BaseOps, Decimal};
use crate::{Money, iso::USD, money};

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
