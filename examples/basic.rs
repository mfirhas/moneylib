// This example demonstrates the basic usage of the moneylib library.
// The moneylib library provides a safe and ergonomic way to work with monetary values,
// handling currency, precision, rounding, and arithmetic operations.

use moneylib::{
    BHD, BaseMoney, BaseOps, Currency, CustomMoney, EUR, GBP, JPY, Money, RoundingStrategy, USD,
    money_macros::dec,
};
use std::str::FromStr;

fn main() {
    println!("=== MoneyLib Basic Examples ===\n");

    // ============================================================================
    // 1. Creating Money - Basic Construction
    // ============================================================================
    println!("1. Creating Money - Basic Construction");
    println!("----------------------------------------");

    // Currency is a compile-time type parameter - use ISO 4217 currency structs (e.g. USD, EUR)
    println!(
        "USD currency: code={}, symbol={}, name={}",
        USD::CODE,
        USD::SYMBOL,
        USD::NAME
    );

    // Create Money with a currency type parameter and decimal amount using the dec! macro
    let money1 = Money::<USD>::new(dec!(100.50)).unwrap();
    println!("Money 1: {} (amount: {})", money1, money1.amount());

    // Money is automatically rounded to the currency's minor unit (2 decimal places for USD)
    let money2 = Money::<USD>::new(dec!(100.567)).unwrap();
    println!(
        "Money 2 (rounded): {} (amount: {})",
        money2,
        money2.amount()
    );
    println!();

    // ============================================================================
    // 2. Creating Money from String
    // ============================================================================
    println!("2. Creating Money from String");
    println!("------------------------------");

    // Parse money from string format "CURRENCY AMOUNT"
    let money_from_str = Money::<USD>::from_str("USD 1,234.56").unwrap();
    println!("Parsed from string: {}", money_from_str);

    // Supports different thousand separator formats
    let money_euro = Money::<EUR>::from_str("EUR 1.234,56").unwrap();
    println!("European format: {}", money_euro);

    // Works without thousand separators too
    let money_simple = Money::<GBP>::from_str("GBP 500.25").unwrap();
    println!("Simple format: {}", money_simple);
    println!();

    // ============================================================================
    // 3. Creating Money from Minor Amount (smallest unit)
    // ============================================================================
    println!("3. Creating Money from Minor Amount");
    println!("------------------------------------");

    // Minor amount represents the smallest unit (e.g., cents for USD)
    // USD has 2 decimal places, so 12345 cents = $123.45
    let from_cents = Money::<USD>::from_minor(12345).unwrap();
    println!("From 12345 cents: {}", from_cents);

    // JPY has 0 decimal places, so minor amount equals the regular amount
    let from_yen = Money::<JPY>::from_minor(1000).unwrap();
    println!("From 1000 yen (minor): {}", from_yen);

    // Negative amounts are supported
    let negative_from_minor = Money::<USD>::from_minor(-5000).unwrap();
    println!("Negative from minor: {}", negative_from_minor);
    println!();

    // ============================================================================
    // 4. Arithmetic Operations - Addition
    // ============================================================================
    println!("4. Arithmetic Operations - Addition");
    println!("------------------------------------");

    let money_a = Money::<USD>::new(dec!(100.00)).unwrap();
    let money_b = Money::<USD>::new(dec!(50.00)).unwrap();

    // Add using the + operator (panics on overflow)
    let sum_operator = money_a + money_b;
    println!("{} + {} = {}", money_a, money_b, sum_operator);

    // Add using the add() method (returns Result for error handling)
    let sum_method = money_a.add(money_b).unwrap();
    println!("Using add() method: {}", sum_method);

    // Can add Decimal values directly
    let sum_decimal = money_a.add(dec!(25.50)).unwrap();
    println!("{} + dec!(25.50) = {}", money_a, sum_decimal);

    // Can add integer values directly
    let sum_int = money_a.add(30_i32).unwrap();
    println!("{} + 30 = {}", money_a, sum_int);
    println!();

    // ============================================================================
    // 5. Arithmetic Operations - Subtraction
    // ============================================================================
    println!("5. Arithmetic Operations - Subtraction");
    println!("---------------------------------------");

    let money_x = Money::<USD>::new(dec!(200.00)).unwrap();
    let money_y = Money::<USD>::new(dec!(75.00)).unwrap();

    // Subtract using the - operator
    let diff_operator = money_x - money_y;
    println!("{} - {} = {}", money_x, money_y, diff_operator);

    // Subtract using the sub() method
    let diff_method = money_x.sub(money_y).unwrap();
    println!("Using sub() method: {}", diff_method);

    // Subtraction can result in negative money
    let negative_result = money_y - money_x;
    println!("{} - {} = {}", money_y, money_x, negative_result);
    println!();

    // ============================================================================
    // 6. Arithmetic Operations - Multiplication
    // ============================================================================
    println!("6. Arithmetic Operations - Multiplication");
    println!("------------------------------------------");

    let money_m = Money::<USD>::new(dec!(50.00)).unwrap();

    // Multiply using the * operator
    let product_operator = money_m * dec!(3);
    println!("{} * 3 = {}", money_m, product_operator);

    // Multiply using the mul() method
    let product_method = money_m.mul(dec!(2.5)).unwrap();
    println!("Using mul() method: {} * 2.5 = {}", money_m, product_method);

    // Can multiply by i32 or i64 values directly
    let product_i32 = money_m.mul(3_i32).unwrap();
    println!("{} * 3 (i32) = {}", money_m, product_i32);
    println!();

    // ============================================================================
    // 7. Arithmetic Operations - Division
    // ============================================================================
    println!("7. Arithmetic Operations - Division");
    println!("------------------------------------");

    let money_d = Money::<USD>::new(dec!(100.00)).unwrap();

    // Divide using the / operator
    let quotient_operator = money_d / dec!(4);
    println!("{} / 4 = {}", money_d, quotient_operator);

    // Divide using the div() method
    let quotient_method = money_d.div(dec!(2.5)).unwrap();
    println!(
        "Using div() method: {} / 2.5 = {}",
        money_d, quotient_method
    );

    // Division results are rounded to currency's minor unit
    let quotient_rounded = money_d / dec!(3);
    println!("{} / 3 = {} (rounded)", money_d, quotient_rounded);
    println!();

    // ============================================================================
    // 8. Comparison Operations
    // ============================================================================
    println!("8. Comparison Operations");
    println!("------------------------");

    let money_100 = Money::<USD>::new(dec!(100.00)).unwrap();
    let money_200 = Money::<USD>::new(dec!(200.00)).unwrap();
    let money_100_copy = Money::<USD>::new(dec!(100.00)).unwrap();

    // Equality comparison
    println!(
        "money_100 == money_100_copy: {}",
        money_100 == money_100_copy
    );
    println!("money_100 == money_200: {}", money_100 == money_200);

    // Ordering comparisons
    println!("money_100 < money_200: {}", money_100 < money_200);
    println!("money_200 > money_100: {}", money_200 > money_100);
    println!(
        "money_100 <= money_100_copy: {}",
        money_100 <= money_100_copy
    );
    println!("money_200 >= money_100: {}", money_200 >= money_100);
    println!();

    // ============================================================================
    // 9. Working with Different Currencies
    // ============================================================================
    println!("9. Working with Different Currencies");
    println!("-------------------------------------");

    // Each currency is a distinct compile-time type - mixing them is a compile error
    let usd_money = Money::<USD>::new(dec!(100.00)).unwrap();
    let eur_money = Money::<EUR>::new(dec!(100.00)).unwrap();
    let gbp_money = Money::<GBP>::new(dec!(100.00)).unwrap();

    println!("USD Money: {}", usd_money);
    println!("EUR Money: {}", eur_money);
    println!("GBP Money: {}", gbp_money);

    // Different currencies with different decimal places
    println!("\nCurrency with no decimal places:");
    let jpy_money = Money::<JPY>::new(dec!(10000)).unwrap();
    println!(
        "JPY Money: {} (minor_unit: {})",
        jpy_money,
        jpy_money.minor_unit()
    );

    // Currency with 3 decimal places
    let bhd_money = Money::<BHD>::new(dec!(12.345)).unwrap(); // Bahraini Dinar
    println!(
        "BHD Money: {} (minor_unit: {})",
        bhd_money,
        bhd_money.minor_unit()
    );
    println!();

    // ============================================================================
    // 10. Formatting Money
    // ============================================================================
    println!("10. Formatting Money");
    println!("--------------------");

    let money_fmt = Money::<USD>::new(dec!(1234.56)).unwrap();

    // Default format uses Display trait (code format with separators)
    println!("Default format: {}", money_fmt);

    // Format with currency code
    println!("Code format: {}", money_fmt.format_code());

    // Format with currency symbol
    println!("Symbol format: {}", money_fmt.format_symbol());

    // Format minor amount (in cents for USD)
    println!("Code minor format: {}", money_fmt.format_code_minor());
    println!("Symbol minor format: {}", money_fmt.format_symbol_minor());

    // Custom format using format string ('c'=code, 's'=symbol, 'a'=amount, 'n'=sign, 'm'=minor symbol)
    println!("Custom format (sa): {}", money_fmt.format("sa"));
    println!("Custom format (c na): {}", money_fmt.format("c na"));
    println!();

    // ============================================================================
    // 11. Rounding Strategies
    // ============================================================================
    println!("11. Rounding Strategies");
    println!("-----------------------");

    let amount_to_round = dec!(123.456);

    // Demonstrate rounding strategies using round_with()
    // from_decimal() skips automatic rounding to preserve the raw value for demonstration
    let money = Money::<USD>::from_decimal(amount_to_round);
    println!(
        "Banker's Rounding: {} -> {}",
        amount_to_round,
        money
            .round_with(2, RoundingStrategy::BankersRounding)
            .amount()
    );
    println!(
        "Half Up:           {} -> {}",
        amount_to_round,
        money.round_with(2, RoundingStrategy::HalfUp).amount()
    );
    println!(
        "Half Down:         {} -> {}",
        amount_to_round,
        money.round_with(2, RoundingStrategy::HalfDown).amount()
    );
    println!(
        "Ceil:              {} -> {}",
        amount_to_round,
        money.round_with(2, RoundingStrategy::Ceil).amount()
    );
    println!(
        "Floor:             {} -> {}",
        amount_to_round,
        money.round_with(2, RoundingStrategy::Floor).amount()
    );

    // round_with also works with different decimal places
    let money_custom_round = money.round_with(1, RoundingStrategy::HalfUp);
    println!(
        "Custom round (1 decimal, HalfUp): {}",
        money_custom_round.amount()
    );
    println!();

    // ============================================================================
    // 12. Custom Currencies
    // ============================================================================
    println!("12. Custom Currencies");
    println!("---------------------");

    // Custom currencies are defined by implementing the Currency trait on a new type.
    // This gives full compile-time type safety just like ISO currencies.
    #[derive(Clone)]
    struct BTC;
    impl Currency for BTC {
        const CODE: &'static str = "BTC";
        const SYMBOL: &'static str = "₿";
        const NAME: &'static str = "Bitcoin";
        const NUMERIC: u16 = 0; // Bitcoin has no official ISO 4217 numeric code
        const MINOR_UNIT: u16 = 8;
        const MINOR_UNIT_SYMBOL: &'static str = "sat";
        const THOUSAND_SEPARATOR: &'static str = ",";
        const DECIMAL_SEPARATOR: &'static str = ".";
    }

    println!(
        "Custom currency: code={}, symbol={}, name={}, minor_unit={}",
        BTC::CODE,
        BTC::SYMBOL,
        BTC::NAME,
        BTC::MINOR_UNIT
    );

    let btc_money = Money::<BTC>::new(dec!(0.12345678)).unwrap();
    println!("Bitcoin money: {}", btc_money);

    // Custom currency for loyalty points
    #[derive(Clone)]
    struct PTS;
    impl Currency for PTS {
        const CODE: &'static str = "PTS";
        const SYMBOL: &'static str = "P";
        const NAME: &'static str = "Loyalty Points";
        const NUMERIC: u16 = 0;
        const MINOR_UNIT: u16 = 0;
        const MINOR_UNIT_SYMBOL: &'static str = "";
        const THOUSAND_SEPARATOR: &'static str = ",";
        const DECIMAL_SEPARATOR: &'static str = ".";
    }

    let points_money = Money::<PTS>::new(dec!(1500)).unwrap();
    println!("Loyalty Points: {}", points_money);
    println!();

    // ============================================================================
    // 13. Negative Amounts
    // ============================================================================
    println!("13. Negative Amounts");
    println!("--------------------");

    let positive = Money::<USD>::new(dec!(100.00)).unwrap();
    let negative = Money::<USD>::new(dec!(-50.00)).unwrap();

    println!("Positive money: {}", positive);
    println!("Negative money: {}", negative);

    // Check if money is positive, negative, or zero
    println!("Is positive positive? {}", positive.is_positive());
    println!("Is negative negative? {}", negative.is_negative());

    // Absolute value
    let abs_negative = negative.abs();
    println!("Absolute value of {}: {}", negative, abs_negative);

    // Operations with negative amounts
    let result = positive + negative;
    println!("{} + {} = {}", positive, negative, result);
    println!();

    // ============================================================================
    // 14. Zero Amounts
    // ============================================================================
    println!("14. Zero Amounts");
    println!("----------------");

    let zero_money = Money::<USD>::new(dec!(0)).unwrap();
    println!("Zero money: {}", zero_money);
    println!("Is zero? {}", zero_money.is_zero());
    println!("Is positive? {}", zero_money.is_positive());
    println!("Is negative? {}", zero_money.is_negative());

    // Operations with zero
    let plus_zero = positive + zero_money;
    println!("{} + {} = {}", positive, zero_money, plus_zero);
    println!();

    // ============================================================================
    // 15. Additional Operations - Min, Max, Clamp
    // ============================================================================
    println!("15. Additional Operations - Min, Max, Clamp");
    println!("--------------------------------------------");

    let money_50 = Money::<USD>::new(dec!(50.00)).unwrap();
    let money_150 = Money::<USD>::new(dec!(150.00)).unwrap();

    // Find minimum of two money values (via Ord trait)
    let min_value = money_50.min(money_150);
    println!("min({}, {}) = {}", money_50, money_150, min_value);

    // Find maximum of two money values (via Ord trait)
    let max_value = money_50.max(money_150);
    println!("max({}, {}) = {}", money_50, money_150, max_value);

    // Clamp money to a range (via Ord trait)
    let money_to_clamp = Money::<USD>::new(dec!(200.00)).unwrap();
    let clamp_min = Money::<USD>::new(dec!(50.00)).unwrap();
    let clamp_max = Money::<USD>::new(dec!(150.00)).unwrap();
    let clamped = money_to_clamp.clamp(clamp_min, clamp_max);
    println!("clamp({}, 50.00..150.00) = {}", money_to_clamp, clamped);

    let money_low = Money::<USD>::new(dec!(25.00)).unwrap();
    let clamped_low = money_low.clamp(clamp_min, clamp_max);
    println!("clamp({}, 50.00..150.00) = {}", money_low, clamped_low);
    println!();

    // ============================================================================
    // 16. Error Handling Examples
    // ============================================================================
    println!("16. Error Handling Examples");
    println!("---------------------------");

    // Invalid money string format (currency code must come FIRST, not last)
    // The correct format is "CURRENCY AMOUNT", not "AMOUNT CURRENCY"
    match Money::<USD>::from_str("100.00 USD") {
        Ok(money) => println!("Money created: {}", money),
        Err(e) => println!("Error parsing money (wrong format): {:?}", e),
    }

    // Currency mismatch in string parsing (type is USD but string contains EUR)
    match Money::<USD>::from_str("EUR 100.00") {
        Ok(money) => println!("Money created: {}", money),
        Err(e) => println!("Error parsing money (currency mismatch): {:?}", e),
    }

    // Division by zero
    let money_100 = Money::<USD>::new(dec!(100.00)).unwrap();
    match money_100.div(dec!(0)) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error in division: {:?}", e),
    }

    // Using the safe add() method which returns Result - useful for error handling
    let large_money = Money::<USD>::new(dec!(100.00)).unwrap();
    match large_money.add(dec!(50.00)) {
        Ok(result) => println!("Addition result: {}", result),
        Err(e) => println!("Error in addition: {:?}", e),
    }
    println!();

    // ============================================================================
    // 17. Accessing Currency Metadata
    // ============================================================================
    println!("17. Accessing Currency Metadata");
    println!("--------------------------------");

    let usd_money2 = Money::<USD>::new(dec!(1000.00)).unwrap();
    println!("USD metadata (via money instance):");
    println!("  Code: {}", usd_money2.code());
    println!("  Symbol: {}", usd_money2.symbol());
    println!("  Name: {}", usd_money2.name());
    println!("  Minor unit: {}", usd_money2.minor_unit());
    println!("  Numeric code: {}", usd_money2.numeric_code());
    println!(
        "  Thousand separator: '{}'",
        usd_money2.thousand_separator()
    );
    println!("  Decimal separator: '{}'", usd_money2.decimal_separator());

    // Currency metadata is also available as compile-time constants
    println!("USD metadata (via constants):");
    println!("  Code: {}", USD::CODE);
    println!("  Symbol: {}", USD::SYMBOL);
    println!("  Minor unit symbol: {}", USD::MINOR_UNIT_SYMBOL);
    println!();

    // ============================================================================
    // 18. Converting to Decimal
    // ============================================================================
    println!("18. Converting to Decimal");
    println!("-------------------------");

    let money_convert = Money::<USD>::new(dec!(123.45)).unwrap();

    // Get the amount as Decimal
    let decimal_amount = money_convert.amount();
    println!("Money: {}", money_convert);
    println!("Amount as Decimal: {}", decimal_amount);

    // Get the amount in minor units (cents for USD)
    let minor = money_convert.minor_amount().unwrap();
    println!("Amount in minor units (cents): {}", minor);
    println!();

    println!("=== End of Examples ===");
}
