use super::{StringReader, Parser};

pub fn default<O1, O2: Default + Clone, F: Parser<O1>>(map_default: F) -> impl Fn(StringReader) -> Option<(StringReader, O2)> {
    let default = O2::default();
    move |input| map_default.parse(input).map(|(reader, _)| (reader, default.clone()))
}

pub fn optional<O, F: Parser<O>>(opt: F) -> impl Fn(StringReader) -> Option<(StringReader, Option<O>)> {
    move |input| {
        if let Some((reader, o)) = opt.parse(input.clone()) {
            Some((reader, Some(o)))
        } else {
            Some((input, None))
        }
    }
}

pub fn take_fold<S: Copy, F: Fn(S, char) -> (S, bool)>(start: S, fold: F) -> impl Fn(StringReader) -> Option<(StringReader, S)> {
    move |mut input| {
        let state = start;
        loop {
            let (state, ok) = fold(state, input[0]);
            input = input.move_head(1)?;
            if !ok {
                return Some((input, state));
            }
        }
    }
}

pub fn map<O1, O2, F: Parser<O1>, M: Fn(O1) -> O2>(part: F, map: M) -> impl Fn(StringReader) -> Option<(StringReader, O2)> {
    move |input| {
        if let Some((take, o)) = part.parse(input) {
            Some((take, map(o)))
        } else {
            None
        }
    }
}