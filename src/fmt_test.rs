use crate::Decimal;
use crate::fmt::{format_128_abs, format_decimal_abs};

#[test]
fn test_format_with_thousands() {
    assert_eq!(format_128_abs(1000, ","), "1,000");
    assert_eq!(format_128_abs(100, ","), "100");
    assert_eq!(format_128_abs(100000, ","), "100,000");
    assert_eq!(format_128_abs(1, ","), "1");
    assert_eq!(format_128_abs(1000000, ","), "1,000,000");
    assert_eq!(format_128_abs(-1000, ","), "1,000");
    assert_eq!(format_128_abs(-100, ","), "100");
    assert_eq!(format_128_abs(0, ","), "0");
}

#[test]
fn test_format_decimal_with_thousands() {
    use std::str::FromStr;

    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000").unwrap(), ",", "."),
        "1,000"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("100").unwrap(), ",", "."),
        "100"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("100000").unwrap(), ",", "."),
        "100,000"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1000.50").unwrap(), ",", "."),
        "1,000.50"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("1234567.89").unwrap(), ",", "."),
        "1,234,567.89"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("-1000").unwrap(), ",", "."),
        "1,000"
    );
    assert_eq!(
        format_decimal_abs(Decimal::from_str("-1000.25").unwrap(), ",", "."),
        "1,000.25"
    );
}
