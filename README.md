# moneylib

Library to deal with money in Rust.

## Code Coverage

This project maintains excellent code coverage.

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

