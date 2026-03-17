/// get current date: Some(year, index of month(january = 1), day)
pub(crate) fn current_date() -> Option<(u32, u32, u32)> {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();

    // days since epoch
    let mut days = secs / 86400;

    // compute year
    let mut year = 1970u32;
    loop {
        let days_in_current_year = days_in_year(year).into();
        if days < days_in_current_year {
            break;
        }
        days -= days_in_current_year;
        year += 1;
    }

    // compute month
    let mut month = 1u32;
    loop {
        let days_in_current_month = days_in_month(year, month)?.into();
        if days < days_in_current_month {
            break;
        }
        days -= days_in_current_month;
        month += 1;
    }

    // remaining days is day index (0-based), +1 for 1-based
    let day: u32 = (days + 1).try_into().ok()?;

    Some((year, month, day))
}

#[cfg(test)]
mod current_data_tests {
    use super::current_date;

    #[test]
    fn test_current_date() {
        let ret = current_date();
        assert!(ret.is_some());
        println!("{:?}", ret.unwrap());
    }
}

pub(crate) fn days_in_month(year: u32, month: u32) -> Option<u32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => {
            if is_leap_year(year) {
                Some(29)
            } else {
                Some(28)
            }
        }
        _ => None,
    }
}

pub(crate) fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || (year.is_multiple_of(400))
}

pub(crate) fn days_in_year(year: u32) -> u32 {
    if is_leap_year(year) { 366 } else { 365 }
}

pub(crate) trait MonthNext {
    /// Returns next year, next month index and number of days in that next month index.
    fn next_month(self, year: u32) -> Option<(u32, u32, u32)>;
}

impl MonthNext for u32 {
    fn next_month(self, year: u32) -> Option<(u32, u32, u32)> {
        let (next_year, next_month) = match self {
            1..=11 => (year, self + 1),
            12 => (year + 1, 1),
            _ => return None,
        };
        let days = days_in_month(next_year, next_month)?;
        Some((next_year, next_month, days))
    }
}

pub(crate) trait DayNext {
    /// Returns (year, month, day, num_of_days_in_that_month) of the next day.
    fn next_day(self, month: u32, year: u32) -> Option<(u32, u32, u32, u32)>;
}

impl DayNext for u32 {
    fn next_day(self, month: u32, year: u32) -> Option<(u32, u32, u32, u32)> {
        let days_in_current = days_in_month(year, month)?;

        if self == 0 || self > days_in_current {
            return None;
        }

        let (next_year, next_month, next_day) = if self < days_in_current {
            (year, month, self + 1)
        } else {
            // self == days_in_current, overflow to next month
            let (next_year, next_month) = match month {
                1..=11 => (year, month + 1),
                12 => (year + 1, 1),
                _ => return None,
            };
            (next_year, next_month, 1)
        };

        let days_in_next = days_in_month(next_year, next_month)?;
        Some((next_year, next_month, next_day, days_in_next))
    }
}

/// Returns [years[months]]
pub(crate) fn get_years_months(
    start_year: u32,
    start_month: u32,
    num_of_months: u32,
) -> Option<Vec<(u32, Vec<u32>)>> {
    if num_of_months == 0 {
        return None;
    }

    // validate start_month
    days_in_month(start_year, start_month)?;

    let mut result: Vec<(u32, Vec<u32>)> = Vec::new();
    let mut remaining = num_of_months;
    let mut year = start_year;
    let mut month = start_month;

    while remaining > 0 {
        match result.last_mut() {
            Some((y, months)) if *y == year => months.push(month),
            _ => result.push((year, vec![month])),
        }

        remaining -= 1;

        if remaining > 0 {
            let (next_year, next_month, _days) = month.next_month(year)?;
            year = next_year;
            month = next_month;
        }
    }

    Some(result)
}

pub(crate) type YearsMonthsDays = Vec<(u32, Vec<(u32, Vec<u32>)>)>;

/// Returns [years[months[days]]]
pub(crate) fn get_years_months_days(
    start_year: u32,
    start_month: u32,
    start_day: u32,
    num_of_days: u32,
) -> Option<YearsMonthsDays> {
    let days_in_start_month = days_in_month(start_year, start_month)?;

    if start_day == 0 || start_day > days_in_start_month {
        return None;
    }

    if num_of_days == 0 {
        return Some(Vec::new());
    }

    let mut result: YearsMonthsDays = Vec::new();
    let mut current_year = start_year;
    let mut current_month = start_month;
    let mut current_day = start_day;

    for _ in 0..num_of_days {
        // Insert current day into result
        match result.iter_mut().find(|(y, _)| *y == current_year) {
            Some((_, months)) => match months.iter_mut().find(|(m, _)| *m == current_month) {
                Some((_, days)) => days.push(current_day),
                None => months.push((current_month, vec![current_day])),
            },
            None => result.push((current_year, vec![(current_month, vec![current_day])])),
        }

        // Advance to next day using the DayNext trait
        if let Some((ny, nm, nd, _)) = current_day.next_day(current_month, current_year) {
            current_year = ny;
            current_month = nm;
            current_day = nd;
        }
    }

    Some(result)
}
