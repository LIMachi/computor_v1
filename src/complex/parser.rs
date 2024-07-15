use parser::prelude::*;
use crate::complex::Complex;

fn complex_part() -> impl Fn(StringReader) -> ParserOut<f32> {
    any((
        preceded(("iI".any(), white, optional('*'), white), optional(unsigned_float).map_ok(|o| o.unwrap_or(1.))),
        terminated(unsigned_float, (white, optional('*'), white, "iI".any()))
    ))
}

pub fn complex(input: StringReader) -> ParserOut<Complex> {
    any((
        (optional("+-".any()), white, complex_part(), white, "+-".any(), white, unsigned_float).map_ok(|(cs, _, i, _, s, _, r)| {
            Complex::new(
                if s == '-' { -r } else { r },
                if cs == Some('-') { -i } else { i }
            )
        }),
        (float, white, "+-".any(), white, complex_part()).map_ok(|(r, _, s, _, i)| {
            Complex::new(r, if s == '-' { -i } else { i })
        }),
        separated_pair(optional("+-".any()), white, complex_part()).map_ok(|(s, i)| {
            Complex::from_i(if let Some('-') = s { -i } else { i })
        }),
        float.map_ok(|r| Complex::from(r)),
    ))(input)
}

#[test]
fn test_complex_parser() {
    dbg!("i+3".parse_with(true, complex));
    dbg!("42+3*i".parse_with(true, complex));
    dbg!("*i".parse_with(true, complex));
    dbg!("I".parse_with(true, complex));
    dbg!("-42e13".parse_with(true, complex));
}