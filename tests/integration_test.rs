extern crate finite_field;
use finite_field::field::base::Fp;
use finite_field::integer_as_type::T3;

fn zero() -> Fp<T3> { Fp::<T3>::new(0) }
fn one() -> Fp<T3> { Fp::<T3>::new(1) }
fn two() -> Fp<T3> { Fp::<T3>::new(2) }

#[test]
fn operators() {
    assert_eq!(zero(), one() + two());
    assert_eq!(zero() - one(), two());
    assert_eq!(two() * two(), one());
    assert_eq!(-one(), two());

    let mut a = one();
    a += two();
    assert_eq!(a, zero());
    a -= one();
    assert_eq!(a, two());
    a *= two();
    assert_eq!(a, one());
}
