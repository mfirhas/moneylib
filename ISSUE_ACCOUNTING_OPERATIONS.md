# Accounting Operations

## Issue Description

This issue tracks the implementation of 20 essential finance operations for the moneylib library. Below are detailed explanations and code samples for each operation.

---

## 1. PERCENTAGE OPERATIONS

### 1.1 Calculate Percentage of Amount (`percentage`)

**What it does:** Calculates what a certain percentage of a money amount equals.

**Real-world uses:**
- Calculate 5% sales tax on $100 purchase → $5.00
- Calculate 15% tip on $80 restaurant bill → $12.00
- Calculate 20% down payment on $300,000 house → $60,000

**Code Sample:**
```rust
let bill = Money::new(currency, dec!(100.00));
let tip = bill.percentage(dec!(15))?;  // Calculate 15% tip
// Result: USD 15.00

let house_price = Money::new(currency, dec!(300000.00));
let down_payment = house_price.percentage(dec!(20))?;  // 20% down
// Result: USD 60,000.00
```

---

### 1.2 Add Percentage (`add_percentage`)

**What it does:** Increases an amount by a percentage (original + percentage of original).

**Real-world uses:**
- Add 8% sales tax to $100 → $108.00 total
- Apply 25% markup to $50 cost → $62.50 retail price
- Add 3% credit card fee to $200 → $206.00 total

**Code Sample:**
```rust
let base_price = Money::new(currency, dec!(100.00));
let with_tax = base_price.add_percentage(dec!(8))?;  // Add 8% tax
// Result: USD 108.00 (not just $8, but $100 + $8)

// Chaining: cost → +100% markup → +8% tax
let cost = Money::new(currency, dec!(50.00));
let retail = cost.add_percentage(dec!(100))?;      // $100.00
let final_price = retail.add_percentage(dec!(8))?;  // $108.00
```

---

### 1.3 Subtract Percentage (`subtract_percentage`)

**What it does:** Reduces an amount by a percentage (original - percentage of original).

**Real-world uses:**
- Apply 20% discount on $200 item → $160.00 sale price
- 10% employee discount on $80 → $72.00
- 50% clearance sale on $150 → $75.00

**Code Sample:**
```rust
let original = Money::new(currency, dec!(200.00));
let sale_price = original.subtract_percentage(dec!(20))?;  // 20% off
// Result: USD 160.00

// Multiple discounts (10% member + 5% coupon)
let member_price = original.subtract_percentage(dec!(10))?;  // $180.00
let final_price = member_price.subtract_percentage(dec!(5))?;  // $171.00
// Note: Discounts stack multiplicatively, not additively
```

---

### 1.4 Calculate What Percentage (`percentage_of`)

**What it does:** Determines what percentage one amount is of another.

**Real-world uses:**
- Sale price $160 of original $200 → 80% (meaning 20% off)
- Spent $750 of $1000 budget → 75% used
- Profit $40 on cost $60 → 166.67% (66.67% profit margin)

**Code Sample:**
```rust
let original = Money::new(currency, dec!(200.00));
let sale = Money::new(currency, dec!(160.00));
let pct = sale.percentage_of(original)?;
// Result: 80.0 (the sale price is 80% of original)

let budget = Money::new(currency, dec!(1000.00));
let spent = Money::new(currency, dec!(750.00));
let usage = spent.percentage_of(budget)?;
// Result: 75.0 (used 75% of budget)
```

---

## 2. SPLIT & ALLOCATION OPERATIONS

### 2.1 Split Into Equal Parts (`split`)

**What it does:** Divides money equally among N people/accounts, handling remainders fairly.

**Why it's complex:** When splitting $10.00 into 3 parts, you can't give everyone $3.33 because $3.33 × 3 = $9.99. The algorithm must give someone $3.34 to make the total exactly $10.00.

**Real-world uses:**
- Split $100 restaurant bill among 3 friends → [$33.34, $33.33, $33.33]
- Divide $500 refund among 4 accounts → [$125.00, $125.00, $125.00, $125.00]
- Split $0.10 among 3 people → [$0.04, $0.03, $0.03]

