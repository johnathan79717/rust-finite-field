use std::marker::PhantomData;
use std::ops::*;

pub trait IntegerAsType {
    fn value() -> i64;
}

#[derive(Debug, PartialEq)]
pub struct Fp<P> {
    rep: i64,
    phantom: PhantomData<P>,
}

impl<P: IntegerAsType> Fp<P> {
    pub fn new(mut rep: i64) -> Fp<P> {
        let p = P::value();
        rep %= p;
        if rep < 0 { rep += p; }
        Fp::<P> {
            rep: rep % P::value(),
            phantom: PhantomData,
        }
    }
}

impl<P: IntegerAsType> AddAssign for Fp<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.rep += rhs.rep;
        let p = P::value();
        if self.rep >= p {
            self.rep -= p;
        }
    }
}

impl<P: IntegerAsType> Add for Fp<P> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<P: IntegerAsType> SubAssign for Fp<P> {
    fn sub_assign(&mut self, rhs: Self) {
        self.rep -= rhs.rep;
        if self.rep < 0 {
            self.rep += P::value();
        }
    }
}

impl<P: IntegerAsType> Sub for Fp<P> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<P: IntegerAsType> MulAssign for Fp<P> {
    fn mul_assign(&mut self, rhs: Self) {
        self.rep *= rhs.rep;
        self.rep %= P::value();
    }
}

impl<P: IntegerAsType> Mul for Fp<P> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<P: IntegerAsType> Neg for Fp<P> {
    type Output = Self;
    fn neg(mut self) -> Self {
        if self.rep != 0 {
            self.rep = P::value() - self.rep;
        }
        self
    }
}

//impl<P: IntegerAsType> DivAssign for Fp<P> {
    //fn div_assign(&mut self, rhs: Self) {

    //}
//}

#[derive(Debug, PartialEq)]
pub struct T2;
impl IntegerAsType for T2 {
    fn value() -> i64 { 2 }
}

#[derive(Debug, PartialEq)]
pub struct T3;
impl IntegerAsType for T3 {
    fn value() -> i64 { 3 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
