# Code Review — moneylib master branch

This document is a thorough review of the current `master` branch, focusing on correctness,
API usability, design concerns, documentation accuracy, and potential runtime hazards.

---

## 1. `money!` / `raw!` macros require `BaseMoney` in scope (breaking ergonomics)

**File:** `src/macros.rs`, lines 36–42, 81–87

Both macros expand to a call like:

```rust
$crate::Money::<$crate::iso::USD>::from_decimal($crate::dec!(100))
```

`from_decimal` is a method defined in the `BaseMoney<C>` trait, **not** an inherent method.
In Rust, calling a trait method requires the trait to be in scope at the call site.
A user who writes:

```rust
use moneylib::macros::money;

let price = money!(USD, 100.00); // compile error: method `from_decimal` not found
```

will receive a confusing "method not found" error.  
The doc comments do show `use moneylib::BaseMoney;` as a necessary import, but they
simultaneously claim "no `use moneylib::iso::USD;` needed", creating a false sense that
no extra imports are required. Many users will discover this only after a cryptic compiler error.

**Possible fixes:**
- Change the macro expansion to use the fully-qualified syntax:
  `<$crate::Money::<$crate::iso::$currency> as $crate::BaseMoney<_>>::from_decimal(...)`.
- Or inject a `use $crate::BaseMoney as _` guard inside the macro body.

The same applies to `raw!` and the `BaseMoney` trait.

---

## 2. Operator overloads (`+`, `-`, `*`, `/`, `%`) panic on overflow

**File:** `src/ops.rs`

Every standard operator overload for `Money<C>` and `RawMoney<C>` calls `.expect(...)`,
e.g.:

```rust
.expect("addition operation overflow")
```

These panics are only documented with internal `// WARN: PANIC!` source comments,
not in the public doc-comments of `Money` or `RawMoney`. A library user who reaches
overflow at runtime will see an opaque panic with no indication it can be avoided.

**Recommendation:** Document the panic behaviour prominently in the `impl Add`, `impl Sub`,
etc. doc blocks, and encourage users to prefer the `BaseOps::checked_*` methods for any
path where overflow is possible (e.g. user-supplied financial data).

---

## 3. `ObjIterOps::checked_sum` is gated behind the `exchange` feature

**File:** `src/obj_money/obj_money.rs`, lines 483–524

```rust
pub trait ObjIterOps {
    #[cfg(feature = "exchange")]
    fn checked_sum(...) -> Result<Box<dyn ObjMoney>, MoneyError>;
}
```

The `ObjIterOps` trait **only has methods when `exchange` is enabled**.  
Users who include `obj_money` but not `exchange` (a reasonable combination) cannot sum a
`Vec<Box<dyn ObjMoney>>` at all — the trait appears empty and the `ObjIterOps` import is
useless. A same-currency summing variant that does not require exchange rates would be a
natural addition.

---

## 4. `Context::set_raw` is a global atomic with no scoped reset and `serial_test` not in `Cargo.toml`

**File:** `src/obj_money/context.rs`, lines 91–93; `Cargo.toml`

`Context::set_raw(true)` flips a process-wide `AtomicBool`.  
The doc comment warns:

> If tests mutate this flag they must run serially (e.g. with `#[serial_test::serial]`)

However, `serial_test` is **not listed in `[dev-dependencies]`** in `Cargo.toml`.
Any user who follows this guidance literally must add the dependency themselves.

Beyond testing, there is no RAII guard or scope-limited API (e.g. `Context::with_raw(|| {...})`),
so even production callers cannot safely toggle raw mode without risking a global state leak
if a panic or early return occurs.

---

## 5. Inconsistent `Result` handling in `ExchangeRates::set` doc examples

**File:** `src/exchange.rs`

The `ExchangeRates` struct-level doc example (line 224–227) and the `Exchange::convert` example
(lines 66–67) call `rates.set(...)` **without handling the returned `Result`**:

```rust
rates.set(EUR::CODE, dec!(0.8));   // Result ignored
rates.set(IDR::CODE, 17_000);      // Result ignored
```

But the `ExchangeRates::set` method-level doc example and the `set_pair` examples
(lines 305–309, 342–344) correctly use `.unwrap()`.  
The inconsistency teaches users that the `Result` can be ignored, which silently discards
potential `OverflowError`s for very large rate values.

