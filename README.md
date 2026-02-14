# moneylib

A library to deal with money using floating-point fixed-precision decimal.

## Overview

`moneylib` provides a safe, robust, and ergonomic way to work with monetary value in Rust.
It handles currency and amount with operations and arithmetics avoiding floating, rounding, and precision issue exist in typical binary floating-point type. 
It also make sure the money always in valid state on every operations and arithmetics done on it avoiding overflow/truncation/wrap and without fractions.

This crate uses [Decimal](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html) type underneath for the amount of money. Using it alone is not enough for many money operations and this crate
also provides `Currency` storing metadata about the money that involves in logics and state of the money. 

This library provides these main components to work with money:
- `Money`: represents the money itself and all operations on it.
- `Currency`: represents the money's currency and all of its metadata. It is involved in money's states and lifecycles throughout its operations.
- `Decimal`: 128 bit floating-point with fixed-precision decimal number. Re-export from [rust_decimal](https://crates.io/crates/rust_decimal) represents main type for money's amount.
- `BaseMoney`: trait of money providing core operations and accessors.
- `BaseOps`: trait for arithmetic and comparison operations on money.
- `CustomMoney`: trait for custom formatting and rounding operations on money.
- `RoundingStrategy`: enum defining rounding strategies (BankersRounding, HalfUp, HalfDown, Ceil, Floor).
- `MoneyError`: enum of possible errors that can occur in money operations.
- `MoneyResult<T>`: Result type alias for operations that can fail, equivalent to `Result<T, MoneyError>`.
- `Country`: re-export from iso_currency_lib for country information.

`Money`, `Currency`, and `Decimal` are all `Copy` so it can be passed around freely without having to worry about borrow checker.

Example:
```rust
use moneylib::{Money, BaseMoney, BaseOps, Currency, money_macros::dec};
use std::str::FromStr;

let usd = Money::from_str("USD 12,000").unwrap();
let add = usd + dec!(500);
println!("{}", add); // prints "USD 12,500.00"
```

## Features
Here are some features supported:
- Type-safe: prevents invalid state and follow monetary standard.
- Value type to represent money.
- Access to its amount and currency's metadata.
- Arithmetics: (*,/,+,-), operator overloading supported.
- Comparisons: (>,<,>=,<=,==,!=), operator overloading supported.
- Negative money.
- Formatting and custom formatting.
- Rounding with multiple strategies: Bankers rounding, half-up, half-down, ceil, and floor.
- Money in form of its smallest amount (minor amount).
- Some basic operations like absolute value, min, max, and clamp.
- Custom currency.

## Invariants
Monetary values are sensitive matter and their invariants must always hold true.

### Decimal
- Significand(m): -2^96 < m < 2^96
- Decimal points(s): 0 <= s <= 28

### Money
- Always rounded to its currency's minor unit using currency's rounding strategy (default to bankers rounding) after each creation and operation done on it.
- Creating money from string only accepts currencies already defined in ISO 4217. For new/custom currencies, create new currency using `Currency::new` function.
- Comparisons: Equality only for money with same currencies. For ordering equality will *PANIC* if currencies are different. Use methods in `BaseOps` for non-panic comparisons.
- Arithmetics:
  - *,+,-: will *PANIC* if: Overflowed, or currencies are different.
  - /: will *PANIC* if: Overflowed, division by zero, or currencies are different.
  - Use methods in `BaseOps` for non-panic arithmetics.

### Currency
- Creation from string accepts ISO 4217 alphabetical code, case insensitive. E.g. USD, usd, uSd, IDR.
- Comparisons and hash are on currency's alphabetical code.

This library maintains type-safety by preventing invalid state either by returning Result(`MoneyResult`) or going *PANIC*.

## Code Coverage

This library maintains excellent code coverage.

To see current coverage results, run the coverage command below.

### Running Code Coverage

To measure code coverage, you need to install `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
```

Then run coverage with:

```bash
cargo tarpaulin --out Stdout --out Html
```

This will:
- Run all tests with coverage instrumentation
- Display coverage results in the terminal
- Generate an HTML report (`tarpaulin-report.html`) for detailed visualization

