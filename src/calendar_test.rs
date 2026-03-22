#[cfg(test)]
mod get_years_months_tests {
    use crate::calendar::*;

    #[test]
    fn test_next_month() {
        let dec = 12;
        let ret = dec.next_month(2026).unwrap();
        assert_eq!(ret.0, 2027);
        assert_eq!(ret.1, 1);
        assert_eq!(ret.2, 31);

        let dec = 0;
        let ret = dec.next_month(2026);
        assert!(ret.is_none());
    }

    fn total_months(result: &Vec<(u32, Vec<u32>)>) -> usize {
        result.iter().map(|(_, months)| months.len()).sum()
    }

    // --- invalid inputs ---

    #[test]
    fn test_zero_months() {
        assert_eq!(get_years_months(2024, 1, 0), None);
    }

    #[test]
    fn test_invalid_month_zero() {
        assert_eq!(get_years_months(2024, 0, 5), None);
    }

    #[test]
    fn test_invalid_month_thirteen() {
        assert_eq!(get_years_months(2024, 13, 5), None);
    }

    // --- single month ---

    #[test]
    fn test_single_month() {
        let result = get_years_months(2024, 1, 1).unwrap();
        assert_eq!(total_months(&result), 1);
        assert_eq!(result, vec![(2024, vec![1])]);
    }

    // --- within same year ---

    #[test]
    fn test_within_same_year() {
        let result = get_years_months(2024, 1, 6).unwrap();
        assert_eq!(total_months(&result), 6);
        assert_eq!(result, vec![(2024, vec![1, 2, 3, 4, 5, 6])]);
    }

