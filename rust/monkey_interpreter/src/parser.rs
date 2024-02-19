use crate::lexer::Lexer;
use crate::token::Token;

enum Expression<'a> {
    Identifier {
        token: Token<'a>,
        value: String,
    }
}

enum Statement<'a> {
    Let {
        token: Token<'a>,
        name: Expression<'a>,
        value: Expression<'a>,
    },
}

pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Option<Token<'a>>,
    peek_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut l = Self {
            lexer,
            cur_token: None,
            peek_token: None,
        };
        l
    }

    pub fn next_token(&'a mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse() -> Program<'a> {
        todo!()
    }
}