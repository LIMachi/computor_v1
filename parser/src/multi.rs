use std::collections::Bound;
use std::ops::RangeBounds;
use macros::{impl_any_for_tuples, impl_parser_for_tuples};
use super::{StringReader, Parser, ParserOut, ParserError, Any, Repeatable, Branch};

impl <O, F: Fn(StringReader) -> ParserOut<O>> Parser<O> for F {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<O> { self }
}

impl Parser<()> for () {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<()> { |input| Ok((input, ())) }
}

impl Any<()> for () {
    fn any(self) -> impl Fn(StringReader) -> ParserOut<()> { |input| Ok((input, ())) }
}

impl <O, F: Fn(StringReader) -> ParserOut<O>> Any<O> for F {
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O> { self }
}

impl <O, P: Parser<O>> Branch<O> for P {
    fn branch<O2, F: Parser<O2>>(self, if_ok: F, if_error: F) -> impl Fn(StringReader) -> ParserOut<O2> {
        let parser = self.parser();
        let if_ok = if_ok.parser();
        let if_error = if_error.parser();
        move |input| {
            if parser(input.clone()).is_ok() {
                if_ok(input)
            } else {
                if_error(input)
            }
        }
    }
}

impl_parser_for_tuples!(20);
impl_any_for_tuples!(20);

impl <O, P: Parser<O>> Parser<Vec<O>> for Vec<P> {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<Vec<O>> {
        let mut parsers = Vec::with_capacity(self.len());
        for p in self {
            parsers.push(p.parser());
        }
        move |mut input| {
            let mut o;
            let mut out = Vec::with_capacity(parsers.len());
            for t in &parsers {
                (input, o) = t(input)?;
                out.push(o);
            }
            Ok((input, out))
        }
    }
}

impl <O, P: Parser<O>> Any<O> for Vec<P> {
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O> {
        let mut parsers = Vec::with_capacity(self.len());
        for p in self {
            parsers.push(p.parser());
        }
        move |input| {
            for p in &parsers {
                if let Ok((reader, o)) = p(input.clone()) {
                    return Ok((reader, o));
                }
            }
            Err(ParserError::NoMatch)
        }
    }
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.parse` and `Parser`
pub fn seq<O, P: Parser<O>>(sequence: P) -> impl Fn(StringReader) -> ParserOut<O> {
    sequence.parser()
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.any` and `Any`
pub fn any<O, A: Any<O>>(any: A) -> impl Fn(StringReader) -> ParserOut<O> {
    any.any()
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.branch` and `Branch`
pub fn branch<O1, O2, B: Parser<O1>, F: Parser<O2>>(branch: B, if_ok: F, if_error: F) -> impl Fn(StringReader) -> ParserOut<O2> {
    branch.branch(if_ok, if_error)
}

impl <O, F: Parser<O>> Repeatable<O> for F {
    fn rep<R: RangeBounds<usize>>(self, range: R, greedy: bool) -> impl Fn(StringReader) -> ParserOut<Vec<O>> {
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
            while greedy || max.map_or(true, |max| out.len() < max) {
                if let Ok((ni, o)) = parser(input.clone()) {
                    out.push(o);
                    input = ni;
                } else {
                    break;
                }
            }
            if range.contains(&out.len()) {
                Ok((input, out))
            } else {
                Err(ParserError::MatchedOutsideOfRange { matched: out.len(), min, max })
            }
        }
    }
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.rep` and 'Repeatable`
pub fn rep<O, F: Repeatable<O>, R: RangeBounds<usize>>(range: R, greedy: bool, rep: F) -> impl Fn(StringReader) -> ParserOut<Vec<O>> {
    rep.rep(range, greedy)
}

///parse 3 expressions, but only keep the result of the middle one (discarding the first and last results)
pub fn delimited<_D1, _D2, O, F: Parser<O>>(before: impl Parser<_D1>, target: F, after: impl Parser<_D2>) -> impl Fn(StringReader) -> ParserOut<O> {
    let before = before.parser();
    let target = target.parser();
    let after = after.parser();
    move |input| {
        let (input, _) = before(input)?;
        let (input, o) = target(input)?;
        after(input).map(|(reader, _)| (reader, o))
    }
}

///parse 2 expressions and discard the first result (keeping the second result)
pub fn preceded<_D, O, F: Parser<O>>(before: impl Parser<_D>, target: F) -> impl Fn(StringReader) -> ParserOut<O> {
    let before = before.parser();
    let target = target.parser();
    move |input| {
        let (input, _) = before(input)?;
        target(input)
    }
}

///parse 2 expressions and keep the first result (discarding the second result)
pub fn terminated<_D, O, F: Parser<O>>(target: F, after: impl Parser<_D>) -> impl Fn(StringReader) -> ParserOut<O> {
    let target = target.parser();
    let after = after.parser();
    move |input| {
        let (input, o) = target(input)?;
        after(input).map(|(reader, _)| (reader, o))
    }
}

///parse 3 expressions and keep only the first and last result, discarding the middle one
pub fn separated_pair<O1, _D, O2, F1: Parser<O1>, F2: Parser<O2>>(first: F1, separator: impl Parser<_D>, second: F2) -> impl Fn(StringReader) -> ParserOut<(O1, O2)> {
    let first = first.parser();
    let separator = separator.parser();
    let second = second.parser();
    move |input| {
        let (input, o1) = first(input)?;
        let (input, _) = separator(input)?;
        second(input).map(|(reader, o2)| (reader, (o1, o2)))
    }
}