# Code Review — `moneylib`

This document summarizes findings from a review of the `moneylib` crate (v0.13.0, current `master`).
It supersedes the earlier review made against v0.12.0.
Items are grouped by severity; a "Previously noted / Addressed" section records fixed issues.

---

## Critical / Bugs

### 1. `ObjMoney::convert()` returns the source currency type, not the target

**Files:** `src/obj_money/money_impl.rs` (lines 57–72), `src/obj_money/raw_money_impl.rs` (lines 57–72)

Both `Money<C>` and `RawMoney<C>` implement `convert(to_code, rate)` by calling `Self::from_decimal(...)`.
`Self` is always the *source* currency type, so the returned `Box<dyn ObjMoney>` is internally a
`Money<C>` (or `RawMoney<C>`) with the *source* currency code — despite having the converted amount.
Calling `.code()` on the result will return the **source** currency code instead of `to_code`.

```rust
// In money_impl.rs – converts USD->EUR but the box is still Money<USD>
Ok(Box::new(Self::from_decimal(   // Self = Money<USD>
    BaseMoney::amount(self)
        .checked_mul(rate.get_rate(BaseMoney::code(self), to_code)...)?
)))
```

This is a silent correctness bug: the caller cannot distinguish the original currency from the
converted one by inspecting the returned object.

**Suggestion:** The erased-type `ObjMoney` conversion cannot create a strongly-typed `Money<To>` without
knowing `To` at compile time. One approach is to store the target currency code as metadata inside
a wrapper newtype; another is to restrict `ObjMoney::convert` to a set of well-known currencies via
an enum, or return `(Box<dyn ObjMoney>, String)` where the `String` is the confirmed target code
and document that `.code()` on the result will reflect the source type. At minimum, the current
behavior must be clearly documented as a known limitation.

---

### 2. `is_positive()` returns `true` for zero (both `BaseMoney` and `ObjMoney`)

**Files:** `src/base.rs` (line 353), `src/obj_money/mod.rs` (line 105)

Both traits delegate to `Decimal::is_sign_positive()`, which returns `true` for zero.
`BaseMoney::is_positive()` documentation only shows examples with non-zero values; a caller
reading the docs would reasonably expect `Money::new(0).unwrap().is_positive()` to be `false`.

`ObjMoney` at least documents "(or zero)" in the method comment, but this inconsistency between the
two traits is confusing.

**Suggestion:** Use `self.amount() > Decimal::ZERO` (and `< Decimal::ZERO` for `is_negative()`), or
align the documentation of `BaseMoney::is_positive` to state explicitly that zero is considered positive.

---

### 3. `Money * Money` and `Money / Money` have incorrect financial semantics

**Files:** `src/ops.rs` (lines 45–79), `src/raw_money/ops.rs` (lines 45–79)

`Money<USD> * Money<USD>` produces a "dollars-squared" result that has no financial meaning.
`Money<USD> / Money<USD>` should yield a dimensionless `Decimal` ratio, not another `Money<USD>`.
These operators are still present and will silently produce nonsensical results.

**Suggestion:** Remove `Mul<Self>` and `Div<Self>` for money×money. If a ratio is needed, return
`Decimal`. See also item #4 below: the `BaseOps` trait super-bounds force these operators to exist,
so fixing this requires relaxing `BaseOps`.

---

## High Severity

### 4. All `std::ops` implementations panic on overflow

**Files:** `src/ops.rs`, `src/dec_ops.rs`, `src/raw_money/ops.rs`, `src/raw_money/dec_ops.rs`

Every `Add`, `Sub`, `Mul`, `Div`, and their `*Assign` variants call `.expect("... overflow")`.
Division by a zero-amount money panics with `"division operation failed"` or
`"division operation overflow"` — not a clear "division by zero" message.
`Sum` (lines 482–498 in `money.rs`) also panics.

The `WARN: PANIC!` comments show awareness, but library users reaching for `+`, `-`, `*`, `/`
naturally get panics with no opportunity to recover.

**Suggestion:**
- Add prominent doc-comments at the trait and operator-impl level that panics are possible.
- Consider deprecating these operators in favour of the `checked_*` methods, or add a
  `saturating_*` / `try_*` set that returns `Result`.
- Fix the division-by-zero panic message to be explicit.

---

### 5. `allocate` silently treats invalid ratios as zero

**File:** `src/split_alloc_ops.rs` (line 243)

```rust
total = total.checked_add(d.get_decimal().unwrap_or_default())?;
```