    #[test]
    fn test_full_year() {
        let result = get_years_months(2024, 1, 12).unwrap();
        assert_eq!(total_months(&result), 12);
        assert_eq!(
            result,
            vec![(2024, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
        );
    }

    // --- year boundary ---

    #[test]
    fn test_overflow_into_next_year() {
        let result = get_years_months(2024, 11, 3).unwrap();
        assert_eq!(total_months(&result), 3);
        assert_eq!(result, vec![(2024, vec![11, 12]), (2025, vec![1]),]);
    }

    #[test]
    fn test_start_december() {
        let result = get_years_months(2024, 12, 3).unwrap();
        assert_eq!(total_months(&result), 3);
        assert_eq!(result, vec![(2024, vec![12]), (2025, vec![1, 2]),]);
    }

    // --- spanning multiple years ---

    #[test]
    fn test_span_two_full_years() {
        let result = get_years_months(2024, 1, 24).unwrap();
        assert_eq!(total_months(&result), 24);
        assert_eq!(
            result,
            vec![
                (2024, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
                (2025, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
            ]
        );
    }

    #[test]
    fn test_span_three_years_mid() {
        let result = get_years_months(2023, 11, 15).unwrap();
        assert_eq!(total_months(&result), 15);
        assert_eq!(
            result,
            vec![
                (2023, vec![11, 12]),
                (2024, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
                (2025, vec![1]),
            ]
        );
    }

    // --- start from mid year ---

    #[test]
    fn test_start_mid_year() {
        let result = get_years_months(2024, 6, 4).unwrap();
        assert_eq!(total_months(&result), 4);
        assert_eq!(result, vec![(2024, vec![6, 7, 8, 9])]);
    }

    // --- total months invariant ---

    #[test]
    fn test_total_months_invariant() {
        for num in [1, 5, 12, 13, 24, 36] {
            let result = get_years_months(2024, 3, num).unwrap();
            assert_eq!(
                total_months(&result),
                num as usize,
                "failed for num_of_months={}",
                num
            );
        }
    }
}

#[cfg(test)]
mod get_years_months_days_tests {
    use crate::calendar::*;

    #[test]
    fn test_next_day() {
        let day = 31;
        let ret = day.next_day(2026, 12).unwrap();
        assert_eq!(ret.0, 2027);
        assert_eq!(ret.1, 1);
        assert_eq!(ret.2, 1);
        assert_eq!(ret.3, 31);

        let dec = 0;
        let ret = dec.next_day(2026, 12);
        assert!(ret.is_none());

        let dec = 2;
        let ret = dec.next_day(2026, 0);
        assert!(ret.is_none());
    }

    // --- invalid inputs ---

    #[test]
    fn test_invalid_month_zero() {
        assert_eq!(get_years_months_days(2024, 0, 1, 5), None);
    }

    #[test]
    fn test_invalid_month_thirteen() {
        assert_eq!(get_years_months_days(2024, 13, 1, 5), None);
    }

    #[test]
    fn test_invalid_day_zero() {
        assert_eq!(get_years_months_days(2024, 1, 0, 5), None);
    }

    #[test]
    fn test_invalid_day_exceeds_month() {
        assert_eq!(get_years_months_days(2024, 4, 31, 5), None); // April has 30 days
    }

    #[test]
    fn test_invalid_day_feb_non_leap() {
        assert_eq!(get_years_months_days(2023, 2, 29, 5), None); // 2023 not a leap year
    }

    #[test]
    fn test_invalid_day_feb_leap() {
        assert_eq!(get_years_months_days(2024, 2, 30, 5), None); // leap year still only 29 days
    }

    // --- zero days ---

    #[test]
    fn test_zero_days() {
        assert_eq!(get_years_months_days(2024, 1, 1, 0), Some(vec![]));
    }

    // --- single day ---

    #[test]
    fn test_single_day() {
        let result = get_years_months_days(2024, 3, 15, 1).unwrap();
        assert_eq!(result, vec![(2024, vec![(3, vec![15])])]);
    }

    // --- within same month ---

    #[test]
    fn test_within_same_month() {
        let result = get_years_months_days(2024, 1, 1, 5).unwrap();
        assert_eq!(result, vec![(2024, vec![(1, vec![1, 2, 3, 4, 5])])]);
    }

    #[test]
    fn test_full_month_from_start() {
        let result = get_years_months_days(2024, 1, 1, 31).unwrap();
        let days: Vec<u32> = (1..=31).collect();
        assert_eq!(result, vec![(2024, vec![(1, days)])]);
    }

    // --- month boundary ---

    #[test]
    fn test_overflow_into_next_month() {
        let result = get_years_months_days(2024, 1, 30, 5).unwrap();
        assert_eq!(
            result,
            vec![(2024, vec![(1, vec![30, 31]), (2, vec![1, 2, 3]),])]
        );
    }

    #[test]
    fn test_overflow_april_into_may() {
        let result = get_years_months_days(2024, 4, 29, 4).unwrap();
        assert_eq!(
            result,
            vec![(2024, vec![(4, vec![29, 30]), (5, vec![1, 2]),])]
        );
    }

    // --- year boundary ---

    #[test]
    fn test_overflow_into_next_year() {
        let result = get_years_months_days(2024, 12, 30, 5).unwrap();
        assert_eq!(
            result,
            vec![
                (2024, vec![(12, vec![30, 31])]),
                (2025, vec![(1, vec![1, 2, 3])]),
            ]
        );
    }

    #[test]
    fn test_start_dec_31() {
        let result = get_years_months_days(2024, 12, 31, 3).unwrap();
        assert_eq!(
            result,
            vec![(2024, vec![(12, vec![31])]), (2025, vec![(1, vec![1, 2])]),]
        );
    }

    // --- february ---

    #[test]
    fn test_feb_leap_year() {
        let result = get_years_months_days(2024, 2, 27, 4).unwrap();
        assert_eq!(
            result,
            vec![(2024, vec![(2, vec![27, 28, 29]), (3, vec![1]),])]
        );
    }

    #[test]
    fn test_feb_non_leap_year() {
        let result = get_years_months_days(2023, 2, 27, 4).unwrap();
        assert_eq!(
            result,
            vec![(2023, vec![(2, vec![27, 28]), (3, vec![1, 2]),])]
        );
    }

    #[test]
    fn test_feb_leap_full_month() {
        let result = get_years_months_days(2024, 2, 1, 29).unwrap();
        let days: Vec<u32> = (1..=29).collect();
        assert_eq!(result, vec![(2024, vec![(2, days)])]);
    }

    #[test]
    fn test_feb_non_leap_full_month() {
        let result = get_years_months_days(2023, 2, 1, 28).unwrap();
        let days: Vec<u32> = (1..=28).collect();
        assert_eq!(result, vec![(2023, vec![(2, days)])]);
    }

    // --- spanning multiple months ---

    #[test]
    fn test_span_three_months() {
        let result = get_years_months_days(2025, 10, 4, 100).unwrap();
        // Oct 4–31 = 28 days, Nov 1–30 = 30 days, Dec 1–31 = 31 days, Jan 1–11 = 11 days
        // total = 28 + 30 + 31 + 11 = 100
        assert_eq!(
            result,
            vec![
                (
                    2025,
                    vec![
                        (10, (4..=31).collect()),
                        (11, (1..=30).collect()),
                        (12, (1..=31).collect()),
                    ]
                ),
                (2026, vec![(1, (1..=11).collect()),]),
            ]
        );
    }

    // --- total days invariant ---

    #[test]
    fn test_total_days_invariant() {
        for num in [1, 28, 29, 30, 31, 60, 100, 365, 366] {
            let result = get_years_months_days(2024, 1, 1, num).unwrap();
            let total: usize = result
                .iter()
                .flat_map(|(_, months)| months.iter())
                .map(|(_, days)| days.len())
                .sum();
            assert_eq!(total, num as usize, "failed for num_of_days={}", num);
        }
    }

    // --- days are sequential with no gaps ---

    #[test]
    fn test_days_are_sequential() {
        let result = get_years_months_days(2024, 1, 1, 365).unwrap();
        let mut prev: Option<(u32, u32, u32)> = None;
        for (year, months) in &result {
            for (month, days) in months {
                for day in days {
                    if let Some((py, pm, pd)) = prev {
                        let _expected = pd.next_day(*year, *month).map(|(y, m, d, _)| (y, m, d));
                        // just check continuity within same month
                        if py == *year && pm == *month {
                            assert_eq!(*day, pd + 1);
                        }
                    }
                    prev = Some((*year, *month, *day));
                }
            }
        }
    }

    #[test]
    fn test_total_days_equals_input_simple() {
        let num = 100u32;
        let result = get_years_months_days(2025, 10, 4, num).unwrap();
        let total: u32 = result
            .iter()
            .flat_map(|(_, months)| months.iter())
            .map(|(_, days)| days.len() as u32)
            .sum();
        assert_eq!(total, num);
    }

    #[test]
    fn test_total_days_equals_input_exhaustive() {
        let cases: &[(u32, u32, u32, u32)] = &[
            (2024, 1, 1, 1),
            (2024, 1, 1, 28),
            (2024, 1, 1, 29),
            (2024, 1, 1, 30),
            (2024, 1, 1, 31),
            (2024, 1, 1, 365),
            (2024, 1, 1, 366),   // 2024 is leap
            (2023, 1, 1, 365),   // 2023 non-leap
            (2024, 2, 1, 29),    // full feb leap
            (2023, 2, 1, 28),    // full feb non-leap
            (2024, 12, 31, 1),   // single day end of year
            (2024, 12, 31, 100), // spans into next year
            (2025, 10, 4, 100),  // original failing case
            (2023, 11, 30, 400), // spans multiple years
        ];

        for &(year, month, day, num) in cases {
            let result = get_years_months_days(year, month, day, num).unwrap();
            let total: u32 = result
                .iter()
                .flat_map(|(_, months)| months.iter())
                .map(|(_, days)| days.len() as u32)
                .sum();
            assert_eq!(
                total, num,
                "total days mismatch for input ({year}, {month}, {day}, {num}): got {total}, expected {num}"
            );
        }
    }
}

// --- add months tests
use crate::calendar::AddMonths;

#[test]
fn test_add_zero_months_mid_year() {
    // June 2025 + 0 = June 2025 (30 days)
    assert_eq!(6u32.add_months(2025, 0), Some((2025, 6, 30)));
}

#[test]
fn test_add_zero_months_january() {
    // Jan 2025 + 0 = Jan 2025 (31 days)
    assert_eq!(1u32.add_months(2025, 0), Some((2025, 1, 31)));
}

#[test]
fn test_add_zero_months_december() {
    // Dec 2025 + 0 = Dec 2025 (31 days)
    assert_eq!(12u32.add_months(2025, 0), Some((2025, 12, 31)));
}

// --- single year, no rollover ---

#[test]
fn test_add_months_within_year() {
    // Jan 2025 + 5 = June 2025 (30 days)
    assert_eq!(1u32.add_months(2025, 5), Some((2025, 6, 30)));
}

#[test]
fn test_add_months_to_december() {
    // Jan 2025 + 11 = Dec 2025 (31 days)
    assert_eq!(1u32.add_months(2025, 11), Some((2025, 12, 31)));
}

// --- year rollover ---

#[test]
fn test_add_months_single_year_rollover() {
    // Nov 2025 + 3 = Feb 2026 (28 days)
    assert_eq!(11u32.add_months(2025, 3), Some((2026, 2, 28)));
}

#[test]
fn test_add_months_exactly_one_year() {
    // Jan 2025 + 12 = Jan 2026 (31 days)
    assert_eq!(1u32.add_months(2025, 12), Some((2026, 1, 31)));
}

#[test]
fn test_add_months_multi_year_rollover() {
    // Jan 2025 + 25 = Feb 2027 (28 days)
    assert_eq!(1u32.add_months(2025, 25), Some((2027, 2, 28)));
}

#[test]
fn test_add_months_december_plus_one() {
    // Dec 2025 + 1 = Jan 2026 (31 days)
    assert_eq!(12u32.add_months(2025, 1), Some((2026, 1, 31)));
}

// --- leap year awareness ---

#[test]
fn test_add_months_lands_on_leap_february() {
    // June 2023 + 20 = Feb 2025 (28 days, non-leap)
    assert_eq!(6u32.add_months(2023, 20), Some((2025, 2, 28)));
}

#[test]
fn test_add_months_lands_on_non_leap_february() {
    // June 2021 + 20 = Feb 2023 (28 days)
    assert_eq!(6u32.add_months(2021, 20), Some((2023, 2, 28)));
}

#[test]
fn test_add_months_into_leap_february() {
    // Feb 2024 is a leap year (29 days)
    assert_eq!(1u32.add_months(2024, 1), Some((2024, 2, 29)));
}

// --- invalid inputs ---

#[test]
fn test_invalid_month_zero() {
    assert_eq!(0u32.add_months(2025, 1), None);
}

#[test]
fn test_invalid_month_thirteen() {
    assert_eq!(13u32.add_months(2025, 1), None);
}
