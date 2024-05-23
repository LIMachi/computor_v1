pub mod number;
pub mod string_reader;
pub mod mappers;
pub mod multi;
pub mod utils;

use std::rc::Rc;

pub type ParserOut<O> = Option<(StringReader, O)>;

#[derive(Clone, Debug)]
pub struct StringReader {
    chars: Rc<Vec<char>>,
    head: usize
}

//represents a parsed number of the forms:
//1, 12.13, -5, -6e12, etc...
#[derive(Copy, Clone, Default, Debug)]
pub struct Number {
    negative: bool,
    integer: u32,
    frac: u32,
    negative_exponent: bool,
    exponent: u32
}

pub trait Parser<O> {
    fn parse(&self, input: StringReader) -> ParserOut<O>;
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<O>;
}