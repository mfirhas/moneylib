use std::{collections::HashMap, marker::PhantomData};

use crate::{
    BaseMoney, BaseOps, Currency, Decimal, Money, RawMoney,
    base::{Amount, DecimalNumber},
    macros::dec,
};

// ========================= Exchange =========================

/// Trait for currency exchange.
/// This does exchange from C into T.
///
/// This trait has blanket implementation for M where M implements `BaseMoney<C>` + `BaseOps<C>` + `Convert<C>`
/// with method `convert` that does the conversion.
pub trait Exchange<C: Currency> {
    /// Target conversion.
    type Target<T: Currency>
    where
        Self: Convert<T>;

    /// Method to do conversion from `Self<C>` into `Target<T>`.
    ///
    /// If C == T, immediately return `Target<T>` with Self's amount.
    ///
    /// # Arguments
    /// - T: Currency = Target conversion currency.
    /// - rate: Rate<C, T> = rate amount accepting these types:
    ///     - `Money<T>` where T is target currency
    ///     - `RawMoney<T>` where T is target currency
    ///     - `Decimal`
    ///     - `f64`
    ///     - `i32`
    ///     - `i64`
    ///     - `i128`
    ///     - `ExchangeRates<'a, C>` where C is base currency of exchange rates
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Money, RawMoney, BaseMoney, BaseOps, Exchange, ExchangeRates, Currency};
    /// use moneylib::iso::{EUR, IDR, IRR, USD, CAD};
    /// use moneylib::macros::dec;
    ///
    /// let money = Money::<USD>::new(123).unwrap();
    /// let ret = money.convert::<EUR>(dec!(0.8));
    /// assert_eq!(ret.unwrap().amount(), dec!(98.4));
    ///
    /// let money = Money::<USD>::new(123).unwrap();
    /// let ret = money.convert::<USD>(2); // rate 2 will be ignored since converting to the same currency.
    /// assert_eq!(ret.unwrap().amount(), dec!(123));
    ///
    /// let money = Money::<USD>::from_decimal(dec!(100));
    /// let ret = money.convert::<EUR>(0.888234);
    /// assert_eq!(ret.unwrap().amount(), dec!(88.82));
    ///
    /// let raw_money = RawMoney::<USD>::from_decimal(dec!(100));
    /// let ret = raw_money.convert::<EUR>(0.8882346);
    /// assert_eq!(ret.unwrap().amount(), dec!(88.82346));
    ///
    /// let money = Money::<USD>::new(123).unwrap();
    /// let mut rates = ExchangeRates::<USD>::new();
    /// rates.set(EUR::CODE, dec!(0.8));
    /// rates.set(IDR::CODE, 17_000);
    /// let ret = money.convert::<EUR>(&rates);
    /// assert_eq!(ret.unwrap().amount(), dec!(98.4));
    /// let ret = money.convert::<IDR>(&rates);
    /// assert_eq!(ret.unwrap().amount(), dec!(2_091_000));
    ///
    /// let money = Money::<EUR>::new(123).unwrap();
    /// let ret = money.convert::<IDR>(rates);
    /// assert_eq!(ret.unwrap().amount(), dec!(2_613_750));
    ///
    /// let rates = ExchangeRates::<USD>::from([
    ///     ("EUR", dec!(0.8)),
    ///     ("IDR", dec!(17_000)),
    ///     ("IRR", dec!(1_321_700)),
    ///     ("USD", dec!(123)), // will be ignored since base already in usd and forced into 1.
    /// ]);
    ///
    /// let money = Money::<USD>::from_decimal(dec!(1000));
    /// assert_eq!(money.convert::<USD>(&rates).unwrap().amount(), dec!(1000));
    /// assert_eq!(money.convert::<EUR>(&rates).unwrap().amount(), dec!(800));
    /// assert_eq!(
    ///     money.convert::<IRR>(&rates).unwrap().amount(),
    ///     dec!(1_321_700_000)
    /// );
    /// assert_eq!(
    ///     money.convert::<IDR>(&rates).unwrap().amount(),
    ///     dec!(17_000_000)
    /// );
    ///
    /// let money = Money::<EUR>::new(12).unwrap();
    /// assert_eq!(
    ///     money.convert::<IDR>(&rates).unwrap().amount(),
    ///     dec!(255_000)
    /// );
    /// let money = Money::<EUR>::new(1230).unwrap();
    /// assert_eq!(
    ///     money.convert::<USD>(&rates).unwrap().amount(),
    ///     dec!(1_537.5)
    /// );
    /// let money = Money::<IDR>::new(15_000_000).unwrap();
    /// assert_eq!(
    ///     money.convert::<IRR>(&rates).unwrap().amount(),
    ///     dec!(1_166_205_882.35)
    /// );
    /// let money = Money::<IRR>::new(5000).unwrap();
    /// assert_eq!(money.convert::<USD>(&rates).unwrap().amount(), dec!(0));
    /// let money = Money::<IRR>::new(10000).unwrap();
    /// assert_eq!(money.convert::<EUR>(&rates).unwrap().amount(), dec!(0.01));
    ///
    /// // CAD is not in the rates, so None returned.
    /// assert!(money.convert::<CAD>(rates).is_none());
    /// ```
    fn convert<T: Currency + Clone>(&self, rate: impl Rate<C, T>) -> Option<Self::Target<T>>
    where
        Self: Convert<T>;
}

