use crate::{BaseMoney, Decimal};

const ESCAPE_SYMBOL: char = '\\';

const AMOUNT_FORMAT_SYMBOL: char = 'a';
const CODE_FORMAT_SYMBOL: char = 'c';
const SYMBOL_FORMAT_SYMBOL: char = 's';
const MINOR_FORMAT_SYMBOL: char = 'm';
const NEGATIVE_FORMAT_SYMBOL: char = 'n';

pub(crate) static FORMAT_SYMBOLS: &[char] = &[
    'a', // amount
    'c', // currency code
    's', // currency symbol
    'm', // minor symbol
    'n', // negative sign
];

pub(crate) const CODE_FORMAT_POSITIVE: &str = "c a"; // E.g. USD 1,000.23
pub(crate) const CODE_FORMAT_NEGATIVE: &str = "c na"; // E.g. USD -1,000.23
pub(crate) const SYMBOL_FORMAT_POSITIVE: &str = "sa"; // E.g. $1,000.23
pub(crate) const SYMBOL_FORMAT_NEGATIVE: &str = "nsa"; // E.g. -$1,000.23

pub(crate) const CODE_FORMAT_POSITIVE_MINOR: &str = "c a m"; // E.g. USD 100,023 cents
pub(crate) const CODE_FORMAT_NEGATIVE_MINOR: &str = "c na m"; // E.g. USD -100,023 cents
pub(crate) const SYMBOL_FORMAT_POSITIVE_MINOR: &str = "sa m"; // E.g. $100,023 cents
pub(crate) const SYMBOL_FORMAT_NEGATIVE_MINOR: &str = "nsa m"; // E.g. -$100,023 cents

/// Format money according to the provided format string.
///
/// Format symbols:
/// - 'a': amount (displayed as absolute value)
/// - 'c': currency code (e.g., "USD")
/// - 's': currency symbol (e.g., "$")
/// - 'm': minor symbol (e.g., "cents")
/// - 'n': negative sign (-), only displayed when amount is negative
///
/// # Arguments
///
/// * `money` - The Money value to format
/// * `format_str` - The format string containing format symbols
///
/// # Examples
///
/// ```ignore
/// use moneylib::{Money, Currency};
/// use moneylib::money_macros::dec;
///
/// let currency = Currency::from_iso("USD").unwrap();
/// let money = Money::new(currency, dec!(100.50));
///
/// // "USD 100.50"
/// assert_eq!(format(money, "c a"), "USD 100.50");
///
/// // "$100.50"
/// assert_eq!(format(money, "sa"), "$100.50");
///
/// // "USD 10,050 ¢" (amount in minor units when 'm' is present)
/// assert_eq!(format(money, "c a m"), "USD 10,050 ¢");
///
/// let negative = Money::new(currency, dec!(-50.00));
/// // "USD -50.00"
/// assert_eq!(format(negative, "c na"), "USD -50.00");
/// // "-$50.00"
/// assert_eq!(format(negative, "nsa"), "-$50.00");
/// ```
pub fn format(money: impl BaseMoney, format_str: &str) -> String {
    let mut result = String::new();
    let is_negative = money.is_negative();

    // Use absolute value for display if negative
    let display_amount = if format_str.contains(MINOR_FORMAT_SYMBOL) {
        format_128_abs(money.minor_amount().unwrap(), money.thousand_separator())
    } else {
        format_decimal_abs(
            money.amount(),
            money.thousand_separator(),
            money.decimal_separator(),
        )
    };

    let mut chars = format_str.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == ESCAPE_SYMBOL {
            if let Some(&next_ch) = chars.peek() {
                if FORMAT_SYMBOLS.contains(&next_ch) || next_ch == ESCAPE_SYMBOL {
                    chars.next();
                    result.push(next_ch);
                    continue;
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        } else {
            match ch {
                AMOUNT_FORMAT_SYMBOL => result.push_str(&display_amount),
                CODE_FORMAT_SYMBOL => result.push_str(money.code()),
                SYMBOL_FORMAT_SYMBOL => result.push_str(money.symbol()),
                MINOR_FORMAT_SYMBOL => result.push_str(money.currency().minor_symbol()),
                NEGATIVE_FORMAT_SYMBOL => {
                    if is_negative {
                        result.push('-');
                    }
                }
                ' ' => result.push(' '),
                _ => result.push(ch),
            }
        }
    }

    result
}

/// Formats an i128 with thousands separators (absolute value)
pub(crate) fn format_128_abs(num: i128, thousand_separator: &str) -> String {
    let abs_num = num.abs();
    let num_str = abs_num.to_string();

    let mut result = String::new();
    let len = num_str.len();

    for (i, ch) in num_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push_str(thousand_separator);
        }
        result.push(ch);
    }

    result
}

/// Formats a Decimal with thousands separators (absolute value)
pub(crate) fn format_decimal_abs(
    decimal: Decimal,
    thousand_separator: &str,
    decimal_separator: &str,
) -> String {
    let abs_decimal = decimal.abs();
    let decimal_str = abs_decimal.to_string();

    // Split into integer and fractional parts
    let parts: Vec<&str> = decimal_str.split('.').collect();
    let integer_part = parts[0];
    let fractional_part = parts.get(1);

    // Format integer part with thousands separators
    let mut result = String::new();
    let len = integer_part.len();

    for (i, ch) in integer_part.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push_str(thousand_separator);
        }
        result.push(ch);
    }

    // Add fractional part if it exists
    if let Some(frac) = fractional_part {
        result.push_str(decimal_separator);
        result.push_str(frac);
    }

    result
}
