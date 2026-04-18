# Monetary Libraries Comparison

A comparison of `moneylib` (Rust) with popular monetary libraries across Rust, Go, Java, C#, Ruby, Python, and JavaScript/TypeScript.

## `moneylib` — Summary of Key Features

| Feature | Details |
|---|---|
| **Language** | Rust |
| **Underlying type** | `rust_decimal` (128-bit fixed-precision decimal) |
| **Type safety** | Compile-time currency type parameter `Money<C>` — mixing currencies won't compile |
| **Money types** | `Money<C>` (auto-rounded) + `RawMoney<C>` (full precision, opt-in) |
| **ISO 4217** | All currencies supported via `currencylib` |
| **Custom currencies** | Yes, implement the `Currency` trait |
| **Rounding** | 5 strategies: BankersRounding, HalfUp, HalfDown, Ceil, Floor |
| **Arithmetic** | `+`, `-`, `*`, `/` with operator overloading; checked (non-panicking) variants |
| **Comparisons** | Operator overloading (`>`, `<`, `==`, etc.); compile-time currency match |
| **Formatting** | Code format (`USD 1,234.56`), symbol format (`$1,234.56`), locale-aware (ICU) |
| **Serde** | Full serialize/deserialize with multiple format modules |
| **Exchange rates** | Built-in `Exchange` trait + `ExchangeRates` map |
| **Split/Allocate** | `split(n)` with remainder, `allocate(&[ratios])` |
| **Percentage ops** | `percent()`, `percent_add()`, `percent_sub()` |
| **Iterator ops** | `checked_sum()`, `mean()`, `median()`, `mode()` |
| **Accounting** | Interest calculations (feature flag) |
| **Negative money** | Supported, with `abs()` |
| **Minor units** | `from_minor()` / `to_minor()` |
| **Copy semantics** | `Money<C>` is `Copy` — no borrow-checker friction |
| **Safety** | `#![forbid(unsafe_code)]`, no float arithmetic, no casts |
| **License** | MIT |

---

## Comparable Libraries by Language

### 1. Rust — [`rusty-money`](https://crates.io/crates/rusty-money) ⭐~200

| Feature | `moneylib` | `rusty-money` |
|---|---|---|
| Decimal backend | `rust_decimal` (128-bit) | `rust_decimal` (128-bit) |
| Compile-time currency safety | ✅ Generic `Money<C>` — mismatches are compile errors | ❌ Runtime currency checks only (`Money` is not generic over currency) |
| Money types | `Money<C>` + `RawMoney<C>` | Single `Money` type |
| Rounding strategies | 5 (Bankers, HalfUp, HalfDown, Ceil, Floor) | Bankers rounding only (auto) |
| Operator overloading | ✅ Full (`+`, `-`, `*`, `/`, comparisons) | ✅ Partial |
| Checked arithmetic | ✅ `checked_add`, `checked_sub`, etc. | ❌ Not exposed |
| ISO 4217 currencies | All | All |
| Custom currencies | ✅ Implement `Currency` trait | ✅ Possible |
| Serde | ✅ Multiple format modules | ✅ Basic |
| Exchange rates | ✅ Built-in trait | ❌ Not included |
| Split/Allocate | ✅ | ❌ |
| Percentage ops | ✅ | ❌ |
| Locale formatting | ✅ ICU-based | ❌ |
| Accounting ops | ✅ (interest calculations) | ❌ |
| Iterator ops | ✅ (sum, mean, median, mode) | ❌ |
| Maintenance | Active (v0.12, 2024–2025) | Inactive (last release 2021) |

**Summary**: `moneylib` is significantly more feature-rich and actively maintained. The biggest differentiator is **compile-time currency type safety** — `rusty-money` uses runtime checks. `moneylib` also provides exchange rates, allocation, percentages, accounting, and locale formatting that `rusty-money` lacks entirely.

---

### 2. Go — [`go-money`](https://github.com/Rhymond/go-money) ⭐~1.6k + [`govalues/money`](https://github.com/govalues/money) ⭐~51

