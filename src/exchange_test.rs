use crate::{
    BaseMoney, Currency, Exchange, ExchangeRates, Money, RawMoney,
    base::Amount,
    iso::{CAD, EUR, IDR, IRR, JPY, USD},
    macros::dec,
};

#[test]
fn test_exchange() {
    let money = Money::<USD>::new(123).unwrap();
    let ret = money.convert::<EUR>(dec!(0.8));
    assert_eq!(ret.unwrap().amount(), dec!(98.4));

    let money = Money::<USD>::new(123).unwrap();
    let ret = money.convert::<USD>(2);
    assert_eq!(ret.unwrap().amount(), dec!(123));

    let money = Money::<USD>::from_decimal(dec!(100));
    let ret = money.convert::<EUR>(0.888234);
    assert_eq!(ret.unwrap().amount(), dec!(88.82));

    let raw_money = RawMoney::<USD>::from_decimal(dec!(100));
    let ret = raw_money.convert::<EUR>(0.8882346);
    assert_eq!(ret.unwrap().amount(), dec!(88.82346));

    let money = Money::<USD>::new(123).unwrap();
    let mut rates = ExchangeRates::<USD>::default();
    assert_eq!(rates.len(), 1);
    assert_eq!(rates.get(USD::CODE).unwrap(), dec!(1));
    rates.set(EUR::CODE, dec!(0.8)).unwrap();
    rates.set(IDR::CODE, 17_000).unwrap();
    rates.set(USD::CODE, 40).unwrap(); // ignored, since base already in USD.
    assert_eq!(rates.base(), "USD");
    let ret = money.convert::<EUR>(&rates);
    assert_eq!(ret.unwrap().amount(), dec!(98.4));
    let ret = money.convert::<IDR>(&rates);
    assert_eq!(ret.unwrap().amount(), dec!(2_091_000));

    rates.set(CAD::CODE, i128::MAX).unwrap_err();
    assert!(rates.get(CAD::CODE).is_none());

    rates.set(crate::iso::SGD::CODE, 0).unwrap();
    let sgd = Money::<crate::iso::SGD>::from_decimal(dec!(2));
    assert!(sgd.convert::<IDR>(&rates).is_err());

    let money = Money::<EUR>::new(123).unwrap();
    let ret = money.convert::<IDR>(rates);
    assert_eq!(ret.unwrap().amount(), dec!(2_613_750));

    let rates = ExchangeRates::<EUR>::from([
        ("IDR", dec!(21_250)),
        ("IRR", dec!(1_652_125)),
        ("USD", dec!(1.25)),
        ("EUR", dec!(0.8)), // will be ignored since base already in eur and forced into 1.
    ]);
    assert_eq!(rates.base(), "EUR");
    assert_eq!(rates.len(), 4);
    assert_eq!(rates.get(EUR::CODE).unwrap(), dec!(1));
    let irr_usd = rates.get_pair("IRR", "USD").unwrap();
    assert_eq!(irr_usd, dec!(0.0000007566013467503972157070));
    let eur_usd = rates.get_pair("EUR", "USD").unwrap();
    assert_eq!(eur_usd, dec!(1.25));
    let idr_irr = rates.get_pair("IDR", "IRR").unwrap();
    assert_eq!(idr_irr, dec!(77.747058823529411764705960100));
    let cad_usd = rates.get_pair("CAD", "USD");
    assert!(cad_usd.is_none());

    let amount = <ExchangeRates<EUR> as Amount<IDR>>::get_decimal(&rates);
    assert_eq!(amount.unwrap(), dec!(21_250));
    let rate_ref = &rates;
    let amount = <&ExchangeRates<EUR> as Amount<IRR>>::get_decimal(&rate_ref);
    assert_eq!(amount.unwrap(), dec!(1_652_125));
    let amount = <&ExchangeRates<EUR> as Amount<crate::iso::SGD>>::get_decimal(&rate_ref);
    assert!(amount.is_none());

    let money = Money::<USD>::from_decimal(dec!(1000));
    assert_eq!(money.convert::<USD>(&rates).unwrap().amount(), dec!(1000));
    assert_eq!(money.convert::<EUR>(&rates).unwrap().amount(), dec!(800));
    assert_eq!(
        money.convert::<IRR>(&rates).unwrap().amount(),
        dec!(1_321_700_000)
    );
    assert_eq!(
        money.convert::<IDR>(&rates).unwrap().amount(),
        dec!(17_000_000)
    );

    let money = Money::<EUR>::new(12).unwrap();
    assert_eq!(
        money.convert::<IDR>(&rates).unwrap().amount(),
        dec!(255_000)
    );
    assert_eq!(money.convert::<EUR>(&rates).unwrap().amount(), dec!(12));
    let money = Money::<EUR>::new(1230).unwrap();
    assert_eq!(
        money.convert::<USD>(&rates).unwrap().amount(),
        dec!(1_537.5)
    );
    let money = Money::<IDR>::new(15_000_000).unwrap();
    assert_eq!(
        money.convert::<IRR>(&rates).unwrap().amount(),
        dec!(1_166_205_882.35)
    );
    let money = Money::<IRR>::new(5000).unwrap();
    assert_eq!(money.convert::<USD>(&rates).unwrap().amount(), dec!(0));
    let money = Money::<IRR>::new(10000).unwrap();
    assert_eq!(money.convert::<EUR>(&rates).unwrap().amount(), dec!(0.01));
    assert_eq!(
        money.convert::<IRR>(rates.clone()).unwrap().amount(),
        dec!(10_000)
    );

    let non_existent_rate = Money::<crate::iso::SGD>::from_decimal(dec!(123));
    assert!(non_existent_rate.convert::<IDR>(&rates).is_err());

    let none_rate = rates.get(crate::iso::SGD::CODE);
    assert!(none_rate.is_none());

    let max_money = Money::<EUR>::from_decimal(crate::Decimal::MAX);
    assert!(max_money.convert::<IDR>(&rates).is_err());

    // CAD is not in the rates, so error returned.
    let not_in_rates = money.convert::<crate::iso::CAD>(rates);
    assert!(matches!(
        not_in_rates,
        Err(crate::MoneyError::ExchangeError(_))
    ));
}

