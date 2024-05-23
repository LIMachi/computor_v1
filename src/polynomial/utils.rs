use std::fmt::{Display, Formatter};
use super::PolynomialEquation;

impl Display for PolynomialEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        if self.0.is_empty() {
            f.write_str("0")?;
        }
        for (i, val) in self.0.iter().enumerate() {
            if val == &0. { continue }
            if first {
                if val < &0. {
                    f.write_str("- ")?;
                }
                first = false;
            } else {
                f.write_fmt(format_args!(" {} ", if val < &0. { '-' } else { '+' }))?;
            }
            let val = val.abs();
            if i > 0 {
                if i == 1 {
                    if val != 1. {
                        f.write_fmt(format_args!("{} * X", val))?;
                    } else {
                        f.write_str("X")?;
                    }
                } else {
                    if val != 1. {
                        f.write_fmt(format_args!("{} * X^{i}", val))?;
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