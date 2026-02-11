# Proposed Finance Operations - Detailed Explanations & Examples

This document provides detailed explanations and code samples for finance operations that could be added to the moneylib library.

## Table of Contents

1. [Percentage Operations](#1-percentage-operations)
2. [Split & Allocation Operations](#2-split--allocation-operations)
3. [Comparison & Validation Operations](#3-comparison--validation-operations)
4. [Exchange Rate Operations](#4-exchange-rate-operations)
5. [Interest Calculations](#5-interest-calculations)
6. [Tax & Discount Operations](#6-tax--discount-operations)
7. [Amortization & Loan Calculations](#7-amortization--loan-calculations)
8. [Advanced Operations](#8-advanced-operations)

---

## 1. Percentage Operations

Percentage calculations are fundamental to finance and used in countless real-world scenarios.

### 1.1 Calculate Percentage of Amount

**Use Case**: Calculate what a certain percentage of a money amount equals.

**Example Scenarios**:
- Calculate 5% sales tax on a purchase
- Calculate 15% tip on a restaurant bill
- Calculate 20% down payment on a house

**Proposed Function Signature**:
```rust
fn percentage(&self, percent: Decimal) -> MoneyResult<Self>
```

**Example Usage**:
```rust
use moneylib::{Money, Currency, Decimal};
use moneylib::money_macros::dec;

let currency = Currency::from_iso("USD")?;
let bill = Money::new(currency, dec!(100.00));

// Calculate 15% tip
let tip = bill.percentage(dec!(15))?;
// tip = USD 15.00

// Calculate 5% sales tax
let tax = bill.percentage(dec!(5))?;
// tax = USD 5.00

// Calculate 2.5% processing fee
let fee = bill.percentage(dec!(2.5))?;
// fee = USD 2.50
```

### 1.2 Add Percentage

**Use Case**: Add a percentage to an amount (common for taxes, markups).

**Example Scenarios**:
- Add 8% sales tax to a product price
- Apply 25% markup to cost price
- Add 3% credit card processing fee

**Proposed Function Signature**:
```rust
fn add_percentage(&self, percent: Decimal) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let base_price = Money::new(currency, dec!(100.00));

// Add 8% sales tax
let total_with_tax = base_price.add_percentage(dec!(8))?;
// total_with_tax = USD 108.00

// Add 25% markup for retail price
let retail_price = base_price.add_percentage(dec!(25))?;
// retail_price = USD 125.00

// Chain multiple percentages (e.g., cost + markup + tax)
let cost = Money::new(currency, dec!(50.00));
let retail = cost.add_percentage(dec!(100))?;  // 100% markup
let final_price = retail.add_percentage(dec!(8))?;  // 8% tax
// cost: $50.00 → retail: $100.00 → final: $108.00
```

### 1.3 Subtract Percentage (Discount)

**Use Case**: Subtract a percentage from an amount (discounts, deductions).

**Example Scenarios**:
- Apply 20% discount on sale items
- Calculate price after 10% employee discount
- Apply 15% early payment discount

**Proposed Function Signature**:
```rust
fn subtract_percentage(&self, percent: Decimal) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let original_price = Money::new(currency, dec!(200.00));

// Apply 20% discount
let sale_price = original_price.subtract_percentage(dec!(20))?;
// sale_price = USD 160.00

// Apply 50% clearance discount
let clearance = original_price.subtract_percentage(dec!(50))?;
// clearance = USD 100.00

// Multiple discounts (e.g., 10% member + 5% coupon)
let member_price = original_price.subtract_percentage(dec!(10))?;
let final_price = member_price.subtract_percentage(dec!(5))?;
// original: $200 → member: $180 → final: $171
```

### 1.4 Calculate What Percentage

**Use Case**: Determine what percentage one amount is of another.

**Example Scenarios**:
- Calculate discount percentage: "Save 20%!"
- Calculate profit margin
- Compare actual vs budget spending

**Proposed Function Signature**:
```rust
fn percentage_of(&self, rhs: Self) -> MoneyResult<Decimal>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let original = Money::new(currency, dec!(200.00));
let discounted = Money::new(currency, dec!(160.00));

// Calculate discount percentage
let discount_pct = discounted.percentage_of(original)?;
// discount_pct = 80.00 (meaning 20% off)

// Calculate profit margin
let cost = Money::new(currency, dec!(60.00));
let revenue = Money::new(currency, dec!(100.00));
let profit_margin = revenue.percentage_of(cost)?;
// profit_margin = 166.67 (66.67% markup)

// Budget analysis
let budget = Money::new(currency, dec!(1000.00));
let spent = Money::new(currency, dec!(750.00));
let percentage_used = spent.percentage_of(budget)?;
// percentage_used = 75.00 (75% of budget used)
```

---

## 2. Split & Allocation Operations

These operations handle dividing money into parts, crucial for bill splitting, invoice distribution, and financial allocation.

### 2.1 Split Into Equal Parts

**Use Case**: Divide money equally among N people/accounts.

**Example Scenarios**:
- Split restaurant bill among friends
- Divide refund among multiple accounts
- Distribute bonus equally among employees

**Proposed Function Signature**:
```rust
fn split(&self, parts: u32) -> MoneyResult<Vec<Self>>
```

**Why It's Complex**: When splitting $10.00 into 3 parts, you get $3.33, $3.33, $3.34 (not $3.333... × 3). The algorithm must ensure the sum equals the original and handle remainders fairly.

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let bill = Money::new(currency, dec!(100.00));

// Split bill among 3 people
let shares = bill.split(3)?;
// shares = [USD 33.34, USD 33.33, USD 33.33]
// Total: $100.00 (perfectly preserved)

// Split among 7 people
let dinner = Money::new(currency, dec!(85.50));
let per_person = dinner.split(7)?;
// per_person = [USD 12.22, USD 12.21, USD 12.21, USD 12.21, USD 12.21, USD 12.21, USD 12.21]
// Total: $85.50

// Edge case: Split $0.10 among 3 people
let small = Money::new(currency, dec!(0.10));
let tiny_shares = small.split(3)?;
// tiny_shares = [USD 0.04, USD 0.03, USD 0.03]
// Total: $0.10 (no money lost!)
```

### 2.2 Allocate by Ratios

**Use Case**: Divide money according to specific ratios or weights.

**Example Scenarios**:
- Distribute profit by ownership percentage (60/40 split)
- Allocate budget across departments by priority
- Split investment returns by shareholding

**Proposed Function Signature**:
```rust
fn allocate(&self, ratios: &[u32]) -> MoneyResult<Vec<Self>>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let profit = Money::new(currency, dec!(10000.00));

// Split profit 60/40 between two partners
let shares = profit.allocate(&[60, 40])?;
// shares = [USD 6000.00, USD 4000.00]

// Split by ownership: 50%, 30%, 20%
let investment_return = Money::new(currency, dec!(5000.00));
let distributions = investment_return.allocate(&[50, 30, 20])?;
// distributions = [USD 2500.00, USD 1500.00, USD 1000.00]

// Unequal ratios: 1:2:1 split (25%, 50%, 25%)
let settlement = Money::new(currency, dec!(400.00));
let parts = settlement.allocate(&[1, 2, 1])?;
// parts = [USD 100.00, USD 200.00, USD 100.00]

// Complex scenario: Budget allocation [35, 25, 20, 15, 5]
let budget = Money::new(currency, dec!(100000.00));
let dept_budgets = budget.allocate(&[35, 25, 20, 15, 5])?;
// dept_budgets = [USD 35000.00, USD 25000.00, USD 20000.00, USD 15000.00, USD 5000.00]
```

### 2.3 Allocate by Exact Percentages

**Use Case**: Allocate using exact percentages (must sum to 100%).

**Example Scenarios**:
- Tax withholding (federal 22%, state 5%, local 1.5%)
- Insurance premium split by coverage type
- Portfolio allocation by asset class

**Proposed Function Signature**:
```rust
fn allocate_by_percentages(&self, percentages: &[Decimal]) -> MoneyResult<Vec<Self>>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let salary = Money::new(currency, dec!(5000.00));

// Tax withholding: Federal 22%, State 5%, Local 1.5%
let withholdings = salary.allocate_by_percentages(&[
    dec!(22.0),   // Federal
    dec!(5.0),    // State
    dec!(1.5),    // Local
])?;
// withholdings = [USD 1100.00, USD 250.00, USD 75.00]
// Error if percentages don't sum to 100%!

// Portfolio rebalancing: 60% stocks, 30% bonds, 10% cash
let investment = Money::new(currency, dec!(100000.00));
let allocation = investment.allocate_by_percentages(&[
    dec!(60.0),   // Stocks
    dec!(30.0),   // Bonds
    dec!(10.0),   // Cash
])?;
// allocation = [USD 60000.00, USD 30000.00, USD 10000.00]
```

### 2.4 Allocate with Remainder Handling

**Use Case**: Control where fractional remainders go when splitting.

**Example Scenarios**:
- Give remainder to first person (restaurant bill split)
- Give remainder to last account (accounting rules)
- Give remainder to specific beneficiary

**Proposed Function Signature**:
```rust
fn allocate_with_remainder(&self, parts: u32, remainder_index: usize) -> MoneyResult<Vec<Self>>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let amount = Money::new(currency, dec!(10.00));

// Split $10 into 3 parts, remainder goes to first person
let split_first = amount.allocate_with_remainder(3, 0)?;
// split_first = [USD 3.34, USD 3.33, USD 3.33]

// Split $10 into 3 parts, remainder goes to last person
let split_last = amount.allocate_with_remainder(3, 2)?;
// split_last = [USD 3.33, USD 3.33, USD 3.34]

// Split $10 into 3 parts, remainder goes to middle person
let split_middle = amount.allocate_with_remainder(3, 1)?;
// split_middle = [USD 3.33, USD 3.34, USD 3.33]
```

---

## 3. Comparison & Validation Operations

Essential for reconciliation, validation, and data integrity checks.

### 3.1 Approximate Equality Check

**Use Case**: Check if two amounts are "close enough" (within tolerance).

**Example Scenarios**:
- Reconcile accounting entries with minor rounding differences
- Validate payment amounts (accept ±$0.01 difference)
- Compare exchange rate calculations from different sources

**Proposed Function Signature**:
```rust
fn is_approximately(&self, other: Self, tolerance: Decimal) -> bool
```

**Why It's Important**: Due to rounding in calculations (especially exchange rates, percentages), two amounts that should be equal might differ by a penny or less.

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let calculated = Money::new(currency, dec!(100.01));
let expected = Money::new(currency, dec!(100.00));

// Check if within $0.05 tolerance
let is_close = calculated.is_approximately(expected, dec!(0.05));
// is_close = true

// Strict check (1 cent tolerance)
let is_exact = calculated.is_approximately(expected, dec!(0.01));
// is_exact = false (difference is exactly $0.01)

// Real-world example: Exchange rate calculation
let usd = Currency::from_iso("USD")?;
let eur_amount = Money::new(Currency::from_iso("EUR")?, dec!(85.50));
let rate = dec!(1.18);
let converted1 = Money::new(usd, eur_amount.amount() * rate);  // USD 100.89
let converted2 = Money::new(usd, dec!(100.90));  // From another source

let matches = converted1.is_approximately(converted2, dec!(0.02));
// matches = true (within 2 cent tolerance)
```

### 3.2 Sum Collection of Money

**Use Case**: Add multiple money amounts together (with currency validation).

**Example Scenarios**:
- Calculate total of line items on an invoice
- Sum daily transactions
- Total portfolio value across accounts

**Proposed Function Signature**:
```rust
fn sum(amounts: &[Self]) -> MoneyResult<Self>
```

**Why It's Important**: 
- Ensures all amounts have the same currency
- Handles overflow protection
- Provides clean API for common operation

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;

// Sum invoice line items
let items = vec![
    Money::new(currency, dec!(25.99)),  // Item 1
    Money::new(currency, dec!(15.50)),  // Item 2
    Money::new(currency, dec!(8.75)),   // Item 3
    Money::new(currency, dec!(12.00)),  // Item 4
];
let total = Money::sum(&items)?;
// total = USD 62.24

// Sum daily sales
let daily_sales = vec![
    Money::new(currency, dec!(1250.50)),  // Monday
    Money::new(currency, dec!(1450.75)),  // Tuesday
    Money::new(currency, dec!(1325.00)),  // Wednesday
    Money::new(currency, dec!(1680.25)),  // Thursday
    Money::new(currency, dec!(2100.00)),  // Friday
];
let weekly_total = Money::sum(&daily_sales)?;
// weekly_total = USD 7806.50

// Error case: Mixed currencies
let mixed = vec![
    Money::new(Currency::from_iso("USD")?, dec!(100.00)),
    Money::new(Currency::from_iso("EUR")?, dec!(85.00)),  // Different currency!
];
let result = Money::sum(&mixed);
// result = Err(MoneyError::CurrencyMismatch)
```

---

## 4. Exchange Rate Operations

Critical for international finance, multi-currency systems, and forex applications.

### 4.1 Convert to Another Currency

**Use Case**: Convert money from one currency to another using an exchange rate.

**Example Scenarios**:
- Convert USD to EUR for international payment
- Convert customer payment to base currency
- Display prices in customer's local currency

**Proposed Function Signature**:
```rust
fn convert_to(&self, target_currency: Currency, exchange_rate: Decimal) -> MoneyResult<Money>
```

**Important Notes**:
- Exchange rate is "how many target currency units per source currency unit"
- Always use checked arithmetic
- Result is rounded according to target currency's rules

**Example Usage**:
```rust
let usd = Currency::from_iso("USD")?;
let eur = Currency::from_iso("EUR")?;
let jpy = Currency::from_iso("JPY")?;

// Convert USD to EUR (rate: 1 USD = 0.85 EUR)
let dollars = Money::new(usd, dec!(100.00));
let euros = dollars.convert_to(eur, dec!(0.85))?;
// euros = EUR 85.00

// Convert USD to JPY (rate: 1 USD = 110 JPY)
let yen = dollars.convert_to(jpy, dec!(110))?;
// yen = JPY 11000 (note: JPY has 0 decimal places)

// Real-world scenario: International invoice
let invoice_usd = Money::new(usd, dec!(1500.00));
let rate_today = dec!(0.92);  // 1 USD = 0.92 EUR
let invoice_eur = invoice_usd.convert_to(eur, rate_today)?;
// invoice_eur = EUR 1380.00

// Chain conversions (USD → EUR → GBP)
let gbp = Currency::from_iso("GBP")?;
let usd_to_eur = dec!(0.92);
let eur_to_gbp = dec!(0.86);
let in_eur = dollars.convert_to(eur, usd_to_eur)?;
let in_gbp = in_eur.convert_to(gbp, eur_to_gbp)?;
// $100 → €92 → £79.12
```

### 4.2 Convert from Another Currency

**Use Case**: Reverse operation - convert TO this currency FROM another.

**Example Scenarios**:
- Calculate cost in base currency
- Aggregate multi-currency transactions
- Convert foreign expense to home currency

**Proposed Function Signature**:
```rust
fn convert_from(source: Self, target_currency: Currency, exchange_rate: Decimal) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let usd = Currency::from_iso("USD")?;
let eur = Currency::from_iso("EUR")?;

// I have EUR 85, what's that in USD? (rate: 1 EUR = 1.18 USD)
let euros = Money::new(eur, dec!(85.00));
let dollars = Money::convert_from(euros, usd, dec!(1.18))?;
// dollars = USD 100.30

// Calculate total expenses in home currency (USD)
let hotel_eur = Money::new(eur, dec!(250.00));
let hotel_usd = Money::convert_from(hotel_eur, usd, dec!(1.18))?;
// hotel_usd = USD 295.00

let meal_gbp = Money::new(Currency::from_iso("GBP")?, dec!(45.00));
let meal_usd = Money::convert_from(meal_gbp, usd, dec!(1.27))?;
// meal_usd = USD 57.15

let total_expenses = hotel_usd + meal_usd;
// total_expenses = USD 352.15
```

---

## 5. Interest Calculations

Essential for loans, savings, investments, and time-value-of-money calculations.

### 5.1 Simple Interest

**Use Case**: Calculate interest that doesn't compound (fixed rate on principal).

**Formula**: Interest = Principal × Rate × Time

**Example Scenarios**:
- Calculate interest on short-term loans
- Compute penalty interest on late payments
- Calculate simple savings account interest

**Proposed Function Signature**:
```rust
fn simple_interest(&self, rate: Decimal, periods: u32) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let principal = Money::new(currency, dec!(1000.00));

// Calculate 5% annual interest for 1 year
let interest = principal.simple_interest(dec!(0.05), 1)?;
// interest = USD 50.00
// Total = $1000 + $50 = $1050

// Calculate 5% annual interest for 3 years
let interest_3yr = principal.simple_interest(dec!(0.05), 3)?;
// interest_3yr = USD 150.00
// Total = $1000 + $150 = $1150

// Monthly interest: 1.5% per month for 12 months
let monthly_interest = principal.simple_interest(dec!(0.015), 12)?;
// monthly_interest = USD 180.00

// Penalty interest: 2% per day for 30 days late
let late_payment = Money::new(currency, dec!(500.00));
let penalty = late_payment.simple_interest(dec!(0.02), 30)?;
// penalty = USD 300.00 (ouch!)
```

### 5.2 Compound Interest

**Use Case**: Calculate interest where interest itself earns interest.

**Formula**: A = P(1 + r)^n - P

**Example Scenarios**:
- Calculate investment growth
- Compute credit card debt accumulation
- Determine future value of savings

**Proposed Function Signature**:
```rust
fn compound_interest(&self, rate: Decimal, periods: u32) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let principal = Money::new(currency, dec!(1000.00));

// 5% annual interest compounded for 1 year
let interest = principal.compound_interest(dec!(0.05), 1)?;
// interest = USD 50.00
// Total = $1050.00 (same as simple for 1 period)

// 5% annual interest compounded for 10 years
let interest_10yr = principal.compound_interest(dec!(0.05), 10)?;
// interest_10yr = USD 628.89
// Total = $1628.89 (vs $1500 with simple interest!)

// Monthly compounding: 6% annual = 0.5% per month for 12 months
let monthly_rate = dec!(0.005);
let interest_monthly = principal.compound_interest(monthly_rate, 12)?;
// interest_monthly = USD 61.68
// Total = $1061.68

// Credit card scenario: 18% APR, compounded monthly for 1 year
let balance = Money::new(currency, dec!(5000.00));
let monthly_apr = dec!(0.18) / dec!(12);  // 1.5% per month
let interest_charged = balance.compound_interest(monthly_apr, 12)?;
// interest_charged = USD 983.75
// Final balance = $5983.75
```

**Comparison Example**:
```rust
let principal = Money::new(currency, dec!(10000.00));
let rate = dec!(0.07);  // 7% annual rate

// Compare simple vs compound over 20 years
let simple = principal.simple_interest(rate, 20)?;
// simple = USD 14000.00 (total: $24,000)

let compound = principal.compound_interest(rate, 20)?;
// compound = USD 28696.84 (total: $38,696.84)

// Difference = $14,696.84 (power of compounding!)
```

---

## 6. Tax & Discount Operations

Critical for retail, e-commerce, invoicing, and accounting systems.

### 6.1 Apply Tax

**Use Case**: Calculate tax amount and total (gross amount).

**Example Scenarios**:
- Add sales tax to product price
- Calculate VAT on invoice
- Compute property tax

**Proposed Function Signature**:
```rust
fn apply_tax(&self, tax_rate: Decimal) -> MoneyResult<(Self, Self)>
// Returns: (tax_amount, total_with_tax)
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let subtotal = Money::new(currency, dec!(100.00));

// Apply 8.5% sales tax
let (tax, total) = subtotal.apply_tax(dec!(0.085))?;
// tax = USD 8.50
// total = USD 108.50

// Apply 20% VAT (common in EU)
let product_price = Money::new(currency, dec!(50.00));
let (vat, total_price) = product_price.apply_tax(dec!(0.20))?;
// vat = USD 10.00
// total_price = USD 60.00

// Invoice with multiple items and tax
let item1 = Money::new(currency, dec!(25.99));
let item2 = Money::new(currency, dec!(45.50));
let item3 = Money::new(currency, dec!(15.00));
let subtotal = item1 + item2 + item3;  // USD 86.49
let (sales_tax, invoice_total) = subtotal.apply_tax(dec!(0.0725))?;
// sales_tax = USD 6.27
// invoice_total = USD 92.76
```

### 6.2 Apply Discount

**Use Case**: Reduce price by a discount rate.

**Example Scenarios**:
- Apply coupon code discount
- Calculate sale price
- Apply bulk purchase discount

**Proposed Function Signature**:
```rust
fn apply_discount(&self, discount_rate: Decimal) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let original = Money::new(currency, dec!(100.00));

// Apply 20% off coupon
let discounted = original.apply_discount(dec!(0.20))?;
// discounted = USD 80.00

// Black Friday: 50% off
let black_friday = original.apply_discount(dec!(0.50))?;
// black_friday = USD 50.00

// Bulk discount: 15% off orders over $500
let bulk_order = Money::new(currency, dec!(750.00));
let bulk_price = bulk_order.apply_discount(dec!(0.15))?;
// bulk_price = USD 637.50

// Stacking discounts (10% member + 5% promo)
let member_price = original.apply_discount(dec!(0.10))?;  // USD 90.00
let final_price = member_price.apply_discount(dec!(0.05))?;  // USD 85.50
// Note: Stacking applies to reduced amount, not original
```

### 6.3 Extract Tax (Reverse Tax Calculation)

**Use Case**: Calculate net amount and tax from a gross (tax-inclusive) amount.

**Example Scenarios**:
- Extract VAT from European prices
- Calculate pre-tax amount from total
- Separate tax for accounting

**Proposed Function Signature**:
```rust
fn extract_tax(&self, tax_rate: Decimal) -> MoneyResult<(Self, Self)>
// Returns: (net_amount, tax_amount)
```

**Why It's Different**: This is NOT simply dividing by (1 + rate). The formula is:
- Net = Gross / (1 + rate)
- Tax = Gross - Net

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;
let gross = Money::new(currency, dec!(108.00));

// Extract 8% sales tax
let (net, tax) = gross.extract_tax(dec!(0.08))?;
// net = USD 100.00
// tax = USD 8.00
// Verify: $100.00 + 8% = $108.00 ✓

// Extract 20% VAT from European price
let eu_price = Money::new(Currency::from_iso("EUR")?, dec!(120.00));
let (net_price, vat) = eu_price.extract_tax(dec!(0.20))?;
// net_price = EUR 100.00
// vat = EUR 20.00

// Real-world: Extract tax from receipt total
let receipt_total = Money::new(currency, dec!(54.32));
let (pretax, sales_tax) = receipt_total.extract_tax(dec!(0.0875))?;
// pretax = USD 49.95
// sales_tax = USD 4.37
// Verify: $49.95 × 1.0875 = $54.32 ✓

// Multiple tax rates (federal + state)
// For $100 gross with 5% federal + 3% state:
// Total tax rate = 8%
let (base, combined_tax) = gross.extract_tax(dec!(0.08))?;
// base = USD 100.00
// combined_tax = USD 8.00
```

---

## 7. Amortization & Loan Calculations

Advanced finance operations for lending, mortgages, and investment analysis.

### 7.1 Loan Payment (PMT)

**Use Case**: Calculate periodic payment amount for a loan.

**Formula**: PMT = P × [r(1+r)^n] / [(1+r)^n - 1]
Where: P = principal, r = rate per period, n = number of periods

**Example Scenarios**:
- Calculate monthly mortgage payment
- Determine auto loan installment
- Calculate student loan payment

**Proposed Function Signature**:
```rust
fn loan_payment(&self, rate: Decimal, periods: u32) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;

// 30-year mortgage: $300,000 at 4% annual (0.333% monthly)
let principal = Money::new(currency, dec!(300000.00));
let monthly_rate = dec!(0.04) / dec!(12);  // 0.00333...
let months = 30 * 12;  // 360 months
let payment = principal.loan_payment(monthly_rate, months)?;
// payment = USD 1432.25 per month

// Auto loan: $25,000 at 6% for 5 years
let car_price = Money::new(currency, dec!(25000.00));
let auto_rate = dec!(0.06) / dec!(12);  // 0.5% monthly
let auto_months = 5 * 12;  // 60 months
let car_payment = car_price.loan_payment(auto_rate, auto_months)?;
// car_payment = USD 483.32 per month

// Student loan: $50,000 at 5.5% for 10 years
let student_loan = Money::new(currency, dec!(50000.00));
let student_rate = dec!(0.055) / dec!(12);
let student_months = 10 * 12;
let student_payment = student_loan.loan_payment(student_rate, student_months)?;
// student_payment = USD 542.89 per month

// Total interest paid over life of loan
let total_paid = student_payment.mul(dec!(120))?;  // 120 months
let total_interest = total_paid - student_loan;
// total_paid = USD 65,146.80
// total_interest = USD 15,146.80
```

### 7.2 Present Value (PV)

**Use Case**: Calculate current value of a future amount.

**Formula**: PV = FV / (1 + r)^n

**Example Scenarios**:
- Determine how much to invest today for future goal
- Calculate bond value
- Value future cash flows

**Proposed Function Signature**:
```rust
fn present_value(future_value: Self, rate: Decimal, periods: u32) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;

// How much to invest today to have $10,000 in 5 years at 6%?
let future = Money::new(currency, dec!(10000.00));
let present = Money::present_value(future, dec!(0.06), 5)?;
// present = USD 7472.58

// College fund: Need $100,000 in 18 years at 7% annual return
let college_goal = Money::new(currency, dec!(100000.00));
let need_today = Money::present_value(college_goal, dec!(0.07), 18)?;
// need_today = USD 29,586.26

// Lottery: $1 million in 20 years vs cash today?
// At 5% discount rate, what's it worth now?
let future_million = Money::new(currency, dec!(1000000.00));
let present_value = Money::present_value(future_million, dec!(0.05), 20)?;
// present_value = USD 376,889.48
// So a $400k cash option might be better!
```

### 7.3 Future Value (FV)

**Use Case**: Calculate future value of present investment.

**Formula**: FV = PV × (1 + r)^n

**Example Scenarios**:
- Project retirement savings growth
- Estimate investment returns
- Calculate future value of annuity

**Proposed Function Signature**:
```rust
fn future_value(&self, rate: Decimal, periods: u32) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;

// Invest $5,000 today for 10 years at 8% annual return
let investment = Money::new(currency, dec!(5000.00));
let future = investment.future_value(dec!(0.08), 10)?;
// future = USD 10,794.62

// Retirement: $100,000 invested for 30 years at 7%
let retirement = Money::new(currency, dec!(100000.00));
let at_retirement = retirement.future_value(dec!(0.07), 30)?;
// at_retirement = USD 761,225.50

// College savings: $10,000 today, 18 years at 6%
let college = Money::new(currency, dec!(10000.00));
let college_future = college.future_value(dec!(0.06), 18)?;
// college_future = USD 28,543.39

// Compare investments with different rates
let amount = Money::new(currency, dec!(1000.00));
let conservative = amount.future_value(dec!(0.04), 20)?;  // USD 2191.12
let moderate = amount.future_value(dec!(0.07), 20)?;      // USD 3869.68
let aggressive = amount.future_value(dec!(0.10), 20)?;    // USD 6727.50
```

---

## 8. Advanced Operations

Additional operations for specialized use cases.

### 8.1 Round to Cash

**Use Case**: Round to actual cash denominations (some currencies don't use smallest units).

**Example Scenarios**:
- Round to nearest nickel (no pennies in some countries)
- Round to nearest 10 cents
- Round to nearest dollar

**Proposed Function Signature**:
```rust
fn round_to_cash(&self) -> Self
```

**Example Usage**:
```rust
let currency = Currency::from_iso("CAD")?;  // Canadian Dollar

// Canada eliminated pennies - round to nearest nickel
let price1 = Money::new(currency, dec!(1.97));
let cash1 = price1.round_to_cash();
// cash1 = CAD 1.95 (rounded down)

let price2 = Money::new(currency, dec!(1.98));
let cash2 = price2.round_to_cash();
// cash2 = CAD 2.00 (rounded up)

// Sweden: Round to nearest krona (no öre coins)
let sek = Currency::from_iso("SEK")?;
let price_sek = Money::new(sek, dec!(19.80));
let cash_sek = price_sek.round_to_cash();
// cash_sek = SEK 20.00
```

### 8.2 Average

**Use Case**: Calculate mean of multiple money amounts.

**Example Scenarios**:
- Calculate average transaction size
- Determine average salary
- Compute average daily sales

**Proposed Function Signature**:
```rust
fn average(amounts: &[Self]) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;

// Average transaction size
let transactions = vec![
    Money::new(currency, dec!(25.50)),
    Money::new(currency, dec!(48.75)),
    Money::new(currency, dec!(15.25)),
    Money::new(currency, dec!(67.00)),
    Money::new(currency, dec!(33.50)),
];
let avg = Money::average(&transactions)?;
// avg = USD 38.00

// Average salary in department
let salaries = vec![
    Money::new(currency, dec!(55000.00)),
    Money::new(currency, dec!(62000.00)),
    Money::new(currency, dec!(58000.00)),
    Money::new(currency, dec!(71000.00)),
    Money::new(currency, dec!(54000.00)),
];
let avg_salary = Money::average(&salaries)?;
// avg_salary = USD 60000.00
```

### 8.3 Median

**Use Case**: Find middle value (not affected by outliers like average).

**Example Scenarios**:
- Find median home price
- Calculate median salary (better than average for skewed data)
- Determine typical transaction size

**Proposed Function Signature**:
```rust
fn median(amounts: &[Self]) -> MoneyResult<Self>
```

**Example Usage**:
```rust
let currency = Currency::from_iso("USD")?;

// Median salary (better than average when there are outliers)
let salaries = vec![
    Money::new(currency, dec!(45000.00)),
    Money::new(currency, dec!(52000.00)),
    Money::new(currency, dec!(58000.00)),
    Money::new(currency, dec!(61000.00)),
    Money::new(currency, dec!(250000.00)),  // Outlier!
];
let average = Money::average(&salaries)?;
// average = USD 93200.00 (skewed by outlier)

let median = Money::median(&salaries)?;
// median = USD 58000.00 (more representative)

// Home prices in neighborhood
let prices = vec![
    Money::new(currency, dec!(350000.00)),
    Money::new(currency, dec!(385000.00)),
    Money::new(currency, dec!(420000.00)),
    Money::new(currency, dec!(395000.00)),
    Money::new(currency, dec!(1200000.00)),  // Mansion!
];
let median_price = Money::median(&prices)?;
// median_price = USD 395000.00 (typical home)
```

---

## Implementation Notes

### Error Handling

All operations should return `MoneyResult<T>` to handle:
- **Currency Mismatches**: Operations between different currencies
- **Arithmetic Overflow**: Very large calculations
- **Invalid Arguments**: Negative splits, zero periods, invalid percentages
- **Division by Zero**: In percentage calculations

### Rounding Considerations

- All operations should respect the currency's `minor_unit` (decimal places)
- Use the currency's `rounding_strategy` (banker's rounding by default)
- For splits/allocations, ensure sum of parts equals original (no money lost!)

### Performance

- Use `checked_*` arithmetic operations throughout
- Avoid floating-point arithmetic (use `Decimal` only)
- Consider caching for expensive operations (compound interest, loan payments)

### Testing Strategy

Each operation should have tests for:
- Basic functionality with common values
- Edge cases (zero amounts, very small amounts, very large amounts)
- Currency mismatch errors
- Overflow scenarios
- Rounding precision (ensure no money is lost/created)
- Real-world scenarios with actual currency values

---

## Priority Implementation Roadmap

### Phase 1: Essential Operations (Weeks 1-2)
1. Percentage calculations (all 4 functions)
2. Split operations (equal split, allocate by ratios)
3. Sum operation

**Rationale**: These are used in almost every financial application.

### Phase 2: Common Operations (Weeks 3-4)
4. Approximate equality
5. Apply tax / Apply discount
6. Exchange rate conversions

**Rationale**: Core functionality for retail, e-commerce, and multi-currency apps.

### Phase 3: Advanced Finance (Weeks 5-6)
7. Interest calculations (simple, compound)
8. Extract tax (reverse calculation)
9. Loan payment calculation

**Rationale**: Needed for lending, investment, and sophisticated financial apps.

### Phase 4: Specialized Operations (As Needed)
10. Present value / Future value
11. Round to cash
12. Statistical operations (average, median)

**Rationale**: Use-case specific, implement based on user demand.

---

## Conclusion

These operations transform moneylib from a basic money type library into a comprehensive financial toolkit. The examples demonstrate real-world applications across:

- **Retail & E-commerce**: Taxes, discounts, pricing
- **Accounting**: Allocations, reconciliation, aggregation
- **Banking**: Loans, interest, payments
- **Investments**: Future value, present value, returns
- **International**: Exchange rates, multi-currency
- **General Finance**: Bill splitting, percentage calculations, comparisons

Each operation is designed with safety, precision, and real-world use cases in mind, maintaining the library's excellent type safety and error handling standards.
