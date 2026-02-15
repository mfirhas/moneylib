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
- Type-safe: prevents invalid state and follow monetary standard.
- Supports ISO 4217 currencies.
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

## Example

```rust
use moneylib::{Money, BaseMoney, BaseOps, Currency, RoundingStrategy, money_macros::dec};
use std::str::FromStr;

// Creating money from string (supports thousand separators)
let usd_money = Money::from_str("USD 1,234.56").unwrap();
println!("{}", usd_money); // USD 1,234.56

// Creating money from minor amount (cents for USD)
let from_cents = Money::from_minor_amount(
    Currency::from_iso("USD").unwrap(), 
    12345
).unwrap();
println!("{}", from_cents); // USD 123.45

// Arithmetic operations with automatic rounding
let money_a = Money::new(Currency::from_iso("USD").unwrap(), dec!(100.00));
let money_b = Money::new(Currency::from_iso("USD").unwrap(), dec!(50.00));
println!("{}", money_a + money_b); // USD 150.00
println!("{}", money_a * dec!(1.5)); // USD 150.00
println!("{}", money_a / dec!(3)); // USD 33.33 (rounded)

// Comparisons
println!("{}", money_a > money_b); // true
println!("{}", money_a == Money::new(Currency::from_iso("USD").unwrap(), dec!(100.00))); // true

// Working with different currencies
let jpy = Currency::from_iso("JPY").unwrap(); // 0 decimal places
let jpy_money = Money::new(jpy, dec!(1000));
println!("{}", jpy_money); // JPY 1,000

let bhd = Currency::from_iso("BHD").unwrap(); // 3 decimal places
let bhd_money = Money::new(bhd, dec!(12.345));
println!("{}", bhd_money); // BHD 12.345

// Custom formatting
let money = Money::new(Currency::from_iso("USD").unwrap(), dec!(1234.56));
println!("{}", money.format_symbol()); // $1,234.56
println!("{}", money.format_code()); // USD 1,234.56

// Rounding strategies
let mut currency = Currency::from_iso("USD").unwrap();
currency.set_rounding_strategy(RoundingStrategy::HalfUp);
let rounded = Money::new(currency, dec!(123.456));
println!("{}", rounded.amount()); // 123.46

// Custom currencies (e.g., cryptocurrencies)
let btc = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
let btc_money = Money::new(btc, dec!(0.12345678));
println!("{}", btc_money); // BTC 0.12345678

// Negative amounts
let negative = Money::new(Currency::from_iso("USD").unwrap(), dec!(-50.00));
println!("{}", negative); // USD -50.00
println!("{}", negative.abs()); // USD 50.00

// Error handling with Result types
match money_a.add(money_b) {
    Ok(sum) => println!("Sum: {}", sum),
    Err(e) => println!("Error: {:?}", e),
}

// Safe operations with different currencies
let eur = Currency::from_iso("EUR").unwrap();
let eur_money = Money::new(eur, dec!(100.00));
match money_a.add(eur_money) {
    Ok(_) => println!("Addition succeeded"),
    Err(e) => println!("Error: {:?}", e), // Error: CurrencyMismatch
}
```

For more examples check inside `examples/` directory.

## Components
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

## Feature flags

### Raw Money(`raw_money`)
This feature flag toggle `RawMoney` type which doesn't do rounding like `Money` type does.
It keeps the precision intact and you can choose when to round it using `.round()` or `.round_with(...)`.
Use method `.finish()` to round it into final form and convert it back to `Money`.
Method `.finish()` will call `.round()` inside and round using currency's rounding rule.
Method `.minor_amount()` will NOT do the rounding on RawMoney, but calculate the minor amount from rounded value of RawMoney's amount.

Where roundings happen:
- `.round()`: round using currency's rounding strategy. Returns `RawMoney`.
- `.round_with(...)`: round using custom decimal points and strategy. Returns `RawMoney`.
- `.finish()`: round using currency's rounding strategy and finalize raw money operations. Returns `Money`.

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

