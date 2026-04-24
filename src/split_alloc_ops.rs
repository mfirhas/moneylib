use crate::base::Amount;
use crate::{BaseMoney, BaseOps, IterOps};
use crate::{Currency, Decimal, base::DecimalNumber};
use rust_decimal::prelude::FromPrimitive;
use std::sync::LazyLock;

/// Split trait containing split function implemented by return types.
///
/// This is for generic parameters and return type of split and allocation operations.
pub trait Split<M, C, P>
where
    Self: Sized,
{
    /// split money by input without losing a single penny.
    fn split(money: M, input: P) -> Option<Self>;
}

impl<M, C> Split<M, C, u32> for (M, M)
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + Ord,
    C: Currency,
{
    fn split(money: M, input: u32) -> Option<Self> {
        split(&money, input)
    }
}

impl<M, C> Split<M, C, u32> for Vec<M>
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + Ord,
    C: Currency,
{
    fn split(money: M, input: u32) -> Option<Self> {
        split_dist(&money, input)
    }
}

macro_rules! impl_split_iterable {
    ($input:ty $(, const $n:ident: usize)?) => {
        impl<M, C, D $(, const $n: usize)?> Split<M, C, $input> for Vec<M>
        where
            M: BaseMoney<C> + BaseOps<C> + Default + Amount<C> + Ord,
            C: Currency,
            D: DecimalNumber + Copy,
        {
            fn split(money: M, input: $input) -> Option<Self> {
                allocate(&money, input)
            }
        }
    };
}

impl_split_iterable!(Vec<D>);
impl_split_iterable!(&Vec<D>);
impl_split_iterable!(&[D]);
impl_split_iterable!([D; N], const N: usize);
impl_split_iterable!(&[D; N], const N: usize);

// max mantissa digits
static DECIMAL_MAX_DIGITS: LazyLock<usize> =
    LazyLock::new(|| crate::Decimal::MAX.mantissa().to_string().len());

/// Get the Unit of Least Precision from a decimal amount.
#[inline(always)]
fn ulp(amount: Decimal) -> Decimal {
    Decimal::new(1, amount.scale())
}

