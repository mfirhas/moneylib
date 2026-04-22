use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{
    BaseMoney, BaseOps, Currency, Decimal, Money, RawMoney,
    base::{Amount, DecimalNumber},
    macros::dec,
};

// ========================= Exchange =========================

/// Trait for currency exchange.
/// This does exchange from `From` into `To`.
///
/// This trait has blanket implementation for M where M implements `BaseMoney<C>` + `BaseOps<C>` + `Convert<C>`
/// with method `convert` that does the conversion.
pub trait Exchange<From: Currency> {
    /// Target conversion.
    type Target<T: Currency>
    where
        Self: Convert<T>;

    /// Method to do conversion from `Self<From>` into `Target<To>`.
    ///
    /// If `From` == `To`, immediately return `Target<To>` with Self's amount.
    ///
    /// # Arguments
    /// - To: Currency = Type parameter as the target of currency conversion.
    /// - rate: Rate<From, To> = exchange rate of From/To accepting value from these types:
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
    fn convert<To: Currency>(&self, rate: impl Rate<From, To>) -> Option<Self::Target<To>>
    where
        Self: Convert<To>;
}

impl<M, From> Exchange<From> for M
where
    M: BaseMoney<From> + BaseOps<From> + Convert<From>,
    From: Currency,
{
    type Target<T: Currency>
        = <M as Convert<T>>::Output
    where
        M: Convert<T>;

    fn convert<To: Currency>(&self, rate: impl Rate<From, To>) -> Option<Self::Target<To>>
    where
        M: Convert<To>,
    {
        match From::CODE == To::CODE {
            false => {
                <M as Convert<To>>::Output::new(self.checked_mul(rate.get_rate()?)?.amount()).ok()
            }
            true => <M as Convert<To>>::Output::new(self.amount()).ok(),
        }
    }
}

/// Trait to define target conversion type which implements BaseMoney<T> where T is target currency.
pub trait Convert<T: Currency> {
    type Output: BaseMoney<T>;
}

impl<C: Currency, T: Currency> Convert<T> for Money<C> {
    type Output = Money<T>;
}

impl<C: Currency, T: Currency> Convert<T> for RawMoney<C> {
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
pub trait Rate<From: Currency, To: Currency>: Amount<To> {
    /// Get T's rate relative to C.
    fn get_rate(&self) -> Option<Decimal> {
        self.get_decimal()
    }
}

impl<From: Currency, To: Currency> Rate<From, To> for Money<To> {}

impl<From: Currency, To: Currency> Rate<From, To> for RawMoney<To> {}

impl<From: Currency, To: Currency> Rate<From, To> for Decimal {}

impl<From: Currency, To: Currency> Rate<From, To> for f64 {}

impl<From: Currency, To: Currency> Rate<From, To> for i32 {}

impl<From: Currency, To: Currency> Rate<From, To> for i64 {}

impl<From: Currency, To: Currency> Rate<From, To> for i128 {}

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
#[derive(Clone)]
pub struct ExchangeRates<'a, Base: Currency> {
    rates: HashMap<&'a str, Decimal>,
    _base: PhantomData<Base>,
}

