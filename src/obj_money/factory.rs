use crate::Decimal;

use super::ObjMoney;

/// Creates a [`Money`](crate::Money)-backed [`ObjMoney`] trait object for the ISO 4217 currency
/// identified by `code`.
///
/// The amount is rounded to the currency's minor unit using banker's rounding, exactly as
/// [`Money::from_decimal`](crate::Money::from_decimal) does.
///
/// Returns `None` when `code` is not a recognised ISO 4217 currency code.
///
/// # Examples
///
/// ```
/// use moneylib::ObjMoney;
/// use moneylib::obj_money::make_money_from_code;
/// use moneylib::macros::dec;
///
/// let usd = make_money_from_code("USD", dec!(100.50)).unwrap();
/// assert_eq!(usd.code(), "USD");
/// assert_eq!(usd.amount(), dec!(100.50));
///
/// let eur = make_money_from_code("EUR", dec!(200.75)).unwrap();
/// assert_eq!(eur.code(), "EUR");
///
/// // Unknown code returns None
/// assert!(make_money_from_code("INVALID", dec!(1)).is_none());
/// ```
pub fn make_money_from_code(code: &str, amount: Decimal) -> Option<Box<dyn ObjMoney>> {
    match code {
        "AED" => Some(Box::new(crate::Money::<crate::iso::AED>::from_decimal(
            amount,
        ))),
        "AFN" => Some(Box::new(crate::Money::<crate::iso::AFN>::from_decimal(
            amount,
        ))),
        "ALL" => Some(Box::new(crate::Money::<crate::iso::ALL>::from_decimal(
            amount,
        ))),
        "AMD" => Some(Box::new(crate::Money::<crate::iso::AMD>::from_decimal(
            amount,
        ))),
        "AOA" => Some(Box::new(crate::Money::<crate::iso::AOA>::from_decimal(
            amount,
        ))),
        "ARS" => Some(Box::new(crate::Money::<crate::iso::ARS>::from_decimal(
            amount,
        ))),
        "AUD" => Some(Box::new(crate::Money::<crate::iso::AUD>::from_decimal(
            amount,
        ))),
        "AWG" => Some(Box::new(crate::Money::<crate::iso::AWG>::from_decimal(
            amount,
        ))),
        "AZN" => Some(Box::new(crate::Money::<crate::iso::AZN>::from_decimal(
            amount,
        ))),
        "BAM" => Some(Box::new(crate::Money::<crate::iso::BAM>::from_decimal(
            amount,
        ))),
        "BBD" => Some(Box::new(crate::Money::<crate::iso::BBD>::from_decimal(
            amount,
        ))),
        "BDT" => Some(Box::new(crate::Money::<crate::iso::BDT>::from_decimal(
            amount,
        ))),
        "BGN" => Some(Box::new(crate::Money::<crate::iso::BGN>::from_decimal(
            amount,
        ))),
        "BHD" => Some(Box::new(crate::Money::<crate::iso::BHD>::from_decimal(
            amount,
        ))),
        "BIF" => Some(Box::new(crate::Money::<crate::iso::BIF>::from_decimal(
            amount,
        ))),
        "BMD" => Some(Box::new(crate::Money::<crate::iso::BMD>::from_decimal(
            amount,
        ))),
        "BND" => Some(Box::new(crate::Money::<crate::iso::BND>::from_decimal(
            amount,
        ))),
        "BOB" => Some(Box::new(crate::Money::<crate::iso::BOB>::from_decimal(
            amount,
        ))),
        "BOV" => Some(Box::new(crate::Money::<crate::iso::BOV>::from_decimal(
            amount,
        ))),
        "BRL" => Some(Box::new(crate::Money::<crate::iso::BRL>::from_decimal(
            amount,
        ))),
        "BSD" => Some(Box::new(crate::Money::<crate::iso::BSD>::from_decimal(
            amount,
        ))),
        "BTN" => Some(Box::new(crate::Money::<crate::iso::BTN>::from_decimal(
            amount,
        ))),
        "BWP" => Some(Box::new(crate::Money::<crate::iso::BWP>::from_decimal(
            amount,
        ))),
        "BYN" => Some(Box::new(crate::Money::<crate::iso::BYN>::from_decimal(
            amount,
        ))),
        "BZD" => Some(Box::new(crate::Money::<crate::iso::BZD>::from_decimal(
            amount,
        ))),
        "CAD" => Some(Box::new(crate::Money::<crate::iso::CAD>::from_decimal(
            amount,
        ))),
        "CDF" => Some(Box::new(crate::Money::<crate::iso::CDF>::from_decimal(
            amount,
        ))),
        "CHE" => Some(Box::new(crate::Money::<crate::iso::CHE>::from_decimal(
            amount,
        ))),
        "CHF" => Some(Box::new(crate::Money::<crate::iso::CHF>::from_decimal(
            amount,
        ))),
        "CHW" => Some(Box::new(crate::Money::<crate::iso::CHW>::from_decimal(
            amount,
        ))),
        "CLF" => Some(Box::new(crate::Money::<crate::iso::CLF>::from_decimal(
            amount,
        ))),
        "CLP" => Some(Box::new(crate::Money::<crate::iso::CLP>::from_decimal(
            amount,
        ))),
        "CNY" => Some(Box::new(crate::Money::<crate::iso::CNY>::from_decimal(
            amount,
        ))),
        "COP" => Some(Box::new(crate::Money::<crate::iso::COP>::from_decimal(
            amount,
        ))),
        "COU" => Some(Box::new(crate::Money::<crate::iso::COU>::from_decimal(
            amount,
        ))),
        "CRC" => Some(Box::new(crate::Money::<crate::iso::CRC>::from_decimal(
            amount,
        ))),
        "CUC" => Some(Box::new(crate::Money::<crate::iso::CUC>::from_decimal(
            amount,
        ))),
        "CUP" => Some(Box::new(crate::Money::<crate::iso::CUP>::from_decimal(
            amount,
        ))),
        "CVE" => Some(Box::new(crate::Money::<crate::iso::CVE>::from_decimal(
            amount,
        ))),
        "CZK" => Some(Box::new(crate::Money::<crate::iso::CZK>::from_decimal(
            amount,
        ))),
        "DJF" => Some(Box::new(crate::Money::<crate::iso::DJF>::from_decimal(
            amount,
        ))),
        "DKK" => Some(Box::new(crate::Money::<crate::iso::DKK>::from_decimal(
            amount,
        ))),
        "DOP" => Some(Box::new(crate::Money::<crate::iso::DOP>::from_decimal(
            amount,
        ))),
        "DZD" => Some(Box::new(crate::Money::<crate::iso::DZD>::from_decimal(
            amount,
        ))),
        "EGP" => Some(Box::new(crate::Money::<crate::iso::EGP>::from_decimal(
            amount,
        ))),
        "ERN" => Some(Box::new(crate::Money::<crate::iso::ERN>::from_decimal(
            amount,
        ))),
        "ETB" => Some(Box::new(crate::Money::<crate::iso::ETB>::from_decimal(
            amount,
        ))),
        "EUR" => Some(Box::new(crate::Money::<crate::iso::EUR>::from_decimal(
            amount,
        ))),
        "FJD" => Some(Box::new(crate::Money::<crate::iso::FJD>::from_decimal(
            amount,
        ))),
        "FKP" => Some(Box::new(crate::Money::<crate::iso::FKP>::from_decimal(
            amount,
        ))),
        "GBP" => Some(Box::new(crate::Money::<crate::iso::GBP>::from_decimal(
            amount,
        ))),
        "GEL" => Some(Box::new(crate::Money::<crate::iso::GEL>::from_decimal(
            amount,
        ))),
        "GHS" => Some(Box::new(crate::Money::<crate::iso::GHS>::from_decimal(
            amount,
        ))),
        "GIP" => Some(Box::new(crate::Money::<crate::iso::GIP>::from_decimal(
            amount,
        ))),
        "GMD" => Some(Box::new(crate::Money::<crate::iso::GMD>::from_decimal(
            amount,
        ))),
        "GNF" => Some(Box::new(crate::Money::<crate::iso::GNF>::from_decimal(
            amount,
        ))),
        "GTQ" => Some(Box::new(crate::Money::<crate::iso::GTQ>::from_decimal(
            amount,
        ))),
        "GYD" => Some(Box::new(crate::Money::<crate::iso::GYD>::from_decimal(
            amount,
        ))),
        "HKD" => Some(Box::new(crate::Money::<crate::iso::HKD>::from_decimal(
            amount,
        ))),
        "HNL" => Some(Box::new(crate::Money::<crate::iso::HNL>::from_decimal(
            amount,
        ))),
        "HRK" => Some(Box::new(crate::Money::<crate::iso::HRK>::from_decimal(
            amount,
        ))),
        "HTG" => Some(Box::new(crate::Money::<crate::iso::HTG>::from_decimal(
            amount,
        ))),
        "HUF" => Some(Box::new(crate::Money::<crate::iso::HUF>::from_decimal(
            amount,
        ))),
        "IDR" => Some(Box::new(crate::Money::<crate::iso::IDR>::from_decimal(
            amount,
        ))),
        "ILS" => Some(Box::new(crate::Money::<crate::iso::ILS>::from_decimal(
            amount,
        ))),
        "INR" => Some(Box::new(crate::Money::<crate::iso::INR>::from_decimal(
            amount,
        ))),
        "IQD" => Some(Box::new(crate::Money::<crate::iso::IQD>::from_decimal(
            amount,
        ))),
        "IRR" => Some(Box::new(crate::Money::<crate::iso::IRR>::from_decimal(
            amount,
        ))),
        "ISK" => Some(Box::new(crate::Money::<crate::iso::ISK>::from_decimal(
            amount,
        ))),
        "JMD" => Some(Box::new(crate::Money::<crate::iso::JMD>::from_decimal(
            amount,
        ))),
        "JOD" => Some(Box::new(crate::Money::<crate::iso::JOD>::from_decimal(
            amount,
        ))),
        "JPY" => Some(Box::new(crate::Money::<crate::iso::JPY>::from_decimal(
            amount,
        ))),
        "KES" => Some(Box::new(crate::Money::<crate::iso::KES>::from_decimal(
            amount,
        ))),
        "KGS" => Some(Box::new(crate::Money::<crate::iso::KGS>::from_decimal(
            amount,
        ))),
        "KHR" => Some(Box::new(crate::Money::<crate::iso::KHR>::from_decimal(
            amount,
        ))),
        "KMF" => Some(Box::new(crate::Money::<crate::iso::KMF>::from_decimal(
            amount,
        ))),
        "KPW" => Some(Box::new(crate::Money::<crate::iso::KPW>::from_decimal(
            amount,
        ))),
        "KRW" => Some(Box::new(crate::Money::<crate::iso::KRW>::from_decimal(
            amount,
        ))),
        "KWD" => Some(Box::new(crate::Money::<crate::iso::KWD>::from_decimal(
            amount,
        ))),
        "KYD" => Some(Box::new(crate::Money::<crate::iso::KYD>::from_decimal(
            amount,
        ))),
        "KZT" => Some(Box::new(crate::Money::<crate::iso::KZT>::from_decimal(
            amount,
        ))),
        "LAK" => Some(Box::new(crate::Money::<crate::iso::LAK>::from_decimal(
            amount,
        ))),
        "LBP" => Some(Box::new(crate::Money::<crate::iso::LBP>::from_decimal(
            amount,
        ))),
        "LKR" => Some(Box::new(crate::Money::<crate::iso::LKR>::from_decimal(
            amount,
        ))),
        "LRD" => Some(Box::new(crate::Money::<crate::iso::LRD>::from_decimal(
            amount,
        ))),
        "LSL" => Some(Box::new(crate::Money::<crate::iso::LSL>::from_decimal(
            amount,
        ))),
        "LYD" => Some(Box::new(crate::Money::<crate::iso::LYD>::from_decimal(
            amount,
        ))),
        "MAD" => Some(Box::new(crate::Money::<crate::iso::MAD>::from_decimal(
            amount,
        ))),
        "MDL" => Some(Box::new(crate::Money::<crate::iso::MDL>::from_decimal(
            amount,
        ))),
        "MGA" => Some(Box::new(crate::Money::<crate::iso::MGA>::from_decimal(
            amount,
        ))),
        "MKD" => Some(Box::new(crate::Money::<crate::iso::MKD>::from_decimal(
            amount,
        ))),
        "MMK" => Some(Box::new(crate::Money::<crate::iso::MMK>::from_decimal(
            amount,
        ))),
        "MNT" => Some(Box::new(crate::Money::<crate::iso::MNT>::from_decimal(
            amount,
        ))),
        "MOP" => Some(Box::new(crate::Money::<crate::iso::MOP>::from_decimal(
            amount,
        ))),
        "MRU" => Some(Box::new(crate::Money::<crate::iso::MRU>::from_decimal(
            amount,
        ))),
        "MUR" => Some(Box::new(crate::Money::<crate::iso::MUR>::from_decimal(
            amount,
        ))),
        "MVR" => Some(Box::new(crate::Money::<crate::iso::MVR>::from_decimal(
            amount,
        ))),
        "MWK" => Some(Box::new(crate::Money::<crate::iso::MWK>::from_decimal(
            amount,
        ))),
        "MXN" => Some(Box::new(crate::Money::<crate::iso::MXN>::from_decimal(
            amount,
        ))),
        "MXV" => Some(Box::new(crate::Money::<crate::iso::MXV>::from_decimal(
            amount,
        ))),
        "MYR" => Some(Box::new(crate::Money::<crate::iso::MYR>::from_decimal(
            amount,
        ))),
        "MZN" => Some(Box::new(crate::Money::<crate::iso::MZN>::from_decimal(
            amount,
        ))),
        "NAD" => Some(Box::new(crate::Money::<crate::iso::NAD>::from_decimal(
            amount,
        ))),
        "NGN" => Some(Box::new(crate::Money::<crate::iso::NGN>::from_decimal(
            amount,
        ))),
        "NIO" => Some(Box::new(crate::Money::<crate::iso::NIO>::from_decimal(
            amount,
        ))),
        "NOK" => Some(Box::new(crate::Money::<crate::iso::NOK>::from_decimal(
            amount,
        ))),
        "NPR" => Some(Box::new(crate::Money::<crate::iso::NPR>::from_decimal(
            amount,
        ))),
        "NZD" => Some(Box::new(crate::Money::<crate::iso::NZD>::from_decimal(
            amount,
        ))),
        "OMR" => Some(Box::new(crate::Money::<crate::iso::OMR>::from_decimal(
            amount,
        ))),
        "PAB" => Some(Box::new(crate::Money::<crate::iso::PAB>::from_decimal(
            amount,
        ))),
        "PEN" => Some(Box::new(crate::Money::<crate::iso::PEN>::from_decimal(
            amount,
        ))),
        "PGK" => Some(Box::new(crate::Money::<crate::iso::PGK>::from_decimal(
            amount,
        ))),
        "PHP" => Some(Box::new(crate::Money::<crate::iso::PHP>::from_decimal(
            amount,
        ))),
        "PKR" => Some(Box::new(crate::Money::<crate::iso::PKR>::from_decimal(
            amount,
        ))),
        "PLN" => Some(Box::new(crate::Money::<crate::iso::PLN>::from_decimal(
            amount,
        ))),
        "PYG" => Some(Box::new(crate::Money::<crate::iso::PYG>::from_decimal(
            amount,
        ))),
        "QAR" => Some(Box::new(crate::Money::<crate::iso::QAR>::from_decimal(
            amount,
        ))),
        "RON" => Some(Box::new(crate::Money::<crate::iso::RON>::from_decimal(
            amount,
        ))),
        "RSD" => Some(Box::new(crate::Money::<crate::iso::RSD>::from_decimal(
            amount,
        ))),
        "RUB" => Some(Box::new(crate::Money::<crate::iso::RUB>::from_decimal(
            amount,
        ))),
        "RWF" => Some(Box::new(crate::Money::<crate::iso::RWF>::from_decimal(
            amount,
        ))),
        "SAR" => Some(Box::new(crate::Money::<crate::iso::SAR>::from_decimal(
            amount,
        ))),
        "SBD" => Some(Box::new(crate::Money::<crate::iso::SBD>::from_decimal(
            amount,
        ))),
        "SCR" => Some(Box::new(crate::Money::<crate::iso::SCR>::from_decimal(
            amount,
        ))),
        "SDG" => Some(Box::new(crate::Money::<crate::iso::SDG>::from_decimal(
            amount,
        ))),
        "SEK" => Some(Box::new(crate::Money::<crate::iso::SEK>::from_decimal(
            amount,
        ))),
        "SGD" => Some(Box::new(crate::Money::<crate::iso::SGD>::from_decimal(
            amount,
        ))),
        "SHP" => Some(Box::new(crate::Money::<crate::iso::SHP>::from_decimal(
            amount,
        ))),
        "SLE" => Some(Box::new(crate::Money::<crate::iso::SLE>::from_decimal(
            amount,
        ))),
        "SLL" => Some(Box::new(crate::Money::<crate::iso::SLL>::from_decimal(
            amount,
        ))),
        "SOS" => Some(Box::new(crate::Money::<crate::iso::SOS>::from_decimal(
            amount,
        ))),
        "SRD" => Some(Box::new(crate::Money::<crate::iso::SRD>::from_decimal(
            amount,
        ))),
        "SSP" => Some(Box::new(crate::Money::<crate::iso::SSP>::from_decimal(
            amount,
        ))),
        "STN" => Some(Box::new(crate::Money::<crate::iso::STN>::from_decimal(
            amount,
        ))),
        "SVC" => Some(Box::new(crate::Money::<crate::iso::SVC>::from_decimal(
            amount,
        ))),
        "SYP" => Some(Box::new(crate::Money::<crate::iso::SYP>::from_decimal(
            amount,
        ))),
        "SZL" => Some(Box::new(crate::Money::<crate::iso::SZL>::from_decimal(
            amount,
        ))),
        "THB" => Some(Box::new(crate::Money::<crate::iso::THB>::from_decimal(
            amount,
        ))),
        "TJS" => Some(Box::new(crate::Money::<crate::iso::TJS>::from_decimal(
            amount,
        ))),
        "TMT" => Some(Box::new(crate::Money::<crate::iso::TMT>::from_decimal(
            amount,
        ))),
        "TND" => Some(Box::new(crate::Money::<crate::iso::TND>::from_decimal(
            amount,
        ))),
        "TOP" => Some(Box::new(crate::Money::<crate::iso::TOP>::from_decimal(
            amount,
        ))),
        "TRY" => Some(Box::new(crate::Money::<crate::iso::TRY>::from_decimal(
            amount,
        ))),
        "TTD" => Some(Box::new(crate::Money::<crate::iso::TTD>::from_decimal(
            amount,
        ))),
        "TWD" => Some(Box::new(crate::Money::<crate::iso::TWD>::from_decimal(
            amount,
        ))),
        "TZS" => Some(Box::new(crate::Money::<crate::iso::TZS>::from_decimal(
            amount,
        ))),
        "UAH" => Some(Box::new(crate::Money::<crate::iso::UAH>::from_decimal(
            amount,
        ))),
        "UGX" => Some(Box::new(crate::Money::<crate::iso::UGX>::from_decimal(
            amount,
        ))),
        "USD" => Some(Box::new(crate::Money::<crate::iso::USD>::from_decimal(
            amount,
        ))),
        "USN" => Some(Box::new(crate::Money::<crate::iso::USN>::from_decimal(
            amount,
        ))),
        "UYI" => Some(Box::new(crate::Money::<crate::iso::UYI>::from_decimal(
            amount,
        ))),
        "UYU" => Some(Box::new(crate::Money::<crate::iso::UYU>::from_decimal(
            amount,
        ))),
        "UYW" => Some(Box::new(crate::Money::<crate::iso::UYW>::from_decimal(
            amount,
        ))),
        "UZS" => Some(Box::new(crate::Money::<crate::iso::UZS>::from_decimal(
            amount,
        ))),
        "VED" => Some(Box::new(crate::Money::<crate::iso::VED>::from_decimal(
            amount,
        ))),
        "VES" => Some(Box::new(crate::Money::<crate::iso::VES>::from_decimal(
            amount,
        ))),
        "VND" => Some(Box::new(crate::Money::<crate::iso::VND>::from_decimal(
            amount,
        ))),
        "VUV" => Some(Box::new(crate::Money::<crate::iso::VUV>::from_decimal(
            amount,
        ))),
        "WST" => Some(Box::new(crate::Money::<crate::iso::WST>::from_decimal(
            amount,
        ))),
        "XAF" => Some(Box::new(crate::Money::<crate::iso::XAF>::from_decimal(
            amount,
        ))),
        "XAG" => Some(Box::new(crate::Money::<crate::iso::XAG>::from_decimal(
            amount,
        ))),
        "XAU" => Some(Box::new(crate::Money::<crate::iso::XAU>::from_decimal(
            amount,
        ))),
        "XBA" => Some(Box::new(crate::Money::<crate::iso::XBA>::from_decimal(
            amount,
        ))),
        "XBB" => Some(Box::new(crate::Money::<crate::iso::XBB>::from_decimal(
            amount,
        ))),
        "XBC" => Some(Box::new(crate::Money::<crate::iso::XBC>::from_decimal(
            amount,
        ))),
        "XBD" => Some(Box::new(crate::Money::<crate::iso::XBD>::from_decimal(
            amount,
        ))),
        "XCD" => Some(Box::new(crate::Money::<crate::iso::XCD>::from_decimal(
            amount,
        ))),
        "XCG" => Some(Box::new(crate::Money::<crate::iso::XCG>::from_decimal(
            amount,
        ))),
        "XDR" => Some(Box::new(crate::Money::<crate::iso::XDR>::from_decimal(
            amount,
        ))),
        "XOF" => Some(Box::new(crate::Money::<crate::iso::XOF>::from_decimal(
            amount,
        ))),
        "XPD" => Some(Box::new(crate::Money::<crate::iso::XPD>::from_decimal(
            amount,
        ))),
        "XPF" => Some(Box::new(crate::Money::<crate::iso::XPF>::from_decimal(
            amount,
        ))),
        "XPT" => Some(Box::new(crate::Money::<crate::iso::XPT>::from_decimal(
            amount,
        ))),
        "XSU" => Some(Box::new(crate::Money::<crate::iso::XSU>::from_decimal(
            amount,
        ))),
        "XTS" => Some(Box::new(crate::Money::<crate::iso::XTS>::from_decimal(
            amount,
        ))),
        "XUA" => Some(Box::new(crate::Money::<crate::iso::XUA>::from_decimal(
            amount,
        ))),
        "XXX" => Some(Box::new(crate::Money::<crate::iso::XXX>::from_decimal(
            amount,
        ))),
        "YER" => Some(Box::new(crate::Money::<crate::iso::YER>::from_decimal(
            amount,
        ))),
        "ZAR" => Some(Box::new(crate::Money::<crate::iso::ZAR>::from_decimal(
            amount,
        ))),
        "ZMW" => Some(Box::new(crate::Money::<crate::iso::ZMW>::from_decimal(
            amount,
        ))),
        "ZWG" => Some(Box::new(crate::Money::<crate::iso::ZWG>::from_decimal(
            amount,
        ))),
        "ZWL" => Some(Box::new(crate::Money::<crate::iso::ZWL>::from_decimal(
            amount,
        ))),
        _ => None,
    }
}

