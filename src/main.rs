mod parser;

use std::collections::HashMap;
use std::env::args;
use std::fmt::{Debug, Display, Formatter, Write};
use crate::parser::parse;

#[derive(Debug)]
struct PolynomialEquation {
    left: Vec<(f32, u32)>,
    right: Vec<(f32, u32)>
}

impl Display for PolynomialEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn poly(f: &mut Formatter<'_>, vec: &Vec<(f32, u32)>) -> std::fmt::Result {
            for (i, (val, exp)) in vec.iter().enumerate() {
                if i == 0 {
                    if val < &0. {
                        f.write_str("- ")?;
                    }
                } else {
                    f.write_fmt(format_args!(" {} ", if val < &0. { '-' } else { '+' }))?;
                }
                if exp > &0 {
                    if exp == &1 {
                        f.write_fmt(format_args!("{} * X", val.abs()))?;
                    } else {
                        f.write_fmt(format_args!("{} * X^{exp}", val.abs()))?;
                    }
                } else {
                    std::fmt::Display::fmt(&val.abs(), f)?;
                }
            }
            Ok(())
        }

        poly(f, &self.left)?;
        f.write_str(" = ")?;
        poly(f, &self.right)
    }
}

impl PolynomialEquation {
    //move the terms from the right to the left (so that the right is equal to 0) and merge the term with the same exponent (removing them if f32 == 0)
    fn simplify(&mut self) {
        let mut map = HashMap::<u32, f32>::new();
        for (val, exp) in &self.left {
            *map.entry(*exp).or_insert(0.) += *val;
        }
        for (val, exp) in &self.right {
            *map.entry(*exp).or_insert(0.) -= *val;
        }
        self.right = vec![(0., 0)];
        self.left = Vec::new();
        for (exp, val) in &map {
            if val != &0. {
                self.left.push((*val, *exp));
            }
        }
        if self.left.is_empty() {
            self.left.push((0., 0));
        }
        self.left.sort_by(|l, r| { l.1.cmp(&r.1) });
    }

    fn degree(&self) -> u32 {
        let mut d = 0;
        for (_, exp) in &self.left {
            d = d.max(*exp);
        }
        for (_, exp) in &self.right {
            d = d.max(*exp);
        }
        d
    }

    fn discriminant(&self) -> f32 {
        0. //TODO
    }
}

fn main() {
    if args().len() != 2 {
        println!("expected a single argument");
        return;
    }
    let mut expr = parse(args().last().unwrap().as_str()).unwrap();
    expr.simplify();
    println!("{}, degree: {}", expr, expr.degree());
}
