/// Creates a [`Money`](crate::Money) instance using a currency type and a decimal amount.
///
/// **Short form (ISO currencies):** pass a bare ISO 4217 currency code — it is resolved from
/// [`crate::iso`] automatically, so no separate `use` import is required.
///
/// **Long form (custom currencies):** pass any path that resolves to a type implementing
/// [`Currency`](crate::Currency). The path is used directly, so the type must be in scope.
///
/// The amount is parsed as a decimal string at initialization time and then wrapped in a [`Money`](crate::Money) value,
/// applying the currency's rounding rules.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, macros::{dec, money}};
///
/// // Short form: no `use moneylib::iso::USD;` needed
/// let m = money!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.24)); // rounded to 2 decimal places for USD
///
/// // Negative amounts
/// let m = money!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.00)); // banker's rounding
/// ```
///
/// ```
/// use moneylib::{BaseMoney, Currency, macros::{dec, money}, iso::USD};
///
/// // Long form: path to a custom currency type (must be in scope)
/// let m = money!(USD, 100.00);
/// assert_eq!(m.amount(), dec!(100.00));
/// ```
#[macro_export]
macro_rules! money {
    // Short form: bare ISO currency identifier, auto-resolved from crate::iso
    ($currency:ident, $($amount:tt)+) => {
        $crate::Money::<$crate::iso::$currency>::from_decimal($crate::dec!($($amount)+))
    };
    // Long form: explicit path for custom currency types (must be in scope)
    ($currency:path, $($amount:tt)+) => {
        $crate::Money::<$currency>::from_decimal($crate::dec!($($amount)+))
    };
}

/// Creates a [`RawMoney`](crate::RawMoney) instance using a currency type and a decimal amount.
///
/// **Short form (ISO currencies):** pass a bare ISO 4217 currency code — it is resolved from
/// [`crate::iso`] automatically, so no separate `use` import is required.
///
/// **Long form (custom currencies):** pass any path that resolves to a type implementing
/// [`Currency`](crate::Currency). The path is used directly, so the type must be in scope.
///
/// The amount is parsed as a decimal string at initialization time without any rounding, preserving
/// the full decimal precision.
///
/// # Examples
///
/// ```
/// use moneylib::{BaseMoney, macros::{dec, raw}};
///
/// // Short form: no `use moneylib::iso::USD;` needed
/// let m = raw!(USD, 40.237);
/// assert_eq!(m.amount(), dec!(40.237));
///
/// // Negative amounts
/// let m = raw!(USD, -10.005);
/// assert_eq!(m.amount(), dec!(-10.005));
/// ```
///
/// ```
/// use moneylib::{BaseMoney, Currency, macros::{dec, raw}, iso::USD};
///
/// // Long form: path to a custom currency type (must be in scope)
/// let m = raw!(USD, 100.123);
/// assert_eq!(m.amount(), dec!(100.123));
/// ```
#[cfg(feature = "raw_money")]
#[macro_export]
macro_rules! raw {
    // Short form: bare ISO currency identifier, auto-resolved from crate::iso
    ($currency:ident, $($amount:tt)+) => {
        $crate::RawMoney::<$crate::iso::$currency>::from_decimal($crate::dec!($($amount)+))
    };
    // Long form: explicit path for custom currency types (must be in scope)
    ($currency:path, $($amount:tt)+) => {
        $crate::RawMoney::<$currency>::from_decimal($crate::dec!($($amount)+))
    };
}

/// Re-export of [`rust_decimal_macros::dec`] with the `reexportable` feature enabled.
///
/// This is an implementation detail used by the `dec!` macro to emit compile-time
/// `Decimal` construction without leaking `::rust_decimal` paths into the caller's crate.
///
/// This is so that users are not required to import rust_decimal or putting Decimal in scope.
#[doc(hidden)]
pub use rust_decimal_macros::dec as __dec_inner;

/// Creates a [`Decimal`](crate::Decimal) value from a numeric literal.
///
/// This is a compile-time checked macro — invalid literals produce a compile error, not a panic.
/// `rust_decimal` does not need to be a direct dependency of the caller's crate.
///
/// From [`rust_decimal_macros`](https://docs.rs/rust_decimal_macros/1.40.0/rust_decimal_macros/macro.dec.html):
///
/// Any Rust number format works, for example:
///
/// ```
/// use moneylib::dec;
///
/// let _ = dec!(1);          // plain positive integer
/// let _ = dec!(-1);         // plain negative integer
/// let _ = dec!(1_999);      // underscore as digit separator (readability)
/// let _ = dec!(- 1_999);    // negative with space before value
///
/// let _ = dec!(0b1);        // binary literal (base 2)
/// let _ = dec!(-0b1_1111);  // negative binary with separator (= -31)
/// let _ = dec!(0o1);        // octal literal (base 8)
/// let _ = dec!(-0o1_777);   // negative octal with separator (= -1023)
/// let _ = dec!(0x1);        // hexadecimal literal (base 16)
/// let _ = dec!(-0x1_Ffff);  // negative hex with separator, mixed case digits (= -131071)
///
/// let _ = dec!(1.);         // decimal point with no fractional digits
/// let _ = dec!(-1.111_009); // negative decimal with underscore separator in fraction
/// let _ = dec!(1e6);        // scientific notation: 1 × 10⁶ = 1_000_000
/// let _ = dec!(-1.2e+6);    // negative scientific notation with explicit '+' exponent
/// let _ = dec!(12e-6);      // scientific notation with negative exponent: 0.000_012
/// let _ = dec!(-1.2e-6);    // negative value with negative exponent: -0.000_001_2
/// ```
///
/// ### Option `radix`
///
/// You can give it integers (not float-like) in any radix from 2 to 36 inclusive, using the letters too:
///
/// ```
/// use moneylib::dec;
///
/// assert_eq!(dec!(100, radix 2),      dec!(4));      // "100" in base 2  = 1×4 = 4
/// assert_eq!(dec!(-1_222, radix 3),   dec!(-53));    // "1222" in base 3 = 27+18+6+2 = 53
/// assert_eq!(dec!(z1, radix 36),      dec!(1261));   // "z1"  in base 36 = 35×36+1 = 1261
/// assert_eq!(dec!(-1_xyz, radix 36),  dec!(-90683)); // "1xyz" in base 36, letters count as digit values 10–35
/// ```
///
/// ### Option `exp`
///
/// This is the same as the `e` 10's exponent in float syntax (except as a Rust expression it doesn't accept
/// a unary `+`.) You need this for other radixes. Currently, it must be between -28 and +28 inclusive:
///
/// ```
/// use moneylib::dec;
///
/// assert_eq!(dec!(10, radix 2, exp 5),        dec!(200_000)); // "10" in base 2 = 2, then ×10⁵ = 200_000
/// assert_eq!(dec!(-1_777, exp -3, radix 8),   dec!(-1.023)); // "1777" in base 8 = 1023, then ×10⁻³ = -1.023
/// ```
///
/// # Examples
///
/// ```
/// use moneylib::macros::dec;
///
/// let d = dec!(2.51);
/// assert_eq!(d.to_string(), "2.51");
/// ```
#[macro_export]
macro_rules! dec {
    ($($amount:tt)+) => {
        {
            use $crate::Decimal;
            $crate::macros::__dec_inner!($($amount)+)
        }
    };
}

pub use crate::money;

#[cfg(feature = "raw_money")]
pub use crate::raw;

pub use crate::dec;
