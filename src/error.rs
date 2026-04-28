use std::{error::Error, fmt::Display};

pub type ErrVal = Box<dyn Error + Send + Sync + 'static>;

const ERROR_PREFIX: &str = "[MONEYLIB]";

/// Error type for moneylib.
#[derive(Debug)]
pub enum MoneyError {
    ParseStrError(ErrVal),
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
            MoneyError::ParseStrError(err) => write!(f, "{ERROR_PREFIX} parsing error: {}", err),

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
