/// Lab 1D - Type Confusion
///
/// Implicit assumption:
///   "If I stored an i64, I will only ever read it as an i64."
///
/// Bug:
///   This code stores values of different logical types (i64 and f64)
///   in a `union` without recording which variant is active.
///   It then reads the union using the wrong field, interpreting the same
///   bits as a different type.
///
/// Your tasks:
///   1) Replace the `union`-based storage (`RawNumber`) with the provided `Number` enum.
///   2) Update the vector to store `Number` instead of `RawNumber`.
///   3) Update the print logic to correctly match on the enum variant.
///
/// Note:
///   This is type confusion: the memory exists and is in-bounds,
///   but the program violates its own type interpretation invariant.
///   This may not crash, but it is logically incorrect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[repr(C)]
union RawNumber {
    i: i64,
    f: f64,
}

pub fn run() {
    println!("\n== type confusion: union without a tag ==");

    let values: Vec<Number> = vec![
        Number::Int(42),
        Number::Float(3.5),
        Number::Int(-7),
    ];

    for v in values {
        match v {
            Number::Int(x) => println!("Int  = {}", x),
            Number::Float(x) => println!("Float = {}", x),
        }
    }
}
