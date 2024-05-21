use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;
use nom::character::complete::u32;
use nom::error::{Error, ErrorKind};
use nom::number::complete::float;
use crate::PolynomialEquation;

fn take_whitespace(input: &str) -> IResult<&str, &str> {
    take_while(|c| char::is_whitespace(c))(input)
}

//matches f32, f32*X^u32, X^u32*f32 and map them to (f32, u32) (u32 defaults to 0)
fn term(input: &str) -> IResult<&str, (f32, u32)> {
    alt((
        map(tuple((
            tag("X"),
            take_whitespace,
            tag("^"),
            take_whitespace,
            u32,
            take_whitespace,
            tag("*"),
            take_whitespace,
            float,
        )), |(_, _, _, _, e, _, _, _, f)| (f, e)),
        map(tuple((
            float,
            take_whitespace,
            tag("*"),
            take_whitespace,
            tag("X"),
            take_whitespace,
            tag("^"),
            take_whitespace,
            u32,
        )), |(f, _, _, _, _, _, _, _, e)| (f, e)),
        map(float, |f| (f, 0)),
    ))(input)
}

//matches any amount of term separated by + or -, store them in a vec and apply the sign to the f32 part of the term
fn poly(input: &str) -> IResult<&str, Vec<(f32, u32)>> {
    many1(delimited(
        take_whitespace,
        alt((
            preceded(tuple((tag("+"), take_whitespace)), term),
            map(preceded(tuple((tag("-"), take_whitespace)), term), |(f, u)| (-f, u)),
            term
        )),
        take_whitespace
    ))(input)
}

//matches poly = poly and check that all the string was consumed
pub fn parse(input: &str) -> Result<PolynomialEquation, nom::Err<Error<String>>> {
    let r = map(separated_pair(
        poly,
        tuple((take_whitespace, tag("="), take_whitespace)),
        poly
    ), |(v1, v2)| PolynomialEquation { left: v1, right: v2 })(input);
    match r {
        Ok((s, mut p)) => {
            if !s.is_empty() {
                Err(nom::Err::Failure(Error::new(format!("input has dangling characters: '{s}'"), ErrorKind::Fail)))
            } else {
                Ok(p)
            }
        }
        Err(e) => {
            Err(e.map(|t| Error::new(t.input.to_string(), t.code)))
        }
    }
}