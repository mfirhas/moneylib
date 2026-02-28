### Prerequisite:
# - cargo-llvm-cov: cargo install cargo-llvm-cov

## Using cargo-llvm-cov to generate lcov.info and html coverage reports covering functions, lines, and regions coverages.
# `branch` uses nightly because cargo-llvm-cov --branch is unstable.

OUT_DIR := target/coverage
OUT_FILE := $(OUT_DIR)/lcov.info

test:
	@echo "Running tests..."
	@cargo test --all-features

lcov:
	@echo "Generating lcov.info..."
	@mkdir -p $(OUT_DIR)
	@cargo llvm-cov test --all-features --ignore-filename-regex "_test\.rs$$" --output-path $(OUT_FILE) --lcov

html:
	@echo "Generating html coverage report..."
	@mkdir -p $(OUT_DIR)
	@cargo llvm-cov test --all-features --ignore-filename-regex "_test\.rs$$" --output-dir $(OUT_DIR) --open

branch:
	@echo "Generate coverage with branch coverage..."
	@mkdir -p $(OUT_DIR)
	@cargo +nightly llvm-cov test --all-features --ignore-filename-regex "_test\.rs$$" --output-dir $(OUT_DIR) --open --branch
