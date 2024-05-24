use super::{StringReader, Parser, ParserOut};

pub trait Mappable<O>: Parser<O> {
    ///map the result of the parser to a new output (does nothing if error)
    fn map<O2, M: Fn(O) -> O2>(self, map: M) -> impl Fn(StringReader) -> ParserOut<O2>;
    ///discards the result of the parser and replaces it with a default value (does nothing if error)
    fn default<O2: Clone>(self, default: O2) -> impl Fn(StringReader) -> ParserOut<O2>;
}

pub trait Optional<O: Clone>: Parser<O> {
    ///discards the result in case of error and replaces it with a default value (makes the parser always succeed)
    fn optional(self, default: O) -> impl Fn(StringReader) -> ParserOut<O>;
}

impl <O, F: Parser<O>> Mappable<O> for F {
    fn map<O2, M: Fn(O) -> O2>(self, map: M) -> impl Fn(StringReader) -> ParserOut<O2> {
        let parser = self.parser();
        move |input| {
            parser(input).map(|(take, o)| (take, map(o)))
        }
    }

    fn default<O2: Clone>(self, default: O2) -> impl Fn(StringReader) -> ParserOut<O2> {
        let parser = self.parser();
        move |input| parser(input).map(|(reader, _)| (reader, default.clone()))
    }
}

impl <O: Clone, F: Parser<O>> Optional<O> for F {
    fn optional(self, default: O) -> impl Fn(StringReader) -> ParserOut<O> {
        let parser = self.parser();
        move |input| {
            if let Ok((reader, o)) = parser(input.clone()) {
                Ok((reader, o))
            } else {
                Ok((input, default.clone()))
            }
        }
    }
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.map` and `Mappable`
pub fn map<O1, O2, F: Parser<O1>, M: Fn(O1) -> O2>(parser: F, map: M) -> impl Fn(StringReader) -> ParserOut<O2> {
    parser.map(map)
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.default` and `Mappable`
pub fn default<O1, O2: Clone, F: Parser<O1>>(parser: F, default: O2) -> impl Fn(StringReader) -> ParserOut<O2> {
    parser.default(default)
}

///helper function to make the parser expression more readable (puts the action at the start of the expression instead of the end)
///see `.optional` and `Optional`
pub fn optional<O: Clone, F: Parser<O>>(parser: F, default: O) -> impl Fn(StringReader) -> ParserOut<O> {
    parser.optional(default)
}

///takes character while the fold function returns true, returning the state at the end (never fails)
pub fn take_fold<S: Copy, F: Fn(S, char) -> (S, bool)>(start: S, fold: F) -> impl Fn(StringReader) -> ParserOut<S> {
    move |mut input| {
        let state = start;
        loop {
            let (state, ok) = fold(state, input[0]);
            if let Ok(t) = input.move_head(1) {
                input = t;
            } else {
                return Ok((input, state));
            }
            if !ok {
                return Ok((input, state));
            }
        }
    }
}