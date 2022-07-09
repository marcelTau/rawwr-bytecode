use crate::token::*;

pub struct Scanner {
    source: String,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Scanner {
            source: source.clone(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.source = self.source[self.current..].to_string();

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'!' => {
                if self.is_match(b'=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            b'=' => {
                if self.is_match(b'=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            b'<' => {
                if self.is_match(b'=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            b'>' => {
                if self.is_match(b'=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            _ => self.error_token("Unexpected character."),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                b' ' | b'\r' | b'\t' => self.advance(),
                b'\n' => {
                    self.line += 1;
                    self.advance()
                }
                _ => return
            };
        }
    }

    fn at(&self, idx: usize) -> u8 {
        self.source.bytes().nth(idx).unwrap()
    }

    fn peek(&self) -> u8 {
        self.at(self.current + 1)
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.at(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.source.len() >= self.current
    }

    fn is_match(&mut self, c: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.at(self.current) != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        Token::new(ttype, &self.source[..self.current], self.line)
    }

    fn error_token(&self, msg: &'static str) -> Token {
        Token::new(TokenType::Error, msg, self.line)
    }
}
