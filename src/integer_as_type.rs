pub trait IntegerAsType {
    fn value() -> i64;
}

pub struct T2;
impl IntegerAsType for T2 {
    fn value() -> i64 { 2 }
}

pub struct T3;
impl IntegerAsType for T3 {
    fn value() -> i64 { 3 }
}
