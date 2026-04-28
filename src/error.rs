use std::{error::Error, fmt::Display};

pub type ErrVal = Box<dyn Error + Send + Sync + 'static>;

const ERROR_PREFIX: &str = "[MONEYLIB]";

/// Error type for moneylib.
#[derive(Debug)]
pub enum MoneyError {
    ParseStrError(ErrVal),
    OverflowError,

    /// CurrencyMismatchError(got, expected)
    CurrencyMismatchError(String, String),

    #[cfg(feature = "locale")]
    ParseLocale(ErrVal),

    #[cfg(feature = "exchange")]
    ExchangeError(ErrVal),
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoneyError::ParseStrError(err) => write!(f, "{ERROR_PREFIX} parsing error: {}", err),

            MoneyError::OverflowError => write!(f, "{ERROR_PREFIX} got overflowed"),

            MoneyError::CurrencyMismatchError(got, expected) => {
                write!(
                    f,
                    "{ERROR_PREFIX} currency mismatch: got {got}, expected {expected}",
                )
            }

            #[cfg(feature = "locale")]
            MoneyError::ParseLocale(err) => {
                write!(f, "{ERROR_PREFIX} error parsing locale: {}", err)
            }

            #[cfg(feature = "exchange")]
            MoneyError::ExchangeError(err) => write!(f, "{ERROR_PREFIX} exchange error: {}", err),
        }
    }
}

impl Error for MoneyError {}
