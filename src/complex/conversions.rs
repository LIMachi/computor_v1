use super::Complex;

impl From<f32> for Complex {
    fn from(value: f32) -> Self {
        Self {
            r: value,
            i: 0.
        }
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Self {
            r: value as f32,
            i: 0.
        }
    }
}

#[derive(Debug)]
pub enum ComplexError {
    CantCoerceComplexToRealDueToImaginaryPart
}

impl TryFrom<Complex> for f32 {
    type Error = ComplexError;

    fn try_from(value: Complex) -> Result<Self, Self::Error> {
        if value.i != 0. {
            Err(ComplexError::CantCoerceComplexToRealDueToImaginaryPart)
        } else {
            Ok(value.r)
        }
    }
}