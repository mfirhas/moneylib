use crate::obj_money::DynMoney;
use crate::obj_money::obj_money::CodeToCurrencyMap;
use crate::{BaseMoney, Decimal, dec};
use crate::{MoneyError, obj_money::*};

#[test]
fn currency_data_conversion_success() {
    let data = currencylib::data::Data {
        code: "USD",
        symbol: "$",
        name: "United States dollar",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "c",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("USD", data);
    let c_map: CodeToCurrencyMap = from.try_into().unwrap();
    let obj_money = ObjMoney::<false>::new(c_map.1, dec!(1));

    assert_eq!(c_map.0, "USD");
    assert_eq!(obj_money.code(), "USD");
    assert_eq!(obj_money.symbol(), "$");
    assert_eq!(obj_money.minor_unit(), 2);
    assert_eq!(obj_money.minor_unit_symbol(), "c");
    assert_eq!(obj_money.name(), "United States dollar");
}

#[test]
fn currency_data_conversion_failed() {
    let data = currencylib::data::Data {
        code: "US😀",
        symbol: "$",
        name: "United States dollar",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "c",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("USD", data);
    let c_map: Result<CodeToCurrencyMap, MoneyError> = from.try_into();
    assert!(c_map.is_err());

    let data = currencylib::data::Data {
        code: "",
        symbol: "$",
        name: "United States dollar",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "c",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("USD", data);
    let c_map: Result<CodeToCurrencyMap, MoneyError> = from.try_into();
    assert!(c_map.is_err());

    let data = currencylib::data::Data {
        code: "USD",
        symbol: "$",
        name: "United States dollar",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "c",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("", data);
    let c_map: Result<CodeToCurrencyMap, MoneyError> = from.try_into();
    assert!(c_map.is_err());

    let data = currencylib::data::Data {
        code: "USD",
        symbol: "",
        name: "United States dollar",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "c",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("USD", data);
    let c_map: Result<CodeToCurrencyMap, MoneyError> = from.try_into();
    assert!(c_map.is_err());

    let data = currencylib::data::Data {
        code: "USD",
        symbol: "$",
        name: "United States dollar",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "cCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("USD", data);
    let c_map: Result<CodeToCurrencyMap, MoneyError> = from.try_into();
    assert!(c_map.is_err());

    let data = currencylib::data::Data {
        code: "USD",
        symbol: "$",
        name: "",
        numeric: 842,
        minor_unit: 2,
        minor_unit_symbol: "c",
        minor_unit_name: "cent",
        thousand_separator: ",",
        decimal_separator: ".",
        origin: "United States",
        locale: "en-us",
    };
    let from = ("USD", data);
    let c_map: Result<CodeToCurrencyMap, MoneyError> = from.try_into();
    assert!(c_map.is_err());
}

#[test]
fn currency_try_new() {
    let currency = ObjCurrency::try_new("USD", "$", "¢", "US Dollar", 2).unwrap();

    let money: ObjMoney = ObjMoney::new(currency, dec!(123.45));

    assert_eq!(money.code(), "USD");
    assert_eq!(money.symbol(), "$");
    assert_eq!(money.minor_unit_symbol(), "¢");
    assert_eq!(money.name(), "US Dollar");
    assert_eq!(money.minor_unit(), 2);
    assert_eq!(money.amount(), dec!(123.45));
}

#[test]
fn currency_try_new_empty_minor_symbol() {
    let currency = ObjCurrency::try_new("JPY", "¥", "", "Japanese Yen", 0).unwrap();

    let obj_money = ObjMoney::<false>::new(currency, dec!(123.98));

    assert_eq!(obj_money.minor_unit(), 0);
    assert_eq!(obj_money.amount(), dec!(124));
}

#[test]
fn currency_try_new_invalid_code() {
    assert!(ObjCurrency::try_new("", "$", "¢", "Dollar", 2).is_err());
}

#[test]
fn currency_try_new_invalid_symbol() {
    assert!(ObjCurrency::try_new("USD", "", "¢", "Dollar", 2).is_err());
}

#[test]
fn currency_try_new_invalid_minor_unit_symbol() {
    assert!(
        ObjCurrency::try_new(
            "USD",
            "$",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "Dollar",
            2
        )
        .is_err()
    );
}

#[test]
fn currency_try_new_invalid_name() {
    assert!(ObjCurrency::try_new("USD", "$", "¢", "", 2).is_err());
}

#[test]
fn obj_money_try_new() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(123.45)).unwrap();

    assert_eq!(money.code(), "USD");
    assert_eq!(money.amount(), dec!(123.45));
    assert_eq!(money.symbol(), "$");
    assert_eq!(money.name(), "United States dollar");
    assert_eq!(money.minor_unit(), 2);
}

#[test]
fn obj_money_try_new_unknown_currency() {
    assert!(ObjMoney::<false>::try_new("ZZZ", dec!(1)).is_err());
}

#[test]
fn obj_money_try_new_invalid_code() {
    assert!(ObjMoney::<false>::try_new("", dec!(1)).is_err());
}

#[test]
fn set_amount() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1))
        .unwrap()
        .set_amount(dec!(99.99));

    assert_eq!(money.amount(), dec!(99.99));
    assert_eq!(money.code(), "USD");
}

#[test]
fn amount_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(123.45)).unwrap();

    assert_eq!(money.amount(), dec!(123.45));
}

