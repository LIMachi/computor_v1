use crate::complex::Complex;

pub mod utils;
pub mod r#impl;
pub mod parser;

#[derive(Debug)]
pub struct PolynomialEquation(Vec<Complex>);