impl<'a, Base: Currency> ExchangeRates<'a, Base> {
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
            rates: HashMap::from([(Base::CODE, dec!(1))]),
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
        Base::CODE
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
        if code != Base::CODE {
            return self.rates.insert(code, rate.get_decimal()?);
        }
        None
    }

    /// Upsert a rate of a pair.
    ///
    /// If value already exist, it got updated and old value returned.
    ///
    /// If one or both `from_code` and `to_code` not in the rates, nothing happen, None returned.
    ///
    /// If one of the rate not exist, it fills it by calculating existing rate against the base rate.
    ///
    /// If both rates exist, it updates the target/quote currency, indirectly updating Base/to_code.
    ///
    /// The rate is updated relative to base currency.
    ///
    /// # Argument
    /// - from_code: currency's code of source conversion(base currency).
    /// - to_code: currency's code of target conversion(quote currency).
    ///
    /// # Examples
    /// ```rust
    /// use moneylib::{ExchangeRates, Exchange, iso::{USD, IDR, JPY, CNY}, dec, money, Currency};
    ///
    /// let mut rates = ExchangeRates::<USD>::new();
    /// assert_eq!(rates.len(), 1);
    /// assert_eq!(rates.get(USD::CODE).unwrap(), dec!(1));
    ///
    /// rates.set("EUR", dec!(0.8));
    /// rates.set("IDR", dec!(17_000));
    /// rates.set("CAD", dec!(1.2));
    /// assert_eq!(rates.len(), 4);
    ///
    /// rates.set_pair("CNY", "IDR", i128::MAX); // wont set
    /// rates.set_pair("CNY", "IDR", 2500);
    /// assert_eq!(rates.len(), 5);
    /// assert_eq!(rates.get("CNY").unwrap(), dec!(6.8));
    ///
    /// let cny_idr_rate = money!(CNY, 5262.657).convert::<IDR>(2500).unwrap();
    /// let cny_idr_rates = money!(CNY, 5262.657).convert::<IDR>(&rates).unwrap();
    /// assert_eq!(cny_idr_rate, cny_idr_rates);
    ///
    /// rates.set_pair("CNY", "IDR", 3000);
    /// let cny_idr_new_rate = money!(CNY, 34989.123).convert::<IDR>(3000).unwrap();
    /// let cny_idr_new_rates = money!(CNY, 34989.123).convert::<IDR>(&rates).unwrap();
    /// assert_eq!(cny_idr_new_rate, cny_idr_new_rates);
    /// ```
    pub fn set_pair(
        &mut self,
        from_code: &'a str,
        to_code: &'a str,
        rate: impl DecimalNumber,
    ) -> Option<Decimal> {
        match (from_code, to_code) {
            // if setting the pair for Base/Base, do nothing
            (from_base, to_base) if from_base == Base::CODE && to_base == Base::CODE => None,
            (from_base, _) if from_base == Base::CODE => self.set(to_code, rate.get_decimal()?),
            (_, to_base) if to_base == Base::CODE => {
                self.set(from_code, dec!(1).checked_div(rate.get_decimal()?)?)
            }
            (from, to) => match (self.get(from), self.get(to)) {
                (Some(base_from_rate), None) => {
                    let base_to_rate = base_from_rate.checked_mul(rate.get_decimal()?)?;
                    self.set(to, base_to_rate)
                }
                (None, Some(base_to_rate)) => {
                    let base_from_rate = base_to_rate.checked_div(rate.get_decimal()?)?;
                    self.set(from, base_from_rate)
                }
                // update Base/to_code rate
                (Some(base_from_rate), Some(_)) => {
                    let new_base_to_rate = base_from_rate.checked_mul(rate.get_decimal()?)?;
                    self.set(to, new_base_to_rate)
                }
                _ => None,
            },
        }
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

impl<'a, I, Base: Currency> From<I> for ExchangeRates<'a, Base>
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

impl<'a, Base: Currency> Default for ExchangeRates<'a, Base> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Base: Currency, To: Currency> Amount<To> for ExchangeRates<'a, Base> {
    fn get_decimal(&self) -> Option<Decimal> {
        Some(*self.rates.get(To::CODE)?)
    }
}

impl<'a, Base: Currency, To: Currency> Amount<To> for &ExchangeRates<'a, Base> {
    fn get_decimal(&self) -> Option<Decimal> {
        <ExchangeRates<Base> as Amount<To>>::get_decimal(self)
    }
}

impl<'a, Base, From, To> Rate<From, To> for ExchangeRates<'a, Base>
where
    Base: Currency,
    From: Currency,
    To: Currency,
{
    fn get_rate(&self) -> Option<Decimal> {
        self.get_pair(From::CODE, To::CODE)
    }
}

impl<'a, Base, From, To> Rate<From, To> for &ExchangeRates<'a, Base>
where
    Base: Currency,
    From: Currency,
    To: Currency,
{
    fn get_rate(&self) -> Option<Decimal> {
        <ExchangeRates<Base> as Rate<From, To>>::get_rate(self)
    }
}

fn exchange_rates_display<Base: Currency>(rates: &ExchangeRates<Base>) -> String {
    let mut ret = format!("Base: {}", Base::CODE);
    ret.push_str(&format!("\n{}/{} = {}", Base::CODE, Base::CODE, dec!(1)));

    for (k, v) in rates.rates.iter() {
        if *k != Base::CODE {
            ret.push_str(&format!("\n{}/{} = {}", Base::CODE, k, v));
        }
    }

    ret
}

impl<Base: Currency> Display for ExchangeRates<'_, Base> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", exchange_rates_display::<Base>(self))
    }
}

impl<Base: Currency> Debug for ExchangeRates<'_, Base> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", exchange_rates_display::<Base>(self))
    }
}
