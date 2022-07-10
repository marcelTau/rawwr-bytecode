use crate::token::*;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    current: usize,
    line: usize,
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
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
        self.current = 0;
        self.skip_whitespace();
        self.source = self.source[self.current..].to_string();
        self.current = 0;

        //println!("Scanning source: '{}'({}) | current = {}", self.source.trim(), self.source.len(), self.current);

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }

        if is_digit(c) {
            return self.number();
        }

        match c {
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
            _ => {
                println!("Unexpected character: {}", c);
                self.error_token("Unexpected character.")
            }
        }
    }

    fn identifier_type(&self) -> TokenType {
        match self.at(0) {
            'a' => return self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => return self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => return self.check_keyword(1, 1, "f", TokenType::If),
            'n' => return self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => return self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => return self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => return self.check_keyword(1, 4, "uper", TokenType::Super),
            'v' => return self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => if self.current > 1 {
                match self.at(1) {
                    'a' => return self.check_keyword(2, 3, "lse", TokenType::False),
                    'o' => return self.check_keyword(2, 1, "r", TokenType::For),
                    'u' => return self.check_keyword(2, 1, "n", TokenType::Fun),
                    _ => {}
                }
            },
            't' => if self.current > 1 {
                match self.at(1) {
                    'h' => return self.check_keyword(2, 2, "is", TokenType::This),
                    'r' => return self.check_keyword(2, 2, "ue", TokenType::True),
                    _ => {}
                }
            },
            _ => {}
        }
        unreachable!();
        //TokenType::Identifier
    }

    // @todo this really needs to be tested
    fn check_keyword(&self, start: usize, length: usize, rest: &str, ttype: TokenType) -> TokenType {
        if self.current == start + length && &self.source[start..(start + length)] == rest {
            ttype
        } else {
            TokenType::Identifier
        }
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
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
            return '\0';
        }
        self.at(self.current)
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.at(self.current + 1)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.at(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.source.len() <= self.current
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
