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

pub struct T5;
impl IntegerAsType for T5 {
    fn value() -> i64 { 5 }
}