---

## 6. `RawMoney` precision is silently lost when converted through `ObjMoney::convert`

**File:** `src/obj_money/raw_money_impl.rs`, lines 121–152

When a `RawMoney<C>` is boxed as `Box<dyn ObjMoney>` and `.convert()` is called, the result
is a `DynMoney` constructed via `DynMoney::new_with_code(to_code, result)`.  
`new_with_code` applies `helpers::amount_with_curr`, which **rounds to `minor_unit` unless
`Context::is_raw()` is `true`**.

Because `Context::is_raw()` defaults to `false`, converting raw money through the `ObjMoney`
interface always rounds — even if the user specifically chose `RawMoney` for full precision.
There is no indication in the doc that conversion through `ObjMoney` loses raw semantics;
users who mix `Money` and `RawMoney` in a `Vec<Box<dyn ObjMoney>>` and then call `convert`
on each element will get inconsistent precision.

The same issue exists when calling `convert` on `Money<C>` via `ObjMoney` with
`Context::is_raw() == true`: the returned `DynMoney` would *skip* rounding, silently changing
the semantics of a rounded `Money`.

---

## 7. `BaseMoney::round_with` doc example does not demonstrate the chosen strategy

**File:** `src/base.rs`, lines 186–197

```rust
/// let money = Money::<USD>::new(dec!(123.456)).unwrap();
/// let rounded = money.round_with(2, RoundingStrategy::Floor);
/// assert_eq!(rounded.amount(), dec!(123.46));
```

