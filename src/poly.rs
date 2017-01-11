use std::default::Default;
use std::ops::*;
use std::cmp::max;

#[derive(PartialEq, Debug)]
pub struct Poly<K> {
    coeff: Vec<K>,
}

impl<K> Poly<K> where K: PartialEq + Default {
    pub fn new(coeff: Vec<K>) -> Self {
        let mut ret = Poly { coeff: coeff };
        ret.normalize();
        ret
    }
    fn normalize(&mut self) {
        while self.coeff.last() == Some(&K::default()) {
            self.coeff.pop();
        }
    }
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

impl<K> SubAssign for Poly<K> where K: Default + Clone + SubAssign {
    fn sub_assign(&mut self, mut rhs: Self) {
        let n = max(self.coeff.len(), rhs.coeff.len());
        self.coeff.resize(n, K::default());
        rhs.coeff.resize(n, K::default());
        for (s, r) in self.coeff.iter_mut().zip(rhs.coeff) {
            *s -= r;
        }
    }
}

impl<K> Sub for Poly<K> where Poly<K>: SubAssign {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

//type Coeff<'a, 'b, K> = <&'a K as Mul<&'b K>>::Output;

//impl<'a, 'b, K> Mul<&'b Poly<K>> for &'a Poly<K>
    //where &'a K: Mul<&'b K>, Coeff<'a, 'b, K>: PartialEq + Default + Clone + AddAssign {
    //type Output = Poly<Coeff<'a, 'b, K>>;
    //fn mul(self, rhs: &'b Poly<K>) -> Self::Output {
        //if self.coeff.is_empty() || rhs.coeff.is_empty() {
            //Poly::new(vec![])
        //} else {
            //let mut ret = vec![<&'a K as Mul<&'b K>>::Output::default(); self.coeff.len() + rhs.coeff.len() - 1];
            //for (i, s) in self.coeff.iter().enumerate() {
                //for (j, r) in rhs.coeff.iter().enumerate() {
                    //ret[i+j] += s * r;
                //}
            //}
            //Poly::new(ret)
        //}
    //}
//}

macro_rules! poly_mul {
    ($rhs:ty, $output:ty, $mul:expr) => {
        type Output = Poly<$output>;
        fn mul(self, rhs: $rhs) -> Self::Output {
            if self.coeff.is_empty() || rhs.coeff.is_empty() {
                Poly::new(vec![])
            } else {
                let mut ret = vec![default::<$output>(); self.coeff.len() + rhs.coeff.len() - 1];
                for (i, s) in self.coeff.iter().enumerate() {
                    for (j, r) in rhs.coeff.iter().enumerate() {
                        ret[i+j] += $mul;
                    }
                }
                Poly::new(ret)
            }
        }
    }
}

impl<K> Mul for Poly<K> where K: Mul + Copy, <K as Mul>::Output: PartialEq + Default + Clone + AddAssign {
    poly_mul!(Self, <K as Mul>::Output, *s * *r);
    //type Output = Poly<MulOutput<K>>;
    //fn mul(self, rhs: Self) -> Self::Output {
        //if self.coeff.is_empty() || rhs.coeff.is_empty() {
            //Poly::new(vec![])
        //} else {
            //let mut ret = vec![MulOutput::<K>::default(); self.coeff.len() + rhs.coeff.len() - 1];
            //for (i, s) in self.coeff.iter().enumerate() {
                //for (j, r) in rhs.coeff.iter().enumerate() {
                    //ret[i+j] += *s * *r;
                //}
            //}
            //Poly::new(ret)
        //}
    //}
}