#[test]
fn minor_amount() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(123.45)).unwrap();

    assert_eq!(money.minor_amount(), Some(12345));
}

#[test]
fn code_getter() {
    let money: ObjMoney = ObjMoney::try_new("EUR", dec!(1)).unwrap();

    assert_eq!(money.code(), "EUR");
}

#[test]
fn symbol_getter() {
    let money: ObjMoney = ObjMoney::try_new("EUR", dec!(1)).unwrap();

    assert_eq!(money.symbol(), "€");
}

#[test]
fn minor_unit_symbol_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();

    assert_eq!(money.minor_unit_symbol(), "¢");
}

#[test]
fn name_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();

    assert_eq!(money.name(), "United States dollar");
}

#[test]
fn minor_unit_getter() {
    let money: ObjMoney = ObjMoney::try_new("USD", dec!(1)).unwrap();

    assert_eq!(money.minor_unit(), 2);
}

#[test]
fn register_currency_test() {
    register_currency("XYZ", "@", "", "Test Currency", 3).unwrap();

    let money: ObjMoney = ObjMoney::try_new("XYZ", dec!(1.234)).unwrap();

    // invalid code
    assert!(register_currency("XY👄", "@", "", "Test Currency", 3).is_err());
    assert!(register_currency("GNU", "", "", "Test Currency", 3).is_err());
    assert!(
        register_currency(
            "DDD",
            "@",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "Test Currency",
            3
        )
        .is_err()
    );
    assert!(register_currency("BBB", "@", "", "", 3).is_err());

    assert_eq!(money.code(), "XYZ");
    assert_eq!(money.symbol(), "@");
    assert_eq!(money.name(), "Test Currency");
    assert_eq!(money.minor_unit(), 3);

    let ret = register_currency("XYZ", "@", "", "Test Currency", 3);
    assert!(ret.is_err());

    // race condition tests
    let currencies = [
        ("AAA", "A", "a", "Axx", 2),
        ("BBB", "B", "b", "Bxx", 1),
        ("CCC", "C", "c", "Cxx", 3),
        ("DDD", "D", "d", "Dxx", 0),
    ];
    let races: Vec<_> = (0..4)
        .map(|i| {
            std::thread::spawn(move || {
                let ret = register_currency(
                    currencies[i].0,
                    currencies[i].1,
                    currencies[i].2,
                    currencies[i].3,
                    currencies[i].4,
                );
                ret
            })
        })
        .collect();

    let results: Vec<_> = races.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(results.iter().all(|r| r.is_ok()));
}

#[test]
fn dyn_money_obj_money() {
    let money = ObjMoney::<false>::try_new("USD", dec!(42)).unwrap();

    let dyn_money: &dyn DynMoney = &money;

    assert_eq!(dyn_money.amount(), dec!(42));
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.symbol(), "$");
    assert_eq!(dyn_money.minor_unit(), 2);
}

#[test]
fn dyn_money_money() {
    let money = crate::Money::<crate::iso::USD>::from_decimal(dec!(42));

    let dyn_money: &dyn DynMoney = &money;

    assert_eq!(dyn_money.amount(), dec!(42));
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.symbol(), "$");
    assert_eq!(dyn_money.minor_unit(), 2);
}

