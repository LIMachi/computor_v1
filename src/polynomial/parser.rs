use crate::parse_utils::{Parser, ParserOut, StringReader};
use crate::parse_utils::mappers::Mappable;
use crate::parse_utils::multi::{Any, delimited, preceded, Repeatable, separated_pair, terminated};
use crate::parse_utils::number::{float, unsigned};
use crate::parse_utils::utils::white;
use crate::polynomial::PolynomialEquation;

//matches f32, f32*X^u32, X^u32*f32, X*u32, u32*X, X and map them to (f32, u32) (u32 defaults to 0)
fn term() -> impl Fn(StringReader) -> ParserOut<(f32, u32)> {
    (
        (('X', white, '^', white), unsigned, (white, '*', white), float).map(|(_, e, _, f)| (f, e)),
        separated_pair(float, (white, '*', white, 'X', white, '^', white), unsigned),
        preceded(('X', white, '*', white), float).map(|f| (f, 1)),
        terminated(float, (white, '*', white, 'X')).map(|f| (f, 1)),
        'X'.map(|_| (1., 1)),
        float.map(|f| (f, 0)),
    ).any()
}

fn poly() -> impl Fn(StringReader) -> ParserOut<Vec<(f32, u32)>> {
    (
        white,
        term(),
        delimited(
            white,
            (
                preceded(('+', white), term()),
                preceded(('-', white), term()).map(|(f, u)| (-f, u)),
            ).any(),
            white
        ).rep(0..)
    ).map(|(_, first, mut vec)| { vec.push(first); vec })
}

pub fn parse(input: String) -> Option<PolynomialEquation> {
    let reader = StringReader::from_string(input);
    let (reader, poly) = separated_pair(poly(), (white, '=', white), poly()).map( |(v1, v2)| PolynomialEquation::new(v1, v2)).parse(reader)?;
    if !reader.finished() {
        None
    } else {
        Some(poly)
    }
}