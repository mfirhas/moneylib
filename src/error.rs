use std::{error::Error, fmt::Display};

pub type ErrVal = Box<dyn Error + Send + Sync + 'static>;

const ERROR_PREFIX: &str = "[MONEYLIB]";

/// Error type for moneylib.
#[derive(Debug)]
pub enum MoneyError {
    ParseStr,
    OverflowError,
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
                "{ERROR_PREFIX} failed parsing from str, use format: `<CODE> <AMOUNT>`, <AMOUNT> can be formatted with thousands and/or decimal separator of `,` or `.`."
            ),

            MoneyError::OverflowError => write!(f, "{ERROR_PREFIX} got overflowed"),

            MoneyError::CurrencyMismatch => {
                write!(f, "{ERROR_PREFIX} currency mismatch")
            }

            #[cfg(feature = "locale")]
            MoneyError::ParseLocale => write!(f, "{ERROR_PREFIX} error parsing locale"),

            #[cfg(feature = "exchange")]
            MoneyError::ExchangeError(err) => write!(f, "{ERROR_PREFIX} exchange error: {}", err),
        }
    }
}

impl Error for MoneyError {}
