extern crate finite_field;
use finite_field::field::base::Fp;
use finite_field::integer_as_type::{T5,T3};

fn zero() -> Fp<T3> { Fp::<T3>::new(0) }
fn one() -> Fp<T3> { Fp::<T3>::new(1) }
fn two() -> Fp<T3> { Fp::<T3>::new(2) }

#[test]
fn operators() {
    assert_eq!(zero(), one() + two());
    assert_eq!(zero() - one(), two());
    assert_eq!(two() * two(), one());
    assert_eq!(-one(), two());
    assert_eq!(two() ^ 2, one());
    assert_eq!(one() / two(), two());

    let mut a = one();
    a += two();
    assert_eq!(a, zero());
    a -= one();
    assert_eq!(a, two());
    a *= two();
    assert_eq!(a, one());
    a = two();
    a ^= 2;
    assert_eq!(a, one());
}

#[test]
fn division() {
    fn f(i: i64) -> Fp<T5> { Fp::<T5>::new(i) }
    for a in 1..5 {
        for b in 1..5 {
            let c = a * b % 5;
            assert_eq!(f(c) / f(a), f(b));
            println!("{} / {} == {}", c, a, b);
        }
    }
}
