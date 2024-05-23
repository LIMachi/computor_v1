mod maths;
mod polynomial;
mod frac;
mod parse_utils;

use std::env::args;
use crate::polynomial::parser::parse;
// use crate::polynomial::parser::parse;

fn main() {
    if args().len() != 2 {
        println!("expected a single argument");
        return;
    }
    let mut expr = parse(args().last().unwrap()).unwrap();
    dbg!(&expr);
    println!("{}\ndegree: {}", expr, expr.degree());
}
