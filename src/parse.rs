/// Validate and build result from parsed parts
fn validate_and_build_result<'a>(
    currency_code: &'a str,
    integer_part: &'a str,
    decimal_part: Option<&'a str>,
    separator: char,
    is_positive: bool,
) -> Option<(&'a str, String)> {
    if integer_part.is_empty() {
        return None;
    }

    // Check if there are separators
    if integer_part.contains(separator) {
        // Validate separator-separated format
        let groups: Vec<&str> = integer_part.split(separator).collect();

        // First group can be 1-3 digits
        if groups[0].is_empty()
            || groups[0].len() > 3
            || !groups[0].chars().all(|c| c.is_ascii_digit())
        {
            return None;
        }

        // All subsequent groups must be exactly 3 digits
        for group in groups.iter().skip(1) {
            if group.len() != 3 || !group.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
        }

        // Build result without separators
        let mut result = groups.join("");
        if let Some(dec) = decimal_part {
            // Decimal part must be all digits
            if dec.is_empty() || !dec.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            result.push('.');
            result.push_str(dec);
        }

        // embed `-` if negative
        if !is_positive {
            result.insert(0, '-');
        }

        Some((currency_code, result))
    } else {
        // No separators, just validate it's all digits
        if !integer_part.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        let mut result = integer_part.to_string();
        if let Some(dec) = decimal_part {
            // Decimal part must be all digits
            if dec.is_empty() || !dec.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }
            result.push('.');
            result.push_str(dec);
        }

        // embed `-` if negative
        if !is_positive {
            result.insert(0, '-');
        }

        Some((currency_code, result))
    }
}

/// Parse money string with comma thousands separator and dot decimal separator
/// Format: "CCC amount" where CCC is 3-letter currency code
/// Examples: "USD 1,234.56", "USD 100.50", "USD 1000000"
///
/// Returns Some((currency_code, amount_without_separators)) on success
/// Returns None if the format doesn't match
pub fn parse_comma_thousands_separator(s: &str) -> Option<(&str, String)> {
    // Split by space (handles multiple spaces automatically)
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let currency_code = parts[0];
    let amount_str = parts[1];

    // Currency code must be exactly 3 alphabetic characters
    if currency_code.len() != 3 || !currency_code.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    // Parse amount with comma thousands separator and optional dot decimal separator
    // Valid formats:
    // - digits only: "123", "1234"
    // - with decimal: "123.45"
    // - with thousands separator: "1,234", "1,234,567"
    // - with both: "1,234.56", "1,000,000.99"

    if amount_str.is_empty() {
        return None;
    }

    // Split by decimal point if present
    let decimal_parts: Vec<&str> = amount_str.split('.').collect();
    if decimal_parts.len() > 2 {
        return None;
    }

    let (integer_part, is_positive) = if let Some(neg_trimmed) = decimal_parts[0].strip_prefix("-")
    {
        (neg_trimmed, false)
    } else {
        (decimal_parts[0], true)
    };
    let decimal_part = if decimal_parts.len() == 2 {
        Some(decimal_parts[1])
    } else {
        None
    };

    validate_and_build_result(currency_code, integer_part, decimal_part, ',', is_positive)
}

/// Parse money string with dot thousands separator and comma decimal separator
/// Format: "CCC amount" where CCC is 3-letter currency code
/// Examples: "EUR 1.234,56", "EUR 100,50", "EUR 1000000"
///
/// Returns Some((currency_code, amount_converted_to_standard)) on success
/// The returned amount has commas converted to dots for decimal separator
/// Returns None if the format doesn't match
pub fn parse_dot_thousands_separator(s: &str) -> Option<(&str, String)> {
    // Split by space (handles multiple spaces automatically)
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let currency_code = parts[0];
    let amount_str = parts[1];

    // Currency code must be exactly 3 alphabetic characters
    if currency_code.len() != 3 || !currency_code.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    // Parse amount with dot thousands separator and optional comma decimal separator
    // Valid formats:
    // - digits only: "123", "1234"
    // - with decimal: "123,45"
    // - with thousands separator: "1.234", "1.234.567"
    // - with both: "1.234,56", "1.000.000,99"

    if amount_str.is_empty() {
        return None;
    }

    // Split by comma (decimal separator) if present
    let decimal_parts: Vec<&str> = amount_str.split(',').collect();
    if decimal_parts.len() > 2 {
        return None;
    }

    let (integer_part, is_positive) = if let Some(neg_trimmed) = decimal_parts[0].strip_prefix("-")
    {
        (neg_trimmed, false)
    } else {
        (decimal_parts[0], true)
    };
    let decimal_part = if decimal_parts.len() == 2 {
        Some(decimal_parts[1])
    } else {
        None
    };

    validate_and_build_result(currency_code, integer_part, decimal_part, '.', is_positive)
}

pub fn parse_symbol_comma_thousands_separator<C: crate::Currency>(
    s: &str,
) -> Option<(&str, String)> {
    let (stripped_money, is_positive) = if let Some(trimmed) = s.strip_prefix('-') {
        (trimmed, false)
    } else {
        (s, true)
    };
    let amount_str = stripped_money.strip_prefix(C::SYMBOL)?;
    if amount_str.is_empty() {
        return None;
    }

    let decimal_parts: Vec<&str> = amount_str.split('.').collect();
    if decimal_parts.len() > 2 {
        return None;
    }

    let (integer_part, decimal_part) = if decimal_parts.len() == 2 {
        (decimal_parts[0], Some(decimal_parts[1]))
    } else {
        (decimal_parts[0], None)
    };

    validate_and_build_result(C::SYMBOL, integer_part, decimal_part, ',', is_positive)
}

pub fn parse_symbol_dot_thousands_separator<C: crate::Currency>(s: &str) -> Option<(&str, String)> {
    let (stripped_money, is_positive) = if let Some(trimmed) = s.strip_prefix('-') {
        (trimmed, false)
    } else {
        (s, true)
    };
    let amount_str = stripped_money.strip_prefix(C::SYMBOL)?;
    if amount_str.is_empty() {
        return None;
    }

    let decimal_parts: Vec<&str> = amount_str.split(',').collect();
    if decimal_parts.len() > 2 {
        return None;
    }

    let (integer_part, decimal_part) = if decimal_parts.len() == 2 {
        (decimal_parts[0], Some(decimal_parts[1]))
    } else {
        (decimal_parts[0], None)
    };

    validate_and_build_result(C::SYMBOL, integer_part, decimal_part, '.', is_positive)
}
