use crate::BabylonianSqrt;
use crate::complex::Complex;
use super::PolynomialEquation;

impl PolynomialEquation {
    pub fn new(left: Vec<(f32, u32)>, right: Vec<(f32, u32)>) -> Self {
        let mut out = Self(Vec::with_capacity(left.len().max(right.len())));
        for (val, exp) in &left {
            while out.0.len() <= *exp as usize {
                out.0.push(Complex::default());
            }
            out.0[*exp as usize] += *val;
        }
        for (val, exp) in &right {
            while out.0.len() <= *exp as usize {
                out.0.push(Complex::default());
            }
            out.0[*exp as usize] -= *val;
        }
        while out.0.last().map(|v| *v == 0.).unwrap_or(false) {
            out.0.pop();
        }
        out
    }

    pub fn degree(&self) -> usize {
        match self.0.len() {
            0 => 0,
            v @ _ => v - 1
        }
    }

    pub fn discriminant(&self) -> Complex {
        match self.degree() {
            0 => Complex::default(), //float = 0, all values of X since X is unbound
            1 => Complex::from(1.), //float +/- float * X = 0
            2 => self.0[1] * self.0[1] - 4. * self.0[0] * self.0[2], //float +/- float * X +/- float * X² = 0 -> b² - 4ac
            3 => {
                let a = self.0[0];
                let b = self.0[1];
                let c = self.0[2];
                let d = self.0[3];
                b*b*c*c - 4.*a*c*c*c - 4.*b*b*b*d - 27.*a*a*d*d + 18.*a*b*c*d //b²c² - 4ac³ - 4b³d - 27a²d² + 18abcd
            }
            4 => {
                let a = self.0[0];
                let b = self.0[1];
                let c = self.0[2];
                let d = self.0[3];
                let e = self.0[4];
                256.*a*a*a*e*e*e - 192.*a*a*b*d*e*e - 128.*a*a*c*c*e*e + 144.*a*a*c*d*d*e
                    - 27.*a*a*d*d*d*d + 144.*a*b*b*c*e*e - 6.*a*b*b*d*d*e - 80.*a*b*c*c*d*e
                    + 18.*a*b*c*d*d*d + 16.*a*c*c*c*c*e - 4.*a*c*c*c*d*d - 27.*b*b*b*b*e*e
                    + 18.*b*b*b*c*d*e - 4.*b*b*b*d*d*d - 4.*b*b*c*c*c*e + b*b*c*c*d*d
            }
            d @ _ => {
                println!("discriminant is not available for degree {d} (> 4)!");
                Complex::default()
            }
        }
    }

    pub fn print_solutions(&self) {
        match self.degree() {
            0 => println!("X is not bound (present), so all values of X are a solution"),
            1 => {
                println!("The solution is:");
                if self.0[0] == 0. {
                    println!("0");
                } else {
                    println!("{}", -self.0[0] / self.0[1]);
                }
            }
            2 => {
                let d = self.discriminant();
                if d.r == 0. {
                    println!("Since the discriminant is zero, there is only one solution:");
                    println!("{}", -self.0[1] / (2. * self.0[2]));
                } else {
                    println!("Discriminant is non-null, the two solutions are:");
                    let sqrt = d.babylonian_sqrt();
                    println!("{}", (-self.0[1] - sqrt) / (2. * self.0[2]));
                    println!("{}", (-self.0[1] + sqrt) / (2. * self.0[2]));
                }
            }
            _ => println!("The polynomial degree is strictly greater than 4, I can't solve.")
        }
    }

    pub fn solve_with(&self, x: Complex) -> Complex {
        let mut acc = Complex::default();
        for (e, mul) in self.0.iter().enumerate() {
            let mut exp = if e == 0 { Complex::from(1.) } else { x };
            for _ in 2..=e {
                exp *= x;
            }
            acc += exp * *mul;
        }
        acc
    }
}