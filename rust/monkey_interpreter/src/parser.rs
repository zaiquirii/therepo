use crate::lexer::{Lexer, RawMonkeyProgram};
use crate::token;
use crate::token::{Token, TokenType};

#[derive(PartialEq)]
struct Identifier {
    token: Token,
    value: String,
}

#[derive(PartialEq)]
enum Expression {
    Placeholder,
    Identifier(Identifier),
}

enum Statement {
    Let {
        token: Token,
        name: Identifier,
        value: Expression,
    },
    Return {
        token: Token,
        value: Expression,
    },
    Expression {
        token: Token,
        expression: Expression,
    },
}

pub struct Program {
    statements: Vec<Statement>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut l = Self {
            lexer,
            cur_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        // Initialize cur_token and peek_token
        l.next_token();
        l.next_token();
        l
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        loop {
            if self.cur_token.is_none() {
                break;
            }
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        use token::TokenType as T;
        match self.cur_token.clone()?.tok_type {
            T::Let => self.parse_let_statement(),
            T::Return => self.parse_return_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let token = self.cur_token.unwrap();
        let name = Identifier {
            token,
            value: self.lexer.program().token_substring(token),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.cur_type_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let {
            token,
            name,
            value: Expression::Placeholder,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let statement = Statement::Return {
            token: self.cur_token.unwrap(),
            value: Expression::Placeholder,
        };
        self.next_token();
        while !self.cur_type_is(TokenType::Semicolon) {
            self.next_token()
        }
        Some(statement)
    }

    fn cur_type_is(&self, tok_type: TokenType) -> bool {
        if let Some(t) = &self.cur_token {
            t.tok_type == tok_type
        } else {
            false
        }
    }

    fn expect_peek(&mut self, tok_type: TokenType) -> bool {
        if self.peek_type_is(tok_type) {
            self.next_token();
            true
        } else {
            self.errors.push(format!("expected next token to be {:?}, got {:?} instead", tok_type, self.peek_token));
            false
        }
    }

    fn peek_type_is(&self, tok_type: TokenType) -> bool {
        if let Some(t) = &self.peek_token {
            t.tok_type == tok_type
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum ParseError {
    Generic
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, RawMonkeyProgram};
    use super::*;

    #[test]
    fn test_let_statements() -> Result<(), ParseError> {
        let input = r"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let p = RawMonkeyProgram::new(input);
        let lexer = Lexer::new(&p);
        let mut parser = Parser::new(lexer);
        let program = parser.parse()?;
        assert_eq!(program.statements.len(), 3);

        let tests = vec![
            ("x",),
            ("y",),
            ("foobar",),
        ];

        for (index, &(ident, )) in tests.iter().enumerate() {
            let statement = &program.statements[index];
            if let Statement::Let { token, name, value } = statement {
                assert_eq!(name.value, ident);
                assert_eq!(ident, p.substring(token.literal))
            } else {
                assert!(false);
            }
        }

        Ok(())
    }

    #[test]
    fn test_return_statements() -> Result<(), ParseError> {
        let input = r"
        return 5;
        return 10;
        return 993322;
        ";

        let p = RawMonkeyProgram::new(input);
        let lexer = Lexer::new(&p);
        let mut parser = Parser::new(lexer);
        let program = parser.parse()?;

        assert_eq!(program.statements.len(), 3);
        for statement in program.statements {
            if let Statement::Return { token, value } = statement {
                assert_eq!(token.tok_type, TokenType::Return);
            } else {
                assert!(false, "statement was not a return statement")
            }
        }

        Ok(())
    }
}