**Code Sample:**
```rust
let bill = Money::new(currency, dec!(100.00));
let shares = bill.split(3)?;
// Result: [USD 33.34, USD 33.33, USD 33.33]
// Total: $100.00 exactly (no money lost or created!)

let small = Money::new(currency, dec!(0.10));
let tiny_shares = small.split(3)?;
// Result: [USD 0.04, USD 0.03, USD 0.03]
// The algorithm puts the remainder in the first person's share
```

---

### 2.2 Allocate by Ratios (`allocate`)

**What it does:** Divides money according to specific ratios/weights.

**Real-world uses:**
- Split $10,000 profit 60/40 between partners → [$6,000, $4,000]
- Allocate $5,000 by ownership 50/30/20 → [$2,500, $1,500, $1,000]
- Budget split 35/25/20/15/5 for 5 departments

**Code Sample:**
```rust
let profit = Money::new(currency, dec!(10000.00));
let shares = profit.allocate(&[60, 40])?;  // 60/40 split
// Result: [USD 6000.00, USD 4000.00]

// Unequal ratios: 1:2:1 means 25%, 50%, 25%
let amount = Money::new(currency, dec!(400.00));
let parts = amount.allocate(&[1, 2, 1])?;
// Result: [USD 100.00, USD 200.00, USD 100.00]

// Budget allocation by priority weights
let budget = Money::new(currency, dec!(100000.00));
let depts = budget.allocate(&[35, 25, 20, 15, 5])?;
// Result: [USD 35000.00, USD 25000.00, USD 20000.00, USD 15000.00, USD 5000.00]
```

---

### 2.3 Allocate by Exact Percentages (`allocate_by_percentages`)

**What it does:** Allocates using exact percentages that must sum to 100%.

**Real-world uses:**
- Tax withholding: 22% federal, 5% state, 1.5% local from $5000 salary
- Portfolio: 60% stocks, 30% bonds, 10% cash
- Insurance split by coverage type

**Code Sample:**
```rust
let salary = Money::new(currency, dec!(5000.00));
let withholdings = salary.allocate_by_percentages(&[
    dec!(22.0),   // Federal tax
    dec!(5.0),    // State tax
    dec!(1.5),    // Local tax
])?;
// Result: [USD 1100.00, USD 250.00, USD 75.00]
// Note: This would error if percentages don't sum to exactly 100%!

let investment = Money::new(currency, dec!(100000.00));
let allocation = investment.allocate_by_percentages(&[
    dec!(60.0),   // Stocks
    dec!(30.0),   // Bonds
    dec!(10.0),   // Cash
])?;
// Result: [USD 60000.00, USD 30000.00, USD 10000.00]
```

---

### 2.4 Allocate with Remainder (`allocate_with_remainder`)

**What it does:** Like split, but you control which person/account gets the extra pennies.

**Real-world uses:**
- Restaurant bill: give remainder to person who paid
- Accounting: give remainder to specific account per company policy
- Beneficiary: give remainder to primary heir

**Code Sample:**
```rust
let amount = Money::new(currency, dec!(10.00));

// Remainder goes to first person (index 0)
let split_first = amount.allocate_with_remainder(3, 0)?;
// Result: [USD 3.34, USD 3.33, USD 3.33]

// Remainder goes to last person (index 2)
let split_last = amount.allocate_with_remainder(3, 2)?;
// Result: [USD 3.33, USD 3.33, USD 3.34]

// Remainder goes to middle person (index 1)
let split_middle = amount.allocate_with_remainder(3, 1)?;
// Result: [USD 3.33, USD 3.34, USD 3.33]
```

---

## 3. COMPARISON & VALIDATION OPERATIONS

### 3.1 Approximate Equality (`is_approximately`)

**What it does:** Checks if two amounts are "close enough" within a tolerance.

**Why it's needed:** Due to rounding in calculations (especially with exchange rates or percentages), two amounts that should theoretically be equal might differ by a penny or less.

**Real-world uses:**
- Reconciling accounting entries (accept ±$0.01 difference)
- Validating payments (transaction might be off by 1-2 cents)
- Comparing exchange rate calculations from different sources

