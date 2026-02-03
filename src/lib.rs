#![forbid(unsafe_code)]

pub use iso_currency::Country;
pub use rust_decimal::Decimal;

mod base;
pub use base::BaseMoney;

mod error;
pub use error::MoneyError;

/// Money result type
pub type MoneyResult<T> = Result<T, MoneyError>;

mod money;
pub use money::Money;

mod currency;
pub use currency::Currency;

#[test]
fn asd() {
    use iso_currency::Currency;
    println!("{:?}", Currency::USD.symbol().subunit_symbol);
    println!("{:?}", Currency::IDR.symbol().subunit_symbol);
    println!("{:?}", Currency::USD.name());

    // let a = Country::
    // let a = Currency::
}
