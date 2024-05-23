use crate::maths::gcd;
use super::{Frac, IrreducibleError};

impl From<i32> for Frac {
    fn from(value: i32) -> Self {
        Self {
            dividend: value,
            divisor: 1,
        }
    }
}

impl TryFrom<f32> for Frac {
    type Error = IrreducibleError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let mut a = value.abs();
        let mut l = 1;
        let mut c = 0;
        while a.fract() > 0. && c < 7 {
            a *= 10.;
            l *= 10;
            c += 1;
        }
        if c >= 1 {
            let mut dividend = a as i32 * if value >= 0. { 1 } else { -1 };
            let mut divisor = value.abs().floor() as i32;
            if divisor == 0 {
                divisor = l;
            } else {
                divisor *= l / 10;
            }
            let g = gcd(dividend, divisor);
            if g > 1 {
                dividend /= g;
                divisor /= g;
            }
            let t = (dividend as f32 / divisor as f32).abs();
            let c = value.abs();
            if t + f32::EPSILON >= c && t - f32::EPSILON <= c {
                Ok(Self{ dividend, divisor })
            } else {
                Err(IrreducibleError::CantConvertToFraction(value))
            }
        } else {
            Ok(Self{ dividend: value as i32, divisor: 1 })
        }
    }
}

impl TryFrom<Frac> for i32 {
    type Error = IrreducibleError;

    fn try_from(value: Frac) -> Result<Self, Self::Error> {
        if value.divisor * (value.dividend / value.divisor) == value.dividend {
            Ok(value.dividend / value.divisor)
        } else {
            Err(IrreducibleError::CantConvertToI32(value.clone()))
        }
    }
}

impl From<Frac> for f32 {
    fn from(value: Frac) -> Self {
        value.dividend as f32 / value.divisor as f32
    }
}