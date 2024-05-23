use std::fmt::{Display, Formatter};
use super::Frac;

impl Display for Frac {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} / {}", self.dividend, self.divisor))
    }
}