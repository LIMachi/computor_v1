mod maths;
mod polynomial;
mod frac;

use std::env::args;
use parser::Parseable;
use crate::polynomial::parser::poly_eq;

fn main() {
    if args().len() != 2 {
        println!("expected a single argument");
        return;
    }
    let expr = args().last().unwrap().parse(true, poly_eq()).unwrap();
    dbg!(&expr);
    println!("{}\ndegree: {}", expr, expr.degree());
}
