# Code Review — `moneylib`

This document summarizes findings from a review of the `moneylib` crate (v0.12.0). Items are grouped by severity.

---

## Critical / Bugs

### 1. `Money * Money` and `Money / Money` semantics are incorrect

**Files:** `src/ops.rs` (lines 46–79), `src/raw_money/ops.rs` (lines 46–79)

Multiplying two monetary values together (`$10 × $5 = $50` in "dollars-squared"?) and dividing two monetary values to get a monetary value (`$10 / $5 = $2`?) are not financially meaningful operations. The result of `Money<USD> / Money<USD>` should be a dimensionless ratio (`Decimal`), not another `Money<USD>`. And `Money<USD> * Money<USD>` has no real-world meaning at all.

These operator implementations will silently produce nonsensical results if users accidentally use them.

**Suggestion:** Remove `Mul<Self>` and `Div<Self>` for `Money`/`RawMoney`. If division of two money values is needed (e.g. for ratios), return `Decimal` instead of `Money<C>`. If `Mul<Self>` and `Div<Self>` are required by the `BaseOps` trait bound, consider relaxing those bounds.

---

### 2. `calendar.rs` — `next_day` returns wrong `days_in_next` for non-overflow case

**File:** `src/calendar.rs` (line 93)

```rust
if self < days_in_current {
    let days_in_next = days_in_month(year, month)?; // BUG: should be days_in_month for next day's month
    Some((year, month, self + 1, days_in_next))
}
```

When the current day is **not** the last day of the month, the code calculates `days_in_next` by calling `days_in_month(year, month)` — the **same** month, not the next. The returned tuple's 4th element (`days_in_next`) is supposed to represent the number of days in the month of the **resulting** day, but this is actually fine for the non-overflow case (the resulting day is still in the same month). However, the variable name `days_in_next` is misleading since it's `days_in_current`. If the intent is "days in the month of the returned date," the current code is functionally correct but confusingly named.

---

### 3. `is_positive()` / `is_negative()` treat zero inconsistently

**File:** `src/base.rs` (lines 370–392)

`is_sign_positive()` from `rust_decimal` returns `true` for zero, and `is_sign_negative()` returns `false` for zero. This means `Money::new(0).is_positive()` returns `true`, which is mathematically wrong — zero is neither positive nor negative. Users checking `is_positive()` to guard against zero amounts will be surprised.

**Suggestion:** Change to `self.amount() > Decimal::ZERO` and `self.amount() < Decimal::ZERO`, or at minimum clearly document that zero is considered positive.

---

## High Severity

### 4. All `std::ops` implementations panic on overflow instead of returning `Result`/`Option`

**Files:** `src/ops.rs`, `src/dec_ops.rs`, `src/raw_money/ops.rs`, `src/raw_money/dec_ops.rs`

Every `Add`, `Sub`, `Mul`, `Div`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign` implementation calls `.expect(...)`, which will panic at runtime on overflow or division by zero. While the library also provides `checked_*` methods that return `Option`, users reaching for the natural `+`, `-`, `*`, `/` operators will get panics instead of errors.

For a financial library where correctness is paramount, this is a significant footgun. The `WARN: PANIC!` comments show awareness, but don't prevent the issue.

**Suggestion:** Consider at least:
- Adding `#[must_use]` and prominent doc-comments on the operator traits warning about panics.
- Providing a `TryAdd`, `TrySub`, etc. trait that returns `Result` (or guiding users strongly toward the `checked_*` methods).
- `Sum` also panics (line 427–429 of `money.rs`).

---

### 5. `Div` by zero-amount `Money` panics without a clear error

**Files:** `src/ops.rs` (line 75), `src/raw_money/ops.rs` (line 75)

`money / Money::default()` will panic with `"division operation overflow"` rather than a clear "division by zero" message.

**Suggestion:** At minimum, improve the panic message. Ideally, don't offer `Div<Self>` at all (see item #1).

---

### 6. `ExchangeRates` uses `&str` keys — no validation of currency codes

**File:** `src/exchange.rs` (line 296)

`ExchangeRates::set()` accepts arbitrary `&str` as a currency code key with no validation. Typos like `rates.set("USE", dec!(0.8))` will silently succeed, and later `convert` calls will return `None` with no indication of the typo.

**Suggestion:** Validate that the code is a known ISO 4217 code, or accept a type-safe `C::CODE` instead of a raw string. At minimum, document this pitfall.

---

## Medium Severity

### 7. Significant code duplication between `Money` and `RawMoney`

**Files:** `src/money.rs` vs `src/raw_money/raw_money.rs`, `src/ops.rs` vs `src/raw_money/ops.rs`, `src/dec_ops.rs` vs `src/raw_money/dec_ops.rs`

The `Money` and `RawMoney` types share nearly identical implementations for operator traits, parsing, formatting, serde, etc. The only real difference is whether `from_decimal` calls `.round()` or not. This duplication is a maintenance burden — any bug fix or new feature must be applied in two places.

**Suggestion:** Consider a shared inner type parameterized by a rounding policy, or use a macro to generate the shared implementations.

---

### 8. `allocate_by_ratios` — `unwrap_or_default` silently swallows errors

**File:** `src/split_alloc_ops.rs` (line 220)

```rust
total = total.checked_add(d.get_decimal().unwrap_or_default())?;
```

If `get_decimal()` returns `None` (e.g. `f64::NAN` or `f64::INFINITY`), this silently treats it as zero instead of propagating the error. A ratio of zero may produce unexpected allocation results.