impl<M, C> Exchange<C> for M
where
    M: BaseMoney<C> + BaseOps<C> + Convert<C>,
    C: Currency,
{
    type Target<T: Currency>
        = <M as Convert<T>>::Output
    where
        M: Convert<T>;

    fn convert<T: Currency + Clone>(&self, rate: impl Rate<C, T>) -> Option<Self::Target<T>>
    where
        M: Convert<T>,
    {
        match C::CODE == T::CODE {
            false => {
                <M as Convert<T>>::Output::new(self.checked_mul(rate.get_rate()?)?.amount()).ok()
            }
            true => <M as Convert<T>>::Output::new(self.amount()).ok(),
        }
    }
}

/// Trait to define target conversion type which implements BaseMoney<T> where T is target currency.
pub trait Convert<T: Currency> {
    type Output: BaseMoney<T>;
}

impl<C: Currency, T: Currency + Clone> Convert<T> for Money<C> {
    type Output = Money<T>;
}

impl<C: Currency, T: Currency + Clone> Convert<T> for RawMoney<C> {
    type Output = RawMoney<T>;
}

// ========================= Rate =========================

/// Trait to define rate amount for conversion input.
///
/// It accepts:
/// - Money<T> where T is target currency
/// - RawMoney<T> where T is target currency
/// - Decimal
/// - f64
/// - i32
/// - i64
/// - i128
/// - ExchangeRates<'a, C> where C is base currency of exchange rates
///
pub trait Rate<C: Currency, T: Currency + Clone>: Amount<T> {
    /// Get T's rate relative to C.
    fn get_rate(&self) -> Option<Decimal> {
        self.get_decimal()
    }
}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for Money<T> {}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for RawMoney<T> {}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for Decimal {}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for f64 {}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for i32 {}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for i64 {}

impl<C: Currency, T: Currency + Clone> Rate<C, T> for i128 {}

// ========================= ExchangeRates =========================

