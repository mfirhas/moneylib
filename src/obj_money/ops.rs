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

impl ::std::ops::Neg for Box<dyn ObjMoney> {
    type Output = Box<dyn ObjMoney>;

    fn neg(self) -> Self::Output {
        let currency = Context::get_currency(self.code())
            .expect("currency from existing ObjMoney must be registered");
        Box::new(DynMoney {
            amount: -self.amount(),
            currency,
        })
    }
}

impl ::std::ops::Neg for &dyn ObjMoney {
    type Output = Box<dyn ObjMoney>;

    fn neg(self) -> Self::Output {
        let currency = Context::get_currency(self.code())
            .expect("currency from existing ObjMoney must be registered");
        Box::new(DynMoney {
            amount: -self.amount(),
            currency,
        })
    }
}