**Code Sample:**
```rust
let calculated = Money::new(currency, dec!(100.01));
let expected = Money::new(currency, dec!(100.00));

// Check within $0.05 tolerance
let is_close = calculated.is_approximately(expected, dec!(0.05));
// Result: true (difference is only $0.01)

// Strict check within 1 cent
let is_exact = calculated.is_approximately(expected, dec!(0.01));
// Result: false (difference is exactly $0.01, not less than $0.01)

// Exchange rate reconciliation
let converted1 = Money::new(currency, dec!(100.89));  // From source 1
let converted2 = Money::new(currency, dec!(100.90));  // From source 2
let matches = converted1.is_approximately(converted2, dec!(0.02));
// Result: true (within 2 cent tolerance)
```

---

### 3.2 Sum Collection (`sum`)

**What it does:** Adds multiple money amounts together, ensuring all have same currency.

**Real-world uses:**
- Total invoice line items
- Sum daily transactions
- Calculate portfolio total across accounts

**Code Sample:**
```rust
// Sum invoice line items
let items = vec![
    Money::new(currency, dec!(25.99)),
    Money::new(currency, dec!(15.50)),
    Money::new(currency, dec!(8.75)),
    Money::new(currency, dec!(12.00)),
];
let total = Money::sum(&items)?;
// Result: USD 62.24

// Sum weekly sales
let daily_sales = vec![
    Money::new(currency, dec!(1250.50)),  // Monday
    Money::new(currency, dec!(1450.75)),  // Tuesday
    Money::new(currency, dec!(1680.25)),  // Wednesday
];
let weekly = Money::sum(&daily_sales)?;
// Result: USD 4381.50

// Error case: Mixed currencies
let mixed = vec![
    Money::new(Currency::from_iso("USD")?, dec!(100.00)),
    Money::new(Currency::from_iso("EUR")?, dec!(85.00)),
];
let result = Money::sum(&mixed);
// Result: Err(MoneyError::CurrencyMismatch)
```

---

## 4. EXCHANGE RATE OPERATIONS

### 4.1 Convert To Currency (`convert_to`)

**What it does:** Converts money from one currency to another using an exchange rate.

**Important:** Exchange rate means "how many target units per 1 source unit"

**Real-world uses:**
- Convert $100 USD to EUR at rate 0.85 → €85.00
- Convert $100 USD to JPY at rate 110 → ¥11,000
- International payments, price display

**Code Sample:**
```rust
let usd = Currency::from_iso("USD")?;
let eur = Currency::from_iso("EUR")?;
let jpy = Currency::from_iso("JPY")?;

let dollars = Money::new(usd, dec!(100.00));

// Convert to EUR (rate: 1 USD = 0.85 EUR)
let euros = dollars.convert_to(eur, dec!(0.85))?;
// Result: EUR 85.00

// Convert to JPY (rate: 1 USD = 110 JPY)
let yen = dollars.convert_to(jpy, dec!(110))?;
// Result: JPY 11000 (note: JPY has 0 decimal places)

// Real invoice conversion
let invoice_usd = Money::new(usd, dec!(1500.00));
let rate_today = dec!(0.92);  // Today's rate
let invoice_eur = invoice_usd.convert_to(eur, rate_today)?;
// Result: EUR 1380.00

// Chain conversions: USD → EUR → GBP
let gbp = Currency::from_iso("GBP")?;
let in_eur = dollars.convert_to(eur, dec!(0.92))?;     // €92
let in_gbp = in_eur.convert_to(gbp, dec!(0.86))?;      // £79.12
```

---

### 4.2 Convert From Currency (`convert_from`)

**What it does:** Converts TO this currency FROM another (reverse direction).

**Real-world uses:**
- "I have €85, what's that in USD?" at rate 1.18
- Aggregate foreign expenses into home currency
- Calculate cost in base currency

