use std::marker::PhantomData;
use std::ops::*;
use integer_as_type::IntegerAsType;

#[derive(Debug, PartialEq)]
pub struct Fp<P> {
    rep: i64,
    phantom: PhantomData<P>,
}

impl<P> Fp<P> where P: IntegerAsType {
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

impl<P> AddAssign for Fp<P> where P: IntegerAsType {
    fn add_assign(&mut self, rhs: Self) {
        self.rep += rhs.rep;
        let p = P::value();
        if self.rep >= p {
            self.rep -= p;
        }
    }
}

impl<P> Add for Fp<P> where P: IntegerAsType {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<P> SubAssign for Fp<P> where P: IntegerAsType {
    fn sub_assign(&mut self, rhs: Self) {
        self.rep -= rhs.rep;
        if self.rep < 0 {
            self.rep += P::value();
        }
    }
}

impl<P> Sub for Fp<P> where P: IntegerAsType {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<P> MulAssign for Fp<P> where P: IntegerAsType {
    fn mul_assign(&mut self, rhs: Self) {
        self.rep *= rhs.rep;
        self.rep %= P::value();
    }
}

impl<P> Mul for Fp<P> where P: IntegerAsType {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<P> Neg for Fp<P> where P: IntegerAsType {
    type Output = Self;
    fn neg(mut self) -> Self {
        if self.rep != 0 {
            self.rep = P::value() - self.rep;
        }
        self
    }
}