**Suggestion:** Replace `unwrap_or_default()` with `?` (returning `None` on failure) to be consistent with the rest of the function.

---

### 9. `split_alloc_ops` functions recalculate `checked_sum()` in a loop — O(n²) complexity

**File:** `src/split_alloc_ops.rs` (lines 101–108, 280–288, 303–310)

Several loops call `parts.checked_sum()?.amount()` on every iteration to compare against the target. Since `checked_sum` iterates the entire vector, this creates O(n²) behavior. For large numbers of parts this is unnecessarily slow.

**Suggestion:** Track a running total incrementally instead of recalculating the full sum each iteration.

---

### 10. `allocate_by_ratios` sorts results, losing correspondence with input ratios

**File:** `src/split_alloc_ops.rs` (line 290)

When `allocated_total > money`, the function adjusts parts and then sorts them in descending order by amount. This means the output order no longer corresponds to the input ratio order, which is surprising and may break callers who expect `parts[i]` to correspond to `ratios[i]`.

**Suggestion:** Distribute the adjustment without sorting, or document this behavior explicitly.

---

### 11. `parse_symbol_comma_thousands_separator` doesn't handle negative amounts correctly for symbol-prefixed format

**File:** `src/parse.rs` (lines 174–199)

The negative sign is expected **before** the symbol: `-$1,234.56`. But some locales and conventions place the negative sign after the symbol or use parentheses. Also, the function strips the `-` then strips the symbol, so `$-1,234.56` would fail to parse, while `-$1,234.56` works. This is inconsistent with `from_code_comma_thousands` which supports `CODE -amount`.

**Suggestion:** Support both `-$` and `$-` orderings, or clearly document the expected format.

---

## Low Severity / Style

### 12. Typos in doc comments

- `src/ops.rs` line 39: `"substraction"` → `"subtraction"` (already correct on line 109)
- `src/percent_ops.rs` line 112: `"Substracts"` → `"Subtracts"`
- `src/percent_ops.rs` line 133: `"Substracts"` → `"Subtracts"`

---

### 13. `#[allow(clippy::len_without_is_empty)]` on `ExchangeRates`

**File:** `src/exchange.rs` (line 360)

The `is_empty` method is suppressed. An `ExchangeRates` always has at least one entry (the base currency), so it can never be empty, but it's better practice to add the `is_empty` method that returns `false` rather than suppressing the lint.

---

### 14. `MoneyError` variants lack context information

**File:** `src/error.rs`

Error variants like `ParseStr` and `CurrencyMismatch` carry no context about what was being parsed or which currencies were mismatched. This makes debugging harder, especially when errors are propagated through several layers.

**Suggestion:** Add fields to carry context, e.g.:
```rust
CurrencyMismatch { expected: &'static str, got: String }
```

---

### 15. `format_decimal_abs` — minor `into()` readability

**File:** `src/fmt.rs` (lines 125, 130)

The expression `frac.len() >= minor_unit.into()` uses a bare `.into()` for `u16 → usize` conversion. While correct, `usize::from(minor_unit)` would be more readable and explicit.

---

### 16. `calendar.rs` — `current_date()` depends on system clock

**File:** `src/calendar.rs` (line 2)

The `current_date()` function uses `SystemTime::now()`, making it non-deterministic and hard to test. The interest calculation defaults use this, meaning test results depend on when they run.

**Suggestion:** Accept a date parameter with a default fallback, or make the system clock injectable for testing.

---

### 17. Missing `#[non_exhaustive]` on `MoneyError`

**File:** `src/error.rs`

Adding new error variants in the future will be a breaking change for downstream match arms. Adding `#[non_exhaustive]` would allow safe extension.

---

### 18. `Interest` builder accepts invalid date combinations silently

**File:** `src/accounting/interest.rs`

The builder allows setting arbitrary year/month/day combinations. There's no validation that the day is valid for the given month (e.g., February 30). Invalid dates will only surface as `None` deep inside the calculation.

**Suggestion:** Validate dates when they are set in the builder.

---

### 19. `Decimal` is re-exported but `rust_decimal` is a hidden dependency

**File:** `src/lib.rs` (line 78)

`pub use rust_decimal::Decimal;` re-exports `Decimal`, but users who want to use `Decimal`-specific methods (like `MathematicalOps`) must add `rust_decimal` to their own `Cargo.toml`. Consider documenting this or re-exporting commonly needed traits.

---

### 20. `clone()` on `Copy` types

**Files:** Various (e.g., `money.rs` line 437, `iter_ops.rs` line 38, `percent_ops.rs` lines 219, 221, 240, 241)

Both `Money` and `RawMoney` derive `Copy`, so calling `.clone()` is unnecessary. While not harmful (it compiles to the same code), it adds noise and suggests the type might not be `Copy`.

**Suggestion:** Replace `.clone()` with direct copies for `Copy` types.

---

## Summary

| Severity | Count |
|----------|-------|
| Critical / Bug | 3 |
| High | 3 |
| Medium | 5 |
| Low / Style | 9 |

The library has a solid foundation with good type safety, strong use of `forbid(unsafe_code)` and clippy lints, comprehensive documentation, and thorough test coverage. The most impactful improvements would be:

1. Removing or fixing `Mul<Self>` / `Div<Self>` for money types (incorrect semantics).
2. Addressing the pervasive panic-on-overflow in operator implementations.
3. Reducing code duplication between `Money` and `RawMoney`.
4. Fixing the `is_positive()`/`is_negative()` zero handling.
