use std::marker::PhantomData;
use std::ops::{Add, Sub};

pub trait IntegerAsType {
    fn value() -> i64;
}

#[derive(Debug, PartialEq)]
pub struct Fp<P> {
    rep: i64,
    phantom: PhantomData<P>,
}

impl<P: IntegerAsType> Fp<P> {
    fn new(mut rep: i64) -> Fp<P> {
        let p = P::value();
        rep %= p;
        if rep < 0 { rep += p; }
        Fp::<P> {
            rep: rep % P::value(),
            phantom: PhantomData,
        }
    }
}

impl<P: IntegerAsType> Add for Fp<P> {
    type Output = Fp<P>;
    fn add(self, other: Self) -> Self::Output {
        Fp::<P>::new(self.rep + other.rep)
    }
}

impl<P: IntegerAsType> Sub for Fp<P> {
    type Output = Fp<P>;
    fn sub(self, other: Self) -> Self::Output {
        Fp::<P>::new(self.rep - other.rep)
    }
}

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
    use super::*;

    fn zero() -> Fp<T3> { Fp::<T3>::new(0) }
    fn one() -> Fp<T3> { Fp::<T3>::new(1) }
    fn two() -> Fp<T3> { Fp::<T3>::new(2) }

    #[test]
    fn operators() {
        assert_eq!(zero(), one() + two());
        assert_eq!(zero() - one(), two());
    }
}