**Code Sample:**
```rust
let usd = Currency::from_iso("USD")?;
let eur = Currency::from_iso("EUR")?;
let gbp = Currency::from_iso("GBP")?;

// I have €85, convert to USD (rate: 1 EUR = 1.18 USD)
let euros = Money::new(eur, dec!(85.00));
let dollars = Money::convert_from(euros, usd, dec!(1.18))?;
// Result: USD 100.30

// Aggregate travel expenses in home currency (USD)
let hotel_eur = Money::new(eur, dec!(250.00));
let hotel_usd = Money::convert_from(hotel_eur, usd, dec!(1.18))?;
// Result: USD 295.00

let meal_gbp = Money::new(gbp, dec!(45.00));
let meal_usd = Money::convert_from(meal_gbp, usd, dec!(1.27))?;
// Result: USD 57.15

let total_expenses = hotel_usd + meal_usd;
// Result: USD 352.15
```

---

## 5. INTEREST CALCULATIONS

### 5.1 Simple Interest (`simple_interest`)

**What it does:** Calculates interest that doesn't compound (fixed rate on principal only).

**Formula:** Interest = Principal × Rate × Periods

**Real-world uses:**
- Short-term loans
- Penalty interest on late payments
- Simple savings accounts

**Code Sample:**
```rust
let principal = Money::new(currency, dec!(1000.00));

// 5% annual interest for 1 year
let interest = principal.simple_interest(dec!(0.05), 1)?;
// Result: USD 50.00
// Total: $1000 + $50 = $1050

// 5% annual interest for 3 years
let interest_3yr = principal.simple_interest(dec!(0.05), 3)?;
// Result: USD 150.00
// Total: $1000 + $150 = $1150

// Monthly interest: 1.5% per month for 12 months
let monthly_interest = principal.simple_interest(dec!(0.015), 12)?;
// Result: USD 180.00

// Penalty: 2% per day for 30 days late
let late_payment = Money::new(currency, dec!(500.00));
let penalty = late_payment.simple_interest(dec!(0.02), 30)?;
// Result: USD 300.00 (ouch! $500 × 0.02 × 30)
```

---

### 5.2 Compound Interest (`compound_interest`)

**What it does:** Calculates interest where interest itself earns interest.

**Formula:** Interest = Principal × (1 + rate)^periods - Principal

**Real-world uses:**
- Investment growth
- Credit card debt accumulation
- Savings accounts

**Code Sample:**
```rust
let principal = Money::new(currency, dec!(1000.00));

// 5% annual, compounded for 1 year
let interest = principal.compound_interest(dec!(0.05), 1)?;
// Result: USD 50.00 (same as simple for 1 period)

// 5% annual, compounded for 10 years
let interest_10yr = principal.compound_interest(dec!(0.05), 10)?;
// Result: USD 628.89
// Total: $1628.89 (vs $1500 with simple interest!)

// Credit card: 18% APR compounded monthly for 1 year
let balance = Money::new(currency, dec!(5000.00));
let monthly_rate = dec!(0.015);  // 18% / 12 = 1.5% per month
let interest_charged = balance.compound_interest(monthly_rate, 12)?;
// Result: USD 983.75
// Final balance: $5983.75

// Comparison over 20 years at 7%
let invest = Money::new(currency, dec!(10000.00));
let simple = invest.simple_interest(dec!(0.07), 20)?;
// Simple: USD 14000 (total: $24,000)
let compound = invest.compound_interest(dec!(0.07), 20)?;
// Compound: USD 28696.84 (total: $38,696.84)
// Difference: $14,696.84 - the power of compounding!
```

---

## 6. TAX & DISCOUNT OPERATIONS

### 6.1 Apply Tax (`apply_tax`)

**What it does:** Calculates tax amount and total (gross amount) from a net amount.

**Returns:** Tuple of (tax_amount, total_with_tax)

**Real-world uses:**
- Add sales tax to product price
- Calculate VAT on invoice
- Compute property tax

