use parser::prelude::*;
use crate::polynomial::PolynomialEquation;

///matches any of: f32, f32*X^u32, X^u32*f32, X*u32, u32*X, X and map them to (f32, u32) (f32 defaults to 1, u32 defaults to 1 if no exponent is present, except if X is not present, then it defaults to 0)
fn term() -> impl Fn(StringReader) -> ParserOut<(f32, u32)> {
    any((
        (('X', white, '^', white), unsigned, (white, '*', white), float).map(|(_, e, _, f)| (f, e)),
        separated_pair(float, (white, '*', white, 'X', white, '^', white), unsigned),
        preceded(('X', white, '*', white), float).map(|f| (f, 1)),
        terminated(float, (white, '*', white, 'X')).map(|f| (f, 1)),
        'X'.map(|_| (1., 1)),
        float.map(|f| (f, 0)),
    ))
}

///matches any amount of term separated by +/- symbols (the - symbols inverts the f32 of the next term)
fn poly() -> impl Fn(StringReader) -> ParserOut<Vec<(f32, u32)>> {
    (
        white,
        term(),
        rep(.., false, delimited(
            white,
            (
                preceded(('+', white), term()),
                preceded(('-', white), term()).map(|(f, u)| (-f, u)),
            ).any(),
            white
        ))
    ).map(|(_, first, mut vec)| { vec.push(first); vec })
}

///matches `poly = poly`
pub fn poly_eq() -> impl Fn(StringReader) -> ParserOut<PolynomialEquation> {
    separated_pair(poly(), (white, '=', white), poly()).map(|(v1, v2)| PolynomialEquation::new(v1, v2))
}