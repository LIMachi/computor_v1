use crate::parse_utils::{Parser, ParserOut, StringReader};

impl <'s> Parser<&'s str> for &'s str {
    fn parse(&self, input: StringReader) -> ParserOut<&'s str> {
        let l = self.chars().count();
        for (i, c) in self.chars().enumerate() {
            if input[i] != c {
                return None;
            }
        }
        Some((input.move_head(l as isize)?, self))
    }

    fn parser(self) -> impl Fn(StringReader) -> ParserOut<&'s str> {
        let l = self.chars().count();
        move |input| {
            for (i, c) in self.chars().enumerate() {
                if input[i] != c {
                    return None;
                }
            }
            Some((input.move_head(l as isize)?, self))
        }
    }
}

impl Parser<char> for char {
    fn parse(&self, input: StringReader) -> ParserOut<char> {
        if input[0] == *self {
            Some((input.move_head(1)?, *self))
        } else {
            None
        }
    }

    fn parser(self) -> impl Fn(StringReader) -> ParserOut<char> {
        move |input| {
            if input[0] == self {
                Some((input.move_head(1)?, self))
            } else {
                None
            }
        }
    }
}

pub fn white(input: StringReader) -> ParserOut<usize> {
    let mut t = 0;
    while input[t].is_whitespace() {
        t += 1;
    }
    Some((input.move_head(t as isize)?, t))
}