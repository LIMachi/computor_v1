use parser::prelude::*;
use crate::polynomial::PolynomialEquation;

///matches any of: f32, f32*X^u32, X^u32*f32, X*u32, u32*X, X, X^u32 and map them to (f32, u32)
///X can also be x, each parts can optionally be separated by whitespaces (ex: "  x *    -10" is a valid term)
///(f32 defaults to 1, u32 defaults to 1 if no exponent is present, except if X is not present, then it defaults to 0)
fn term() -> impl Fn(StringReader) -> ParserOut<(f32, u32)> {
    any((
        (("Xx".any(), white, '^', white), unsigned, (white, '*', white), float).map_ok(|(_, e, _, f)| (f, e)),
        separated_pair(float, (white, '*', white, "Xx".any(), white, '^', white), unsigned),
        preceded(("Xx".any(), white, '*', white), float).map_ok(|f| (f, 1)),
        preceded(("Xx".any(), white, '^', white), unsigned).map_ok(|e| (1., e)),
        terminated(float, (white, '*', white, "Xx".any())).map_ok(|f| (f, 1)),
        "Xx".any().map_ok(|_| (1., 1)),
        float.map_ok(|f| (f, 0)),
    ))
}

///matches any amount of term separated by +/- symbols (the - symbols inverts the f32 of the next term)
fn poly() -> impl Fn(StringReader) -> ParserOut<Vec<(f32, u32)>> {
    (
        white,
        optional("+-".any()),
        white,
        term(),
        rep(.., false, delimited(
            white,
            (
                preceded(('+', white), term()),
                preceded(('-', white), term()).map_ok(|(f, u)| (-f, u)),
            ).any(),
            white
        ))
    ).map_ok(|(_, s, _, first, mut vec)| { vec.push(if s == Some('-') { (-first.0, first.1) } else { first }); vec })
}

///matches `poly = poly`
pub fn poly_eq() -> impl Fn(StringReader) -> ParserOut<PolynomialEquation> {
    separated_pair(poly(), (white, '=', white), poly()).map_ok(|(v1, v2)| PolynomialEquation::new(v1, v2))
}