**Code Sample:**
```rust
let subtotal = Money::new(currency, dec!(100.00));

// Apply 8.5% sales tax
let (tax, total) = subtotal.apply_tax(dec!(0.085))?;
// tax = USD 8.50
// total = USD 108.50

// Apply 20% VAT (common in EU)
let product = Money::new(currency, dec!(50.00));
let (vat, price_with_vat) = product.apply_tax(dec!(0.20))?;
// vat = USD 10.00
// price_with_vat = USD 60.00

// Multi-item invoice with tax
let item1 = Money::new(currency, dec!(25.99));
let item2 = Money::new(currency, dec!(45.50));
let subtotal = item1 + item2;  // USD 71.49
let (sales_tax, total) = subtotal.apply_tax(dec!(0.0725))?;
// sales_tax = USD 5.18
// total = USD 76.67
```

---

### 6.2 Apply Discount (`apply_discount`)

**What it does:** Reduces price by a discount percentage.

**Real-world uses:**
- Apply coupon code
- Calculate sale price
- Bulk purchase discount

**Code Sample:**
```rust
let original = Money::new(currency, dec!(100.00));

// 20% off coupon
let sale_price = original.apply_discount(dec!(0.20))?;
// Result: USD 80.00

// Black Friday: 50% off
let black_friday = original.apply_discount(dec!(0.50))?;
// Result: USD 50.00

// Bulk discount: 15% off on $750 order
let bulk_order = Money::new(currency, dec!(750.00));
let discounted = bulk_order.apply_discount(dec!(0.15))?;
// Result: USD 637.50

// Stacking discounts (10% member + 5% promo)
let step1 = original.apply_discount(dec!(0.10))?;  // USD 90.00
let step2 = step1.apply_discount(dec!(0.05))?;     // USD 85.50
// Note: Applies to already-discounted price
// Not 15% off original! (that would be $85)
```

---

### 6.3 Extract Tax (Reverse Tax) (`extract_tax`)

**What it does:** Separates net amount and tax from a gross (tax-inclusive) amount.

**Returns:** Tuple of (net_amount, tax_amount)

**Why it's different:** You can't just subtract 8% from $108. The formula is:
- Net = Gross / (1 + tax_rate)
- Tax = Gross - Net

**Real-world uses:**
- Extract VAT from European prices (which include VAT)
- Calculate pre-tax amount from receipt
- Separate tax for accounting

**Code Sample:**
```rust
let gross = Money::new(currency, dec!(108.00));

// Extract 8% sales tax
let (net, tax) = gross.extract_tax(dec!(0.08))?;
// net = USD 100.00
// tax = USD 8.00
// Verify: $100 + 8% = $100 × 1.08 = $108 ✓

// Extract 20% VAT from European price
let eu_price = Money::new(eur_currency, dec!(120.00));
let (net_price, vat) = eu_price.extract_tax(dec!(0.20))?;
// net_price = EUR 100.00
// vat = EUR 20.00
// Verify: €100 × 1.20 = €120 ✓

// Real receipt: Extract 8.75% tax from $54.32 total
let receipt = Money::new(currency, dec!(54.32));
let (pretax, sales_tax) = receipt.extract_tax(dec!(0.0875))?;
// pretax = USD 49.95
// sales_tax = USD 4.37
// Verify: $49.95 × 1.0875 = $54.32 ✓

// Why you can't just subtract 8%:
// Wrong: $108 - 8% = $108 - $8.64 = $99.36 ❌
// Right: $108 / 1.08 = $100.00, then $100 × 0.08 = $8.00 ✓
```

---

## 7. AMORTIZATION & LOAN CALCULATIONS

### 7.1 Loan Payment / PMT (`loan_payment`)

**What it does:** Calculates periodic payment amount for a loan.

**Formula:** PMT = Principal × [r(1+r)^n] / [(1+r)^n - 1]

**Real-world uses:**
- Monthly mortgage payment
- Auto loan installment
- Student loan payment

