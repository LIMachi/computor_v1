use std::fmt::{Debug, Display, Formatter};
use crate::BabylonianSqrt;
use super::Complex;

impl Default for Complex {
    fn default() -> Self {
        Self {
            r: 0.0,
            i: 0.0,
        }
    }
}

impl Complex {
    pub fn new(r: f32, i: f32) -> Self {
        Self { r, i }
    }

    pub fn from_i(i: f32) -> Self {
        Self {
            r: 0.,
            i
        }
    }

    pub fn abs(&self) -> Self {
        if self.r == 0. {
            Self {
                r: if self.i < 0. { -self.i } else { self.i },
                i: 0.
            }
        } else if self.i == 0. {
            Self {
                r: if self.r < 0. { -self.r } else { self.r },
                i: 0.
            }
        } else {
            Self {
                r: (self.r * self.r + self.i * self.i).babylonian_sqrt(),
                i: 0.
            }
        }
    }

    pub fn eq_epsilon(&self, rhs: Self) -> bool {
        self.r + f32::EPSILON >= rhs.r && self.r - f32::EPSILON <= rhs.r && self.i + f32::EPSILON >= rhs.i && self.i - f32::EPSILON <= rhs.i
    }
}

impl BabylonianSqrt for Complex {
    fn babylonian_sqrt(self) -> Self {
        if self.i == 0. {
            if self.r < 0. {
                Self {
                    r: 0.,
                    i: (-self.r).babylonian_sqrt()
                }
            } else {
                Self {
                    r: self.r.babylonian_sqrt(),
                    i: 0.
                }
            }
        } else {
            let a = self.abs().r;
            let r = ((a + self.r) / 2.).babylonian_sqrt();
            let i = ((a - self.r) / 2.).babylonian_sqrt() * self.i / a;
            Self { r, i }
        }
    }
}

#[test]
fn test_sqrt() {
    dbg!(Complex::from_i(1.).babylonian_sqrt()); //should be around: sqrt(0.5) + i * sqrt(0.5) or 0.707106781 + 0.707106781 i
}

impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{r: {}, i: {}}}", self.r, self.i))
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.i == 0. {
            std::fmt::Display::fmt(&self.r, f)
        } else if self.r == 0. {
            if self.i.abs() == 1. {
                f.write_fmt(format_args!("{}i", if self.i < 0. { "-" } else { "" }))
            } else {
                f.write_fmt(format_args!("{}i", self.i))
            }
        } else {
            if self.i.abs() == 1. {
                f.write_fmt(format_args!("{}{}i", self.r, if self.i < 0. { "-" } else { "+" }))
            } else {
                f.write_fmt(format_args!("{}{}{}i", self.r, if self.i >= 0. { "+" } else { "" }, self.i))
            }
        }
    }
}

impl Clone for Complex {
    fn clone(&self) -> Self {
        Self {
            r: self.r,
            i: self.i
        }
    }
}

impl Copy for Complex {}