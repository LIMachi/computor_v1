use proc_macro::TokenStream;
use std::str::FromStr;

/*
impl <O0, O1, M0: Parser<O0>, M1: Parser<O1>> Parser<(O0, O1)> for (M0, M1) {
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<(O0, O1)> {
        let parsers = (self.0.parser(), self.1.parser());
        move |input| {
            let (input, o0) = parsers.0(input)?;
            let (input, o1) = parsers.1(input)?;
            Ok((input, (o0, o1)))
        }
    }
}

impl <O, M0: Parser<O>, M1: Parser<O>> Any<O> for (M0, M1) {
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O> {
        let parsers = (self.0.parser(), self.1.parser());
        move |input| {
            if let Ok(t) = parsers.0(input.clone()) { return Ok(t); }
            if let Ok(t) = parsers.1(input.clone()) { return Ok(t); }
            Err(ParserError::NoMatch { head: input.true_index(0) })
        }
    }
}

impl <O0, M0: Parser<O0>, O1, M1: Parser<O1>> Permutation<(O0, O1)> for (M0, M1) {
    fn permute(self) -> impl Fn(StringReader) -> ParserOut<(O0, O1)> {
        let parsers = (self.0.parser(), self.1.parser());
        move |mut input| {
            let mut o0 = None;
            let mut o1 = None;
            for _ in 0..2 {
                if o0.is_none() {
                    if let Ok((reader, o)) = parsers.0(input.clone()) {
                        input = reader;
                        o0 = Some(o);
                        continue;
                    }
                }
                if o1.is_none() {
                    if let Ok((reader, o)) = parsers.1(input.clone()) {
                        input = reader;
                        o1 = Some(o);
                        continue;
                    }
                }
                return Err(ParserError::NoMatch { head: input.true_index(0) });
            }
            Ok((input, (o0.unwrap(), o1.unwrap())))
        }
    }
}
*/

#[proc_macro]
pub fn impl_tuples(input: TokenStream) -> TokenStream {
    let rec = usize::from_str(input.to_string().as_str()).unwrap();
    let mut out = String::new();

    for l in 1..rec {
        let mut generic_output_list = String::new(); //ex: "O0, O1, O2"
        let mut generic_modules_list = String::new(); //ex: "M0, M1, M2"
        let mut generic_parser_list = String::new(); //ex: "M0: Parser<O0>, M1: Parser<O1>"
        let mut generic_parser_list_with_o = String::new(); //ex: "M0: Parser<O>, M1: Parser<O>"
        let mut mapped_parsers = String::new(); //ex: "self.0.parser(), self.1.parser()"
        let mut parsers_output = String::new(); //ex: "o0, o1, o2"
        let mut parser_impl_lines = String::new(); //ex: "let (input, o0) = parsers.0(input)?;\n"
        let mut any_impl_lines = String::new(); //ex: "if let Ok(t) = parsers.0(input.clone()) { return Ok(t); }\n"
        let mut optional_output = String::new(); //ex: let mut o0 = None;
        let mut unwrap_output = String::new(); //o0.unwrap(), o1.unwrap()
        let mut permut_impl_block = String::new();
        for i in 0..=l {
            generic_output_list += format!("O{i}").as_str();
            generic_modules_list += format!("M{i}").as_str();
            generic_parser_list += format!("M{i}: Parser<O{i}>").as_str();
            generic_parser_list_with_o += format!("M{i}: Parser<O>").as_str();
            mapped_parsers += format!("self.{i}.parser()").as_str();
            parsers_output += format!("o{i}").as_str();
            parser_impl_lines += format!("let (input, o{i}) = parsers.{i}(input)?;").as_str();
            any_impl_lines += format!("if let Ok(t) = parsers.{i}(input.clone()) {{ return Ok(t); }}").as_str();
            optional_output += format!("let mut o{i} = None;").as_str();
            unwrap_output += format!("o{i}.unwrap()").as_str();
            permut_impl_block += format!("if o{i}.is_none() {{
                if let Ok((reader, o)) = parsers.{i}(input.clone()) {{
                    input = reader;
                    o{i} = Some(o);
                    continue;
                }}
            }}").as_str();
            if i != l {
                generic_output_list += ", ";
                generic_modules_list += ", ";
                generic_parser_list += ", ";
                generic_parser_list_with_o += ", ";
                mapped_parsers += ", ";
                parsers_output += ", ";
                parser_impl_lines += "\n            ";
                any_impl_lines += "\n            ";
                optional_output += "\n            ";
                unwrap_output += ", ";
                permut_impl_block += "\n                ";
            }
        }

        out += format!(r"

impl <{generic_output_list}, {generic_parser_list}> Parser<({generic_output_list})> for ({generic_modules_list}) {{
    fn parser(self) -> impl Fn(StringReader) -> ParserOut<({generic_output_list})> {{
        let parsers = ({mapped_parsers});
        move |input| {{
            {parser_impl_lines}
            Ok((input, ({parsers_output})))
        }}
    }}
}}

impl <O, {generic_parser_list_with_o}> Any<O> for ({generic_modules_list}) {{
    fn any(self) -> impl Fn(StringReader) -> ParserOut<O> {{
        let parsers = ({mapped_parsers});
        move |input| {{
            {any_impl_lines}
            Err(ParserError::NoMatch {{ head: input.true_index(0) }})
        }}
    }}
}}

impl <{generic_output_list}, {generic_parser_list}> Permutation<({generic_output_list})> for ({generic_modules_list}) {{
    fn permute(self) -> impl Fn(StringReader) -> ParserOut<({generic_output_list})> {{
        let parsers = ({mapped_parsers});
        move |mut input| {{
            {optional_output}
            for _ in 0..{l} {{
                {permut_impl_block}
                return Err(ParserError::NoMatch {{ head: input.true_index(0) }});
            }}
            Ok((input, ({unwrap_output})))
        }}
    }}
}}
").as_str();
    }
    out.parse().unwrap()
}