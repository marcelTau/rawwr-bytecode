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
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.is_match('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::NumberLiteral)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.")
        }

        self.advance();
        self.make_token(TokenType::StringLiteral)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance()
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break
                    }
                    ' ' // @FIXME hacky workaround, how can i avoid this
                }

                _ => break,
            };
        }
    }

    fn at(&self, idx: usize) -> char {
        self.source.bytes().nth(idx).unwrap() as char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }
        self.at(self.current + 1)
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }
        self.at(self.current + 2)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.at(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.source.len() >= self.current
    }

    fn is_match(&mut self, c: char) -> bool {
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