If a ratio value cannot be converted to `Decimal` (e.g. `f64::NAN`, `f64::INFINITY`), it is
silently replaced with `Decimal::ZERO`. This can cause the total ratio to be less than the
sum of the intended ratios, silently skewing all allocations.

**Suggestion:** Replace `.unwrap_or_default()` with `?` and propagate `None` as a failure signal,
consistent with every other `checked_*` usage in the codebase.

---

### 6. `ExchangeRates::set()` accepts unvalidated `&str` currency codes

**File:** `src/exchange.rs` (line 312)

Arbitrary strings (e.g. `"USE"` instead of `"USD"`) are accepted without validation.
Later `convert` / `get_pair` calls will simply return `None`/`Err` with no clue about
the root cause.

**Suggestion:** Validate that the code is a known ISO 4217 alphabetic code (e.g. against
`currencylib`), or accept a type-safe `C: Currency` parameter and use `C::CODE` instead of `&str`.

---

### 7. `ExchangeRates::From` silently discards `set()` errors

**File:** `src/exchange.rs` (lines 475–484)

```rust
fn from(value: I) -> Self {
    // ...
    for (k, v) in value {
        if k != Base::CODE {
            let _ = exchange_rates.set(k, v);  // ← errors silently dropped
        }
    }
    exchange_rates
}
```

`set()` now returns `Result<(), MoneyError>`, but all errors are discarded with `let _ = ...`.
An overflow during rate conversion (unlikely but possible with extreme values) will silently
produce an incomplete `ExchangeRates` with no indication of failure.

**Suggestion:** Return `Result<ExchangeRates<Base>, MoneyError>` from a fallible constructor, or
at minimum use a `TryFrom` impl. The infallible `From` impl should only be used for value types
that truly cannot fail.

---

## Medium Severity

### 8. `allocate` sorts the output when adjusting, breaking ratio correspondence

**File:** `src/split_alloc_ops.rs` (lines 313–315)

```rust
parts.sort_by_key(|b| std::cmp::Reverse(b.amount()));
```

When `allocated_total > money`, the surplus-adjustment loop reduces values and then sorts them
in descending order. After this sort, `parts[i]` no longer corresponds to `ratios[i]`. Callers
reasonably expect `parts[i]` to be the allocation for `ratios[i]`.

**Suggestion:** Distribute the adjustment (subtract ULP from the largest parts first) without
changing the order, so the correspondence is preserved.

---

### 9. `ExchangeRates<'a, Base>` has an unnecessary lifetime parameter

**File:** `src/exchange.rs` (line 253)

```rust
pub struct ExchangeRates<'a, Base: Currency> {
    rates: HashMap<&'a str, Decimal>,
    ...
}
```

Currency codes used as keys are always `&'static str` (they come from `C::CODE`). The `'a`
lifetime propagates into every type that holds `ExchangeRates`, requiring annotations like
`ExchangeRates<'static, USD>` in structs. Using `&'static str` directly (or `String`) would
simplify the public API.

---

### 10. `ObjIterOps` trait is only meaningful behind the `exchange` feature

**File:** `src/obj_money/mod.rs` (lines 190–200)

```rust
pub trait ObjIterOps {
    #[cfg(feature = "exchange")]
    fn checked_sum<M, To>(&self, rates: impl crate::exchange::ObjRate) -> Result<M, MoneyError>
    where ...;
}
```

The `ObjIterOps` trait has exactly one method and that method is `#[cfg(feature = "exchange")]`.
Without the `exchange` feature the trait is empty (zero methods), making the blanket `impl`
meaningless and the trait name misleading.

**Suggestion:** Gate the entire `ObjIterOps` trait and its `impl` behind `#[cfg(feature = "exchange")]`.

---

### 11. Significant code duplication between `Money` and `RawMoney`

**Files:** `src/money.rs` vs `src/raw_money/raw_money.rs`, `src/ops.rs` vs `src/raw_money/ops.rs`, `src/dec_ops.rs` vs `src/raw_money/dec_ops.rs`, `src/serde/money.rs` vs `src/serde/raw_money.rs`

The two types share nearly identical operator, parsing, formatting, and serde implementations.
The only real difference is whether `from_decimal` applies rounding. This duplication is a
maintenance burden: every bug fix and new feature must be applied twice.

**Suggestion:** Parameterise a shared inner implementation by a rounding policy (a zero-sized type
or a const bool), or use a declarative macro to stamp out the shared implementations once.

---

## Low Severity / Style

