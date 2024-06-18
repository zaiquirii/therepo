#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Identifier,
    Int,

    // Operators
    Bang,
    Assign,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

impl TokenType {
    pub fn from_identifier(raw: &[char]) -> Self {
        let s = String::from_iter(raw.iter());
        use TokenType as T;
        match s.as_str() {
            "fn" => T::Function,
            "let" => T::Let,
            "if" => T::If,
            "else" => T::Else,
            "return" => T::Return,
            "true" => T::True,
            "false" => T::False,
            _ => T::Identifier
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Slice {
    pub start: usize,
    pub len: usize,
}

impl Slice {
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn as_slice<'a, T>(&self, data: &'a [T]) -> &'a [T] {
        &data[self.start..self.start + self.len]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
    pub tok_type: TokenType,
    pub literal: Slice,
}

impl Token {
    pub fn new(t: TokenType, literal: Slice) -> Self {
        Self {
            tok_type: t,
            literal,
        }
    }
}