**Code Sample:**
```rust
// 30-year mortgage: $300,000 at 4% annual
let principal = Money::new(currency, dec!(300000.00));
let monthly_rate = dec!(0.04) / dec!(12);  // 0.00333...
let months = 30 * 12;  // 360 months
let payment = principal.loan_payment(monthly_rate, months)?;
// Result: USD 1432.25 per month

// Auto loan: $25,000 at 6% for 5 years
let car_price = Money::new(currency, dec!(25000.00));
let auto_rate = dec!(0.06) / dec!(12);  // 0.5% monthly
let car_payment = car_price.loan_payment(auto_rate, 60)?;
// Result: USD 483.32 per month
// Total paid: $483.32 × 60 = $28,999.20
// Interest paid: $28,999.20 - $25,000 = $3,999.20

// Student loan: $50,000 at 5.5% for 10 years
let loan = Money::new(currency, dec!(50000.00));
let rate = dec!(0.055) / dec!(12);
let payment = loan.loan_payment(rate, 120)?;
// Result: USD 542.89 per month
// Total paid: $542.89 × 120 = $65,146.80
// Interest: $15,146.80
```

---

### 7.2 Present Value / PV (`present_value`)

**What it does:** Calculates current value of a future amount (time value of money).

**Formula:** PV = FV / (1 + rate)^periods

**Real-world uses:**
- How much to invest today for future goal
- Bond valuation
- Lottery: lump sum vs annuity decision

**Code Sample:**
```rust
// Goal: Have $10,000 in 5 years at 6% return
// How much to invest today?
let future = Money::new(currency, dec!(10000.00));
let present = Money::present_value(future, dec!(0.06), 5)?;
// Result: USD 7472.58
// Meaning: Invest $7,472.58 today → $10,000 in 5 years

// College fund: Need $100,000 in 18 years at 7%
let goal = Money::new(currency, dec!(100000.00));
let need_now = Money::present_value(goal, dec!(0.07), 18)?;
// Result: USD 29,586.26

// Lottery example: $1M in 20 years vs cash now
// At 5% discount rate, what's it worth today?
let future_million = Money::new(currency, dec!(1000000.00));
let present_value = Money::present_value(future_million, dec!(0.05), 20)?;
// Result: USD 376,889.48
// So if offered $400k cash today, take it!
```

---

### 7.3 Future Value / FV (`future_value`)

**What it does:** Calculates future value of present investment.

**Formula:** FV = PV × (1 + rate)^periods

**Real-world uses:**
- Project retirement savings
- Estimate investment returns
- Calculate growth of single deposit

**Code Sample:**
```rust
// Invest $5,000 today for 10 years at 8%
let investment = Money::new(currency, dec!(5000.00));
let future = investment.future_value(dec!(0.08), 10)?;
// Result: USD 10,794.62
// Your $5,000 becomes $10,794.62

// Retirement: $100,000 invested for 30 years at 7%
let retirement = Money::new(currency, dec!(100000.00));
let at_retirement = retirement.future_value(dec!(0.07), 30)?;
// Result: USD 761,225.50
// Amazing growth over 30 years!

// College: $10,000 today, 18 years at 6%
let college = Money::new(currency, dec!(10000.00));
let college_future = college.future_value(dec!(0.06), 18)?;
// Result: USD 28,543.39

// Compare investment strategies over 20 years
let seed = Money::new(currency, dec!(1000.00));
let conservative = seed.future_value(dec!(0.04), 20)?;  // USD 2191.12
let moderate = seed.future_value(dec!(0.07), 20)?;      // USD 3869.68
let aggressive = seed.future_value(dec!(0.10), 20)?;    // USD 6727.50
```

---

## 8. ADVANCED OPERATIONS

### 8.1 Round to Cash (`round_to_cash`)

