use super::{StringReader, Parser};

pub trait Mappable<O>: Parser<O> {
    fn map<O2, M: Fn(O) -> O2>(self, map: M) -> impl Fn(StringReader) -> Option<(StringReader, O2)>;
    fn default<O2: Default + Clone>(self) -> impl Fn(StringReader) -> Option<(StringReader, O2)>;
    fn optional(self) -> impl Fn(StringReader) -> Option<(StringReader, Option<O>)>;
}

impl <O, F: Parser<O>> Mappable<O> for F {
    fn map<O2, M: Fn(O) -> O2>(self, map: M) -> impl Fn(StringReader) -> Option<(StringReader, O2)> {
        move |input| {
            if let Some((take, o)) = self.parse(input) {
                Some((take, map(o)))
            } else {
                None
            }
        }
    }

    fn default<O2: Default + Clone>(self) -> impl Fn(StringReader) -> Option<(StringReader, O2)> {
        let default = O2::default();
        move |input| self.parse(input).map(|(reader, _)| (reader, default.clone()))
    }

    fn optional(self) -> impl Fn(StringReader) -> Option<(StringReader, Option<O>)> {
        move |input| {
            if let Some((reader, o)) = self.parse(input.clone()) {
                Some((reader, Some(o)))
            } else {
                Some((input, None))
            }
        }
    }
}

pub fn take_fold<S: Copy, F: Fn(S, char) -> (S, bool)>(start: S, fold: F) -> impl Fn(StringReader) -> Option<(StringReader, S)> {
    move |mut input| {
        let state = start;
        loop {
            let (state, ok) = fold(state, input[0]);
            if let Some(t) = input.move_head(1) {
                input = t;
            } else {
                return Some((input, state));
            }
            if !ok {
                return Some((input, state));
            }
        }
    }
}