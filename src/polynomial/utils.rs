use std::fmt::{Display, Formatter};
use super::PolynomialEquation;

impl Display for PolynomialEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            f.write_str("0")?;
        }
        for (i, val) in self.0.iter().enumerate() {
            if val == &0. { continue }
            if i == 0 {
                if val < &0. {
                    f.write_str("- ")?;
                }
            } else {
                f.write_fmt(format_args!(" {} ", if val < &0. { '-' } else { '+' }))?;
            }
            if i > 0 {
                if i == 1 {
                    f.write_fmt(format_args!("{} * X", val.abs()))?;
                } else {
                    f.write_fmt(format_args!("{} * X^{i}", val.abs()))?;
                }
            } else {
                std::fmt::Display::fmt(&val.abs(), f)?;
            }
        }
        f.write_str(" = 0")
    }
}