pub mod number;
pub mod string_reader;
pub mod mappers;
pub mod multi;
pub mod utils;

pub mod prelude {
    pub use crate::{ParserError, ParserOut, Parser, ExpectedChar, StringReader, Number, Any, Repeatable, Branch, Parseable};
    pub use crate::number::{int, float, unsigned};
    pub use crate::mappers::{map, default, optional, Mappable, Optional};
    pub use crate::multi::{branch, rep, delimited, seq, separated_pair, any, preceded, terminated};
    pub use crate::utils::white;
}

use std::error::Error;
use std::ops::RangeBounds;
use std::rc::Rc;

#[derive(Debug)]
pub enum ExpectedChar {
    Single(char),
    Any(&'static str)
}

#[derive(Debug)]
pub enum ParserError {
    EndOfInput, //reached the end of the input before finishing the parser
    NoMatch, //the any method did not find a valid match, or a sequential parser did not fully match the input
    InvalidNumberCast { from: Number, to: &'static str }, //tried to cast a Number to i32/u32 while being an f32/-i32
    MatchedOutsideOfRange { matched: usize, min: usize, max: Option<usize> }, //returned if rep did not match enough times or too many times
    DanglingCharacters { head: usize, length: usize }, //returned by StringReader finished if there is still characters to process in the buffer
    InvalidCharacter { char: char, pos: usize, expected: ExpectedChar }, //tried to match a character but failed
    Custom(Box<dyn Error>) //custom parser error emitted by the user
}

///the expected return value of a parser (the new input to continue the chain and the result O, or an error if this parser failed)
pub type ParserOut<O> = Result<(StringReader, O), ParserError>;

///a cloneable buffer used for recursive access by indexes (the use of Rc make the clone about as cheap as a copy in most cases)
#[derive(Clone, Debug)]
pub struct StringReader {
    chars: Rc<Vec<char>>,
    head: usize
}

///represents a parsed number of the forms:
///1, 12.13, -5, -6e12, etc...
///can be transformed to f32 (from) and i32/u32 (try_from)
#[derive(Copy, Clone, Default, Debug)]
pub struct Number {
    negative: bool,
    integer: u32,
    frac: u32,
    negative_exponent: bool,
    exponent: u32
}

///anything that can be parsed using a parser
pub trait Parseable<O>: Into<StringReader> {
    ///take self, transform it into a buffer StringReader and apply the given parser to it, returning the result
    fn parse(self, all: bool, parser: impl Parser<O>) -> Result<O, ParserError>;
}

///anything that can be parsed in order (full match)
pub trait Parser<O> {
    ///return the output if all matched (if this is a set aka tuple, vec, etc...) or if it matched (fn, other impls)
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<O>;
}

///anything that can be parsed in order, return the first valid match
pub trait Any<O> {
    ///return the first valid match in order if this is a set (tuple, vec, etc...)
    ///if this is not a set, it is basically equivalent to `.parser()`
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O>;
}

///anything that can be parsed multiple times to construct a vec of results
pub trait Repeatable<O>: Parser<O> {
    ///try to match self multiple time (greedy or lazy), discard the match if it is not in the range
    ///in greedy mode, can match more times than asked and result in an error
    ///in lazy mode, it will stop as soon as it reaches the maximum of the range
    ///(a range with no upper bound is always greedy)
    fn rep<R: RangeBounds<usize>>(self, range: R, greedy: bool) -> impl Fn(StringReader) -> ParserOut<Vec<O>>;
}

pub trait Branch<O>: Parser<O> {
    ///continue parsing using either the ok or error branch after executing itself
    ///the input of if_ok is the original input, not the input after matching self
    ///(so branch is basically a look ahead), and the output of self is discarded
    fn branch<O2, F: Parser<O2>>(self, if_ok: F, if_error: F) -> impl Fn(StringReader) -> ParserOut<O2>;
}