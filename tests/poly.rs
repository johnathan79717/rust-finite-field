extern crate finite_field;
use finite_field::poly::Poly;

#[test]
fn add() {
    let a = Poly::new(vec![1, 2]);
    let b = Poly::new(vec![2, 3, 4, 0]);
    assert_eq!(a + b, Poly::new(vec![3, 5, 4]));
    let b = Poly::new(vec![1, 2]);
    let a = Poly::new(vec![2, 3, 4, 0]);
    assert_eq!(a + b, Poly::new(vec![3, 5, 4]));
}

#[test]
fn sub() {
    let a = Poly::new(vec![1, 2]);
    let b = Poly::new(vec![2, 3, 4, 0]);
    assert_eq!(a - b, Poly::new(vec![-1, -1, -4]));
    let b = Poly::new(vec![1, 2]);
    let a = Poly::new(vec![2, 3, 4, 0]);
    assert_eq!(a - b, Poly::new(vec![1, 1, 4]));
}

#[test]
fn mul() {
    let a = Poly::new(vec![1, 2]);
    let b = Poly::new(vec![2, 3, 4, 0]);
    assert_eq!(a * b, Poly::new(vec![2, 7, 10, 8]));
}