| Feature | `moneylib` | `go-money` | `govalues/money` |
|---|---|---|---|
| Internal representation | 128-bit decimal | `int64` (minor units / cents) | Custom decimal type |
| Currency safety | Compile-time (generics) | Runtime only (returns error) | Runtime only |
| Rounding strategies | 5 strategies | N/A (integer cents, no rounding needed) | Half-to-even |
| Arithmetic | `+`, `-`, `*`, `/` with operators | `Add()`, `Subtract()`, `Multiply()` (no division) | Full arithmetic |
| Split/Allocate | ✅ | ✅ (round-robin remainder distribution) | ✅ |
| Formatting | Code, symbol, locale | `Display()` | `Display()` |
| Exchange rates | ✅ Built-in | ❌ | ✅ Built-in |
| Serde | ✅ | JSON marshaling | N/A |
| Percentage ops | ✅ | ❌ | ❌ |
| ISO 4217 | All | All | All |
| Custom currencies | ✅ | ✅ | ❌ |

**Summary**: `go-money` is the most popular Go money library and follows Martin Fowler's Money pattern using integer cents. This avoids floating-point issues but limits precision (no sub-cent amounts). `moneylib` uses true 128-bit decimals, supports compile-time currency safety (impossible in Go until recently), and has far more operations (percentages, interest, locale formatting). `govalues/money` is newer and uses proper decimal types similar to `moneylib`.

---

### 3. Java — [`Joda-Money`](https://github.com/JodaOrg/joda-money) ⭐~678 + [`JavaMoney/Moneta` (JSR 354)](https://github.com/JavaMoney/jsr354-ri) ⭐~370

| Feature | `moneylib` | `Joda-Money` | `Moneta (JSR 354)` |
|---|---|---|---|
| Underlying type | `rust_decimal` 128-bit | `BigDecimal` | `BigDecimal` / `long` |
| Currency safety | Compile-time generics | Runtime (`CurrencyMismatchException`) | Runtime |
| Rounding | 5 strategies | Java `RoundingMode` (8 modes) | Java `RoundingMode` |
| Immutability | ✅ (Copy type) | ✅ | ✅ |
| Arithmetic | Operator overloading | `plus()`, `minus()`, `multipliedBy()` etc. | `add()`, `subtract()`, `multiply()`, `divide()` |
| ISO 4217 | All | All | All |
| Custom currencies | ✅ | ❌ (ISO only) | ✅ |
| Exchange rates | ✅ Built-in | ❌ | ✅ `ExchangeRateProvider` |
| Formatting | Code, symbol, locale | `toString()`, formatting via `MoneyFormatterBuilder` | `MonetaryAmountFormat` |
| Split/Allocate | ✅ | ❌ | ❌ (via extensions) |
| Percentage ops | ✅ | ❌ | ❌ |
| Serde | ✅ | ❌ (Java serialization) | ❌ |
| Standard | Crate | De facto standard | JSR 354 (Java standard) |

**Summary**: Java has the most mature monetary ecosystem. `Moneta` is the reference implementation of the **JSR 354 Money & Currency API** (a Java standard). `Joda-Money` is battle-tested and widely used. Both use `BigDecimal` for precision. `moneylib`'s advantage is **compile-time currency safety** (Java relies on runtime exceptions). Java has more rounding modes (8 vs 5). `moneylib` has more built-in convenience operations (split, allocate, percentages, accounting).

---

### 4. C# — [`NodaMoney`](https://github.com/RemyDuijkeren/NodaMoney) ⭐~252

