use super::{StringReader, Number, ParserOut, ParserError, ExpectedChar};

pub fn float(input: StringReader) -> ParserOut<f32> {
    Number::read(false, input).map(|(reader, num)| (reader, f32::from(num)))
}

///same as float, but does not allow a preceding +/-
pub fn unsigned_float(input: StringReader) -> ParserOut<f32> {
    Number::read(true, input).map(|(reader, num)| (reader, f32::from(num)))
}

pub fn int(input: StringReader) -> ParserOut<i32> {
    Number::read(false, input).and_then(|(reader, num)| i32::try_from(num).map(|v| (reader, v)).map_err(|_| ParserError::InvalidNumberCast { from: num, to: "i32" }))
}

pub fn unsigned(input: StringReader) -> ParserOut<u32> {
    Number::read(false, input).and_then(|(reader, num)| u32::try_from(num).map(|v| (reader, v)).map_err(|_| ParserError::InvalidNumberCast { from: num, to: "u32" }))
}

impl TryFrom<Number> for u32 {
    type Error = ();

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        if value.negative || value.frac > 0 {
            Err(())
        } else {
            let mut e = 1.;
            let mut exp = value.exponent;
            while exp > 0 {
                exp -= 1;
                if value.negative_exponent {
                    e /= 10.;
                } else {
                    e *= 10.;
                }
            }
            let t =  value.integer as f32 * e;
            if t.fract() > 0. {
                Err(())
            } else {
                Ok(t as u32)
            }
        }
    }
}

impl TryFrom<Number> for i32 {
    type Error = ();

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        if value.frac > 0 {
            Err(())
        } else {
            let mut e = 1.;
            let mut exp = value.exponent;
            while exp > 0 {
                exp -= 1;
                if value.negative_exponent {
                    e /= 10.;
                } else {
                    e *= 10.;
                }
            }
            let t =  value.integer as f32 * e;
            if t.fract() > 0. {
                Err(())
            } else {
                Ok((t * if value.negative { -1. } else { 1. }) as i32)
            }
        }
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> f32 {
        let frac = if value.frac > 0 {
            let mut frac = value.frac;
            while frac % 10 == 0 {
                frac /= 10;
            }
            let f = frac;
            let mut m = 1;
            while frac > 0 {
                frac /= 10;
                m *= 10;
            }
            f as f32 / m as f32
        } else {
            0.
        };
        let mut e = 1.;
        let mut exp = value.exponent;
        while exp > 0 {
            exp -= 1;
            if value.negative_exponent {
                e /= 10.;
            } else {
                e *= 10.;
            }
        }
        (if value.negative { -1. } else { 1. }) * (value.integer as f32 + frac) * e
    }
}

#[derive(PartialEq)]
enum NumberState {
    Integer,
    Fractional,
    Exponent
}

impl Number {
    fn read(unsigned: bool, reader: StringReader) -> ParserOut<Self> {
        let r = reader.clone();
        let r = r.skip_whitespaces();
        let mut state = NumberState::Integer; //integer allow transition to dot/e, dot is always consumed, e is consumed only if an exponent is present
        let mut out = Self::default();
        match r[0] {
            '-' if !unsigned => { out.negative = true; },
            '+' if !unsigned => {},
            c @ '0' ..= '9' => { out.integer = c as u32 - '0' as u32; },
            c @ _ => { return Err(ParserError::InvalidCharacter { pos: r.true_index(0), char: c, expected: ExpectedChar::Any(format!("{}0123456789", if unsigned { "" } else { "+-" })) }); }
        }
        let mut r = r.move_head(1)?;
        loop {
            match r[0] {
                '.' if state == NumberState::Integer => { state = NumberState::Fractional; },
                'e' => {
                    if state == NumberState::Integer || state == NumberState::Fractional {
                        if (r[1] == '+' || r[1] == '-') && r[2] >= '0' && r[2] <= '9' {
                            if r[1] == '-' {
                                out.negative_exponent = true;
                            }
                            r = r.move_head(1)?;
                        }
                        if r[1] >= '0' && r[1] <= '9' {
                            state = NumberState::Exponent;
                            out.exponent = r[1] as u32 - '0' as u32;
                            r = r.move_head(1)?;
                        }
                    }
                    if state != NumberState::Exponent {
                        return Ok((r.move_head(-1)?, out));
                    }
                },
                c @ '0' ..= '9' => {
                    let v = c as u32 - '0' as u32;
                    match state {
                        NumberState::Integer => if out.integer <= 100000000 {
                            out.integer = out.integer * 10 + v;
                        },
                        NumberState::Fractional => if out.frac <= 100000000 {
                            out.frac = out.frac * 10 + v;
                        },
                        NumberState::Exponent => if out.exponent <= 100000000 {
                            out.exponent = out.exponent * 10 + v
                        }
                    }
                }
                _ => { return Ok((r, out)); }
            }
            let t = r.move_head(1);
            if t.is_err() { return Ok((r, out)); }
            r = t.unwrap();
        }
    }
}