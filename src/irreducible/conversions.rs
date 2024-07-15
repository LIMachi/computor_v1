use std::rc::Rc;
use crate::irreducible::Irreducible;
use crate::irreducible::utils::gcd;

impl TryFrom<f32> for Irreducible {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

macro_rules! irreducible_from_int {
    ($($t:ident),+) => {
        $(
            impl From<$t> for Irreducible {
                fn from(value: $t) -> Self {
                    Self::Integer(value as i128)
                }
            }
        )+
    };
}

irreducible_from_int!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

impl TryFrom<f64> for Irreducible {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let mut a = value.abs();
        let mut l = 1i128;
        let mut c = 0i128;
        while a.fract() > 0. && c < 7 {
            a *= 10.;
            l *= 10;
            c += 1;
        }
        if c >= 1 {
            let mut dividend = a as i128 * if value >= 0. { 1 } else { -1 };
            let mut divisor = value.abs().floor() as i128;
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
            let t = (dividend as f64 / divisor as f64).abs();
            let c = value.abs();
            if t + f64::EPSILON >= c && t - f64::EPSILON <= c {
                Ok(Self::Division { dividend: Rc::new(Self::from(dividend)), divisor: Rc::new(Self::from(divisor)) })
            } else {
                Err(())
            }
        } else {
            Ok(Self::Division { dividend: Rc::new(Self::from(value as i128)), divisor: Rc::new(Self::from(1i128)) })
        }
    }
}