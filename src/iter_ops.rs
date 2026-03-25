use rust_decimal::prelude::FromPrimitive;

use crate::{BaseMoney, BaseOps, Currency, Decimal, IterOps, macros::dec};

impl<I: ?Sized, T, C> IterOps<C> for I
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    T: BaseMoney<C> + BaseOps<C> + Default,
    C: Currency,
{
    type Item = T;

    fn checked_sum(&self) -> Option<T> {
        self.into_iter().next()?;
        self.into_iter()
            .try_fold(T::default(), |acc, b| acc.checked_add(b.amount()))
    }

    fn mean(&self) -> Option<Self::Item> {
        let items: Vec<&T> = self.into_iter().collect();
        let count = items.len();
        if count == 0 {
            return None;
        }
        let sum = self.checked_sum()?;
        let count_decimal = Decimal::from_usize(count)?;
        sum.checked_div(count_decimal)
    }

    fn median(&self) -> Option<Self::Item> {
        let mut items: Vec<&T> = self.into_iter().collect();
        if items.is_empty() {
            return None;
        }
        items.sort_by_key(|a| a.amount());
        let len = items.len();
        if len % 2 == 1 {
            Some(items[len / 2].clone())
        } else {
            let mid = len / 2;
            let sum = items[mid - 1].checked_add(items[mid].amount())?;
            sum.checked_div(dec!(2))
        }
    }

    fn mode(&self) -> Option<Vec<Self::Item>> {
        let items: Vec<&T> = self.into_iter().collect();
        //#1 If vector is empty, return none. vec![] -> None
        if items.is_empty() {
            return None;
        }

        //#2 If vector only has 1 element, return that element. vec![5] -> Some(vec![5])
        if items.len() == 1 {
            return Some(vec![items[0].clone()]);
        }

        // Count occurrences of each distinct amount value in O(n)
        let mut counts = std::collections::HashMap::<Decimal, usize>::new();
        for item in &items {
            *counts.entry(item.amount()).or_insert(0) += 1;
        }

        //#3 If vector has multiple elements, and all elements are the same, return that elements. E.g. vec![5,5,5,5]; -> Some(vec![5]).
        if counts.len() == 1 {
            return Some(vec![items[0].clone()]);
        }

        // Find the maximum frequency
        let max_count = *counts.values().max()?;
        // Collect all distinct amounts that appear at the maximum frequency
        let mode_amounts: std::collections::HashSet<Decimal> = counts
            .iter()
            .filter(|(_, c)| **c == max_count)
            .map(|(k, _)| *k)
            .collect();
        // If every distinct value is at max frequency and there is more than one
        // distinct value, there is no dominant mode group → return None.
        //#4 If vector has multiple different elements, and ALL of them share the same occurrences, return none, meaning no modes. vec![1,1,2,2,3,3] -> None
        if mode_amounts.len() == counts.len() && counts.len() > 1 {
            return None;
        }
        // Return one representative per mode amount, preserving first-occurrence order
        //#5 If vector has multiple different elements, only SOME elements share the same occurrences, return those elements as Vector vec![1,1,1,2,2,3,3,3] -> Some(vec![1,3])
        let mut seen = std::collections::HashSet::<Decimal>::new();
        let result: Vec<T> = items
            .into_iter()
            .filter(|item| mode_amounts.contains(&item.amount()))
            .filter(|item| seen.insert(item.amount()))
            .cloned()
            .collect();
        Some(result)
    }
}
