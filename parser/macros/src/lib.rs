use proc_macro::TokenStream;
use std::str::FromStr;

/*
impl <O0, F0: Parser<O0>, O1, F1: Parser<O1>> Parser<(O0, O1)> for (F0, F1) {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<(O0, O1)> {
        let parsers = (self.0.parser(), self.1.parser());
        move |input| {
            let (input, o0) = parsers.0(input)?;
            let (input, o1) = parsers.1(input)?;
            Ok((input, (o0, o1)))
        }
    }
}
*/

#[proc_macro]
pub fn impl_parser_for_tuples(input: TokenStream) -> TokenStream {
    let mut s = String::new();
    let rec = usize::from_str(input.to_string().as_str()).unwrap();
    for l in 1..rec {
        s += "impl <";
        for i in 0..l {
            s += format!("O{i}, F{i}: Parser<O{i}>, ").as_str();
        }
        s += format!("O{l}, F{l}: Parser<O{l}>> Parser<(").as_str();
        for i in 0..l {
            s += format!("O{i}, ").as_str();
        }
        s += format!("O{l})> for (").as_str();
        for i in 0..l {
            s += format!("F{i}, ").as_str();
        }
        s += format!("F{l}) {{\n\t\n\n\tfn parser(self) -> impl Fn(StringReader) -> ParserOut<(").as_str();
        for i in 0..l {
            s += format!("O{i}, ").as_str();
        }
        s += format!("O{l})> {{\n\tlet parsers = (").as_str();
        for i in 0..l {
            s += format!("self.{i}.parser(), ").as_str();
        }
        s += format!("self.{l}.parser());\n\tmove |input| {{\n\t\t").as_str();
        for i in 0..=l {
            s += format!("let (input, o{i}) = parsers.{i}(input)?;\n\t\t").as_str();
        }
        s += "Ok((input, (";
        for i in 0..l {
            s += format!("o{i}, ").as_str();
        }
        s += format!("o{l})))\n\t\t}}\n\t}}\n}}\n\n").as_str();
    }
    s.parse().unwrap()
}

/*
impl <O, F1: Parser<O>, F2: Parser<O>> Any<O> for (F1, F2) {
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O> {
        let parsers = (self.0.parser(), self.1.parser());
        move |input| {
            if let Ok(t) = parsers.0(input.clone()) { return Ok(t); }
            if let Ok(t) = parsers.1(input.clone()) { return Ok(t); }
            Err(ParserError::NoMatch)
        }
    }
}
*/

#[proc_macro]
pub fn impl_any_for_tuples(input: TokenStream) -> TokenStream {
    let mut s = String::new();
    let rec = usize::from_str(input.to_string().as_str()).unwrap();
    for l in 1..rec {
        s += "impl <O, ";
        for i in 0..l {
            s += format!("F{i}: Parser<O>, ").as_str();
        }
        s += format!("F{l}: Parser<O>> Any<O> for (").as_str();
        for i in 0..l {
            s += format!("F{i}, ").as_str();
        }
        s += format!("F{l}) {{\n\tfn any(self) -> impl Fn(StringReader) -> ParserOut<O> {{\n\t\tlet parsers = (").as_str();
        for i in 0..l {
            s += format!("self.{i}.parser(), ").as_str();
        }
        s += format!("self.{l}.parser());\n\t\tmove |input| {{\n").as_str();
        for i in 0..=l {
            s += format!("\t\t\tif let Ok(t) = parsers.{i}(input.clone()) {{ return Ok(t); }}\n").as_str();
        }
        s += "\t\t\tErr(ParserError::NoMatch)\n\t\t}\n\t}\n}\n\n";
    }
    s.parse().unwrap()
}