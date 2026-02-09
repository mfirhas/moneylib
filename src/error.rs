use thiserror::Error;

const ERROR_PREFIX: &str = "[MONEYLIB]";

#[derive(Debug, Error)]
pub enum MoneyError {
    #[error(
        "{ERROR_PREFIX} new currency must have code, symbol, name, and minor unit atleast, and not already existed in ISO 4217"
    )]
    NewCurrency,

    #[error(
        "{ERROR_PREFIX} this currency is already existed in ISO 4217 list, use Currency::from_iso to create ISO 4217 currency"
    )]
    ExistsInISO,

    #[error(
        "{ERROR_PREFIX} failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator of `,` or `.`."
    )]
    ParseStr,

    #[error("{ERROR_PREFIX} invalid currency, please use currencies supported by ISO 4217")]
    InvalidCurrency,

    //--- arithmetic errors
    #[error("{ERROR_PREFIX} cannot divide by zero")]
    DivisionByZero,

    #[error("{ERROR_PREFIX} failed converting Decimal to integer types")]
    DecimalToInteger,

    #[error("{ERROR_PREFIX} Arithmetic overflow")]
    ArithmeticOverflow,
}
