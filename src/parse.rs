use crate::{Currency, MoneyError};

/// Validate and build string amount.
/// Thousand separators removed, and decimal separator use dot.
/// E.g 42344.1233
pub(super) fn parse_into_string_amount<'a>(
    integer_part: &'a str,
    decimal_part: Option<&'a str>,
    thousand_separator: &'a str,
    is_negative: bool,
) -> Result<String, MoneyError> {
    if integer_part.is_empty() {
        return Err(MoneyError::ParseStrError("integer part is empty".into()));
    }

    // Check if there are separators
    if integer_part.contains(thousand_separator) {
        // Validate separator-separated format
        let groups: Vec<&str> = integer_part.split(thousand_separator).collect();

        // First group can be 1-3 digits
        if groups[0].is_empty()
            || groups[0].len() > 3
            || !groups[0].chars().all(|c| c.is_ascii_digit())
        {
            return Err(MoneyError::ParseStrError(format!("first group of integer part is empty or more than 3 digits or not all ascii numbers: {}", integer_part).into()));
        }

        // All subsequent groups must be exactly 3 digits
        for group in groups.iter().skip(1) {
            if group.len() != 3 || !group.chars().all(|c| c.is_ascii_digit()) {
                return Err(MoneyError::ParseStrError(format!("second and subsequent parts of integer is not 3 digits or not all ascii numbers: {}", integer_part).into()));
            }
        }

        // Build result without separators
        let mut result = groups.join("");
        // append decimal with dot separator
        if let Some(dec) = decimal_part {
            // Decimal part must be all digits
            if dec.is_empty() || !dec.chars().all(|c| c.is_ascii_digit()) {
                return Err(MoneyError::ParseStrError(
                    "decimal part is empty or not all ascii numbers".into(),
                ));
            }
            result.push('.');
            result.push_str(dec);
        }

        // embed `-` if negative
        if is_negative {
            result.insert(0, '-');
        }

        Ok(result)
    } else {
        // No separators, just validate it's all digits
        if !integer_part.chars().all(|c| c.is_ascii_digit()) {
            return Err(MoneyError::ParseStrError(
                "integer part not all ascii numbers".into(),
            ));
        }

        let mut result = integer_part.to_string();
        if let Some(dec) = decimal_part {
            // Decimal part must be all digits
            if dec.is_empty() || !dec.chars().all(|c| c.is_ascii_digit()) {
                return Err(MoneyError::ParseStrError(
                    "decimal part is empty or not all ascii numbers".into(),
                ));
            }
            result.push('.');
            result.push_str(dec);
        }

        // embed `-` if negative
        if is_negative {
            result.insert(0, '-');
        }

        Ok(result)
    }
}

/// Parse money string with code `<CODE> <AMOUNT>`,
/// where `<CODE>` is currency alpha code.
///
/// It returns string amount without thousand separator and with dot decimal separator.
pub(crate) fn parse_str_code<C: Currency>(
    str_code: &str,
    thousand_separator: &str,
    decimal_separator: &str,
) -> Result<String, MoneyError> {
    let str_code = str_code.trim();

    // Split by space (handles multiple spaces automatically)
    let parts: Vec<&str> = str_code.split_whitespace().collect();
    if parts.len() != 2
        || parts[0].is_empty()
        || !parts[0].chars().all(|c| c.is_ascii_alphabetic())
        || parts[1].is_empty()
    {
        return Err(MoneyError::ParseStrError(
            format!(
                "invalid currency with code, expected: <CODE> <AMOUNT> with <CODE> and <AMOUNT> all in ascii, found: {}",
                str_code
            )
            .into(),
        ));
    }

    let currency_code = parts[0];
    let amount_str = parts[1];

    if currency_code != C::CODE {
        return Err(MoneyError::CurrencyMismatchError(
            currency_code.into(),
            C::CODE.into(),
        ));
    }

    let amount_parts: Vec<&str> = amount_str.split(decimal_separator).collect();
    // splitting amount part by decimal point must have at most 2 parts(integer and decimal).
    if amount_parts.len() > 2 {
        return Err(MoneyError::ParseStrError(
            format!(
                "splitting by decimal separator({}) must not more than 2 parts: {}",
                decimal_separator, amount_str
            )
            .into(),
        ));
    }

    let (integer_part, is_negative) = if let Some(neg_trimmed) = amount_parts[0].strip_prefix("-") {
        (neg_trimmed, true)
    } else {
        (amount_parts[0], false)
    };
    let decimal_part = if amount_parts.len() == 2 {
        Some(amount_parts[1])
    } else {
        None
    };

    parse_into_string_amount(integer_part, decimal_part, thousand_separator, is_negative)
}

/// parse money string with symbol `<SYMBOL><AMOUNT>`,
/// where `<SYMBOL>` is currency alpha code.
///
/// It returns string amount without thousand separator and with dot decimal separator.
pub(crate) fn parse_str_symbol<C: Currency>(
    str_symbol: &str,
    thousand_separator: &str,
    decimal_separator: &str,
) -> Result<String, MoneyError> {
    let str_symbol = str_symbol.trim();

    let (abs_money, is_negative) = if let Some(trimmed) = str_symbol.strip_prefix('-') {
        (trimmed, true)
    } else {
        (str_symbol, false)
    };
    let amount_str = abs_money.strip_prefix(C::SYMBOL);
    let amount_str = if let Some(amount) = amount_str
        && !amount.is_empty()
    {
        amount
    } else {
        return Err(MoneyError::CurrencyMismatchError(
            str_symbol.into(),
            C::SYMBOL.into(),
        ));
    };

    let amount_parts: Vec<&str> = amount_str.split(decimal_separator).collect();
    // splitting amount part by decimal point must have at most 2 parts(integer and decimal).
    if amount_parts.len() > 2 {
        return Err(MoneyError::ParseStrError(
            format!(
                "splitting by decimal separator({}) must not more than 2 parts: {}",
                decimal_separator, amount_str
            )
            .into(),
        ));
    }

    let (integer_part, decimal_part) = if amount_parts.len() == 2 {
        (amount_parts[0], Some(amount_parts[1]))
    } else {
        (amount_parts[0], None)
    };

    parse_into_string_amount(integer_part, decimal_part, thousand_separator, is_negative)
}
