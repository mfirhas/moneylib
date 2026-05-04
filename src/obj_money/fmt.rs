use crate::Decimal;
use crate::fmt::{FORMAT_SYMBOLS, format_128_abs, format_decimal_abs};
use crate::macros::dec;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::ToPrimitive;

const ESCAPE_SYMBOL: char = '\\';

const AMOUNT_FORMAT_SYMBOL: char = 'a';
const CODE_FORMAT_SYMBOL: char = 'c';
const SYMBOL_FORMAT_SYMBOL: char = 's';
const MINOR_FORMAT_SYMBOL: char = 'm';
const NEGATIVE_FORMAT_SYMBOL: char = 'n';

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

/// Formats money parts into a string using a format template with runtime `&str` fields
/// for code, symbol and minor-unit symbol (as opposed to compile-time `Currency` constants).
fn format_parts(
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

/// Runtime counterpart of `format_with_separator<C>`: formats money described by plain `&str`
/// fields rather than by a generic `C: Currency` type parameter.
///
/// Used by [`super::ObjMoney`] default implementations so that a single
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