/// Contains list of rates with a Base currency.
///
/// It can be used as input for conversion with Exchange's trait convert method.
///
/// # Examples
///
/// ```
/// use moneylib::{Currency, Exchange, ExchangeRates, iso::{USD, EUR, IDR, IRR, CAD}, macros::dec};
///
/// let mut rates = ExchangeRates::<USD>::new();
/// rates.set(EUR::CODE, dec!(0.8));
/// rates.set(IDR::CODE, 17000);
/// rates.set(IRR::CODE, 1_321_700i128);
/// rates.set(CAD::CODE, 1.8);
///
/// assert_eq!(rates.get(USD::CODE).unwrap(), dec!(1));
/// assert_eq!(rates.get(EUR::CODE).unwrap(), dec!(0.8));
/// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(17_000));
/// assert_eq!(rates.get(IRR::CODE).unwrap(), dec!(1_321_700));
/// assert_eq!(rates.get(CAD::CODE).unwrap(), dec!(1.8));
///
/// // or you can initiate the rates with types implementing IntoIter<Item = (&'a str, Decimal)>
/// // where it's mapping currency code(&str) to its rate(Decimal).
///
/// let rates = ExchangeRates::<USD>::from([
///     (EUR::CODE, dec!(0.8)),
///     (IDR::CODE, dec!(17000)),
///     (IRR::CODE, dec!(1_321_700)),
///     (CAD::CODE, dec!(1.8)),
/// ]);
///
/// assert_eq!(rates.get(USD::CODE).unwrap(), dec!(1));
/// assert_eq!(rates.get(EUR::CODE).unwrap(), dec!(0.8));
/// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(17_000));
/// assert_eq!(rates.get(IRR::CODE).unwrap(), dec!(1_321_700));
/// assert_eq!(rates.get(CAD::CODE).unwrap(), dec!(1.8));
///
/// ```
#[derive(Debug, Clone)]
pub struct ExchangeRates<'a, Base: Currency> {
    rates: HashMap<&'a str, Decimal>,
    _base: PhantomData<Base>,
}

impl<'a, C: Currency + Clone> ExchangeRates<'a, C> {
    /// Initiate new ExchangeRates with base currency and 1 entry to the base with value 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{ExchangeRates, iso::USD};
    ///
    /// let rates = ExchangeRates::<USD>::new();
    /// assert_eq!(rates.len(), 1); // USD/USD = 1
    /// ```
    pub fn new() -> Self {
        Self {
            rates: HashMap::from([(C::CODE, dec!(1))]),
            _base: PhantomData,
        }
    }