#[cfg(feature = "raw_money")]
#[test]
fn dyn_money_raw_money() {
    let money = crate::RawMoney::<crate::iso::USD>::from_decimal(dec!(42));

    let dyn_money: &dyn DynMoney = &money;

    assert_eq!(dyn_money.amount(), dec!(42));
    assert_eq!(dyn_money.code(), "USD");
    assert_eq!(dyn_money.symbol(), "$");
    assert_eq!(dyn_money.minor_unit(), 2);
}

// raw object moneys
#[test]
fn raw_obj_money_test() {
    let raw_obj_money = ObjMoney::<true>::try_new("EUR", dec!(123.43284)).unwrap();
    assert_eq!(raw_obj_money.code(), "EUR");
    assert_eq!(raw_obj_money.amount(), dec!(123.43284));

    let minor_amount = raw_obj_money.minor_amount().unwrap();
    assert_eq!(minor_amount, 12343);

    let rounded = raw_obj_money.round();
    assert_eq!(rounded.code(), "EUR");
    assert_eq!(rounded.amount(), dec!(123.43));

    let rounded = raw_obj_money.round_with(3, crate::RoundingStrategy::BankersRounding);
    assert_eq!(rounded.code(), "EUR");
    assert_eq!(rounded.amount(), dec!(123.433));
}