/// Creates a [`RawMoney`](crate::RawMoney)-backed [`ObjMoney`] trait object for the ISO 4217
/// currency identified by `code`.
///
/// The amount is stored without any rounding, preserving full decimal precision, exactly as
/// [`RawMoney::from_decimal`](crate::RawMoney::from_decimal) does.
///
/// Returns `None` when `code` is not a recognised ISO 4217 currency code.
///
/// # Examples
///
/// ```
/// use moneylib::ObjMoney;
/// use moneylib::obj_money::make_raw_money_from_code;
/// use moneylib::macros::dec;
///
/// let usd = make_raw_money_from_code("USD", dec!(100.56789)).unwrap();
/// assert_eq!(usd.code(), "USD");
/// assert_eq!(usd.amount(), dec!(100.56789));
///
/// // Unknown code returns None
/// assert!(make_raw_money_from_code("INVALID", dec!(1)).is_none());
/// ```
#[cfg(feature = "raw_money")]
pub fn make_raw_money_from_code(code: &str, amount: Decimal) -> Option<Box<dyn ObjMoney>> {
    match code {
        "AED" => Some(Box::new(crate::RawMoney::<crate::iso::AED>::from_decimal(
            amount,
        ))),
        "AFN" => Some(Box::new(crate::RawMoney::<crate::iso::AFN>::from_decimal(
            amount,
        ))),
        "ALL" => Some(Box::new(crate::RawMoney::<crate::iso::ALL>::from_decimal(
            amount,
        ))),
        "AMD" => Some(Box::new(crate::RawMoney::<crate::iso::AMD>::from_decimal(
            amount,
        ))),
        "AOA" => Some(Box::new(crate::RawMoney::<crate::iso::AOA>::from_decimal(
            amount,
        ))),
        "ARS" => Some(Box::new(crate::RawMoney::<crate::iso::ARS>::from_decimal(
            amount,
        ))),
        "AUD" => Some(Box::new(crate::RawMoney::<crate::iso::AUD>::from_decimal(
            amount,
        ))),
        "AWG" => Some(Box::new(crate::RawMoney::<crate::iso::AWG>::from_decimal(
            amount,
        ))),
        "AZN" => Some(Box::new(crate::RawMoney::<crate::iso::AZN>::from_decimal(
            amount,
        ))),
        "BAM" => Some(Box::new(crate::RawMoney::<crate::iso::BAM>::from_decimal(
            amount,
        ))),
        "BBD" => Some(Box::new(crate::RawMoney::<crate::iso::BBD>::from_decimal(
            amount,
        ))),
        "BDT" => Some(Box::new(crate::RawMoney::<crate::iso::BDT>::from_decimal(
            amount,
        ))),
        "BGN" => Some(Box::new(crate::RawMoney::<crate::iso::BGN>::from_decimal(
            amount,
        ))),
        "BHD" => Some(Box::new(crate::RawMoney::<crate::iso::BHD>::from_decimal(
            amount,
        ))),
        "BIF" => Some(Box::new(crate::RawMoney::<crate::iso::BIF>::from_decimal(
            amount,
        ))),
        "BMD" => Some(Box::new(crate::RawMoney::<crate::iso::BMD>::from_decimal(
            amount,
        ))),
        "BND" => Some(Box::new(crate::RawMoney::<crate::iso::BND>::from_decimal(
            amount,
        ))),
        "BOB" => Some(Box::new(crate::RawMoney::<crate::iso::BOB>::from_decimal(
            amount,
        ))),
        "BOV" => Some(Box::new(crate::RawMoney::<crate::iso::BOV>::from_decimal(
            amount,
        ))),
        "BRL" => Some(Box::new(crate::RawMoney::<crate::iso::BRL>::from_decimal(
            amount,
        ))),
        "BSD" => Some(Box::new(crate::RawMoney::<crate::iso::BSD>::from_decimal(
            amount,
        ))),
        "BTN" => Some(Box::new(crate::RawMoney::<crate::iso::BTN>::from_decimal(
            amount,
        ))),
        "BWP" => Some(Box::new(crate::RawMoney::<crate::iso::BWP>::from_decimal(
            amount,
        ))),
        "BYN" => Some(Box::new(crate::RawMoney::<crate::iso::BYN>::from_decimal(
            amount,
        ))),
        "BZD" => Some(Box::new(crate::RawMoney::<crate::iso::BZD>::from_decimal(
            amount,
        ))),
        "CAD" => Some(Box::new(crate::RawMoney::<crate::iso::CAD>::from_decimal(
            amount,
        ))),
        "CDF" => Some(Box::new(crate::RawMoney::<crate::iso::CDF>::from_decimal(
            amount,
        ))),
        "CHE" => Some(Box::new(crate::RawMoney::<crate::iso::CHE>::from_decimal(
            amount,
        ))),
        "CHF" => Some(Box::new(crate::RawMoney::<crate::iso::CHF>::from_decimal(
            amount,
        ))),
        "CHW" => Some(Box::new(crate::RawMoney::<crate::iso::CHW>::from_decimal(
            amount,
        ))),
        "CLF" => Some(Box::new(crate::RawMoney::<crate::iso::CLF>::from_decimal(
            amount,
        ))),
        "CLP" => Some(Box::new(crate::RawMoney::<crate::iso::CLP>::from_decimal(
            amount,
        ))),
        "CNY" => Some(Box::new(crate::RawMoney::<crate::iso::CNY>::from_decimal(
            amount,
        ))),
        "COP" => Some(Box::new(crate::RawMoney::<crate::iso::COP>::from_decimal(
            amount,
        ))),
        "COU" => Some(Box::new(crate::RawMoney::<crate::iso::COU>::from_decimal(
            amount,
        ))),
        "CRC" => Some(Box::new(crate::RawMoney::<crate::iso::CRC>::from_decimal(
            amount,
        ))),
        "CUC" => Some(Box::new(crate::RawMoney::<crate::iso::CUC>::from_decimal(
            amount,
        ))),
        "CUP" => Some(Box::new(crate::RawMoney::<crate::iso::CUP>::from_decimal(
            amount,
        ))),
        "CVE" => Some(Box::new(crate::RawMoney::<crate::iso::CVE>::from_decimal(
            amount,
        ))),
        "CZK" => Some(Box::new(crate::RawMoney::<crate::iso::CZK>::from_decimal(
            amount,
        ))),
        "DJF" => Some(Box::new(crate::RawMoney::<crate::iso::DJF>::from_decimal(
            amount,
        ))),
        "DKK" => Some(Box::new(crate::RawMoney::<crate::iso::DKK>::from_decimal(
            amount,
        ))),
        "DOP" => Some(Box::new(crate::RawMoney::<crate::iso::DOP>::from_decimal(
            amount,
        ))),
        "DZD" => Some(Box::new(crate::RawMoney::<crate::iso::DZD>::from_decimal(
            amount,
        ))),
        "EGP" => Some(Box::new(crate::RawMoney::<crate::iso::EGP>::from_decimal(
            amount,
        ))),
        "ERN" => Some(Box::new(crate::RawMoney::<crate::iso::ERN>::from_decimal(
            amount,
        ))),
        "ETB" => Some(Box::new(crate::RawMoney::<crate::iso::ETB>::from_decimal(
            amount,
        ))),
        "EUR" => Some(Box::new(crate::RawMoney::<crate::iso::EUR>::from_decimal(
            amount,
        ))),
        "FJD" => Some(Box::new(crate::RawMoney::<crate::iso::FJD>::from_decimal(
            amount,
        ))),
        "FKP" => Some(Box::new(crate::RawMoney::<crate::iso::FKP>::from_decimal(
            amount,
        ))),
        "GBP" => Some(Box::new(crate::RawMoney::<crate::iso::GBP>::from_decimal(
            amount,
        ))),
        "GEL" => Some(Box::new(crate::RawMoney::<crate::iso::GEL>::from_decimal(
            amount,
        ))),
        "GHS" => Some(Box::new(crate::RawMoney::<crate::iso::GHS>::from_decimal(
            amount,
        ))),
        "GIP" => Some(Box::new(crate::RawMoney::<crate::iso::GIP>::from_decimal(
            amount,
        ))),
        "GMD" => Some(Box::new(crate::RawMoney::<crate::iso::GMD>::from_decimal(
            amount,
        ))),
        "GNF" => Some(Box::new(crate::RawMoney::<crate::iso::GNF>::from_decimal(
            amount,
        ))),
        "GTQ" => Some(Box::new(crate::RawMoney::<crate::iso::GTQ>::from_decimal(
            amount,
        ))),
        "GYD" => Some(Box::new(crate::RawMoney::<crate::iso::GYD>::from_decimal(
            amount,
        ))),
        "HKD" => Some(Box::new(crate::RawMoney::<crate::iso::HKD>::from_decimal(
            amount,
        ))),
        "HNL" => Some(Box::new(crate::RawMoney::<crate::iso::HNL>::from_decimal(
            amount,
        ))),
        "HRK" => Some(Box::new(crate::RawMoney::<crate::iso::HRK>::from_decimal(
            amount,
        ))),
        "HTG" => Some(Box::new(crate::RawMoney::<crate::iso::HTG>::from_decimal(
            amount,
        ))),
        "HUF" => Some(Box::new(crate::RawMoney::<crate::iso::HUF>::from_decimal(
            amount,
        ))),
        "IDR" => Some(Box::new(crate::RawMoney::<crate::iso::IDR>::from_decimal(
            amount,
        ))),
        "ILS" => Some(Box::new(crate::RawMoney::<crate::iso::ILS>::from_decimal(
            amount,
        ))),
        "INR" => Some(Box::new(crate::RawMoney::<crate::iso::INR>::from_decimal(
            amount,
        ))),
        "IQD" => Some(Box::new(crate::RawMoney::<crate::iso::IQD>::from_decimal(
            amount,
        ))),
        "IRR" => Some(Box::new(crate::RawMoney::<crate::iso::IRR>::from_decimal(
            amount,
        ))),
        "ISK" => Some(Box::new(crate::RawMoney::<crate::iso::ISK>::from_decimal(
            amount,
        ))),
        "JMD" => Some(Box::new(crate::RawMoney::<crate::iso::JMD>::from_decimal(
            amount,
        ))),
        "JOD" => Some(Box::new(crate::RawMoney::<crate::iso::JOD>::from_decimal(
            amount,
        ))),
        "JPY" => Some(Box::new(crate::RawMoney::<crate::iso::JPY>::from_decimal(
            amount,
        ))),
        "KES" => Some(Box::new(crate::RawMoney::<crate::iso::KES>::from_decimal(
            amount,
        ))),
        "KGS" => Some(Box::new(crate::RawMoney::<crate::iso::KGS>::from_decimal(
            amount,
        ))),
        "KHR" => Some(Box::new(crate::RawMoney::<crate::iso::KHR>::from_decimal(
            amount,
        ))),
        "KMF" => Some(Box::new(crate::RawMoney::<crate::iso::KMF>::from_decimal(
            amount,
        ))),
        "KPW" => Some(Box::new(crate::RawMoney::<crate::iso::KPW>::from_decimal(
            amount,
        ))),
        "KRW" => Some(Box::new(crate::RawMoney::<crate::iso::KRW>::from_decimal(
            amount,
        ))),
        "KWD" => Some(Box::new(crate::RawMoney::<crate::iso::KWD>::from_decimal(
            amount,
        ))),
        "KYD" => Some(Box::new(crate::RawMoney::<crate::iso::KYD>::from_decimal(
            amount,
        ))),
        "KZT" => Some(Box::new(crate::RawMoney::<crate::iso::KZT>::from_decimal(
            amount,
        ))),
        "LAK" => Some(Box::new(crate::RawMoney::<crate::iso::LAK>::from_decimal(
            amount,
        ))),
        "LBP" => Some(Box::new(crate::RawMoney::<crate::iso::LBP>::from_decimal(
            amount,
        ))),
        "LKR" => Some(Box::new(crate::RawMoney::<crate::iso::LKR>::from_decimal(
            amount,
        ))),
        "LRD" => Some(Box::new(crate::RawMoney::<crate::iso::LRD>::from_decimal(
            amount,
        ))),
        "LSL" => Some(Box::new(crate::RawMoney::<crate::iso::LSL>::from_decimal(
            amount,
        ))),
        "LYD" => Some(Box::new(crate::RawMoney::<crate::iso::LYD>::from_decimal(
            amount,
        ))),
        "MAD" => Some(Box::new(crate::RawMoney::<crate::iso::MAD>::from_decimal(
            amount,
        ))),
        "MDL" => Some(Box::new(crate::RawMoney::<crate::iso::MDL>::from_decimal(
            amount,
        ))),
        "MGA" => Some(Box::new(crate::RawMoney::<crate::iso::MGA>::from_decimal(
            amount,
        ))),
        "MKD" => Some(Box::new(crate::RawMoney::<crate::iso::MKD>::from_decimal(
            amount,
        ))),
        "MMK" => Some(Box::new(crate::RawMoney::<crate::iso::MMK>::from_decimal(
            amount,
        ))),
        "MNT" => Some(Box::new(crate::RawMoney::<crate::iso::MNT>::from_decimal(
            amount,
        ))),
        "MOP" => Some(Box::new(crate::RawMoney::<crate::iso::MOP>::from_decimal(
            amount,
        ))),
        "MRU" => Some(Box::new(crate::RawMoney::<crate::iso::MRU>::from_decimal(
            amount,
        ))),
        "MUR" => Some(Box::new(crate::RawMoney::<crate::iso::MUR>::from_decimal(
            amount,
        ))),
        "MVR" => Some(Box::new(crate::RawMoney::<crate::iso::MVR>::from_decimal(
            amount,
        ))),
        "MWK" => Some(Box::new(crate::RawMoney::<crate::iso::MWK>::from_decimal(
            amount,
        ))),
        "MXN" => Some(Box::new(crate::RawMoney::<crate::iso::MXN>::from_decimal(
            amount,
        ))),
        "MXV" => Some(Box::new(crate::RawMoney::<crate::iso::MXV>::from_decimal(
            amount,
        ))),
        "MYR" => Some(Box::new(crate::RawMoney::<crate::iso::MYR>::from_decimal(
            amount,
        ))),
        "MZN" => Some(Box::new(crate::RawMoney::<crate::iso::MZN>::from_decimal(
            amount,
        ))),
        "NAD" => Some(Box::new(crate::RawMoney::<crate::iso::NAD>::from_decimal(
            amount,
        ))),
        "NGN" => Some(Box::new(crate::RawMoney::<crate::iso::NGN>::from_decimal(
            amount,
        ))),
        "NIO" => Some(Box::new(crate::RawMoney::<crate::iso::NIO>::from_decimal(
            amount,
        ))),
        "NOK" => Some(Box::new(crate::RawMoney::<crate::iso::NOK>::from_decimal(
            amount,
        ))),
        "NPR" => Some(Box::new(crate::RawMoney::<crate::iso::NPR>::from_decimal(
            amount,
        ))),
        "NZD" => Some(Box::new(crate::RawMoney::<crate::iso::NZD>::from_decimal(
            amount,
        ))),
        "OMR" => Some(Box::new(crate::RawMoney::<crate::iso::OMR>::from_decimal(
            amount,
        ))),
        "PAB" => Some(Box::new(crate::RawMoney::<crate::iso::PAB>::from_decimal(
            amount,
        ))),
        "PEN" => Some(Box::new(crate::RawMoney::<crate::iso::PEN>::from_decimal(
            amount,
        ))),
        "PGK" => Some(Box::new(crate::RawMoney::<crate::iso::PGK>::from_decimal(
            amount,
        ))),
        "PHP" => Some(Box::new(crate::RawMoney::<crate::iso::PHP>::from_decimal(
            amount,
        ))),
        "PKR" => Some(Box::new(crate::RawMoney::<crate::iso::PKR>::from_decimal(
            amount,
        ))),
        "PLN" => Some(Box::new(crate::RawMoney::<crate::iso::PLN>::from_decimal(
            amount,
        ))),
        "PYG" => Some(Box::new(crate::RawMoney::<crate::iso::PYG>::from_decimal(
            amount,
        ))),
        "QAR" => Some(Box::new(crate::RawMoney::<crate::iso::QAR>::from_decimal(
            amount,
        ))),
        "RON" => Some(Box::new(crate::RawMoney::<crate::iso::RON>::from_decimal(
            amount,
        ))),
        "RSD" => Some(Box::new(crate::RawMoney::<crate::iso::RSD>::from_decimal(
            amount,
        ))),
        "RUB" => Some(Box::new(crate::RawMoney::<crate::iso::RUB>::from_decimal(
            amount,
        ))),
        "RWF" => Some(Box::new(crate::RawMoney::<crate::iso::RWF>::from_decimal(
            amount,
        ))),
        "SAR" => Some(Box::new(crate::RawMoney::<crate::iso::SAR>::from_decimal(
            amount,
        ))),
        "SBD" => Some(Box::new(crate::RawMoney::<crate::iso::SBD>::from_decimal(
            amount,
        ))),
        "SCR" => Some(Box::new(crate::RawMoney::<crate::iso::SCR>::from_decimal(
            amount,
        ))),
        "SDG" => Some(Box::new(crate::RawMoney::<crate::iso::SDG>::from_decimal(
            amount,
        ))),
        "SEK" => Some(Box::new(crate::RawMoney::<crate::iso::SEK>::from_decimal(
            amount,
        ))),
        "SGD" => Some(Box::new(crate::RawMoney::<crate::iso::SGD>::from_decimal(
            amount,
        ))),
        "SHP" => Some(Box::new(crate::RawMoney::<crate::iso::SHP>::from_decimal(
            amount,
        ))),
        "SLE" => Some(Box::new(crate::RawMoney::<crate::iso::SLE>::from_decimal(
            amount,
        ))),
        "SLL" => Some(Box::new(crate::RawMoney::<crate::iso::SLL>::from_decimal(
            amount,
        ))),
        "SOS" => Some(Box::new(crate::RawMoney::<crate::iso::SOS>::from_decimal(
            amount,
        ))),
        "SRD" => Some(Box::new(crate::RawMoney::<crate::iso::SRD>::from_decimal(
            amount,
        ))),
        "SSP" => Some(Box::new(crate::RawMoney::<crate::iso::SSP>::from_decimal(
            amount,
        ))),
        "STN" => Some(Box::new(crate::RawMoney::<crate::iso::STN>::from_decimal(
            amount,
        ))),
        "SVC" => Some(Box::new(crate::RawMoney::<crate::iso::SVC>::from_decimal(
            amount,
        ))),
        "SYP" => Some(Box::new(crate::RawMoney::<crate::iso::SYP>::from_decimal(
            amount,
        ))),
        "SZL" => Some(Box::new(crate::RawMoney::<crate::iso::SZL>::from_decimal(
            amount,
        ))),
        "THB" => Some(Box::new(crate::RawMoney::<crate::iso::THB>::from_decimal(
            amount,
        ))),
        "TJS" => Some(Box::new(crate::RawMoney::<crate::iso::TJS>::from_decimal(
            amount,
        ))),
        "TMT" => Some(Box::new(crate::RawMoney::<crate::iso::TMT>::from_decimal(
            amount,
        ))),
        "TND" => Some(Box::new(crate::RawMoney::<crate::iso::TND>::from_decimal(
            amount,
        ))),
        "TOP" => Some(Box::new(crate::RawMoney::<crate::iso::TOP>::from_decimal(
            amount,
        ))),
        "TRY" => Some(Box::new(crate::RawMoney::<crate::iso::TRY>::from_decimal(
            amount,
        ))),
        "TTD" => Some(Box::new(crate::RawMoney::<crate::iso::TTD>::from_decimal(
            amount,
        ))),
        "TWD" => Some(Box::new(crate::RawMoney::<crate::iso::TWD>::from_decimal(
            amount,
        ))),
        "TZS" => Some(Box::new(crate::RawMoney::<crate::iso::TZS>::from_decimal(
            amount,
        ))),
        "UAH" => Some(Box::new(crate::RawMoney::<crate::iso::UAH>::from_decimal(
            amount,
        ))),
        "UGX" => Some(Box::new(crate::RawMoney::<crate::iso::UGX>::from_decimal(
            amount,
        ))),
        "USD" => Some(Box::new(crate::RawMoney::<crate::iso::USD>::from_decimal(
            amount,
        ))),
        "USN" => Some(Box::new(crate::RawMoney::<crate::iso::USN>::from_decimal(
            amount,
        ))),
        "UYI" => Some(Box::new(crate::RawMoney::<crate::iso::UYI>::from_decimal(
            amount,
        ))),
        "UYU" => Some(Box::new(crate::RawMoney::<crate::iso::UYU>::from_decimal(
            amount,
        ))),
        "UYW" => Some(Box::new(crate::RawMoney::<crate::iso::UYW>::from_decimal(
            amount,
        ))),
        "UZS" => Some(Box::new(crate::RawMoney::<crate::iso::UZS>::from_decimal(
            amount,
        ))),
        "VED" => Some(Box::new(crate::RawMoney::<crate::iso::VED>::from_decimal(
            amount,
        ))),
        "VES" => Some(Box::new(crate::RawMoney::<crate::iso::VES>::from_decimal(
            amount,
        ))),
        "VND" => Some(Box::new(crate::RawMoney::<crate::iso::VND>::from_decimal(
            amount,
        ))),
        "VUV" => Some(Box::new(crate::RawMoney::<crate::iso::VUV>::from_decimal(
            amount,
        ))),
        "WST" => Some(Box::new(crate::RawMoney::<crate::iso::WST>::from_decimal(
            amount,
        ))),
        "XAF" => Some(Box::new(crate::RawMoney::<crate::iso::XAF>::from_decimal(
            amount,
        ))),
        "XAG" => Some(Box::new(crate::RawMoney::<crate::iso::XAG>::from_decimal(
            amount,
        ))),
        "XAU" => Some(Box::new(crate::RawMoney::<crate::iso::XAU>::from_decimal(
            amount,
        ))),
        "XBA" => Some(Box::new(crate::RawMoney::<crate::iso::XBA>::from_decimal(
            amount,
        ))),
        "XBB" => Some(Box::new(crate::RawMoney::<crate::iso::XBB>::from_decimal(
            amount,
        ))),
        "XBC" => Some(Box::new(crate::RawMoney::<crate::iso::XBC>::from_decimal(
            amount,
        ))),
        "XBD" => Some(Box::new(crate::RawMoney::<crate::iso::XBD>::from_decimal(
            amount,
        ))),
        "XCD" => Some(Box::new(crate::RawMoney::<crate::iso::XCD>::from_decimal(
            amount,
        ))),
        "XCG" => Some(Box::new(crate::RawMoney::<crate::iso::XCG>::from_decimal(
            amount,
        ))),
        "XDR" => Some(Box::new(crate::RawMoney::<crate::iso::XDR>::from_decimal(
            amount,
        ))),
        "XOF" => Some(Box::new(crate::RawMoney::<crate::iso::XOF>::from_decimal(
            amount,
        ))),
        "XPD" => Some(Box::new(crate::RawMoney::<crate::iso::XPD>::from_decimal(
            amount,
        ))),
        "XPF" => Some(Box::new(crate::RawMoney::<crate::iso::XPF>::from_decimal(
            amount,
        ))),
        "XPT" => Some(Box::new(crate::RawMoney::<crate::iso::XPT>::from_decimal(
            amount,
        ))),
        "XSU" => Some(Box::new(crate::RawMoney::<crate::iso::XSU>::from_decimal(
            amount,
        ))),
        "XTS" => Some(Box::new(crate::RawMoney::<crate::iso::XTS>::from_decimal(
            amount,
        ))),
        "XUA" => Some(Box::new(crate::RawMoney::<crate::iso::XUA>::from_decimal(
            amount,
        ))),
        "XXX" => Some(Box::new(crate::RawMoney::<crate::iso::XXX>::from_decimal(
            amount,
        ))),
        "YER" => Some(Box::new(crate::RawMoney::<crate::iso::YER>::from_decimal(
            amount,
        ))),
        "ZAR" => Some(Box::new(crate::RawMoney::<crate::iso::ZAR>::from_decimal(
            amount,
        ))),
        "ZMW" => Some(Box::new(crate::RawMoney::<crate::iso::ZMW>::from_decimal(
            amount,
        ))),
        "ZWG" => Some(Box::new(crate::RawMoney::<crate::iso::ZWG>::from_decimal(
            amount,
        ))),
        "ZWL" => Some(Box::new(crate::RawMoney::<crate::iso::ZWL>::from_decimal(
            amount,
        ))),
        _ => None,
    }
}
