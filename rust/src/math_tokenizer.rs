use crate::scanner::Scanner;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum MathToken {
    Unknown(String),
    Number(f32),
    Imaginary(f32),
    Variable(String),
    Function(String, usize), // arity
    UOp(String),
    BOp(String),
    OParen,
    CParen,
    Comma,
}

pub struct MathTokenizer<I: Iterator<Item = char>> {
    src: Scanner<I>,
    prev: Option<MathToken>,
}
impl fmt::Display for MathToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<I: Iterator<Item = char>> MathTokenizer<I> {
    pub fn new(source: I) -> Self {
        MathTokenizer {
            src: Scanner::new(source),
            prev: None,
        }
    }

    pub fn scanner(source: I) -> Scanner<Self> {
        Scanner::new(Self::new(source))
    }

    // when would a minus be unary? we need to know the prev token
    fn makes_unary(prev: &Option<MathToken>) -> bool {
        match *prev {
            Some(MathToken::Number(_)) => false,
            Some(MathToken::Variable(_)) => false,
            Some(MathToken::CParen) => false,
            _ => true,
        }
    }

    fn get_token(&mut self) -> Option<MathToken> {
        self.src.scan_whitespace(); // discard whatever came before + and spaces
        if let Some(op) = self.src.scan_math_op() {
            return match op.as_ref() {
                "(" => Some(MathToken::OParen),
                ")" => Some(MathToken::CParen),
                "," => Some(MathToken::Comma),
                "!" => Some(MathToken::UOp(op)),
                "-" if Self::makes_unary(&self.prev) => Some(MathToken::UOp(op)),
                _ => Some(MathToken::BOp(op)),
            };
        }
        if let Some(id) = self.src.scan_identifier() {
            if id.eq("i") {
                return Some(MathToken::Imaginary(1.));
            };
            return match self.src.peek() {
                Some('(') => Some(MathToken::Function(id, 0)),
                _ => Some(MathToken::Variable(id)),
            };
        }
        if let Some(num) = self.src.scan_number() {
            self.src.scan_whitespace(); // discard whatever came before + and spaces
            use std::str::FromStr;
            let value = f32::from_str(&num).unwrap();
            if self.src.scan_unit() {
                return Some(MathToken::Imaginary(value));
            }
            return Some(MathToken::Number(value));
        }
        if self.src.next().is_some() {
            return Some(MathToken::Unknown(self.src.extract_string()));
        }
        None
    }
}

impl<I: Iterator<Item = char>> Iterator for MathTokenizer<I> {
    type Item = MathToken;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.get_token();
        self.prev = token.clone();
        token
    }
}