`Money::<USD>::new(dec!(123.456))` already rounds `123.456` to `123.46` (banker's rounding)
on construction. The subsequent `round_with(2, RoundingStrategy::Floor)` has no observable
effect. The example therefore does not actually demonstrate `Floor` rounding; it would be
identical with `BankersRounding`, `HalfUp`, or any other strategy.

A more useful example would use `raw!` or start from an amount that lies strictly between
two representable values, such as `dec!(123.455)`, to make the floor strategy observable.

---

## 8. `Debug` for `DynMoney` embeds global `Context::is_raw()` state

**File:** `src/obj_money/dyn_money.rs`, lines 704–712

```rust
impl Debug for DynMoney {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynMoney({}, {}, is_raw: {})", self.code(), self.amount, super::Context::is_raw())
    }
}
```

The debug representation of **any** `DynMoney` value changes depending on the global raw flag.
Two identical `DynMoney` values serialised to `Debug` at different points in the program
may produce different output. This is surprising and makes debug logs unreliable as audit
records. The `is_raw` flag reflects a global operational mode, not a property of the value.

---

## 9. No negation for `Box<dyn ObjMoney>` / `&dyn ObjMoney`

**File:** `src/obj_money/ops.rs`

`std::ops::Neg` is implemented only for `DynMoney` directly.  
Users who hold a `Box<dyn ObjMoney>` (the common heterogeneous collection type) cannot
negate it without downcasting:

```rust
let m: Box<dyn ObjMoney> = ...;
let neg = -m; // compile error
```

This is inconsistent with `Money<C>` and `RawMoney<C>`, which both support unary negation.
Adding `Neg` for `Box<dyn ObjMoney>` (e.g. returning `Box<dyn ObjMoney>`) or adding a
`fn neg(&self) -> Box<dyn ObjMoney>` method to the `ObjMoney` trait would close this gap.

---

## 10. `Context::register_currency` error message references `currencylib::get_data` for custom currencies

**File:** `src/obj_money/context.rs`, lines 114–133

```rust
return Err(MoneyError::ObjMoneyError(
    format!(
        "Currency with code {} already exist: {:?}",
        C::CODE,
        get_data(C::CODE)  // returns None for custom codes
    ).into(),
));
```

When a user attempts to register a *custom* currency that is already in the registry,
`get_data(C::CODE)` returns `None`, producing the message:

```
Currency with code XYZ already exist: None
```

The trailing `None` is confusing (it implies the currency doesn't exist when in fact it does).
The `get_data` call adds no useful information for custom currencies.

---

## 11. `ExchangeRates::is_empty` documentation contradicts implementation intent

**File:** `src/exchange.rs`, lines 466–471

```rust
/// Check if rates is empty.
///
/// Rates is atleast contains base rate, so this always return false.
pub fn is_empty(&self) -> bool {
    self.rates.is_empty()
}
```

The doc says "this always return false", but the method genuinely delegates to
`HashMap::is_empty()`. While in practice `ExchangeRates::new()` and `From<I>` always
insert the base rate, the guarantee is not enforced by the type system.  
This `is_empty` method risks being a source of confusion — Clippy will also flag a `len()`
method without `is_empty()`, but when `is_empty` is documented as always returning `false`,
the method is arguably useless and should either be removed or correctly documented.

---

## 12. `ObjMoney` doc example uses `raw!` which requires the `raw_money` feature

**File:** `src/obj_money/obj_money.rs`, lines 35–47

The `ObjMoney` trait-level doc example:

```rust
use moneylib::{Money, raw, obj_money::ObjMoney, Decimal, BaseMoney, macros::dec, iso::{USD, EUR, JPY}};

let portfolio: Vec<Box<dyn ObjMoney>> = vec![
    ...
    Box::new(raw!(BHD, 8392.098)),
    Box::new(raw!(CAD, 6942.6942)),
];
```

uses the `raw!` macro, which is only compiled when `raw_money` is enabled. A user who
enables only `obj_money` (without `raw_money`) will see a compilation error in this doc
example if it is executed directly. The example should either be guarded with
`#[cfg(all(feature = "obj_money", feature = "raw_money"))]` or rewritten to avoid `raw!`.

---

## 13. `DynCurrency` public API only exposes `code()`

**File:** `src/obj_money/dyn_money.rs`, lines 112–127

`DynCurrency` has eleven fields but only `code()` is a public method. All other fields
(`symbol`, `name`, `numeric`, `minor_unit`, etc.) are `pub(super)`.  
A user who receives a `DynCurrency` (e.g. from `Context::get_currency`) cannot read any
metadata other than the code without going through a `DynMoney`. This makes `DynCurrency`
less useful as a standalone type and forces unnecessary allocation.

---

## 14. `format_obj_money` uses `is_sign_negative()` which is true for negative zero

**File:** `src/obj_money/fmt.rs`, line 114

```rust
let is_negative = amount.is_sign_negative();
```

`Decimal::is_sign_negative()` returns `true` for `-0`, meaning a `DynMoney` holding `-0.00`
would be formatted as `"USD -0.00"`. The static counterpart in `src/fmt.rs` uses
`money.is_negative()` (which correctly returns `false` for zero), so the two code paths
are inconsistent. A user comparing `format_code()` results from a `DynMoney` and a
`Money<C>` with value `-0` would see different strings.

---

## 15. `ObjMoney` methods returning `Box<dyn ObjMoney>` make chaining on `DynMoney` awkward

**File:** `src/obj_money/obj_money.rs`, `src/obj_money/dyn_money.rs`

All value-producing methods on the `ObjMoney` trait (`abs`, `round`, `round_with`,
`truncate`, `truncate_with`, `checked_*`) return `Box<dyn ObjMoney>`.  
For `DynMoney` — which is a `Copy` value type — these operations allocate a heap box for
every intermediate result. Code like:

```rust
let m: DynMoney = ...;
let result = m.abs().round();  // two heap allocations
```

loses the `Copy` benefit. `DynMoney` could override these methods to return `Self` (or
expose additional inherent methods that return `DynMoney`), reserving the
`Box<dyn ObjMoney>` return only for the trait-object path.

---

## 16. `percent_adds_fixed` implementation rebuilds amount through `new()`, losing custom rounding context

**File:** `src/percent_ops.rs`, lines 202–212

```rust
fn percent_adds_fixed<D, I>(&self, pcns: I) -> Option<Self::Output>
...
{
    let mut result = self.amount();
    for pcn in pcns.into_iter() {
        result = result.checked_add(self.percent(pcn.get_decimal()?)?.amount())?;
    }
    Self::Output::new(result).ok()   // silently returns None on any error
}
```

The final `Self::Output::new(result).ok()` discards the error if `new()` fails.  
Additionally, this calls `new()` (which round-trips through `DecimalNumber`) rather than
`from_decimal` (direct). For a `RawMoney`, this means the result passes through `new` which
calls `from_decimal` — consistent — but the error context is lost. A `?` propagation via
`Option` is acceptable here, but the `.ok()` silently swallows the distinction between
"no error, zero result" and "error during construction".

---

## 17. `ExchangeRates` lifetime parameter `'a` on string keys creates friction

**File:** `src/exchange.rs`, struct `ExchangeRates<'a, Base>`

`ExchangeRates` stores rate keys as `&'a str`. This ties the exchange-rates map to the
lifetime of the string sources. Because currency codes are typically `'static` (they come
from `C::CODE`), the common usage is always `ExchangeRates<'static, _>`, but the lifetime
annotation must appear explicitly everywhere:

```rust
let rates: ExchangeRates<'static, USD> = ExchangeRates::new();
// or more commonly, inferred but verbose in function signatures
```

If dynamic string keys are never needed in practice (all codes are `'static`), the lifetime
parameter could be removed by storing `&'static str` keys, simplifying the API significantly.

---

## 18. `RawMoney::into_raw()` doc example has a misleading precision result

**File:** `src/raw_money/money_ext.rs`, lines 26–31

```rust
/// let result = raw * (dec!(1) / dec!(3));
/// // Convert back when ready to round
/// let final_money = result.finish();
/// assert_eq!(final_money.amount(), dec!(33.50));
```

`dec!(1) / dec!(3)` in Rust Decimal yields approximately `0.3333333333333333333333333333`
(28 significant digits). Multiplying `100.50` by that gives approximately
`33.50000000000000000000000000`. Rounded to USD minor unit (2 dp) with banker's rounding,
the result is `33.50`. The example is numerically correct but somewhat misleads by using
a "nice" division; a more instructive example would demonstrate a case where raw arithmetic
diverges from direct rounded arithmetic.

---

## 19. `ExchangeRates::From<I>` skips the base-currency guard silently

**File:** `src/exchange.rs`, lines 474–495

```rust
impl<'a, I, Base: Currency> From<I> for ExchangeRates<'a, Base>
...
{
    fn from(value: I) -> Self {
        ...
        for (k, v) in value {
            if k != Base::CODE {
                rates.insert(k, v);
            }
        }
        ...
    }
}
```

This `From` impl accepts `Item = (&'a str, Decimal)`. Values are inserted directly without
the `DecimalNumber::get_decimal()` overflow guard that `set()` applies. In practice,
`Decimal` values are already validated, so no overflow can arise. However, the doc comment
says "If some of the sets failed, will be skipped" — this comment is incorrect; no set can
fail here since there is no validation at all. The misleading comment may cause confusion.

---

## 20. `serde_yaml = "0.9"` dev-dependency is deprecated / abandoned

**File:** `Cargo.toml`, line 44

`serde_yaml` 0.9 is effectively unmaintained. While it is only a dev-dependency and does not
affect library consumers, it may cause CI breakage as the ecosystem evolves. Consider
replacing it with an actively maintained alternative such as `serde_yml` or `serde-yaml2`.

---

## 21. Operator overloads missing `Div<$T<C>> for Decimal` (asymmetric arithmetic)

**File:** `src/ops.rs`

The `impl_money_ops!` macro implements `d + M` and `d * M` (decimal left-hand-side) for
commutativity, but does **not** implement `d - M` or `d / M`.  
The omission is consistent with mathematics (`d / M` is not a Money value), but `d - M`
gives a negated result that some financial calculations legitimately use (e.g.
`budget - spent = remaining` where `budget` is a plain scalar). The missing impl is
undocumented, so users who try `dec!(500) - money!(USD, 100)` will get a confusing compile
error.

---

## 22. `MoneyError` variants are feature-gated, complicating exhaustive matching

**File:** `src/error.rs`

`MoneyError` is `#[non_exhaustive]` (correct) but also has variants that are conditionally
compiled:

```rust
#[cfg(feature = "locale")]
ParseLocale(ErrVal),
#[cfg(feature = "exchange")]
ExchangeError(ErrVal),
#[cfg(feature = "obj_money")]
ObjMoneyError(ErrVal),
```

A consumer who writes `match err { MoneyError::ParseStrError(_) => ... }` and
compiles with `--all-features` sees different match arms than with minimal features.
The `#[non_exhaustive]` attribute does not fully protect against this because the set of
arms itself changes. Error types that add or remove variants based on features make
`match` expressions fragile across feature configurations.

---

## 23. Missing `serial_test` in `Cargo.toml` despite docs recommending it

**File:** `Cargo.toml` / `src/obj_money/context.rs`, line 77

The `Context::set_raw` documentation explicitly says:

> If tests mutate this flag they must run serially (e.g. with `#[serial_test::serial]`)

`serial_test` is **not present in `[dev-dependencies]`**. The advice in the docs is
actionable only after the dependency is added. The library should either add it as a
dev-dep and demonstrate its use in the test for `set_raw`, or remove the recommendation
and replace it with a note about running tests with `-- --test-threads=1`.

---

## 24. `DynCurrency::from_code` error message leaks internal representation

**File:** `src/obj_money/dyn_money.rs`, lines 100–103

```rust
Err(MoneyError::ObjMoneyError(
    format!("currency {} not found", code).into(),
))
```

This is fine. However, the symmetric helper `DynMoney::new_with_code` and
`DynMoney::set_curr_from_code` produce the same message. There is no single canonical
"currency not found" error, so multiple call sites produce slightly different strings
(`"currency XYZ not found"` in some places). A centralised `MoneyError::CurrencyNotFound`
variant (or a shared constructor) would make error matching easier for library consumers.

---

## Summary Table

| # | Severity | Category | Issue |
|---|----------|----------|-------|
| 1 | **High** | API usability | `money!`/`raw!` macros require `BaseMoney` in scope |
| 2 | **High** | Runtime safety | Arithmetic operators panic on overflow without doc warning |
| 3 | **Medium** | API usability | `ObjIterOps::checked_sum` unavailable without `exchange` feature |
| 4 | **Medium** | Concurrency | `Context::set_raw` global state, no scoped API, `serial_test` missing |
| 5 | **Medium** | Documentation | `ExchangeRates::set` result silently ignored in doc examples |
| 6 | **Medium** | Correctness | `RawMoney` precision lost through `ObjMoney::convert` |
| 7 | **Low** | Documentation | `round_with` Floor example doesn't demonstrate Floor |
| 8 | **Low** | Debugging | `DynMoney::Debug` embeds mutable global state |
| 9 | **Low** | API usability | No `Neg` for `Box<dyn ObjMoney>` |
| 10 | **Low** | UX | `register_currency` error message shows `None` for custom currencies |
| 11 | **Low** | Documentation | `is_empty` claims "always false" but could theoretically return `true` |
| 12 | **Low** | Documentation | `ObjMoney` doc example requires `raw_money` feature but no guard |
| 13 | **Low** | API usability | `DynCurrency` only exposes `code()` publicly |
| 14 | **Low** | Correctness | `format_obj_money` uses `is_sign_negative` (true for `-0`), inconsistent with static path |
| 15 | **Low** | Performance | `DynMoney` trait methods allocate `Box` even though `DynMoney` is `Copy` |
| 16 | **Low** | Correctness | `percent_adds_fixed` uses `new().ok()`, discards error information |
| 17 | **Low** | API ergonomics | `ExchangeRates<'a, _>` lifetime parameter unnecessary when codes are always `'static` |
| 18 | **Low** | Documentation | `into_raw` / `finish` doc example misleads about raw arithmetic |
| 19 | **Low** | Documentation | `ExchangeRates::From` doc comment says "skipped on failure" but no failure is possible |
| 20 | **Low** | Maintenance | `serde_yaml = "0.9"` is deprecated |
| 21 | **Low** | API usability | `d - M` not implemented (no `Sub<Money<C>> for Decimal`) |
| 22 | **Low** | API usability | Feature-gated `MoneyError` variants complicate `match` across configurations |
| 23 | **Low** | Documentation | `serial_test` recommended in docs but not in `Cargo.toml` |
| 24 | **Low** | UX | Multiple duplicated "currency not found" error messages, no unified variant |