// conversion & ops
#[test]
fn obj_money_ops_tests() {
    let raw_obj: ObjMoney<true> = ObjMoney::try_new("USD", dec!(123.456)).unwrap();
    assert_eq!(raw_obj.code(), "USD");
    assert_eq!(raw_obj.amount(), dec!(123.456));

    assert!(raw_obj.is_positive());

    // raw -> round
    let rounded_raw_obj: ObjMoney<false> = raw_obj.into();
    assert_eq!(rounded_raw_obj.code(), "USD");
    assert_eq!(rounded_raw_obj.amount(), dec!(123.46));

    let ret: ObjMoney<true> = rounded_raw_obj.into();
    assert_eq!(ret.code(), "USD");
    assert_eq!(ret.amount(), dec!(123.46));

    // raw rounded
    let raw_obj_rounded: ObjMoney<true> = raw_obj.round();
    assert_eq!(raw_obj_rounded.code(), "USD");
    assert_eq!(raw_obj_rounded.amount(), dec!(123.46));

    assert_eq!(rounded_raw_obj.amount(), raw_obj_rounded.amount());

    // negated
    let neg_raw_obj = -raw_obj;
    assert_eq!(neg_raw_obj.code(), "USD");
    assert_eq!(neg_raw_obj.amount(), dec!(-123.456));

    let neg_rounded_obj = -rounded_raw_obj;
    assert_eq!(neg_rounded_obj.code(), "USD");
    assert_eq!(neg_rounded_obj.amount(), dec!(-123.46));

    // ops
    // addition
    let neg_ops = raw_obj.checked_add(neg_raw_obj).unwrap();
    assert_eq!(neg_ops.code(), "USD");
    assert_eq!(neg_ops.amount(), dec!(0));

    assert!(!neg_ops.is_positive() && !neg_ops.is_negative() && neg_ops.is_zero());

    let add = raw_obj.checked_add(raw_obj).unwrap();
    assert_eq!(add.code(), "USD");
    assert_eq!(add.amount(), dec!(246.912));

    let add = raw_obj.checked_add(rounded_raw_obj).unwrap();
    assert_eq!(add.code(), "USD");
    assert_eq!(add.amount(), dec!(246.916));

    let eur = ObjMoney::<false>::try_new("EUR", dec!(234)).unwrap();
    let eur_raw = ObjMoney::<true>::try_new("EUR", dec!(234.4440)).unwrap();
    let ret = eur_raw.checked_add(eur).unwrap();
    assert_eq!(ret.code(), "EUR");
    assert_eq!(ret.amount(), dec!(468.444));

    let curr_mismatch = raw_obj.checked_add(eur);
    assert!(curr_mismatch.is_err());

    let max = ObjMoney::<false>::try_new("USD", Decimal::MAX).unwrap();
    let ret = max.checked_add(raw_obj);
    assert!(ret.is_err());

    // substraction
    let neg_ops = raw_obj.checked_sub(neg_raw_obj).unwrap();
    assert_eq!(neg_ops.code(), "USD");
    assert_eq!(neg_ops.amount(), dec!(246.912));

    let sub = raw_obj.checked_sub(raw_obj).unwrap();
    assert_eq!(sub.code(), "USD");
    assert_eq!(sub.amount(), dec!(0));

    let sub = raw_obj.checked_sub(rounded_raw_obj).unwrap();
    assert_eq!(sub.code(), "USD");
    assert_eq!(sub.amount(), dec!(-0.004));

    let eur = ObjMoney::<false>::try_new("EUR", dec!(234)).unwrap();
    let eur_raw = ObjMoney::<true>::try_new("EUR", dec!(234.4440)).unwrap();
    let ret = eur_raw.checked_sub(eur).unwrap();
    assert_eq!(ret.code(), "EUR");
    assert_eq!(ret.amount(), dec!(0.444));

    let curr_mismatch = raw_obj.checked_sub(eur);
    assert!(curr_mismatch.is_err());

    let min = ObjMoney::<false>::try_new("USD", Decimal::MIN).unwrap();
    let ret = min.checked_sub(raw_obj);
    assert!(ret.is_err());

    // multiplication
    let mul = rounded_raw_obj.checked_mul(4).unwrap();
    assert_eq!(mul.code(), "USD");
    assert_eq!(mul.amount(), dec!(493.84));

    let mul = rounded_raw_obj.checked_mul(-2.5).unwrap();
    assert_eq!(mul.code(), "USD");
    assert_eq!(mul.amount(), dec!(-308.650));

    let max = ObjMoney::<false>::try_new("USD", Decimal::MAX).unwrap();
    let ret = max.checked_mul(23);
    assert!(ret.is_err());
    let ret = raw_obj.checked_mul(i128::MAX);
    assert!(ret.is_err());

    // division
    let div = neg_raw_obj.checked_div(10).unwrap();
    assert_eq!(div.code(), "USD");
    assert_eq!(div.amount(), dec!(-12.3456));

    let div = raw_obj.checked_div(4).unwrap();
    assert_eq!(div.code(), "USD");
    assert_eq!(div.amount(), dec!(30.864));

    let absed = neg_rounded_obj.abs();
    assert_eq!(absed.code(), "USD");
    assert_eq!(absed.amount(), dec!(123.46));

    let max = ObjMoney::<false>::try_new("USD", Decimal::MAX).unwrap();
    let ret = max.checked_div(0);
    assert!(ret.is_err());
    let ret = raw_obj.checked_div(i128::MAX);
    assert!(ret.is_err());

    // rem
    let m = ObjMoney::<true>::try_new("IDR", dec!(43_000.248)).unwrap();
    let ret = m.checked_rem(3).unwrap();
    assert_eq!(ret.amount(), dec!(1.248));
    assert_eq!(dec!(43_000.248).checked_rem(dec!(3)).unwrap(), dec!(1.248));

    let max = ObjMoney::<false>::try_new("USD", Decimal::MAX).unwrap();
    let ret = max.checked_rem(0);
    assert!(ret.is_err());
    let ret = raw_obj.checked_rem(i128::MAX);
    assert!(ret.is_err());
}

#[test]
fn test_obj_money_parsing() {
    let input_money_comma = "USD 40,023.498";
    let input_money_dot = "USD 40.023,498";

    let tender_money_comma = ObjMoney::<false>::from_str_code(input_money_comma, ",", ".").unwrap();
    assert_eq!(tender_money_comma.code(), "USD");
    assert_eq!(tender_money_comma.amount(), dec!(40_023.50));

    let raw_money_comma = ObjMoney::<true>::from_str_code(input_money_comma, ",", ".").unwrap();
    assert_eq!(raw_money_comma.code(), "USD");
    assert_eq!(raw_money_comma.amount(), dec!(40_023.4980));

    let tender_money_dot = ObjMoney::<false>::from_str_code(input_money_dot, ".", ",").unwrap();
    assert_eq!(tender_money_dot.code(), "USD");
    assert_eq!(tender_money_dot.amount(), dec!(40_023.50));

    let raw_money_dot = ObjMoney::<true>::from_str_code(input_money_dot, ".", ",").unwrap();
    assert_eq!(raw_money_dot.code(), "USD");
    assert_eq!(raw_money_dot.amount(), dec!(40_023.4980));

    let empty = "";
    let failed = ObjMoney::<false>::from_str_code(empty, ",", ".");
    assert!(failed.is_err());

    let empty = " ";
    let failed = ObjMoney::<false>::from_str_code(empty, ",", ".");
    assert!(failed.is_err());

    let invalid_amount = "USD nganu ";
    let failed = ObjMoney::<false>::from_str_code(invalid_amount, ",", ".");
    assert!(failed.is_err());

    let overflow_amount =
        "USD 123,234,234,345,234,345,112,234,000,000,000,000,000,555.928349823942834";
    let failed = ObjMoney::<false>::from_str_code(overflow_amount, ",", ".");
    assert!(failed.is_err());
}

