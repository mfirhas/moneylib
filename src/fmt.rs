use crate::Currency;

use crate::macros::dec;
use crate::{BaseMoney, Decimal};
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::ToPrimitive;

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

pub(crate) const CODE_FORMAT: &str = "c na"; // E.g. USD 1,000.23 or USD -1,000.23
pub(crate) const SYMBOL_FORMAT: &str = "nsa"; // E.g. $1,000.23 or -$1,000.23

pub(crate) const CODE_FORMAT_MINOR: &str = "c na m"; // E.g. USD 100,023 cents or USD -100,023 cents
pub(crate) const SYMBOL_FORMAT_MINOR: &str = "nsa m"; // E.g. $100,023 cents or -$100,023 cents

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
/// # Literal Blocks
///
/// Use `\{...}` to print the contents of the curly braces literally, without any
/// interpretation of format symbols inside. This is an alternative to escaping
/// individual characters.
///
/// Examples:
/// - `\{Total:} c na` outputs "Total: USD 1,000.23"
/// - `\{Price (USD):} na` outputs "Price (USD): 1,000.23"
/// - `\{a, c, s} a` outputs "a, c, s 100.50"
///
/// If the closing `}` is omitted, the contents are still printed literally to the end.
///
/// # Arguments
///
/// * `money` - The Money value to format
/// * `format_str` - The format string containing format symbols and optional literal text
pub(crate) fn format<C: Currency>(money: impl BaseMoney<C>, format_str: &str) -> String {
    format_with_separator(
        money,
        format_str,
        C::THOUSAND_SEPARATOR,
        C::DECIMAL_SEPARATOR,
    )
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
        if frac.len() >= minor_unit.into() {
            result.push_str(frac);
        } else {
            result.push_str(frac);
            let frac_len = frac.len();
            let minor_unit_len: usize = minor_unit.into();
            let remaining_frac_len = minor_unit_len - frac_len;
            result.push_str(&"0".repeat(remaining_frac_len));
        }
    } else if minor_unit > 0 {
        // If no fractional part and minor_unit > 0, append decimal separator with zeros
        result.push_str(decimal_separator);
        result.push_str(&"0".repeat(minor_unit.into()));
    }

    result
}

pub(crate) fn format_with_separator<C: Currency>(
    money: impl BaseMoney<C>,
    format_str: &str,
    thousand_separator: &str,
    decimal_separator: &str,
) -> String {
    let is_negative = money.is_negative();

    // Use absolute value for display if negative
    let display_amount = if contains_active_format_symbol(format_str, MINOR_FORMAT_SYMBOL) {
        if let Ok(minor_amount) = money.minor_amount() {
            format_128_abs(minor_amount, thousand_separator)
        } else {
            "OVERFLOWED_AMOUNT".into()
        }
    } else {
        format_decimal_abs(
            money.amount(),
            thousand_separator,
            decimal_separator,
            C::MINOR_UNIT,
        )
    };

    format_with_amount::<C>(&display_amount, is_negative, format_str)
}

/// Returns true if `symbol` appears as an active (non-escaped, non-literal-block) format symbol
/// in `format_str`.
fn contains_active_format_symbol(format_str: &str, symbol: char) -> bool {
    let mut chars = format_str.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == ESCAPE_SYMBOL {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '{' {
                    chars.next(); // consume '{'
                    // skip everything until '}'
                    for inner_ch in chars.by_ref() {
                        if inner_ch == '}' {
                            break;
                        }
                    }
                } else {
                    // single-char escape: skip the next character
                    chars.next();
                }
            }
        } else if ch == symbol {
            return true;
        }
    }
    false
}

/// format money with amount and format, the amount is in absolute form.
pub(crate) fn format_with_amount<C: Currency>(
    display_amount: &str,
    is_negative: bool,
    format_str: &str,
) -> String {
    let mut chars = format_str.chars().peekable();

    let mut result = String::new();
    while let Some(ch) = chars.next() {
        if ch == ESCAPE_SYMBOL {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '{' {
                    chars.next(); // consume '{'
                    // collect everything until closing '}', output literally
                    for inner_ch in chars.by_ref() {
                        if inner_ch == '}' {
                            break;
                        }
                        result.push(inner_ch);
                    }
                    continue;
                } else if FORMAT_SYMBOLS.contains(&next_ch) || next_ch == ESCAPE_SYMBOL {
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
                AMOUNT_FORMAT_SYMBOL => result.push_str(display_amount),
                CODE_FORMAT_SYMBOL => result.push_str(C::CODE),
                SYMBOL_FORMAT_SYMBOL => result.push_str(C::SYMBOL),
                MINOR_FORMAT_SYMBOL => result.push_str(C::MINOR_UNIT_SYMBOL),
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

/// Runtime counterpart of [`format_with_amount`]: takes code/symbol/minor_unit_symbol as
/// `&str` at runtime rather than as compile-time constants from a `Currency` type.
pub(crate) fn format_parts(
    display_amount: &str,
    is_negative: bool,
    code: &str,
    symbol: &str,
    minor_unit_symbol: &str,
    format_str: &str,
) -> String {
    let mut chars = format_str.chars().peekable();

    let mut result = String::new();
    while let Some(ch) = chars.next() {
        if ch == ESCAPE_SYMBOL {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '{' {
                    chars.next(); // consume '{'
                    // collect everything until closing '}', output literally
                    for inner_ch in chars.by_ref() {
                        if inner_ch == '}' {
                            break;
                        }
                        result.push(inner_ch);
                    }
                    continue;
                } else if FORMAT_SYMBOLS.contains(&next_ch) || next_ch == ESCAPE_SYMBOL {
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
                AMOUNT_FORMAT_SYMBOL => result.push_str(display_amount),
                CODE_FORMAT_SYMBOL => result.push_str(code),
                SYMBOL_FORMAT_SYMBOL => result.push_str(symbol),
                MINOR_FORMAT_SYMBOL => result.push_str(minor_unit_symbol),
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

/// Runtime counterpart of [`format_with_separator`]: formats money described by plain `&str`
/// fields rather than by a generic `C: Currency` type parameter.
///
/// Used by [`crate::ObjMoney`] default implementations so that a single
/// `dyn ObjMoney` trait object can be formatted without knowing the concrete currency type.
#[allow(clippy::too_many_arguments)]
pub(crate) fn format_obj_money(
    amount: Decimal,
    code: &str,
    symbol: &str,
    minor_unit_symbol: &str,
    minor_unit: u16,
    thousand_separator: &str,
    decimal_separator: &str,
    format_str: &str,
) -> String {
    let is_negative = amount.is_sign_negative();

    let display_amount = if contains_active_format_symbol(format_str, MINOR_FORMAT_SYMBOL) {
        let minor_result = dec!(10)
            .checked_powu(minor_unit.into())
            .and_then(|factor| amount.checked_mul(factor))
            .and_then(|m| m.to_i128());
        if let Some(n) = minor_result {
            format_128_abs(n, thousand_separator)
        } else {
            "OVERFLOWED_AMOUNT".into()
        }
    } else {
        format_decimal_abs(amount, thousand_separator, decimal_separator, minor_unit)
    };

    format_parts(
        &display_amount,
        is_negative,
        code,
        symbol,
        minor_unit_symbol,
        format_str,
    )
}
