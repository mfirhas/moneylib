use crate::base::Amount;
use crate::{BaseMoney, BaseOps, IterOps, MoneyFormatter};
use crate::{Currency, Decimal, base::DecimalNumber, macros::dec};
use rust_decimal::prelude::FromPrimitive;

/// Get the Unit of Least Precision from a decimal amount.
#[inline(always)]
fn ulp(amount: Decimal) -> Decimal {
    Decimal::new(1, amount.scale())
}

/// Split money into equal parts leaving a remainder.
pub(crate) fn split<M, C>(money: &M, n: u32) -> Option<(M, M)>
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + Ord,
    C: Currency,
{
    if n == 0 {
        return None;
    }

    let is_negative = money.is_negative();
    let money = if is_negative {
        money.abs()
    } else {
        money.clone()
    };

    let split_num = Decimal::from_u32(n)?;
    let mut equal_part = money.checked_div(split_num)?;
    let total = equal_part.checked_mul(split_num)?;

    // total might be bigger than original amount due to rounding. E.g. 10.01 split by 3 = 3.33666666667 -> 3.34 and 3.34 * 3 = 10.02.
    if total.amount() > money.amount() {
        let n_usize = n.try_into().ok()?;
        let mut parts = vec![equal_part; n_usize];

        let ulp = ulp(total.amount());
        let mut i: usize = 0;
        while parts.checked_sum()?.amount() > money.amount() {
            parts[i] = parts[i].checked_sub(ulp)?;
            i += 1;
            if i >= parts.len() {
                i = 0;
            }
        }

        // sort ascending and get equal part as first item.
        parts.sort();

        let mut new_equal_part = parts[0].clone();
        let new_total = new_equal_part.checked_mul(split_num)?;
        let mut new_remainder = money.checked_sub(new_total)?;

        if is_negative {
            new_equal_part = -new_equal_part;
            new_remainder = -new_remainder;
        }

        return Some((new_equal_part, new_remainder));
    };

    let mut remainder = money.checked_sub(total)?;

    if is_negative {
        equal_part = -equal_part;
        remainder = -remainder;
    }

    Some((equal_part, remainder))
}

/// Split money into equal parts and distribute the remainder equally into parts.
pub(crate) fn split_dist<M, C>(money: &M, n: u32) -> Option<Vec<M>>
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + Ord,
    C: Currency,
{
    if n == 0 {
        return None;
    }

    let is_negative = money.is_negative();
    let money = if is_negative {
        money.abs()
    } else {
        money.clone()
    };

    let (equal_part, mut remainder) = split(&money, n)?;

    let split_num: usize = n.try_into().ok()?;

    let ulp = ulp(remainder.amount());

    let mut parts = vec![equal_part; split_num];

    let mut i = 0;
    while remainder.amount() >= ulp {
        parts[i] = parts[i].checked_add(ulp)?;
        remainder = remainder.checked_sub(ulp)?;
        i += 1;
        if i >= split_num {
            i = 0;
        }
    }

    if is_negative {
        parts = parts.into_iter().map(|p| -p).collect::<Vec<_>>();
    }

    Some(parts)
}

/// Allocate money by percentages.
pub(crate) fn allocate<M, C, D>(money: &M, pcns: &[D]) -> Option<Vec<M>>
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + MoneyFormatter<C>,
    C: Currency,
    D: DecimalNumber + Copy,
{
    if pcns.is_empty() {
        return None;
    }

    let is_negative = money.is_negative();
    let money = if is_negative {
        money.abs()
    } else {
        money.clone()
    };

    let mut total = Decimal::ZERO;
    for p in pcns {
        total = total.checked_add(p.get_decimal()?)?;
    }
    if total != dec!(100) {
        return None;
    }
    let mut ret = allocate_by_ratios::<M, C, _>(&money, pcns);

    if is_negative && let Some(m) = ret {
        ret = Some(m.into_iter().map(|r| -r).collect::<Vec<_>>());
    }

    ret
}

/// Allocate money by ratios.
pub(crate) fn allocate_by_ratios<M, C, D>(money: &M, ratios: &[D]) -> Option<Vec<M>>
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + MoneyFormatter<C>,
    C: Currency,
    D: DecimalNumber + Copy,
{
    if ratios.is_empty() {
        return None;
    }

    let is_negative = money.is_negative();
    let money = if is_negative {
        money.abs()
    } else {
        money.clone()
    };

    let total_ratio: Decimal = {
        let mut total = Decimal::ZERO;
        for d in ratios {
            total = total.checked_add(d.get_decimal().unwrap_or_default())?;
        }
        total
    };

    // allocate base for each ratio
    let mut parts: Vec<M> = ratios
        .iter()
        .map(|r| {
            let share = money.checked_mul(*r)?.checked_div(total_ratio)?;
            Some(share)
        })
        .collect::<Option<Vec<_>>>()?;

    let allocated_total = parts.checked_sum()?;

    if allocated_total.amount() > money.amount() {
        let mut i = 0;
        let ulp = ulp(allocated_total.amount());
        while parts.checked_sum()?.amount() > money.amount() && i < parts.len() {
            parts[i] = parts[i].checked_sub(ulp)?;
            i += 1;
        }

        parts.sort_by_key(|b| std::cmp::Reverse(b.amount()));

        return Some(parts);
    }

    let mut remainder = money.checked_sub(allocated_total.clone())?;

    let ulp = ulp(allocated_total.amount());

    let mut i = 0;
    while remainder.amount() >= ulp && i < parts.len() {
        parts[i] = parts[i].checked_add(ulp)?;
        remainder = remainder.checked_sub(ulp)?;
        i += 1;
    }

    if is_negative {
        parts = parts.into_iter().map(|r| -r).collect::<Vec<_>>();
    }

    Some(parts)
}
