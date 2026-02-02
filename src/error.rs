use thiserror::Error;

const ERROR_PREFIX: &'static str = "[MONEYLIB_ERROR]";

#[derive(Debug, Error)]
pub enum MoneyError {
    #[error("{ERROR_PREFIX} generic error: {0}")]
    Error(String),

    #[error(
        "{ERROR_PREFIX} failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator."
    )]
    ParseStr,

    #[error("{ERROR_PREFIX} invalid currency, please use currencies supported by ISO 4217")]
    InvalidCurrency,

    #[error("{ERROR_PREFIX} invalid amount, please use comma and dot separator accordingly")]
    InvalidAmount,

    //--- arithmetic errors
    #[error("{ERROR_PREFIX} cannot divide by zero")]
    DivisionByZero,

    #[error("{ERROR_PREFIX} failed converting Decimal to integer types")]
    DecimalToInteger,

    #[error("{ERROR_PREFIX} Arithmetic overflow")]
    ArithmeticOverflow,
}
