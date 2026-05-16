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
        (*self).neg()
    }
}