#[test]
fn test_exchange_rates() {
    let mut rates = ExchangeRates::<USD>::new();
    assert_eq!(rates.len(), 1);
    assert_eq!(rates.get(USD::CODE).unwrap(), dec!(1));
    println!("after initiation: {:?}", &rates);
    println!("--------------------------------");

    rates.set("EUR", dec!(0.8)).unwrap();
    rates.set("IDR", dec!(17_000)).unwrap();
    rates.set("CAD", dec!(1.2)).unwrap();
    assert_eq!(rates.len(), 4);
    println!("{}", rates);
    println!("--------------------------------");

    rates.set_pair("CNY", "IDR", i128::MAX).unwrap_err(); // wont set, return error
    rates.set_pair("CNY", "IDR", 2500).unwrap();
    assert_eq!(rates.len(), 5);
    assert_eq!(rates.get("CNY").unwrap(), dec!(6.8));

    use crate::money;
    let cny_idr_rate = money!(CNY, 5262.657).convert::<IDR>(2500).unwrap();
    let cny_idr_rates = money!(CNY, 5262.657).convert::<IDR>(&rates).unwrap();
    assert_eq!(cny_idr_rate, cny_idr_rates);

    println!("after setting CNY/IDR: {}", rates);
    println!("--------------------------------");

    rates.set_pair("CNY", "IDR", 3000).unwrap();
    let cny_idr_new_rate = money!(CNY, 34989.123).convert::<IDR>(3000).unwrap();
    let cny_idr_new_rates = money!(CNY, 34989.123).convert::<IDR>(&rates).unwrap();
    assert_eq!(cny_idr_new_rate, cny_idr_new_rates);
    println!("after updating CNY/IDR: {}", rates);
    println!("--------------------------------");

    // set base/base
    rates.set_pair("USD", "USD", 123).unwrap(); // should be ignored
    assert_eq!(rates.get("USD").unwrap(), dec!(1));
    assert_eq!(rates.get_pair("USD", "USD").unwrap(), dec!(1));

    rates.set_pair("USD", "EUR", i128::MAX).unwrap_err();
    rates.set_pair("USD", "EUR", dec!(0.85)).unwrap();
    assert_eq!(rates.get_pair("USD", "EUR").unwrap(), dec!(0.85));
    assert_eq!(
        money!(USD, 123).convert::<EUR>(dec!(0.85)).unwrap(),
        money!(USD, 123).convert::<EUR>(&rates).unwrap()
    );
    println!("after setting USD/EUR: {}", rates);
    println!("--------------------------------");

    rates.set_pair("CAD", "USD", i128::MAX).unwrap_err();
    rates.set_pair("CAD", "USD", dec!(0.9)).unwrap();
    assert_eq!(
        money!(CAD, 123).convert::<USD>(dec!(0.9)).unwrap(),
        money!(CAD, 123).convert::<USD>(&rates).unwrap()
    );
    println!("after setting CAD/USD: {}", rates);
    println!("--------------------------------");

    rates.set_pair("CNY", "JPY", i128::MAX).unwrap_err();
    rates.set_pair("CNY", "JPY", 23).unwrap();
    assert_eq!(
        money!(CNY, 123).convert::<JPY>(dec!(23)).unwrap(),
        money!(CNY, 123).convert::<JPY>(&rates).unwrap()
    );
    println!("after setting CNY/JPY: {}", rates);
    println!("--------------------------------");

    // set both not exist
    let both_not_exist = rates.set_pair("SGD", "HKD", 6);
    assert!(matches!(
        both_not_exist,
        Err(crate::MoneyError::ExchangeError(_))
    ));
    println!("after setting SGD/HKD: {}", rates);
    println!("--------------------------------");

    // set max value, failed
    rates.set_pair("CNY", "JPY", i128::MAX).unwrap_err();
    assert_eq!(
        money!(CNY, 123).convert::<JPY>(dec!(23)).unwrap(),
        money!(CNY, 123).convert::<JPY>(&rates).unwrap()
    );
    rates.set_pair("CNY", "JPY", i128::MAX).unwrap_err();
    assert_eq!(
        money!(CNY, 123).convert::<JPY>(dec!(23)).unwrap(),
        money!(CNY, 123).convert::<JPY>(&rates).unwrap()
    );
}
