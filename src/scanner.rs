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

        self.error_token("Unexpected character.")
    }

    fn is_at_end(&self) -> bool {
        self.source.len() >= self.current
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        Token::new(ttype, &self.source[..self.current], self.line)
    }

    fn error_token(&self, msg: &'static str) -> Token {
        Token::new(TokenType::Error, msg, self.line)
    }
}
