use std::collections::Bound;
use std::ops::RangeBounds;
use macros::{impl_any_for_tuples, impl_parser_for_tuples};
use super::{StringReader, Parser, ParserOut};

pub trait Any<O> {
    fn any_parse(&self, input: StringReader) -> ParserOut<O>;
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O>;
}

impl <O, F: Fn(StringReader) -> Option<(StringReader, O)>> Parser<O> for F {
    fn parse(&self, input: StringReader) -> Option<(StringReader, O)> {
        self(input)
    }

    fn parser(self) -> impl Fn(StringReader) -> Option<(StringReader, O)> {
        self
    }
}

impl Parser<()> for () {
    fn parse(&self, input: StringReader) -> ParserOut<()> {
        Some((input, ()))
    }

    fn parser(self) -> impl Fn(StringReader) -> ParserOut<()> {
        |input| Some((input, ()))
    }
}

impl Any<()> for () {
    fn any_parse(&self, input: StringReader) -> Option<(StringReader, ())> {
        Some((input, ()))
    }

    fn any(self) -> impl Fn(StringReader) -> ParserOut<()> {
        |input| Some((input, ()))
    }
}

impl <O, F: Fn(StringReader) -> Option<(StringReader, O)>> Any<O> for F {
    fn any_parse(&self, input: StringReader) -> Option<(StringReader, O)> {
        self(input)
    }

    fn any(self) -> impl Fn(StringReader) -> ParserOut<O> {
        self
    }
}

impl_parser_for_tuples!(20);
impl_any_for_tuples!(20);

pub trait Repeatable<O>: Parser<O> {
    fn rep<R: RangeBounds<usize>>(self, range: R) -> impl Fn(StringReader) -> ParserOut<Vec<O>>;
}

impl <O, F: Parser<O>> Repeatable<O> for F {
    fn rep<R: RangeBounds<usize>>(self, range: R) -> impl Fn(StringReader) -> ParserOut<Vec<O>> {
        let min = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v + 1,
            Bound::Unbounded => 0
        };
        let max = match range.end_bound() {
            Bound::Included(v) => Some(*v),
            Bound::Excluded(v) => Some(*v - 1),
            Bound::Unbounded => None
        };
        let parser = self.parser();
        move |mut input| {
            let mut out = Vec::with_capacity(min);
            while max.map_or(true, |max| out.len() < max) {
                if let Some((ni, o)) = parser(input.clone()) {
                    out.push(o);
                    input = ni;
                } else {
                    break;
                }
            }
            if range.contains(&out.len()) {
                Some((input, out))
            } else {
                None
            }
        }
    }
}

pub fn delimited<_D1, _D2, O, F: Parser<O>>(before: impl Parser<_D1>, target: F, after: impl Parser<_D2>) -> impl Fn(StringReader) -> Option<(StringReader, O)> {
    move |input| {
        let (input, _) = before.parse(input)?;
        let (input, o) = target.parse(input)?;
        after.parse(input).map(|(reader, _)| (reader, o))
    }
}

pub fn preceded<_D, O, F: Parser<O>>(before: impl Parser<_D>, target: F) -> impl Fn(StringReader) -> Option<(StringReader, O)> {
    move |input| {
        let (input, _) = before.parse(input)?;
        target.parse(input)
    }
}

pub fn terminated<_D, O, F: Parser<O>>(target: F, after: impl Parser<_D>) -> impl Fn(StringReader) -> Option<(StringReader, O)> {
    move |input| {
        let (input, o) = target.parse(input)?;
        after.parse(input).map(|(reader, _)| (reader, o))
    }
}

pub fn separated_pair<O1, _D, O2, F1: Parser<O1>, F2: Parser<O2>>(first: F1, separator: impl Parser<_D>, second: F2) -> impl Fn(StringReader) -> Option<(StringReader, (O1, O2))> {
    move |input| {
        let (input, o1) = first.parse(input)?;
        let (input, _) = separator.parse(input)?;
        second.parse(input).map(|(reader, o2)| (reader, (o1, o2)))
    }
}