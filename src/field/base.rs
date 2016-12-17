use std::marker::PhantomData;
use std::ops::*;
use integer_as_type::IntegerAsType;
use std::clone::Clone;
use std::fmt;

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

impl<P> Clone for Fp<P> where P: IntegerAsType {
    fn clone(&self) -> Self { Fp::<P>::new(self.rep) }
}

impl<P> PartialEq for Fp<P> {
    fn eq(&self, rhs: &Self) -> bool {
        self.rep == rhs.rep
    }
}

impl<P> fmt::Debug for Fp<P> where P: IntegerAsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fp<{}>({})", P::value(), self.rep)
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

impl<P> BitXorAssign<i64> for Fp<P> where P: IntegerAsType {
    fn bitxor_assign(&mut self, pow: i64) {
        if pow == 0 {
            *self = Fp::<P>::new(1);
        } else if pow % 2 == 1 {
            // XXX Not sure if clone() is necessary
            *self *= self.clone() ^ (pow - 1);
        } else {
            // XXX Not sure if clone() is necessary
            *self *= self.clone();
            *self ^= pow / 2;
        }
    }
}

impl<P> BitXor<i64> for Fp<P> where P: IntegerAsType {
    type Output = Self;
    fn bitxor(mut self, pow: i64) -> Self {
        self ^= pow;
        self
    }
}

impl<P> DivAssign for Fp<P> where P: IntegerAsType {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.rep == 0 {
            panic!("Fp dividing zero");
        }
        *self *= rhs ^ (P::value() - 2);
    }
}

impl<P> Div for Fp<P> where P: IntegerAsType {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        self /= rhs;
        self
    }
}
