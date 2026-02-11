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
/// # Escaping Format Symbols
///
/// To display format symbols as literal characters, prefix them with a backslash (\).
/// This allows you to:
/// 1. Insert literal format symbol characters (a, c, s, m, n) into the output
/// 2. Mix escaped symbols with actual format symbols in the same string
///
/// Escape sequences:
/// - `\a` outputs literal "a"
/// - `\c` outputs literal "c"
/// - `\s` outputs literal "s"
/// - `\m` outputs literal "m"
/// - `\n` outputs literal "n"
/// - `\\` (double backslash in source) outputs literal "\"
/// - `\x` (where x is not a format symbol or backslash) outputs literal "\x"
///
/// # Arguments
///
/// * `money` - The Money value to format
/// * `format_str` - The format string containing format symbols and optional literal text
pub(crate) fn format(money: impl BaseMoney, format_str: &str) -> String {
    let mut result = String::new();
    let is_negative = money.is_negative();

    // Use absolute value for display if negative
    let display_amount = if format_str.contains(MINOR_FORMAT_SYMBOL) {
        if let Ok(minor_amount) = money.minor_amount() {
            format_128_abs(minor_amount, money.thousand_separator())
        } else {
            "OVERFLOWED_AMOUNT".into()
        }
    } else {
        format_decimal_abs(
            money.amount(),
            money.thousand_separator(),
            money.decimal_separator(),
            money.minor_unit(),
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
        if i > 0 && (len - i).is_multiple_of(3) {
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
    minor_unit: u16,
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
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push_str(thousand_separator);
        }
        result.push(ch);
    }

    // Add fractional part if it exists, or append zeros if None
    if let Some(frac) = fractional_part {
        result.push_str(decimal_separator);
        result.push_str(frac);
    } else if minor_unit > 0 {
        // If no fractional part and minor_unit > 0, append decimal separator with zeros
        result.push_str(decimal_separator);
        result.push_str(&"0".repeat(minor_unit.into()));
    }

    result
}
