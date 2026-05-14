use super::DynMoney;

impl ::std::ops::Neg for DynMoney {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            ..self
        }
    }
}
