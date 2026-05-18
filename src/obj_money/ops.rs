use super::DynMoney;
use super::ObjMoney;

impl ::std::ops::Neg for DynMoney {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.set_amount(-self.amount())
    }
}

#[inline]
fn negate_obj(m: &dyn ObjMoney) -> Box<dyn ObjMoney> {
    let dyn_m: DynMoney = m.try_into().unwrap_or_else(|_| unreachable!());
    Box::new(dyn_m.set_amount(-m.amount()))
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