| Feature | `moneylib` | `NodaMoney` |
|---|---|---|
| Underlying type | `rust_decimal` 128-bit | `decimal` (128-bit, native C#) |
| Currency safety | Compile-time generics | Runtime (`InvalidCurrencyException`) |
| Rounding | 5 strategies | .NET `MidpointRounding` |
| Immutability | ✅ (Copy) | ✅ (struct) |
| Operator overloading | ✅ | ✅ (`+`, `-`, `*`, `/`, comparisons) |
| ISO 4217 | All | All |
| Custom currencies | ✅ | ✅ |
| Exchange rates | ✅ | ✅ `ExchangeRate` class |
| Formatting | Code, symbol, locale | `ToString()` with `IFormatProvider`, culture-aware |
| Split/Allocate | ✅ | ✅ `SafeAllocate` |
| Percentage ops | ✅ | ❌ |
| Serde | ✅ | JSON.NET support |

**Summary**: `NodaMoney` is the closest library in philosophy to `moneylib` — both use 128-bit decimal types, support operator overloading, allocation, and exchange rates. The key difference is again **compile-time vs runtime currency safety**. C#'s native `decimal` type is analogous to `rust_decimal`. `moneylib` adds percentage and accounting operations that `NodaMoney` doesn't have.

---

### 5. Ruby — [`RubyMoney/money`](https://github.com/RubyMoney/money) ⭐~2,852

| Feature | `moneylib` | `RubyMoney/money` |
|---|---|---|
| Underlying type | 128-bit decimal | `BigDecimal` or integer cents (configurable) |
| Currency safety | Compile-time | Runtime (raises error on mismatch) |
| Rounding | 5 strategies | Multiple `Bank::*` rounding modes |
| Immutability | ✅ | Varies (mostly immutable) |
| Arithmetic | Operator overloading | Operator overloading (`+`, `-`, `*`, `/`) |
| ISO 4217 | All | All (via `money` gem data) |
| Custom currencies | ✅ | ✅ `Money::Currency.register` |
| Exchange rates | ✅ Built-in | ✅ Pluggable `Bank` backends (e.g., `money-open-exchange-rates`) |
| Formatting | Code, symbol, locale | `format()` with extensive options (symbol, delimiter, separator, etc.) |
| Split/Allocate | ✅ | ✅ `allocate([ratios])` |
| Percentage ops | ✅ | ❌ (manual `* 0.10`) |
| Serde | ✅ | JSON/YAML via `to_hash` |

**Summary**: `RubyMoney/money` is one of the oldest and most battle-tested monetary libraries (since 2008, 2.8k+ stars). It has a very flexible formatting system and pluggable exchange rate backends (banks). Ruby being dynamically typed, there's no compile-time safety. `moneylib` matches it on core features and adds compile-time safety, percentage operations, and accounting features. Ruby's library excels in its plugin ecosystem (e.g., `money-rails` for ActiveRecord integration).

---

### 6. Python — [`py-moneyed`](https://github.com/py-moneyed/py-moneyed) ⭐~472

| Feature | `moneylib` | `py-moneyed` |
|---|---|---|
| Underlying type | 128-bit decimal | Python `Decimal` (arbitrary precision) |
| Currency safety | Compile-time | Runtime (type checks) |
| Arithmetic | Operator overloading | Operator overloading |
| ISO 4217 | All | All |
| Custom currencies | ✅ | ✅ |
| Exchange rates | ✅ | ❌ |
| Formatting | Code, symbol, locale | Basic formatting; locale via Babel integration |
| Split/Allocate | ✅ | ❌ |
| Rounding strategies | 5 | Python `decimal` module strategies |
| Percentage ops | ✅ | ❌ |
| Serde | ✅ | ❌ (manual) |

**Summary**: `py-moneyed` is intentionally minimal — it provides `Money` and `Currency` classes and leaves everything else to Python's standard library (`decimal` module, `Babel` for locale formatting). `moneylib` is far more feature-complete with built-in exchange rates, allocation, percentages, and accounting. Python's `Decimal` offers arbitrary precision (vs `moneylib`'s 128-bit with 28 decimal digits), which may matter for extreme precision use cases.

---

### 7. JavaScript/TypeScript — [`dinero.js`](https://github.com/dinerojs/dinero.js) ⭐~6,723

| Feature | `moneylib` | `dinero.js` |
|---|---|---|
| Underlying type | 128-bit decimal | `number` or `bigint` (pluggable) |
| Currency safety | Compile-time generics | TypeScript type inference (partial compile-time) |
| Immutability | ✅ (Copy) | ✅ (functional, pure) |
| Arithmetic | Operator overloading | `add()`, `subtract()`, `multiply()`, etc. (functional API) |
| ISO 4217 | All | All (separate `@dinero.js/currencies`) |
| Custom currencies | ✅ | ✅ |
| Exchange rates | ✅ | ✅ `convert()` |
| Formatting | Code, symbol, locale | `toDecimal()`, `toFormat()` with Intl support |
| Split/Allocate | ✅ | ✅ `allocate([ratios])` |
| Rounding strategies | 5 | Half-to-even, half-up, half-down, etc. |
| Percentage ops | ✅ | ❌ (manual) |
| Non-decimal currencies | ❌ | ✅ (any base, e.g., MGA 1/5 subdivision) |
| Tree-shakeable | N/A (Rust) | ✅ (functional API) |
| Serde | ✅ | `toSnapshot()` / `dinero()` for serialization |

**Summary**: `dinero.js` is the most popular monetary library in the JS/TS ecosystem (6.7k stars). It uses a **functional, tree-shakeable API** (functions, not methods). TypeScript gives partial compile-time safety but not as strict as Rust generics — you can still mix currencies at compile time in some cases. `dinero.js` uniquely supports **non-decimal currencies** (e.g., Malagasy ariary with 1/5 subdivisions). `moneylib` has richer built-in operations (percentages, accounting, iterator ops).

---

## Overall Comparison Matrix

| Feature | moneylib (Rust) | rusty-money (Rust) | go-money (Go) | Joda-Money (Java) | Moneta JSR354 (Java) | NodaMoney (C#) | RubyMoney (Ruby) | py-moneyed (Python) | dinero.js (TS) |
|---|---|---|---|---|---|---|---|---|---|
| **⭐ Stars** | ~new | ~200 | ~1.6k | ~678 | ~370 | ~252 | ~2.8k | ~472 | ~6.7k |
| **Compile-time currency safety** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | Partial |
| **Decimal precision** | 128-bit (28 digits) | 128-bit | int64 (cents) | BigDecimal | BigDecimal | 128-bit | BigDecimal | Arbitrary | number/bigint |
| **Operator overloading** | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ |
| **Rounding strategies** | 5 | 1 | N/A | 8 | 8 | .NET modes | Multiple | Python modes | Multiple |
| **Exchange rates** | ✅ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ (plugins) | ❌ | ✅ |
| **Split/Allocate** | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| **Percentage ops** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Accounting/Interest** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Iterator ops (sum/mean/median)** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Locale formatting** | ✅ (ICU) | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ (Babel) | ✅ (Intl) |
| **Serde/Serialization** | ✅ (rich) | ✅ | JSON | Java Serializable | ❌ | JSON.NET | Hash | ❌ | Snapshot |
| **Custom currencies** | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Immutable/Copy** | ✅ Copy | ✅ | ✅ | ✅ | ✅ | ✅ struct | ~✅ | ✅ | ✅ |
| **No unsafe code** | ✅ `forbid(unsafe)` | ❌ | N/A | N/A | N/A | N/A | N/A | N/A | N/A |
| **Active maintenance** | ✅ | ❌ (stale) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## Key Takeaways

1. **`moneylib`'s unique strength** is **compile-time currency type safety** via Rust's generics (`Money<USD>` and `Money<EUR>` are different types). No other library in any language achieves this level of safety — all others rely on runtime checks.

2. **Feature completeness**: `moneylib` is arguably the most feature-complete single-package monetary library across all languages, with built-in percentage operations, accounting/interest, iterator helpers (mean, median, mode), and rich serde support that others lack.

3. **Closest competitors by feature parity**: `dinero.js` (JS/TS) and `RubyMoney/money` (Ruby) come closest in breadth of features but still lack percentage ops, accounting, and compile-time safety.

4. **Java has the most mature ecosystem**: JSR 354 (Moneta) is an actual language standard with the richest rounding and formatting options, but its API is more verbose and spread across multiple packages.

5. **Trade-offs**: `moneylib` is Rust-only (no WASM/FFI bindings yet), newer (smaller community/ecosystem), and panics on overflow by default for operator overloading (though safe variants exist). Libraries like `go-money` and `dinero.js` use errors/results exclusively for arithmetic.
