use proc_macro::TokenStream;

/*
impl <O0, F0: Parser<O0>, O1, F1: Parser<O1>> Parser<(O0, O1)> for (F0, F1) {
    fn parse(&self, input: StringReader) -> Option<(StringReader, (O0, O1))> {
        let (input, o0) = self.0.parse(input)?;
        let (input, o1) = self.1.parse(input)?;
        Some((input, (o0, o1)))
    }

    fn parser(&self, input: StringReader) -> Option<(StringReader, (O0, O1))> {
        move |input| {
            let (input, o0) = self.0.parse(input)?;
            let (input, o1) = self.1.parse(input)?;
            Some((input, (o0, o1)))
        }
    }
}
*/

#[proc_macro]
pub fn impl_parser_for_tuples(_input: TokenStream) -> TokenStream {
    let mut s = String::new();
    for l in 1..20 {
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
        s += format!("F{l}) {{\n\tfn parse(&self, input: StringReader) -> Option<(StringReader, (").as_str();
        for i in 0..l {
            s += format!("O{i}, ").as_str();
        }
        s += format!("O{l}))> {{\n\t\t").as_str();
        let mut lines = String::new();
        for i in 0..=l {
            lines += format!("let (input, o{i}) = self.{i}.parse(input)?;\n\t\t").as_str();
        }
        lines += "Some((input, (";
        for i in 0..l {
            lines += format!("o{i}, ").as_str();
        }
        lines += format!("o{l})))\n\t").as_str();
        s += lines.as_str();
        s += "}\n\n\tfn parser(&self) -> impl Fn(StringReader) -> Option<(StringReader, (";
        for i in 0..l {
            s += format!("O{i}, ").as_str();
        }
        s += format!("O{l}))> {{\n\tmove |input| {{\n\t\t").as_str();
        s += lines.as_str();
        s += "\t}\n\t}\n}\n\n";
    }
    s.parse().unwrap()
}

/*
impl <O, F1: Parser<O>, F2: Parser<O>> Any<O> for (F1, F2) {
    fn any(&self, input: StringReader) -> Option<(StringReader, O)> {
        if let Some(t) = self.0.parse(input.clone()) {
            return Some(t);
        }
        if let Some(t) = self.1.parse(input.clone()) {
            return Some(t);
        }
        None
    }
}
*/

#[proc_macro]
pub fn impl_any_for_tuples(_input: TokenStream) -> TokenStream {
    let mut s = String::new();
    for l in 1..20 {
        s += "impl <O, ";
        for i in 0..l {
            s += format!("F{i}: Parser<O>, ").as_str();
        }
        s += format!("F{l}: Parser<O>> Any<O> for (").as_str();
        for i in 0..l {
            s += format!("F{i}, ").as_str();
        }
        s += format!("F{l}) {{\n\tfn any(&self, input: StringReader) -> Option<(StringReader, O)> {{\n").as_str();
        for i in 0..=l {
            s += format!("\t\tif let Some(t) = self.{i}.parse(input.clone()) {{\n\t\t\treturn Some(t);\n\t\t}}\n").as_str();
        }
        s += "\t\tNone\n\t}\n}\n\n";
    }
    s.parse().unwrap()
}