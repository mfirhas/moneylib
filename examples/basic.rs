// This example demonstrates the basic usage of the moneylib library.
// The moneylib library provides a safe and ergonomic way to work with monetary values,
// handling currency, precision, rounding, and arithmetic operations.

use moneylib::{
    BaseMoney, BaseOps, Currency, CustomMoney, Decimal, Money, RoundingStrategy,
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

    // Create a Currency first (using ISO 4217 standard codes)
    let usd = Currency::from_iso("USD").unwrap();
    println!("Created USD currency: code={}, symbol={}, name={}", 
        usd.code(), usd.symbol(), usd.name());

    // Create Money with currency and decimal amount using the dec! macro
    let money1 = Money::new(usd, dec!(100.50));
    println!("Money 1: {} (amount: {})", money1, money1.amount());

    // Money is automatically rounded to the currency's minor unit (2 decimal places for USD)
    let money2 = Money::new(usd, dec!(100.567));
    println!("Money 2 (rounded): {} (amount: {})", money2, money2.amount());
    println!();

    // ============================================================================
    // 2. Creating Money from String
    // ============================================================================
    println!("2. Creating Money from String");
    println!("------------------------------");

    // Parse money from string format "CURRENCY AMOUNT"
    let money_from_str = Money::from_str("USD 1,234.56").unwrap();
    println!("Parsed from string: {}", money_from_str);

    // Supports different thousand separator formats
    let money_euro = Money::from_str("EUR 1.234,56").unwrap();
    println!("European format: {}", money_euro);

    // Works without thousand separators too
    let money_simple = Money::from_str("GBP 500.25").unwrap();
    println!("Simple format: {}", money_simple);
    println!();

    // ============================================================================
    // 3. Creating Money from Minor Amount (smallest unit)
    // ============================================================================
    println!("3. Creating Money from Minor Amount");
    println!("------------------------------------");

    // Minor amount represents the smallest unit (e.g., cents for USD)
    // USD has 2 decimal places, so 12345 cents = $123.45
    let from_cents = Money::from_minor_amount(usd, 12345).unwrap();
    println!("From 12345 cents: {}", from_cents);

    // JPY has 0 decimal places, so minor amount equals the regular amount
    let jpy = Currency::from_iso("JPY").unwrap();
    let from_yen = Money::from_minor_amount(jpy, 1000).unwrap();
    println!("From 1000 yen (minor): {}", from_yen);

    // Negative amounts are supported
    let negative_from_minor = Money::from_minor_amount(usd, -5000).unwrap();
    println!("Negative from minor: {}", negative_from_minor);
    println!();

    // ============================================================================
    // 4. Arithmetic Operations - Addition
    // ============================================================================
    println!("4. Arithmetic Operations - Addition");
    println!("------------------------------------");

    let money_a = Money::new(usd, dec!(100.00));
    let money_b = Money::new(usd, dec!(50.00));

    // Add using the + operator (panics on currency mismatch or overflow)
    let sum_operator = money_a + money_b;
    println!("{} + {} = {}", money_a, money_b, sum_operator);

    // Add using the add() method (returns Result for error handling)
    let sum_method = money_a.add(money_b).unwrap();
    println!("Using add() method: {}", sum_method);

    // Can add Decimal values directly
    let sum_decimal = money_a.add(dec!(25.50)).unwrap();
    println!("{} + dec!(25.50) = {}", money_a, sum_decimal);

    // Can add integer values directly
    let sum_int = money_a.add(30).unwrap();
    println!("{} + 30 = {}", money_a, sum_int);
    println!();

    // ============================================================================
    // 5. Arithmetic Operations - Subtraction
    // ============================================================================
    println!("5. Arithmetic Operations - Subtraction");
    println!("---------------------------------------");

    let money_x = Money::new(usd, dec!(200.00));
    let money_y = Money::new(usd, dec!(75.00));

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

    let money_m = Money::new(usd, dec!(50.00));

    // Multiply using the * operator
    let product_operator = money_m * dec!(3);
    println!("{} * 3 = {}", money_m, product_operator);

    // Multiply using the mul() method
    let product_method = money_m.mul(dec!(2.5)).unwrap();
    println!("Using mul() method: {} * 2.5 = {}", money_m, product_method);

    // Can multiply by different numeric types
    let product_f64 = money_m.mul(1.5).unwrap();
    println!("{} * 1.5 = {}", money_m, product_f64);
    println!();

    // ============================================================================
    // 7. Arithmetic Operations - Division
    // ============================================================================
    println!("7. Arithmetic Operations - Division");
    println!("------------------------------------");

    let money_d = Money::new(usd, dec!(100.00));

    // Divide using the / operator
    let quotient_operator = money_d / dec!(4);
    println!("{} / 4 = {}", money_d, quotient_operator);

    // Divide using the div() method
    let quotient_method = money_d.div(dec!(2.5)).unwrap();
    println!("Using div() method: {} / 2.5 = {}", money_d, quotient_method);

    // Division results are rounded to currency's minor unit
    let quotient_rounded = money_d / dec!(3);
    println!("{} / 3 = {} (rounded)", money_d, quotient_rounded);
    println!();

    // ============================================================================
    // 8. Comparison Operations
    // ============================================================================
    println!("8. Comparison Operations");
    println!("------------------------");

    let money_100 = Money::new(usd, dec!(100.00));
    let money_200 = Money::new(usd, dec!(200.00));
    let money_100_copy = Money::new(usd, dec!(100.00));

    // Equality comparison
    println!("money_100 == money_100_copy: {}", money_100 == money_100_copy);
    println!("money_100 == money_200: {}", money_100 == money_200);

    // Ordering comparisons
    println!("money_100 < money_200: {}", money_100 < money_200);
    println!("money_200 > money_100: {}", money_200 > money_100);
    println!("money_100 <= money_100_copy: {}", money_100 <= money_100_copy);
    println!("money_200 >= money_100: {}", money_200 >= money_100);
    println!();

    // ============================================================================
    // 9. Working with Different Currencies
    // ============================================================================
    println!("9. Working with Different Currencies");
    println!("-------------------------------------");

    let usd_money = Money::new(usd, dec!(100.00));
    let eur = Currency::from_iso("EUR").unwrap();
    let eur_money = Money::new(eur, dec!(100.00));
    let gbp = Currency::from_iso("GBP").unwrap();
    let gbp_money = Money::new(gbp, dec!(100.00));

    println!("USD Money: {}", usd_money);
    println!("EUR Money: {}", eur_money);
    println!("GBP Money: {}", gbp_money);

    // Equality comparison works across different currencies
    println!("USD == EUR (same amount): {}", usd_money == eur_money);

    // Different currencies with different decimal places
    println!("\nCurrency with no decimal places:");
    let jpy_money = Money::new(jpy, dec!(10000));
    println!("JPY Money: {} (minor_unit: {})", jpy_money, jpy.minor_unit());

    // Currency with 3 decimal places
    let bhd = Currency::from_iso("BHD").unwrap(); // Bahraini Dinar
    let bhd_money = Money::new(bhd, dec!(12.345));
    println!("BHD Money: {} (minor_unit: {})", bhd_money, bhd.minor_unit());
    println!();

    // ============================================================================
    // 10. Formatting Money
    // ============================================================================
    println!("10. Formatting Money");
    println!("--------------------");

    let money_fmt = Money::new(usd, dec!(1234.56));

    // Default format uses Display trait (code format with separators)
    println!("Default format: {}", money_fmt);

    // Format with currency code
    println!("Code format: {}", money_fmt.format_code());

    // Format with currency symbol
    println!("Symbol format: {}", money_fmt.format_symbol());

    // Format minor amount (in cents for USD)
    println!("Code minor format: {}", money_fmt.format_code_minor());
    println!("Symbol minor format: {}", money_fmt.format_symbol_minor());

    // Custom separator formatting
    let mut money_custom = Money::new(usd, dec!(9876.54));
    money_custom.set_thousand_separator(".");
    money_custom.set_decimal_separator(",");
    println!("Custom separators: {}", money_custom);
    println!();

    // ============================================================================
    // 11. Rounding Strategies
    // ============================================================================
    println!("11. Rounding Strategies");
    println!("-----------------------");

    let amount_to_round = dec!(123.456);

    // Different rounding strategies
    let mut currency_bankers = usd;
    currency_bankers.set_rounding_strategy(RoundingStrategy::BankersRounding);
    let money_bankers = Money::new(currency_bankers, amount_to_round);
    println!("Banker's Rounding: {} -> {}", amount_to_round, money_bankers.amount());

    let mut currency_half_up = usd;
    currency_half_up.set_rounding_strategy(RoundingStrategy::HalfUp);
    let money_half_up = Money::new(currency_half_up, amount_to_round);
    println!("Half Up: {} -> {}", amount_to_round, money_half_up.amount());

    let mut currency_half_down = usd;
    currency_half_down.set_rounding_strategy(RoundingStrategy::HalfDown);
    let money_half_down = Money::new(currency_half_down, amount_to_round);
    println!("Half Down: {} -> {}", amount_to_round, money_half_down.amount());

    let mut currency_ceil = usd;
    currency_ceil.set_rounding_strategy(RoundingStrategy::Ceil);
    let money_ceil = Money::new(currency_ceil, amount_to_round);
    println!("Ceil: {} -> {}", amount_to_round, money_ceil.amount());

    let mut currency_floor = usd;
    currency_floor.set_rounding_strategy(RoundingStrategy::Floor);
    let money_floor = Money::new(currency_floor, amount_to_round);
    println!("Floor: {} -> {}", amount_to_round, money_floor.amount());

    // Custom rounding with round_with()
    let money_custom_round = money_bankers.round_with(1, RoundingStrategy::HalfUp);
    println!("Custom round (1 decimal, HalfUp): {}", money_custom_round.amount());
    println!();

    // ============================================================================
    // 12. Custom Currencies
    // ============================================================================
    println!("12. Custom Currencies");
    println!("---------------------");

    // Create a custom currency for cryptocurrency
    let btc = Currency::new("BTC", "₿", "Bitcoin", 8).unwrap();
    println!("Custom currency: code={}, symbol={}, name={}, minor_unit={}", 
        btc.code(), btc.symbol(), btc.name(), btc.minor_unit());

    let btc_money = Money::new(btc, dec!(0.12345678));
    println!("Bitcoin money: {}", btc_money);

    // Custom currency for loyalty points
    let points = Currency::new("PTS", "P", "Loyalty Points", 0).unwrap();
    let points_money = Money::new(points, dec!(1500));
    println!("Loyalty Points: {}", points_money);
    println!();

    // ============================================================================
    // 13. Negative Amounts
    // ============================================================================
    println!("13. Negative Amounts");
    println!("--------------------");

    let positive = Money::new(usd, dec!(100.00));
    let negative = Money::new(usd, dec!(-50.00));

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

    let zero_money = Money::new(usd, dec!(0));
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

    let money_50 = Money::new(usd, dec!(50.00));
    let money_150 = Money::new(usd, dec!(150.00));

    // Find minimum of two money values
    let min_value = money_50.min(money_150);
    println!("min({}, {}) = {}", money_50, money_150, min_value);

    // Find maximum of two money values
    let max_value = money_50.max(money_150);
    println!("max({}, {}) = {}", money_50, money_150, max_value);

    // Clamp money to a range
    let money_to_clamp = Money::new(usd, dec!(200.00));
    let clamped = money_to_clamp.clamp(dec!(50.00), dec!(150.00));
    println!("clamp({}, 50.00..150.00) = {}", money_to_clamp, clamped);

    let money_low = Money::new(usd, dec!(25.00));
    let clamped_low = money_low.clamp(dec!(50.00), dec!(150.00));
    println!("clamp({}, 50.00..150.00) = {}", money_low, clamped_low);
    println!();

    // ============================================================================
    // 16. Error Handling Examples
    // ============================================================================
    println!("16. Error Handling Examples");
    println!("---------------------------");

    // Invalid currency code
    match Currency::from_iso("INVALID") {
        Ok(curr) => println!("Currency created: {}", curr.code()),
        Err(e) => println!("Error creating currency: {:?}", e),
    }

    // Invalid money string format
    match Money::from_str("100.00 USD") {
        Ok(money) => println!("Money created: {}", money),
        Err(e) => println!("Error parsing money: {:?}", e),
    }

    // Currency mismatch in operations (using method that returns Result)
    let eur_money_err = Money::new(eur, dec!(100.00));
    match usd_money.add(eur_money_err) {
        Ok(result) => println!("Addition result: {}", result),
        Err(e) => println!("Error in addition: {:?}", e),
    }

    // Division by zero
    match money_100.div(dec!(0)) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error in division: {:?}", e),
    }

    // Cannot create custom currency with ISO code
    match Currency::new("USD", "$", "My Dollar", 2) {
        Ok(curr) => println!("Currency created: {}", curr.code()),
        Err(e) => println!("Error: Cannot create custom currency with ISO code - {:?}", e),
    }
    println!();

    // ============================================================================
    // 17. Accessing Currency Metadata
    // ============================================================================
    println!("17. Accessing Currency Metadata");
    println!("--------------------------------");

    println!("USD metadata:");
    println!("  Code: {}", usd.code());
    println!("  Symbol: {}", usd.symbol());
    println!("  Name: {}", usd.name());
    println!("  Minor unit: {}", usd.minor_unit());
    println!("  Numeric code: {}", usd.numeric_code());
    println!("  Thousand separator: '{}'", usd.thousand_separator());
    println!("  Decimal separator: '{}'", usd.decimal_separator());
    println!("  Minor symbol: {}", usd.minor_symbol());

    // Get countries that use this currency
    if let Some(countries) = usd.countries() {
        println!("  Used by {} countries", countries.len());
    }
    println!();

    // ============================================================================
    // 18. Converting to Decimal
    // ============================================================================
    println!("18. Converting to Decimal");
    println!("-------------------------");

    let money_convert = Money::new(usd, dec!(123.45));
    
    // Get the amount as Decimal
    let decimal_amount = money_convert.amount();
    println!("Money: {}", money_convert);
    println!("Amount as Decimal: {}", decimal_amount);

    // Convert Money to Decimal using Into trait
    let converted: Decimal = money_convert.into();
    println!("Converted to Decimal: {}", converted);
    println!();

    println!("=== End of Examples ===");
}
