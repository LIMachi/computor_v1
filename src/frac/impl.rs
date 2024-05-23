use crate::maths::gcd;
use super::Frac;

impl Frac {
    fn reduce(&mut self) -> &mut Self {
        let g = gcd(self.dividend, self.divisor);
        if g > 1 {
            self.dividend /= g;
            self.divisor /= g;
        }
        self
    }
}