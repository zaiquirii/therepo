#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct Token<'a> {
    pub tok_type: TokenType,
    pub literal: &'a [char],
}

impl<'a> Token<'a> {
    pub fn new(t: TokenType, literal: &'a [char]) -> Self {
        Self {
            tok_type: t,
            literal,
        }
    }
}