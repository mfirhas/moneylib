pub use iso_currency::Country;
pub use iso_currency::Currency;

mod base;
pub use base::{BaseMoney, MoneyResult};

mod error;
pub use error::MoneyError;

mod money;

#[test]
fn asd() {
    println!("{:?}", Currency::USD.symbol().subunit_symbol);
    println!("{:?}", Currency::IDR.symbol().subunit_symbol);
    println!("{:?}", Currency::DZD.numeric());

    // let a = Currency::
}
