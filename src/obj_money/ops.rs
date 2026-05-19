use super::DynMoney;
use super::ObjMoney;

impl ::std::ops::Neg for DynMoney {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.set_amount(-self.amount())
    }
}
