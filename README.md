# moneylib

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![Crates.io](https://img.shields.io/crates/v/moneylib.svg)](https://crates.io/crates/moneylib)
[![Documentation](https://docs.rs/moneylib/badge.svg)](https://docs.rs/moneylib)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/mfirhas/moneylib/blob/master/LICENSE)

A library to deal with money safely using floating-point fixed-precision decimal.

## Overview

`moneylib` provides a safe, robust, and ergonomic way to work with monetary value in Rust.
It handles currency and amount with operations and arithmetics avoiding floating, rounding, and precision issue exist in typical binary floating-point type. 
It also make sure the money always in valid state on every operations and arithmetics done on it avoiding overflow/truncation/wrap and without fractions.

This crate uses [Decimal](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html) type underneath for the amount of money. Using it alone is not enough for many money operations and this crate
also provides `Currency` storing metadata about the money that involves in logics and state of the money. 

## Features
Here are some features supported:
- Type-safe: prevents invalid state and follow monetary standard with compile-time currency checking.
- Value type to represent money.
- Access to its amount and currency's metadata.
- Arithmetics: (*,/,+,-), operator overloading supported.
- Comparisons: (>,<,>=,<=,==,!=), operator overloading supported.
- Negative money.
- Formatting and custom formatting.
- Rounding with multiple strategies: Bankers rounding, half-up, half-down, ceil, and floor.
- Money in form of its smallest amount (minor amount).
- Some basic operations like absolute value, min, max, and clamp.
- Support for all ISO 4217 currencies.

## Example

```rust
use moneylib::{Money, BaseMoney, BaseOps, CustomMoney, RoundingStrategy, USD, JPY, BHD, EUR, money_macros::dec};
use std::str::FromStr;

// Creating money from string (supports thousand separators)
let usd_money = Money::<USD>::from_str("USD 1,234.56").unwrap();
println!("{}", usd_money); // USD 1,234.56

// Creating money from minor amount (cents for USD)
let from_cents = Money::<USD>::from_minor(12345).unwrap();
println!("{}", from_cents); // USD 123.45

// Arithmetic operations with automatic rounding
let money_a = Money::<USD>::new(dec!(100.00)).unwrap();
let money_b = Money::<USD>::new(dec!(50.00)).unwrap();
println!("{}", money_a + money_b); // USD 150.00
println!("{}", money_a * dec!(1.5)); // USD 150.00
println!("{}", money_a / dec!(3)); // USD 33.33 (rounded)

// Comparisons
println!("{}", money_a > money_b); // true
println!("{}", money_a == Money::<USD>::new(dec!(100.00)).unwrap()); // true

// Working with different currencies
// JPY has 0 decimal places
let jpy_money = Money::<JPY>::new(dec!(1000)).unwrap();
println!("{}", jpy_money); // JPY 1,000

// BHD has 3 decimal places
let bhd_money = Money::<BHD>::new(dec!(12.345)).unwrap();
println!("{}", bhd_money); // BHD 12.345

// Custom formatting
let money = Money::<USD>::new(dec!(1234.56)).unwrap();
println!("{}", money.format_symbol()); // $1,234.56
println!("{}", money.format_code()); // USD 1,234.56

// Rounding with round_with method
let rounded = Money::<USD>::new(dec!(123.456)).unwrap();
let half_up_rounded = rounded.round_with(2, RoundingStrategy::HalfUp);
println!("{}", half_up_rounded.amount()); // 123.46

// Negative amounts
let negative = Money::<USD>::new(dec!(-50.00)).unwrap();
println!("{}", negative); // USD -50.00
println!("{}", negative.abs()); // USD 50.00

// Error handling with Result types
match money_a.add(money_b) {
    Ok(sum) => println!("Sum: {}", sum),
    Err(e) => println!("Error: {:?}", e),
}

// Safe operations with different currencies (won't compile due to type safety)
let eur_money = Money::<EUR>::new(dec!(100.00)).unwrap();
// This won't compile because USD and EUR are different types:
// let result = money_a + eur_money; // Compile error!
```

## Components
This library provides these main components to work with money:
- `Money<C>`: represents the money itself and all operations on it. Generic over currency type `C`.
- `Currency`: trait that defines currency behavior and metadata. Implemented by currency marker types (e.g., `USD`, `EUR`, `JPY`).
- `Decimal`: 128 bit floating-point with fixed-precision decimal number. Re-export from [rust_decimal](https://crates.io/crates/rust_decimal) represents main type for money's amount.
- `BaseMoney`: trait of money providing core operations and accessors.
- `BaseOps`: trait for arithmetic and comparison operations on money.
- `CustomMoney`: trait for custom formatting and rounding operations on money.
- `RoundingStrategy`: enum defining rounding strategies (BankersRounding, HalfUp, HalfDown, Ceil, Floor).
- `MoneyError`: enum of possible errors that can occur in money operations.
- `MoneyResult<T>`: Result type alias for operations that can fail, equivalent to `Result<T, MoneyError>`.

`Money<C>` and `Decimal` are `Copy` types so they can be passed around freely without having to worry about borrow checker.
Currency marker types are zero-sized types (ZST) for compile-time type safety.

## Invariants
Monetary values are sensitive matter and their invariants must always hold true.

### Decimal
- Significand(m): -2^96 < m < 2^96
- Decimal points(s): 0 <= s <= 28

### Money
- Always rounded to its currency's minor unit using bankers rounding after each creation and operation done on it.
- Creating money from string only accepts currencies already defined in ISO 4217.
- Comparisons: Currency type safety is enforced at compile time. Operations between different currencies won't compile.
- Arithmetics:
  - *,+,-: will *PANIC* if overflowed. Currency mismatches are prevented at compile time.
  - /: will *PANIC* if overflowed or division by zero. Currency mismatches are prevented at compile time.
  - Use methods in `BaseOps` for non-panic arithmetics.

### Currency
- Currency types are defined at compile time using marker types (e.g., `USD`, `EUR`, `JPY`).
- All ISO 4217 currencies are supported via the `Currency` trait.
- Currency information is available through trait methods: `code()`, `symbol()`, `name()`, `minor_unit()`.

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

