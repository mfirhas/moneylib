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

This crate uses [Decimal](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html) type underneath for the amount of money. 

## Features
Here are some features supported:
- Type-safe: 
  - Compile-time check for arithmetics and operations.
  - Runtime check for overflowed/wrapped/truncated amount.
  - Prevents currencies mixing at compile-time.
- Value type to represent money.
  - `Money`: represents money in amount rounded to the currency's minor unit.
  - `RawMoney`: represents money in raw amount keeping the precisions and choose when to round. 
- Access to its amount and currency's metadata.
- Arithmetics: (*,/,+,-), operator overloading supported.
- Comparisons: (>,<,>=,<=,==,!=), operator overloading supported.
- Negative money.
- Formatting and custom formatting.
- Rounding with multiple strategies: Bankers rounding, half-up, half-down, ceil, and floor.
- Money in form of its smallest amount (minor amount).
- Some basic operations like absolute value, min, max, and clamp.
- Support for all ISO 4217 currencies.
- New/custom currency by implementing `Currency` trait.
- Some accounting operations.(TODO)

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
- Comparisons: Currency type-safety is enforced at compile time. Operations between different currencies won't compile.
- Arithmetics:
  - *,+,-: will *PANIC* if overflowed. Currency mismatches are prevented at compile time.
  - /: will *PANIC* if overflowed or division by zero. Currency mismatches are prevented at compile time.
  - Use methods in `BaseOps` for non-panic arithmetics.

### Currency
- Currency types are defined at compile time using marker types (e.g., `USD`, `EUR`, `JPY`).
- All ISO 4217 currencies are supported via the `Currency` trait.
- Currency information is available through trait methods: `code()`, `symbol()`, `name()`, `minor_unit()`.
- New/custom currency is supported by implementing `Currency` trait.

This library maintains type-safety by preventing invalid state either by returning `Result` or going *PANIC*.

## Feature Flags

### `raw_money`

Enables the `RawMoney<C>` type which doesn't do automatic rounding like `Money<C>` does.
It keeps full decimal precision and lets callers decide when to round.

```toml
[dependencies]
moneylib = { version = "...", features = ["raw_money"] }
```

```rust
use moneylib::{BaseMoney, RawMoney, USD, Money, money_macros::dec};

// RawMoney preserves all decimal precision
let raw = RawMoney::<USD>::new(dec!(100.567)).unwrap();
assert_eq!(raw.amount(), dec!(100.567)); // Not rounded!

// Convert from Money using into_raw()
let money = Money::<USD>::new(dec!(100.50)).unwrap();
let raw = money.into_raw();

// Perform precise calculations
let result = raw * dec!(1.08875); // Apply tax

// Convert back to Money with rounding using finish()
let final_money = result.finish();
```

Where rounding happens:
- `.round()`: rounds to currency's minor unit using bankers rounding. Returns `RawMoney`.
- `.round_with(...)`: rounds using custom decimal points and strategy. Returns `RawMoney`.
- `.finish()`: rounds to currency's minor unit using bankers rounding back to `Money`.

### `serde`

Enables serialization and deserialization for Money/RawMoney(`raw_money`) types.
By default it will serialize/deserialize as numbers from numbers or from string numbers.
If you want to serialize/deserialize as string money format with code or symbol, you can use provided serde interface inside `serde` module:
- `moneylib::serde::money::comma_str_code`: Serialize into code format(e.g. "USD 1,234.56") with separators from currency's setting. Deserialize with code formatted with comma separated thousands.
- `moneylib::serde::money::option_comma_str_code`: Same as above, with nullability.
- `moneylib::serde::money::comma_str_symbol`: Serialize into symbol format(e.g. "$1,234.56") with separators from currency's setting. Deserialize with symbol formatted with comma separated thousands.
- `moneylib::serde::money::option_comma_str_symbol`: Same as above, with nullability.
- `moneylib::serde::money::dot_str_code`: Serialize into code format(e.g. "EUR 1.234,56") with separators from currency's setting. Deserialize with code formatted with dot separated thousands.
- `moneylib::serde::money::option_dot_str_code`: Same as above, with nullability.
- `moneylib::serde::money::dot_str_symbol`: Serialize into symbol format(e.g. "€1,234.56") with separators from currency's setting. Deserialize with symbol formatted with dot separated thousands.
- `moneylib::serde::money::option_dot_str_symbol`: Same as above, with nullability.

