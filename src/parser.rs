use colorful::Color;

#[derive(Debug)]
pub struct ParserResult {
    pub string: String,
    pub color: Color, //&'static str,
}

#[derive(Debug)]
pub struct Parser {
    pub result: Vec<ParserResult>,
    pub input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            result: vec![],
            input,
        }
    }
}
