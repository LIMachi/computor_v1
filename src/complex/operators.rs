use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super::Complex;

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i
    }
}

impl PartialEq<f32> for Complex {
    fn eq(&self, other: &f32) -> bool {
        self.i == 0. && self.r == *other
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add<f32> for Complex {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r + rhs,
            i: self.i
        }
    }
}

impl Add<Complex> for f32 {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        rhs + self
    }
}

impl AddAssign<f32> for Complex {
    fn add_assign(&mut self, rhs: f32) {
        self.r += rhs;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub<f32> for Complex {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r - rhs,
            i: self.i
        }
    }
}

impl Sub<Complex> for f32 {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::from(self) - rhs
    }
}

impl SubAssign<f32> for Complex {
    fn sub_assign(&mut self, rhs: f32) {
        self.r -= rhs;
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            i: self.i * rhs
        }
    }
}

impl Mul<Complex> for f32 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Complex {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let div = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r + self.i * rhs.i) / div,
            i: (self.i * rhs.r - self.r * rhs.i) / div
        }
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Div<f32> for Complex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            i: self.i / rhs
        }
    }
}

impl Div<Complex> for f32 {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        Complex::from(self) / rhs
    }
}

impl DivAssign<f32> for Complex {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}