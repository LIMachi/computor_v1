mod polynomial;
mod complex;

use std::env::args;
use parser::prelude::*;
use crate::complex::Complex;
use crate::complex::parser::complex;
use crate::polynomial::parser::poly_eq;
use crate::polynomial::PolynomialEquation;

///since sqrt is not allowed, I had to redo it (subject only allows add, sub, mul and div operations on real numbers, sqrt is not sited as a valid math operation)
///https://blogs.sas.com/content/iml/2016/05/16/babylonian-square-roots.html

pub trait BabylonianSqrt {
    fn babylonian_sqrt(self) -> Self;
}

#[cfg(not(feature = "std-sqrt"))]
impl BabylonianSqrt for f32 {
    fn babylonian_sqrt(self) -> Self {
        if self <= 0. {
            0. //handles negative numbers as 0 (should result in an error or complex number to be exact)
        } else if self == 1. {
            1.
        } else {
            let mut mean = (self + 1.) / 2.;
            for _ in 0..8 { //use 8 iterations by default (since f32 has a maximal exponent of 7 bits, I use one more to be extra cautious)
                let estimate = self / mean;
                mean = (mean + estimate) / 2.;
                let t = mean * mean;
                if t + f32::EPSILON > self && t - f32::EPSILON < self {
                    break; //if we are close enough (in f32 error range), stop the guessing early
                }
            }
            mean
        }
    }
}

#[cfg(feature = "std-sqrt")]
impl BabylonianSqrt for f32 {
    #[inline]
    fn babylonian_sqrt(self) -> Self {
        self.sqrt()
    }
}

fn computor_v1<S: Into<StringReader>>(expr: S) {
    let (expr, test_with): (PolynomialEquation, Option<Complex>) = expr.parse_with(true, (poly_eq(), optional(preceded((white, ',', white), complex))).parser()).unwrap();
    println!("Reduced form: {expr}\nPolynomial degree: {}\n", expr.degree());
    if let Some(test) = test_with {
        println!("value of X provided ({test}), let's test the result:");
        let solution = expr.solve_with(test);
        println!("'{}' = {}", format!("{}", expr).replace("X", format!("{}", test).as_str()).replace(" = 0", ""), solution);
        if solution.eq_epsilon(Complex::default()) { //since I work with approximations of sqrt, I allow myself an error of ~e-7
            println!("Success!");
        } else {
            println!("So close yet so far :(");
        }
    } else {
        println!("Discriminant: {}\n", expr.discriminant());
        expr.print_solutions();
    }
}

fn main() {
    if args().len() != 2 {
        println!("expected a single argument");
        return;
    }
    computor_v1(args().last().unwrap());
}
