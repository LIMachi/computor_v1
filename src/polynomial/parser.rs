use crate::parse_utils::{Parser, ParserOut, skip_whitespace, StringReader, tag};
use crate::parse_utils::mappers::map;
use crate::parse_utils::multi::{alt, delimited, preceded, rep, separated_pair};
use crate::parse_utils::number::{float, unsigned};
use crate::polynomial::PolynomialEquation;

//matches f32, f32*X^u32, X^u32*f32 and map them to (f32, u32) (u32 defaults to 0)
fn term(input: StringReader) -> ParserOut<(f32, u32)> {
    alt((
        map(
            (
                tag("X"),
                skip_whitespace,
                tag("^"),
                skip_whitespace,
                unsigned,
                skip_whitespace,
                tag("*"),
                skip_whitespace,
                float,
            ),
            |(_, _, _, _, e, _, _, _, f)| (f, e)
        ),
        map(
            (
                float,
                skip_whitespace,
                tag("*"),
                skip_whitespace,
                tag("X"),
                skip_whitespace,
                tag("^"),
                skip_whitespace,
                unsigned,
            ),
            |(f, _, _, _, _, _, _, _, e)| (f, e)
        ),
        map(float, |f| (f, 0)),
    ))(input)
}

fn poly(input: StringReader) -> ParserOut<Vec<(f32, u32)>> {
    rep(1.., delimited(
        skip_whitespace,
        alt((
            preceded((tag("+"), skip_whitespace), term),
            map(preceded((tag("-"), skip_whitespace), term), |(f, u)| (-f, u)),
            term
        )),
        skip_whitespace
    ))(input)
}

pub fn parse(input: String) -> Option<PolynomialEquation> {
    let reader = StringReader::from_string(input);
    let (reader, poly) = map(separated_pair(poly, (skip_whitespace, tag("="), skip_whitespace), poly), |(v1, v2)| PolynomialEquation::new(v1, v2))(reader)?;
    if !reader.finished() {
        None
    } else {
        Some(poly)
    }
}