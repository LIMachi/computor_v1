use crate::{ExpectedChar, Parser, ParserError, ParserOut, StringReader};

impl <'s> Parser<&'s str> for &'s str {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<&'s str> {
        let l = self.chars().count();
        move |input| {
            for (i, c) in self.chars().enumerate() {
                if input[i] != c {
                    return Err(ParserError::InvalidCharacter { pos: input.true_index(i), char: input[i], expected: ExpectedChar::Single(c) });
                }
            }
            Ok((input.move_head(l as isize)?, self))
        }
    }
}

impl Parser<char> for char {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<char> {
        move |input| {
            if input[0] == self {
                Ok((input.move_head(1)?, self))
            } else {
                Err(ParserError::InvalidCharacter { pos: input.true_index(0), char: input[0], expected: ExpectedChar::Single(self) })
            }
        }
    }
}

pub fn white(input: StringReader) -> ParserOut<usize> {
    let mut t = 0;
    while input[t].is_whitespace() {
        t += 1;
    }
    Ok((input.move_head(t as isize)?, t))
}