#[test]
fn test_obj_money_equality_ordering() {
    let tender = ObjMoney::<false>::try_new("USD", dec!(123.468)).unwrap();
    let raw = ObjMoney::<true>::try_new("USD", dec!(123.46997)).unwrap();
    assert!(tender > raw);
    assert!(raw <= tender);

    let eur = ObjMoney::<true>::try_new("EUR", dec!(124.46997)).unwrap();
    assert_eq!(eur > raw, false);

    assert!(tender == raw.round());
    let rounded: ObjMoney<false> = raw.into();
    assert!(tender == rounded);
}

// comptime vs objmoney conversion
#[test]
fn comptime_obj_money_conversion_test() {
    use crate::{Money, RawMoney, money, raw};

    let money = money!(USD, 2390.3324);
    let raw = raw!(USD, 123.0987);
    let obj_money = ObjMoney::<false>::try_new("USD", dec!(123.123)).unwrap();
    let obj_money_raw = ObjMoney::<true>::try_new("USD", dec!(123.123)).unwrap();
    let obj_money_eur = ObjMoney::<false>::try_new("EUR", dec!(123.123)).unwrap();
    let obj_money_raw_eur = ObjMoney::<true>::try_new("EUR", dec!(123.123)).unwrap();

    // comptime -> runtime
    let money_to_obj_money: ObjMoney = money.try_into().unwrap();
    assert_eq!(money_to_obj_money.code(), "USD");
    assert_eq!(money_to_obj_money.amount(), dec!(2390.33));

    let money_to_obj_money_raw: ObjMoney<true> = money.try_into().unwrap();
    assert_eq!(money_to_obj_money_raw.code(), "USD");
    assert_eq!(money_to_obj_money_raw.amount(), dec!(2390.33)); // money! already rounded it

    let raw_to_obj_money: ObjMoney = raw.try_into().unwrap();
    assert_eq!(raw_to_obj_money.code(), "USD");
    assert_eq!(raw_to_obj_money.amount(), dec!(123.1));

    let raw_to_obj_money_raw: ObjMoney<true> = raw.try_into().unwrap();
    assert_eq!(raw_to_obj_money_raw.code(), "USD");
    assert_eq!(raw_to_obj_money_raw.amount(), dec!(123.0987));

    // runtime -> comptime
    let obj_money_to_money: Money<crate::iso::USD> = obj_money.try_into().unwrap();
    assert_eq!(BaseMoney::amount(&obj_money_to_money), dec!(123.12));

    let obj_money_to_raw: RawMoney<crate::iso::USD> = obj_money.try_into().unwrap();
    assert_eq!(BaseMoney::amount(&obj_money_to_raw), dec!(123.12));

    let obj_money_raw_to_money: Money<crate::iso::USD> = obj_money_raw.try_into().unwrap();
    assert_eq!(BaseMoney::amount(&obj_money_raw_to_money), dec!(123.12));

    let obj_money_raw_to_raw: RawMoney<crate::iso::USD> = obj_money_raw.try_into().unwrap();
    assert_eq!(BaseMoney::amount(&obj_money_raw_to_raw), dec!(123.123));

    // --
    let obj_money_eur_to_money: Result<Money<crate::iso::USD>, MoneyError> =
        obj_money_eur.try_into();
    assert!(obj_money_eur_to_money.is_err());

    let obj_money_raw_eur_to_raw_money: Result<RawMoney<crate::iso::USD>, MoneyError> =
        obj_money_raw_eur.try_into();
    assert!(obj_money_raw_eur_to_raw_money.is_err());
}