    /// Get base currency of exchange rates.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{ExchangeRates, iso::USD};
    ///
    /// let rates = ExchangeRates::<USD>::new();
    /// assert_eq!(rates.base(), "USD");
    /// ```
    #[inline]
    pub const fn base(&self) -> &'static str {
        C::CODE
    }

    /// Upsert rate into exchange rates.
    ///
    /// If value already exist, it got updated and old value returned.
    ///
    /// # Argument
    /// - code: &str, currency code, e.g. "USD", "EUR", etc.
    /// - rate: DecimalNumber, accepts Decimal, f64, i32, i64, i128.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Currency, ExchangeRates, iso::{USD, IDR, EUR}, macros::dec};
    ///
    /// let mut rates = ExchangeRates::<USD>::new();
    /// let entry = rates.set(IDR::CODE, 16000);
    /// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(16000));
    /// let updated_entry = rates.set(IDR::CODE, dec!(17000.23));
    /// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(17000.23));
    /// let another = rates.set(EUR::CODE, dec!(0.8));
    /// assert_eq!(rates.get(EUR::CODE).unwrap(), dec!(0.8));
    ///
    /// assert!(entry.is_none());
    /// assert_eq!(updated_entry.unwrap(), dec!(16000));
    /// assert!(another.is_none());
    /// ```
    pub fn set(&mut self, code: &'a str, rate: impl DecimalNumber) -> Option<Decimal> {
        self.rates.insert(code, rate.get_decimal()?)
    }

    /// Get a rate for a currency exists in rates by code, e.g "USD", "EUR", etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Currency, ExchangeRates, iso::{USD, IDR, EUR}, macros::dec};
    ///
    /// let mut rates = ExchangeRates::<USD>::new();
    /// rates.set(IDR::CODE, 16000);
    /// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(16000));
    /// rates.set(IDR::CODE, dec!(17000.23));
    /// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(17000.23));
    /// rates.set(EUR::CODE, dec!(0.8));
    /// assert_eq!(rates.get(EUR::CODE).unwrap(), dec!(0.8));
    /// ```
    pub fn get(&self, code: &str) -> Option<Decimal> {
        Some(*self.rates.get(code)?)
    }

    /// Get rate of a pair currencies <`from_code`> to <`to_code`>.
    ///
    /// E.g. from_code: USD, to_code: EUR -> get rate of USD/EUR.
    ///
    /// # Examples
    ///
    /// ```
    /// use moneylib::{Currency, ExchangeRates, iso::{USD, IDR, EUR, CAD}, macros::dec};
    ///
    /// let mut rates = ExchangeRates::<USD>::new();
    /// rates.set(IDR::CODE, 16000);
    /// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(16000));
    /// rates.set(IDR::CODE, dec!(17000.23));
    /// assert_eq!(rates.get(IDR::CODE).unwrap(), dec!(17000.23));
    /// rates.set(EUR::CODE, dec!(0.8));
    /// assert_eq!(rates.get(EUR::CODE).unwrap(), dec!(0.8));
    ///
    /// let usd_idr = rates.get_pair(USD::CODE, IDR::CODE).unwrap();
    /// assert_eq!(usd_idr, dec!(17000.23));
    ///
    /// let eur_usd = rates.get_pair(EUR::CODE, USD::CODE).unwrap();
    /// assert_eq!(eur_usd, dec!(1.25));
    ///
    /// let idr_usd = rates.get_pair(IDR::CODE, USD::CODE).unwrap();
    /// assert_eq!(idr_usd, dec!(0.0000588227335747810470799513));
    ///
    /// let idr_eur = rates.get_pair(IDR::CODE, EUR::CODE).unwrap();
    /// assert_eq!(idr_eur, dec!(0.0000470581868598248376639610));
    ///
    /// let cad_idr = rates.get_pair(CAD::CODE, IDR::CODE);
    /// assert!(cad_idr.is_none()); // CAD is not in the exchange rates `rates`
    /// ```
    pub fn get_pair(&self, from_code: &str, to_code: &str) -> Option<Decimal> {
        dec!(1)
            .checked_div(self.get(from_code)?)?
            .checked_mul(self.get(to_code)?)
    }

    #[allow(clippy::len_without_is_empty)]
    /// Get length of exchange rates list.
    pub fn len(&self) -> usize {
        self.rates.len()
    }
}

impl<'a, I, Base: Currency + Clone> From<I> for ExchangeRates<'a, Base>
where
    I: IntoIterator<Item = (&'a str, Decimal)>,
{
    fn from(value: I) -> Self {
        let mut exchange_rates = Self::new();
        for (k, v) in value {
            if k != Base::CODE {
                exchange_rates.set(k, v);
            }
        }

        exchange_rates
    }
}

impl<'a, C: Currency + Clone> Default for ExchangeRates<'a, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, C: Currency, T: Currency> Amount<T> for ExchangeRates<'a, C> {
    fn get_decimal(&self) -> Option<Decimal> {
        Some(*self.rates.get(T::CODE)?)
    }
}

impl<'a, C: Currency, T: Currency> Amount<T> for &ExchangeRates<'a, C> {
    fn get_decimal(&self) -> Option<Decimal> {
        <ExchangeRates<C> as Amount<T>>::get_decimal(self)
    }
}

impl<'a, Base, C, T> Rate<C, T> for ExchangeRates<'a, Base>
where
    Base: Currency + Clone,
    C: Currency + Clone,
    T: Currency + Clone,
{
    fn get_rate(&self) -> Option<Decimal> {
        self.get_pair(C::CODE, T::CODE)
    }
}

impl<'a, Base, C, T> Rate<C, T> for &ExchangeRates<'a, Base>
where
    Base: Currency + Clone,
    C: Currency + Clone,
    T: Currency + Clone,
{
    fn get_rate(&self) -> Option<Decimal> {
        <ExchangeRates<Base> as Rate<C, T>>::get_rate(self)
    }
}
