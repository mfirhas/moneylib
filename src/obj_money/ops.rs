use super::Context;
use super::DynMoney;
use super::ObjMoney;

impl ::std::ops::Neg for DynMoney {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            ..self
        }
    }
}

#[inline]
fn negate_obj(m: &dyn ObjMoney) -> Box<dyn ObjMoney> {
    let currency = Context::get_currency(m.code()).unwrap_or_else(|| unreachable!());
    Box::new(DynMoney {
        amount: -m.amount(),
        currency,
    })
}

impl ::std::ops::Neg for Box<dyn ObjMoney> {
    type Output = Box<dyn ObjMoney>;

    fn neg(self) -> Self::Output {
        negate_obj(self.as_ref())
    }
}

impl ::std::ops::Neg for &dyn ObjMoney {
    type Output = Box<dyn ObjMoney>;

    fn neg(self) -> Self::Output {
        negate_obj(self)
    }
}
