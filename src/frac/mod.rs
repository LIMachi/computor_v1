pub mod conversions;
pub mod r#impl;
pub mod ops;
pub mod utils;

#[derive(Copy, Clone, Debug)]
pub struct Frac {
    dividend: i32,
    divisor: i32
}

#[derive(Debug)]
pub enum IrreducibleError {
    CantConvertToFraction(f32),
    CantConvertToI32(Frac)
}