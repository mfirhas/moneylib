# moneylib

Library to deal with money in Rust.

## Code Coverage

This project has **100.00% code coverage** (305/305 lines covered).

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

### Coverage Results

```
Tested/Total Lines:
src/base.rs: 83/83 (100%)
src/currency.rs: 63/63 (100%)
src/dec_ops.rs: 34/34 (100%)
src/money.rs: 85/85 (100%)
src/ops.rs: 40/40 (100%)

Total: 100.00% coverage, 305/305 lines covered
```
