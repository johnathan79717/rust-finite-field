//use std::marker::PhantomData;
use std::default::Default;
use std::ops::*;
use std::cmp::max;

pub struct Poly<K> {
    coeff: Vec<K>,
}

impl<K> AddAssign for Poly<K> where K: Default + Clone + AddAssign {
    fn add_assign(&mut self, mut rhs: Self) {
        let n = max(self.coeff.len(), rhs.coeff.len());
        self.coeff.resize(n, K::default());
        rhs.coeff.resize(n, K::default());
        for (s, r) in self.coeff.iter_mut().zip(rhs.coeff) {
            *s += r;
        }
    }
}

impl<K> Add for Poly<K> where Poly<K>: AddAssign {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}