/// Get equal part of money splitting
pub(crate) fn get_equal_part<M, C>(money: &M, split: u32) -> Option<M>
where
    M: BaseMoney<C> + BaseOps<C> + PartialOrd,
    C: Currency,
{
    let is_negative = money.is_negative();
    let money = money.abs();

    let split_dec = Decimal::from_u32(split)?;
    let equal_part = money.checked_div(split_dec)?;
    let total_parts = equal_part.checked_mul(split_dec)?;

    let rem = money.checked_rem(split_dec)?;

    let equal_part_scale = equal_part.scale();
    let equal_part_digits_len = equal_part.amount().mantissa().to_string().len();

    // If remainder is not zero, but the total parts summed back to origin money amount,
    // then there's rounding in the calculation.
    //
    // For this case, we truncate the digit from most right(after decimal point), until total_parts < money(origin).
    if !rem.amount().is_zero()
        && total_parts >= money
        && equal_part_scale > 0
        && equal_part_digits_len >= *DECIMAL_MAX_DIGITS
    {
        let mut truncated_equal_part_mantissa = equal_part.amount().mantissa().checked_div(10)?;
        let mut truncated_equal_part_scale = equal_part_scale - 1;
        let mut truncated_equal_part_amount = Decimal::try_from_i128_with_scale(
            truncated_equal_part_mantissa,
            truncated_equal_part_scale,
        )
        .ok()?;
        while truncated_equal_part_scale > 0
            && truncated_equal_part_amount.checked_mul(split_dec)? >= money.amount()
        {
            truncated_equal_part_mantissa /= 10;
            truncated_equal_part_scale -= 1;
            truncated_equal_part_amount = Decimal::try_from_i128_with_scale(
                truncated_equal_part_mantissa,
                truncated_equal_part_scale,
            )
            .ok()?;
        }

        if is_negative {
            truncated_equal_part_amount = -truncated_equal_part_amount;
        }

        return M::new(truncated_equal_part_amount).ok();
    }

    if is_negative {
        return Some(-equal_part);
    }

    // if total_parts NOT rounded up
    Some(equal_part)
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
    let money = money.abs();

    let split_num = Decimal::from_u32(n)?;
    let mut equal_part = get_equal_part(&money, n)?;
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
    let money = money.abs();

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

/// Allocate money by ratios.
pub(crate) fn allocate<M, C, I, D>(money: &M, ratios: I) -> Option<Vec<M>>
where
    M: BaseMoney<C> + BaseOps<C> + Default + Amount<C>,
    C: Currency,
    I: AsRef<[D]>,
    D: DecimalNumber + Copy,
{
    if ratios.as_ref().is_empty() {
        return None;
    }

    let is_negative = money.is_negative();
    let money = money.abs();

    let total_ratio: Decimal = {
        let mut total = Decimal::ZERO;
        for d in ratios.as_ref() {
            total = total.checked_add(d.get_decimal().unwrap_or_default())?;
        }
        total
    };

    // Check if one of the parts has long scale equals to decimal max digits.
    //
    // If it is, then it must be RawMoney. It uses for checking for trimming the scale after truncating.
    let mut is_long_scale = false;

    // the shortest scale if is_long_scale
    let mut shortest_scale: Option<u32> = None;

    // allocate base for each ratio
    let mut parts: Vec<M> = ratios
        .as_ref()
        .iter()
        .map(|r| {
            let share = money.checked_mul(*r)?.checked_div(total_ratio)?;
            let mantissa = share.amount().mantissa();
            let scale = share.amount().scale();
            // Only track scale for non-zero shares; a zero share's scale of 0
            // must not over-aggressively truncate high-precision parts.
            if !share.amount().is_zero() {
                if let Some(shortest) = shortest_scale
                    && scale < shortest
                {
                    shortest_scale = Some(scale);
                }
                if shortest_scale.is_none() {
                    shortest_scale = Some(scale);
                }
            }
            // only happen to raw money
            if mantissa.to_string().len() >= *DECIMAL_MAX_DIGITS && share.scale() > 0 {
                is_long_scale = true;
                let new_mantissa = mantissa / 10;
                let new_scale = share.scale() - 1;
                if let Some(shortest) = shortest_scale
                    && new_scale < shortest
                {
                    shortest_scale = Some(new_scale);
                }
                return M::new(Decimal::from_i128_with_scale(new_mantissa, new_scale)).ok();
            }
            Some(share)
        })
        .collect::<Option<Vec<_>>>()?;

    if let Some(shortest) = shortest_scale
        && is_long_scale
    {
        parts = parts
            .iter()
            .map(|p| p.truncate_with(shortest))
            .collect::<Vec<_>>();
    }

    let allocated_total = parts.checked_sum()?;

    if allocated_total.amount() > money.amount() {
        let mut i = 0;
        while parts.checked_sum()?.amount() > money.amount() {
            let ulp = ulp(parts[i].amount());
            parts[i] = parts[i].checked_sub(ulp)?;
            i += 1;
            if i >= parts.len() {
                i = 0;
            }
        }

        parts.sort_by_key(|b| std::cmp::Reverse(b.amount()));

        if is_negative {
            parts = parts.into_iter().map(|r| -r).collect::<Vec<_>>();
        }

        return Some(parts);
    }

    let mut remainder = money.checked_sub(allocated_total.clone())?;

    let ulp = ulp(remainder.amount());

    let mut i = 0;
    while remainder.amount() >= ulp {
        parts[i] = parts[i].checked_add(ulp)?;
        remainder = remainder.checked_sub(ulp)?;
        i += 1;
        if i >= parts.len() {
            i = 0;
        }
    }

    if is_negative {
        parts = parts.into_iter().map(|r| -r).collect::<Vec<_>>();
    }

    Some(parts)
}
