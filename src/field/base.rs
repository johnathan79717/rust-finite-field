use std::marker::PhantomData;
use std::ops::*;
use integer_as_type::*;
use std::clone::Clone;
use std::fmt;

pub struct Fp<N> {
    rep: i64,
    phantom: PhantomData<N>,
}

impl<N> Fp<N> where N: IntegerAsType {
    pub fn new(mut rep: i64) -> Fp<N> {
        let p = N::value();
        rep %= p;
        if rep < 0 { rep += p; }
        Fp::<N> {
            rep: rep % N::value(),
            phantom: PhantomData,
        }
    }
}

impl<N> Clone for Fp<N> where N: IntegerAsType {
    fn clone(&self) -> Self { Fp::<N>::new(self.rep) }
}

impl<N> PartialEq for Fp<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.rep == rhs.rep
    }
}

impl<N> fmt::Debug for Fp<N> where N: IntegerAsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fp<{}>({})", N::value(), self.rep)
    }
}

impl<N> AddAssign for Fp<N> where N: IntegerAsType {
    fn add_assign(&mut self, rhs: Self) {
        self.rep += rhs.rep;
        let p = N::value();
        if self.rep >= p {
            self.rep -= p;
        }
    }
}

impl<N> Add for Fp<N> where N: IntegerAsType {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<N> SubAssign for Fp<N> where N: IntegerAsType {
    fn sub_assign(&mut self, rhs: Self) {
        self.rep -= rhs.rep;
        if self.rep < 0 {
            self.rep += N::value();
        }
    }
}

impl<N> Sub for Fp<N> where N: IntegerAsType {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<N> MulAssign for Fp<N> where N: IntegerAsType {
    fn mul_assign(&mut self, rhs: Self) {
        self.rep *= rhs.rep;
        self.rep %= N::value();
    }
}

impl<N> Mul for Fp<N> where N: IntegerAsType {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<N> Neg for Fp<N> where N: IntegerAsType {
    type Output = Self;
    fn neg(mut self) -> Self {
        if self.rep != 0 {
            self.rep = N::value() - self.rep;
        }
        self
    }
}

impl<N> BitXorAssign<i64> for Fp<N> where N: IntegerAsType {
    fn bitxor_assign(&mut self, pow: i64) {
        if pow == 0 {
            *self = Fp::<N>::new(1);
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

impl<N> BitXor<i64> for Fp<N> where N: IntegerAsType {
    type Output = Self;
    fn bitxor(mut self, pow: i64) -> Self {
        self ^= pow;
        self
    }
}

impl<N> DivAssign for Fp<N> where N: IntegerAsType {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.rep == 0 {
            panic!("Fp dividing zero");
        }
        *self *= rhs ^ (N::value() - 2);
    }
}

impl<N> Div for Fp<N> where N: IntegerAsType {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        self /= rhs;
        self
    }
}

pub type F2 = Fp<T2>;
pub type F3 = Fp<T3>;
pub type F5 = Fp<T5>;
