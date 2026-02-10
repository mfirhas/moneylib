use std::{error::Error, fmt::Display};

const ERROR_PREFIX: &str = "[MONEYLIB]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    NewCurrency,
    ExistsInISO,
    ParseStr,
    InvalidCurrency,
    DivisionByZero,
    DecimalToInteger,
    ArithmeticOverflow,

    NewMoney(String),
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoneyError::NewCurrency => write!(
                f,
                "{} new currency must have code, symbol, name, and minor unit atleast, and not already existed in ISO 4217",
                ERROR_PREFIX
            ),
            MoneyError::ExistsInISO => write!(
                f,
                "{} this currency is already existed in ISO 4217 list, use Currency::from_iso to create ISO 4217 currency",
                ERROR_PREFIX
            ),
            MoneyError::ParseStr => write!(
                f,
                "{} failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator of `,` or `.`.",
                ERROR_PREFIX
            ),
            MoneyError::InvalidCurrency => write!(
                f,
                "{} invalid currency, please use currencies supported by ISO 4217",
                ERROR_PREFIX
            ),
            MoneyError::DivisionByZero => write!(f, "{} cannot divide by zero", ERROR_PREFIX),
            MoneyError::DecimalToInteger => write!(
                f,
                "{} failed converting Decimal to integer types",
                ERROR_PREFIX
            ),
            MoneyError::ArithmeticOverflow => write!(f, "{} Arithmetic overflow", ERROR_PREFIX),

            Self::NewMoney(err_msg) => {
                write!(f, "{} failed creating new money: {}", ERROR_PREFIX, err_msg)
            }
        }
    }
}

impl Error for MoneyError {}
