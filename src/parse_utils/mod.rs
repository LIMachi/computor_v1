pub mod number;
pub mod string_reader;
pub mod mappers;
pub mod multi;

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
    fn parser(&self) -> impl Fn(StringReader) -> ParserOut<O>;
}

pub fn tag<'a>(tag: &'a str) -> impl Fn(StringReader) -> ParserOut<&'a str> + 'a {
    let l = tag.chars().count();
    move |input| {
        for (i, c) in tag.chars().enumerate() {
            if input[i] != c {
                return None;
            }
        }
        Some((input.move_head(l as isize)?, tag))
    }
}

pub fn skip_whitespace(input: StringReader) -> ParserOut<usize> {
    let mut t = 0;
    while input[t].is_whitespace() {
        t += 1;
    }
    Some((input.move_head(t as isize)?, t))
}