use super::PolynomialEquation;

impl PolynomialEquation {
    pub fn new(left: Vec<(f32, u32)>, right: Vec<(f32, u32)>) -> Self {
        let mut out = Self(Vec::with_capacity(left.len().max(right.len())));
        for (val, exp) in &left {
            while out.0.len() <= *exp as usize {
                out.0.push(0.);
            }
            out.0[*exp as usize] += val;
        }
        for (val, exp) in &right {
            while out.0.len() <= *exp as usize {
                out.0.push(0.);
            }
            out.0[*exp as usize] -= val;
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

    pub fn discriminant(&self) -> f32 {
        0. //TODO
    }
}