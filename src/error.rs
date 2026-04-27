use std::{error::Error, fmt::Display};

pub type ErrVal = Box<dyn Error + Send + Sync + 'static>;

const ERROR_PREFIX: &str = "[MONEYLIB]";

/// Error type for moneylib.
#[derive(Debug)]
pub enum MoneyError {
    ParseStr,
    DecimalConversion,
    ArithmeticOverflow,
    CurrencyMismatch,

    #[cfg(feature = "locale")]
    ParseLocale,

    #[cfg(feature = "exchange")]
    ExchangeError(ErrVal),
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

            #[cfg(feature = "locale")]
            MoneyError::ParseLocale => write!(f, "{} error parsing locale", ERROR_PREFIX),

            #[cfg(feature = "exchange")]
            MoneyError::ExchangeError(err) => write!(f, "{ERROR_PREFIX} exchange error: {}", err),
        }
    }
}

impl Error for MoneyError {}
