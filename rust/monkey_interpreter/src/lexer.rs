use std::marker::PhantomData;
use crate::token::{TokenType, Token, Slice};

pub struct RawMonkeyProgram {
    input: Vec<char>,
}

impl RawMonkeyProgram {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect()
        }
    }

    pub fn slice(&self, slice: Slice) -> &[char] {
        slice.as_slice(self.input.as_slice())
    }

    pub fn substring(&self, slice: Slice) -> String {
        String::from_iter(self.slice(slice).iter())
    }

    pub fn token_substring(&self, token: Token) -> String {
        String::from_iter(self.slice(token.literal).iter())
    }
}


impl From<&str> for RawMonkeyProgram {
    fn from(value: &str) -> Self {
        RawMonkeyProgram::new(value)
    }
}

pub struct Lexer<'a> {
     program: &'a RawMonkeyProgram,
    /// current position in input (points to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    current_ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a RawMonkeyProgram) -> Self {
        Self {
            program,
            position: 0,
            read_position: 0,
            current_ch: '\0',
        }
    }

    pub fn program(&self) -> &'a RawMonkeyProgram {
        self.program
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();
        if self.position >= self.program.input.len() {
            return None;
        }
        use TokenType as T;
        let (t, len) = match self.program.input[self.position] {
            '=' => {
                if self.program.input[self.position + 1] == '=' {
                    (T::Eq, 2)
                } else {
                    (T::Assign, 1)
                }
            }
            '+' => (T::Plus, 1),
            '-' => (T::Minus, 1),
            '!' => {
                if self.program.input[self.position + 1] == '=' {
                    (T::NotEq, 2)
                } else {
                    (T::Bang, 1)
                }
            }
            '/' => (T::Slash, 1),
            '*' => (T::Asterisk, 1),
            '<' => (T::Lt, 1),
            '>' => (T::Gt, 1),
            ';' => (T::Semicolon, 1),
            ',' => (T::Comma, 1),
            '(' => (T::LParen, 1),
            ')' => (T::RParen, 1),
            '{' => (T::LBrace, 1),
            '}' => (T::RBrace, 1),
            ch => {
                if is_identifier_char(ch) {
                    let len = find_identifier_end(&self.program.input[self.position..]);
                    let tok_type = TokenType::from_identifier(
                        &self.program.input[self.position..self.position + len]);
                    (tok_type, len)
                } else if ch.is_ascii_digit() {
                    let len = self.program.input[self.position..].iter()
                        .take_while(|ch| ch.is_ascii_digit())
                        .count();
                    (T::Int, len)
                } else {
                    (T::Illegal, 1)
                }
            }
        };
        let p = self.position;
        self.position += len;
        Some(Token::new(t, Slice::new(p, len)))
    }

    fn eat_whitespace(&mut self) {
        self.position +=
            self.program.input[self.position..].iter()
                .take_while(|ch| ch.is_whitespace())
                .count()
    }
}

//
// impl<'a> Iterator for Lexer<'a> {
//     type Item = Token<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//     }
// }

fn is_identifier_char(ch: char) -> bool {
    ch.is_alphabetic()
}

fn find_identifier_end(chs: &[char]) -> usize {
    chs.iter()
        .take_while(|ch| is_identifier_char(**ch))
        .count()
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, RawMonkeyProgram};
    use crate::token::TokenType;

    #[test]
    fn test_next_token_basics() {
        let input = "=+(){},;";

        use TokenType as T;
        let tests: Vec<(TokenType, &str)> = vec![
            (T::Assign, "="),
            (T::Plus, "+"),
            (T::LParen, "("),
            (T::RParen, ")"),
            (T::LBrace, "{"),
            (T::RBrace, "}"),
            (T::Comma, ","),
            (T::Semicolon, ";"),
            (T::Eof, ""),
        ];

        let p = RawMonkeyProgram::new(input);
        let mut l = Lexer::new(&p);
        for (i, expected) in tests.iter().enumerate() {
            if let Some(t) = l.next_token() {
                assert_eq!(t.tok_type, expected.0, "test {} {:?}", i, expected);
                let t_data = t.literal.as_slice(p.input.as_slice());
                assert_eq!(t_data, expected.1.chars().collect::<Vec<_>>(), "test {} {:?}", i, expected);
            } else if expected.0 == TokenType::Eof {
                // Do nothing
            } else {
                panic!("bad value")
            }
        }
    }

    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}
10 == 10;
10 != 9;
";

        use TokenType as T;
        let tests: Vec<(TokenType, &str)> = vec![
            (T::Let, "let"),
            (T::Identifier, "five"),
            (T::Assign, "="),
            (T::Int, "5"),
            (T::Semicolon, ";"),
            (T::Let, "let"),
            (T::Identifier, "ten"),
            (T::Assign, "="),
            (T::Int, "10"),
            (T::Semicolon, ";"),
            (T::Let, "let"),
            (T::Identifier, "add"),
            (T::Assign, "="),
            (T::Function, "fn"),
            (T::LParen, "("),
            (T::Identifier, "x"),
            (T::Comma, ","),
            (T::Identifier, "y"),
            (T::RParen, ")"),
            (T::LBrace, "{"),
            (T::Identifier, "x"),
            (T::Plus, "+"),
            (T::Identifier, "y"),
            (T::Semicolon, ";"),
            (T::RBrace, "}"),
            (T::Semicolon, ";"),
            (T::Let, "let"),
            (T::Identifier, "result"),
            (T::Assign, "="),
            (T::Identifier, "add"),
            (T::LParen, "("),
            (T::Identifier, "five"),
            (T::Comma, ","),
            (T::Identifier, "ten"),
            (T::RParen, ")"),
            (T::Semicolon, ";"),
            (T::Bang, "!"),
            (T::Minus, "-"),
            (T::Slash, "/"),
            (T::Asterisk, "*"),
            (T::Int, "5"),
            (T::Semicolon, ";"),
            (T::Int, "5"),
            (T::Lt, "<"),
            (T::Int, "10"),
            (T::Gt, ">"),
            (T::Int, "5"),
            (T::Semicolon, ";"),
            (T::If, "if"),
            (T::LParen, "("),
            (T::Int, "5"),
            (T::Lt, "<"),
            (T::Int, "10"),
            (T::RParen, ")"),
            (T::LBrace, "{"),
            (T::Return, "return"),
            (T::True, "true"),
            (T::Semicolon, ";"),
            (T::RBrace, "}"),
            (T::Else, "else"),
            (T::LBrace, "{"),
            (T::Return, "return"),
            (T::False, "false"),
            (T::Semicolon, ";"),
            (T::RBrace, "}"),
            (T::Int, "10"),
            (T::Eq, "=="),
            (T::Int, "10"),
            (T::Semicolon, ";"),
            (T::Int, "10"),
            (T::NotEq, "!="),
            (T::Int, "9"),
            (T::Semicolon, ";"),
            (T::Eof, ""),
        ];

        let p = RawMonkeyProgram::new(input);
        let mut l = Lexer::new(&p);
        for (i, expected) in tests.iter().enumerate() {
            if let Some(t) = l.next_token() {
                assert_eq!(t.tok_type, expected.0, "test {} {:?}", i, expected);
                let t_data = t.literal.as_slice(p.input.as_slice());
                assert_eq!(t_data, expected.1.chars().collect::<Vec<_>>(), "test {} {:?}", i, expected);
            } else if expected.0 == TokenType::Eof {
                // Do nothing
            } else {
                panic!("bad value")
            }
        }
    }
}