use crate::token::{Token, TokenType};

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !Self::is_at_end() {
            self.start = self.current;
            Self::scan_token();
        }

        self.tokens.push(Token {
            type_: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        self.tokens
    }

    fn is_at_end() -> bool {
        unimplemented!();
    }

    fn scan_token() {
        unimplemented!();
    }
}
