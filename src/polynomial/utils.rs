use std::fmt::{Display, Formatter};
use crate::complex::Complex;
use super::PolynomialEquation;

impl Display for PolynomialEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        if self.0.is_empty() {
            f.write_str("0")?;
        }
        for (i, val) in self.0.iter().enumerate() {
            if val == &Complex::default() { continue }
            if first {
                if val.r < 0. {
                    f.write_str("-")?;
                }
                first = false;
            } else {
                f.write_fmt(format_args!(" {} ", if val.r < 0. { '-' } else { '+' }))?;
            }
            let val = Complex::new(val.r.abs(), val.i);
            if i > 0 {
                if i == 1 {
                    if val != Complex::from(0.) {
                        if val.i != 0. {
                            f.write_fmt(format_args!("({val}) * X"))?;
                        } else {
                            f.write_fmt(format_args!("{val} * X"))?;
                        }
                    } else {
                        f.write_str("X")?;
                    }
                } else {
                    if val != Complex::from(0.) {
                        if val.i != 0. {
                            f.write_fmt(format_args!("({val}) * X^{i}"))?;
                        } else {
                            f.write_fmt(format_args!("{val} * X^{i}"))?;
                        }
                    } else {
                        f.write_fmt(format_args!("X^{i}"))?;
                    }
                }
            } else {
                std::fmt::Display::fmt(&val.abs(), f)?;
            }
        }
        f.write_str(" = 0")
    }
}