```toml
[dependencies]
moneylib = { version = "...", features = ["serde"] }
```
or serde for `RawMoney`:
```toml
[dependencies]
moneylib = { version = "...", features = ["serde", "raw_money"] }
```

```rust
use moneylib::{BaseMoney, Money, RawMoney, money_macros::dec};
use moneylib::{CAD, EUR, GBP, IDR, JPY, USD};

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
    struct All {
        amount_from_f64: Money<USD>,

        // `default` must be declared if you want to let users omit this field giving it money with zero amount.
        #[serde(default)]
        amount_from_f64_omit: Money<IDR>,

        // `default` must be declared if you want to let users omit this field giving it money with zero amount.
        #[serde(default)]
        amount_from_str_omit: Money<CAD>,

        amount_from_i64: Money<EUR>,

        amount_from_u64: Money<USD>,

        amount_from_i128: Money<USD>,

        amount_from_u128: Money<USD>,

        amount_from_str: Money<USD>,

        raw_amount_from_f64: RawMoney<USD>,

        raw_amount_from_str: RawMoney<USD>,

        #[serde(with = "moneylib::serde::money::comma_str_code")]
        amount_from_str_comma_code: Money<USD>,

        #[serde(with = "moneylib::serde::money::option_comma_str_code")]
        amount_from_str_comma_code_some: Option<Money<USD>>,

        #[serde(with = "moneylib::serde::money::option_comma_str_code")]
        amount_from_str_comma_code_none: Option<Money<USD>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "moneylib::serde::money::option_comma_str_code", default)]
        amount_from_str_comma_code_omit: Option<Money<USD>>,

        #[serde(with = "moneylib::serde::money::comma_str_symbol")]
        amount_from_str_comma_symbol: Money<USD>,

        #[serde(with = "moneylib::serde::money::option_comma_str_symbol")]
        amount_from_str_comma_symbol_some: Option<Money<USD>>,

        #[serde(with = "moneylib::serde::money::option_comma_str_symbol")]
        amount_from_str_comma_symbol_none: Option<Money<USD>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "moneylib::serde::money::option_comma_str_symbol", default)]
        amount_from_str_comma_symbol_omit: Option<Money<USD>>,

        #[serde(with = "moneylib::serde::raw_money::comma_str_code")]
        raw_amount_from_str_comma_code: RawMoney<USD>,

        // dot
        #[serde(with = "moneylib::serde::money::dot_str_code")]
        amount_from_str_dot_code: Money<EUR>,

        #[serde(with = "moneylib::serde::money::option_dot_str_code")]
        amount_from_str_dot_code_some: Option<Money<EUR>>,

        #[serde(with = "moneylib::serde::money::option_dot_str_code")]
        amount_from_str_dot_code_none: Option<Money<EUR>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "moneylib::serde::money::option_dot_str_code", default)]
        amount_from_str_dot_code_omit: Option<Money<EUR>>,

        #[serde(with = "moneylib::serde::money::dot_str_symbol")]
        amount_from_str_dot_symbol: Money<EUR>,

        #[serde(with = "moneylib::serde::money::option_dot_str_symbol")]
        amount_from_str_dot_symbol_some: Option<Money<EUR>>,

        #[serde(with = "moneylib::serde::money::option_dot_str_symbol")]
        amount_from_str_dot_symbol_none: Option<Money<EUR>>,

        // `default` must be declared if you want to let users omit this field making it `None`.
        #[serde(with = "moneylib::serde::money::option_dot_str_symbol", default)]
        amount_from_str_dot_symbol_omit: Option<Money<EUR>>,

        #[serde(with = "moneylib::serde::raw_money::dot_str_symbol")]
        raw_amount_from_str_dot_symbol: RawMoney<EUR>,
    }

    let json_str = r#"
        {
          "amount_from_f64": 1234.56988,
          "amount_from_i64": -1234,
          "amount_from_u64": 18446744073709551615,
          "amount_from_i128": -1844674407370955161588,
          "amount_from_u128": 34028236692093846346337,
          "amount_from_str": "1234.56",
          "raw_amount_from_f64": -1004.1234,
          "raw_amount_from_str": "1230.4993",
          "amount_from_str_comma_code": "USD 1,234.56",
          "amount_from_str_comma_code_some": "USD 2,000.00",
          "amount_from_str_comma_code_none": null,
          "amount_from_str_comma_symbol": "$1,234.56",
          "amount_from_str_comma_symbol_some": "$2,345.6799",
          "amount_from_str_comma_symbol_none": null,
          "raw_amount_from_str_comma_code": "USD -42.42424242",
          "amount_from_str_dot_code": "EUR 1.234,5634",
          "amount_from_str_dot_code_some": "EUR 2.000,00",
          "amount_from_str_dot_code_none": null,
          "amount_from_str_dot_symbol": "€1.234,56",
          "amount_from_str_dot_symbol_some": "€2.345,67",
          "amount_from_str_dot_symbol_none": null,
          "raw_amount_from_str_dot_symbol": "-€69,69696969"
        }
    "#;
    let all = serde_json::from_str::<All>(json_str);
    dbg!(&all);
    assert!(all.is_ok());

    let ret = all.unwrap();
    assert_eq!(ret.amount_from_f64.amount(), dec!(1234.57));
    assert_eq!(ret.amount_from_f64_omit.amount(), dec!(0));
    assert_eq!(ret.amount_from_str_omit.amount(), dec!(0));

    assert_eq!(ret.amount_from_i64.amount(), dec!(-1234));
    assert_eq!(ret.amount_from_u64.amount(), dec!(18446744073709551615));

    assert_eq!(ret.amount_from_i128.amount(), dec!(-1844674407370955161588));
    assert_eq!(ret.amount_from_u128.amount(), dec!(34028236692093846346337));

    assert_eq!(ret.amount_from_str.amount(), dec!(1234.56));

    assert_eq!(ret.raw_amount_from_f64.amount(), dec!(-1004.1234,));
    assert_eq!(ret.raw_amount_from_str.amount(), dec!(1230.4993));

    // comma + code
    assert_eq!(ret.amount_from_str_comma_code.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_comma_code_some.is_some());
    assert_eq!(
        ret.amount_from_str_comma_code_some
            .as_ref()
            .unwrap()
            .amount(),
        dec!(2000.00)
    );
    assert!(ret.amount_from_str_comma_code_none.is_none());
    assert!(ret.amount_from_str_comma_code_omit.is_none());

    // comma + symbol
    assert_eq!(ret.amount_from_str_comma_symbol.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_comma_symbol_some.is_some());
    // "$2,345.6799" -> rounded to 2 decimal places -> 2345.68
    assert_eq!(
        ret.amount_from_str_comma_symbol_some
            .as_ref()
            .unwrap()
            .amount(),
        dec!(2345.68)
    );
    assert!(ret.amount_from_str_comma_symbol_none.is_none());
    assert_eq!(ret.raw_amount_from_str_comma_code.amount(), dec!(-42.42424242));
    assert!(ret.amount_from_str_comma_symbol_omit.is_none());

    // dot + code (European formatting)
    // "EUR 1.234,5634" -> 1234.5634 -> rounded to 1234.56 (third decimal is 3 -> round down)
    assert_eq!(ret.amount_from_str_dot_code.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_dot_code_some.is_some());
    assert_eq!(
        ret.amount_from_str_dot_code_some.as_ref().unwrap().amount(),
        dec!(2000.00)
    );
    assert!(ret.amount_from_str_dot_code_none.is_none());
    assert!(ret.amount_from_str_dot_code_omit.is_none());

    // dot + symbol
    assert_eq!(ret.amount_from_str_dot_symbol.amount(), dec!(1234.56));
    assert!(ret.amount_from_str_dot_symbol_some.is_some());
    assert_eq!(
        ret.amount_from_str_dot_symbol_some
            .as_ref()
            .unwrap()
            .amount(),
        dec!(2345.67)
    );
    assert!(ret.amount_from_str_dot_symbol_none.is_none());
    assert!(ret.amount_from_str_dot_symbol_omit.is_none());
    assert_eq!(ret.raw_amount_from_str_dot_symbol.amount(), dec!(-69.69696969));
```

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