**What it does:** Rounds to actual cash denominations (some countries don't use smallest units).

**Real-world uses:**
- Canada: Round to nearest 5¢ (no pennies since 2013)
- Sweden: Round to nearest krona (no öre)
- Switzerland: Round to nearest 5 centimes

**Code Sample:**
```rust
// Canada eliminated pennies - round to nickel
let cad = Currency::from_iso("CAD")?;

let price1 = Money::new(cad, dec!(1.97));
let cash1 = price1.round_to_cash();
// Result: CAD 1.95 (rounds down to nearest 5¢)

let price2 = Money::new(cad, dec!(1.98));
let cash2 = price2.round_to_cash();
// Result: CAD 2.00 (rounds up to nearest 5¢)

let price3 = Money::new(cad, dec!(5.92));
let cash3 = price3.round_to_cash();
// Result: CAD 5.90

let price4 = Money::new(cad, dec!(5.93));
let cash4 = price4.round_to_cash();
// Result: CAD 5.95

// Sweden: No öre coins, round to krona
let sek = Currency::from_iso("SEK")?;
let price_sek = Money::new(sek, dec!(19.80));
let cash_sek = price_sek.round_to_cash();
// Result: SEK 20.00
```

---

### 8.2 Average (`average`)

**What it does:** Calculates arithmetic mean of money amounts.

**Real-world uses:**
- Average transaction size
- Mean salary
- Average daily sales

**Code Sample:**
```rust
// Average transaction size
let transactions = vec![
    Money::new(currency, dec!(25.50)),
    Money::new(currency, dec!(48.75)),
    Money::new(currency, dec!(15.25)),
    Money::new(currency, dec!(67.00)),
    Money::new(currency, dec!(33.50)),
];
let avg = Money::average(&transactions)?;
// Result: USD 38.00
// (25.50 + 48.75 + 15.25 + 67.00 + 33.50) / 5 = 190 / 5 = 38

// Average department salary
let salaries = vec![
    Money::new(currency, dec!(55000.00)),
    Money::new(currency, dec!(62000.00)),
    Money::new(currency, dec!(58000.00)),
    Money::new(currency, dec!(71000.00)),
    Money::new(currency, dec!(54000.00)),
];
let avg_salary = Money::average(&salaries)?;
// Result: USD 60,000.00
```

---

### 8.3 Median (`median`)

**What it does:** Finds middle value (not affected by outliers).

**Why it's better than average:** When data has outliers (very high or low values), median represents "typical" better than average.

**Real-world uses:**
- Median home price (better than average)
- Median salary (not skewed by executives)
- Typical transaction size

**Code Sample:**
```rust
// Salary comparison: Average vs Median
let salaries = vec![
    Money::new(currency, dec!(45000.00)),
    Money::new(currency, dec!(52000.00)),
    Money::new(currency, dec!(58000.00)),
    Money::new(currency, dec!(61000.00)),
    Money::new(currency, dec!(250000.00)),  // Executive (outlier!)
];

let average = Money::average(&salaries)?;
// Result: USD 93,200.00 (skewed high by executive)

let median = Money::median(&salaries)?;
// Result: USD 58,000.00 (middle value, more representative)

// Home prices in neighborhood
let prices = vec![
    Money::new(currency, dec!(350000.00)),
    Money::new(currency, dec!(385000.00)),
    Money::new(currency, dec!(420000.00)),
    Money::new(currency, dec!(395000.00)),
    Money::new(currency, dec!(1200000.00)),  // Mansion!
];

let avg_price = Money::average(&prices)?;
// Result: USD 550,000.00 (skewed by mansion)

let median_price = Money::median(&prices)?;
// Result: USD 395,000.00 (typical home)

// Even number of values: median is average of middle two
let values = vec![
    Money::new(currency, dec!(10.00)),
    Money::new(currency, dec!(20.00)),
    Money::new(currency, dec!(30.00)),
    Money::new(currency, dec!(40.00)),
];
let median = Money::median(&values)?;
// Result: USD 25.00 (average of 20 and 30)
```

---

## Implementation Notes

Each operation should:
- ✅ Handle rounding correctly (no money lost/created)
- ✅ Use checked arithmetic (no overflow)
- ✅ Validate inputs (currency matching, valid percentages)
- ✅ Return `Result` types (proper error handling)
- ✅ Respect currency rules (decimal places, rounding strategy)

## Application Domains

These 20 operations cover essential functionality for:

- **Retail & E-commerce:** Taxes, discounts, pricing
- **Bill Splitting:** Restaurant bills, shared expenses
- **Accounting:** Allocations, reconciliation, aggregation
- **Banking:** Loans, interest, payments
- **Investments:** PV, FV, returns
- **International:** Multi-currency conversions
- **Analysis:** Statistics, comparisons
