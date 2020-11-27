use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl std::ops::Neg for Literal {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Number(n) => Self::Number(-n),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Not for Literal {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::Boolean(n) => Self::Boolean(!n),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Literal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => Self::Number(x + y),
            (Self::String(x), Self::String(y)) => Self::String(x + &y),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Sub for Literal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => Self::Number(x - y),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul for Literal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => Self::Number(x * y),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Div for Literal {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => Self::Number(x / y),
            _ => unreachable!(),
        }
    }
}
