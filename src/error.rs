use std::{error::Error, fmt::Display};

const ERROR_PREFIX: &str = "[MONEYLIB]";

/// Error type for moneylib
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    ParseStr,
    DecimalConversion,
    ArithmeticOverflow,
    CurrencyMismatch,
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoneyError::ParseStr => write!(
                f,
                "{} failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator of `,` or `.`.",
                ERROR_PREFIX
            ),
            MoneyError::DecimalConversion => {
                write!(f, "{} failed converting to/from Decimal", ERROR_PREFIX)
            }
            MoneyError::ArithmeticOverflow => write!(f, "{} arithmetic overflow", ERROR_PREFIX),
            MoneyError::CurrencyMismatch => {
                write!(f, "{} currency mismatch", ERROR_PREFIX)
            }
        }
    }
}

impl Error for MoneyError {}
