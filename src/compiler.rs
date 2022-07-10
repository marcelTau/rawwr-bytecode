use crate::token::*;
use crate::scanner::*;

pub struct Parser<'a> {
    previous: Token<'a>,
    current: Token<'a>,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        // TODO make the previous and current option<Token> i guess
    }
    pub fn advance(&mut self, scanner: &'a mut Scanner) {
        self.previous = self.current;

        loop {
            self.current = scanner.scan_token();

            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current();
        }
    }

    fn error_at_current(&mut self) {
        self.error_at(self.current, self.current.start);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => {}
            _ => eprint!(" at {}", token.start),
        }

        eprintln!(": {message}");
        self.had_error = true;
    }
}
