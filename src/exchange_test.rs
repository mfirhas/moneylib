use crate::{
    BaseMoney, Currency, Exchange, ExchangeRates, Money, RawMoney,
    base::Amount,
    iso::{CAD, EUR, IDR, IRR, USD},
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
    rates.set(EUR::CODE, dec!(0.8));
    rates.set(IDR::CODE, 17_000);
    rates.set(USD::CODE, 40); // ignored, since base already in USD.
    assert_eq!(rates.base(), "USD");
    let ret = money.convert::<EUR>(&rates);
    assert_eq!(ret.unwrap().amount(), dec!(98.4));
    let ret = money.convert::<IDR>(&rates);
    assert_eq!(ret.unwrap().amount(), dec!(2_091_000));

    let set_max_i128 = rates.set(CAD::CODE, i128::MAX);
    assert!(set_max_i128.is_none());

    rates.set(crate::iso::SGD::CODE, 0);
    let sgd = Money::<crate::iso::SGD>::from_decimal(dec!(2));
    assert!(sgd.convert::<IDR>(&rates).is_none());

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
    assert!(non_existent_rate.convert::<IDR>(&rates).is_none());

    let none_rate = rates.get(crate::iso::SGD::CODE);
    assert!(none_rate.is_none());

    let max_money = Money::<EUR>::from_decimal(crate::Decimal::MAX);
    assert!(max_money.convert::<IDR>(&rates).is_none());

    // CAD is not in the rates, so None returned.
    assert!(money.convert::<crate::iso::CAD>(rates).is_none());
}
