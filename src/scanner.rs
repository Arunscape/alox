use crate::{
    error,
    token::{Literal, Token, TokenType},
};

pub struct Scanner {
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
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            ' ' | '\r' | '\t' => {
                //ignore whitespace
            }
            '"' => self.string(),
            '\n' => self.line += 1,
            c if Self::is_digit(c) => self.number(),
            c if Self::is_alpha(c) => self.identifier(),
            _ => error::error(self.tokens[self.current].clone(), "Unexpected character!"),
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.into(),
            literal,
            line: self.line,
        })
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error::error(
                self.tokens[self.current].clone(),
                "Unterminated string! Try adding a \"",
            );
            return;
        }

        self.advance(); // the other "

        // trim the quotes
        let value = self.source[self.start + 1..self.current - 1].into();

        self.add_token(TokenType::STRING, Some(Literal::String(value)))
    }

    fn is_digit(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // look for fraction part
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let number = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::NUMBER, Some(Literal::Number(number)));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_alpha(c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }
    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        match text {
            "and" => self.add_token(TokenType::AND, None),
            "class" => self.add_token(TokenType::CLASS, None),
            "else" => self.add_token(TokenType::ELSE, None),
            "false" => self.add_token(TokenType::FALSE, None),
            "for" => self.add_token(TokenType::FOR, None),
            "fun" => self.add_token(TokenType::FUN, None),
            "if" => self.add_token(TokenType::IF, None),
            "nil" => self.add_token(TokenType::NIL, None),
            "or" => self.add_token(TokenType::OR, None),
            "print" => self.add_token(TokenType::PRINT, None),
            "return" => self.add_token(TokenType::RETURN, None),
            "super" => self.add_token(TokenType::SUPER, None),
            "this" => self.add_token(TokenType::THIS, None),
            "true" => self.add_token(TokenType::TRUE, None),
            "var" => self.add_token(TokenType::VAR, None),
            "while" => self.add_token(TokenType::WHILE, None),
            _ => self.add_token(TokenType::IDENTIFIER, None),
        }
    }
}