### 12. `allocate` in `split_alloc_ops.rs` has O(n²) sum recalculation

**File:** `src/split_alloc_ops.rs` (lines 303–315, 154–160)

Adjustment loops call `parts.checked_sum()?` on every iteration. Since `checked_sum` iterates
the entire vector, these loops are O(n²). For large allocations this is unnecessarily slow.

**Suggestion:** Track a running total incrementally instead of re-summing on every iteration.

---

### 13. Typos in doc comments

- `src/ops.rs` line 38: `"substraction"` → `"subtraction"`
- `src/percent_ops.rs` line 112: `"Substracts"` → `"Subtracts"`
- `src/percent_ops.rs` line 133: `"Substracts"` → `"Subtracts"`

---

### 14. `#[allow(clippy::len_without_is_empty)]` on `ExchangeRates`

**File:** `src/exchange.rs` (line 461)

`ExchangeRates` can never be empty (the base currency is always present), so an `is_empty()`
method could simply return `false`. Adding it removes the need for the suppression attribute
and makes the struct consistent with Rust's collection conventions.

---

### 15. `Interest` builder accepts invalid date combinations silently

**File:** `src/accounting/interest.rs`

The `year()`, `month()`, `day()` builder methods accept arbitrary `u32` values. An impossible
date like (February 30) will produce a `None` only deep inside the calculation, making
debugging difficult.

**Suggestion:** Validate the day against the month (and year for leap-year aware checks)
in `day()` and return `Option<Self>` on invalid input.

---

### 16. `calendar.rs` — `current_date()` depends on the system clock

**File:** `src/calendar.rs`

The default start date for interest calculations is taken from `SystemTime::now()`.
Tests that use the default date are non-deterministic and can produce different results on
different days (e.g. leap-year boundaries, month-length differences).

**Suggestion:** Accept an explicit date parameter and use the "current date" only as a
convenience default, or make the clock injectable.

---

### 17. Missing `#[non_exhaustive]` on `MoneyError`

**File:** `src/error.rs`

Adding a new error variant in a future version will be a breaking change for downstream users
who exhaustively `match` on `MoneyError`. Adding `#[non_exhaustive]` prevents this.

---

### 18. `clone()` called on `Copy` types

**Files:** `src/percent_ops.rs` (lines 219, 221, 240, 241), `src/iter_ops.rs` (various), `src/money.rs` (line 496)

`Money` and `RawMoney` derive `Copy`, so `.clone()` is a no-op copy. While harmless, it adds
noise and implies the type might not be `Copy`.

**Suggestion:** Replace `.clone()` with direct use or `let binding =` of the `Copy` value.

---

### 19. `BaseMoney::is_positive()` documentation inconsistency

**File:** `src/base.rs` (line 337)

The doc comment says "Returns `true` if the amount is positive" and the example only shows
non-zero cases. `ObjMoney::is_positive()` (line 103) correctly documents "(or zero)".
These should be aligned to say the same thing.

---

## Summary

| Severity | Count |
|----------|-------|
| Critical / Bug | 3 |
| High | 4 |
| Medium | 4 |
| Low / Style | 7 |

### Previously noted issues — now addressed ✅

The following items from the v0.12.0 review have been resolved:

| # | Issue | Resolution |
|---|-------|-----------|
| 14 | `MoneyError` variants lacked context | ✅ `CurrencyMismatchError(String, String)`, `ParseStrError(ErrVal)`, `ExchangeError(ErrVal)`, `ParseLocale(ErrVal)` added |
| Old | `ExchangeRates::set()` returned `()` silently | ✅ Now returns `Result<(), MoneyError>` |
| Old | `BaseMoney` required `Sized + FromStr` | ✅ Only `Clone` required now |
| Old | No `ObjMoney` / dynamic dispatch support | ✅ New `ObjMoney` trait added (v0.13.0) |
| Old | No `Send + Sync` on object-safe traits | ✅ `ObjMoney: Send + Sync`, `ObjRate: Send + Sync` added |

### Top priorities

1. **`ObjMoney::convert()` returns wrong currency type** — silent correctness bug that undermines the whole feature.
2. **`is_positive()` returns `true` for zero** — mathematical incorrectness; at minimum fix the documentation mismatch.
3. **Operators panic on overflow** — `checked_*` exists and should be the primary API; panicking operators are dangerous in financial code.
4. **`allocate` uses `unwrap_or_default()`** — silent swallowing of invalid inputs.
5. **`ExchangeRates::From` discards errors** — broken rates inserted silently from bulk construction.
