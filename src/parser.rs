/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;

*/
use crate::token::{Token, TokenType};

enum Expr {
    Binary { left: Option<Box<Expr>> },
}

/// A recursive descent parser
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        todo!()
    }

    fn previous(&self) -> Token {
        todo!()
    }

    fn expression(&mut self) -> Expr {
        todo!();
        //self.equality()
    }

    fn equality(&mut self) -> &dyn Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = &BinaryExpr {
                left: Some(Box::new(*expr)),
                operator,
                right: Some(Box::new(right)),
            };
        }
        expr.into()
    }

    fn comparison(&mut self) -> &dyn Expr {
        todo!()
    }
    fn term(&mut self) -> &dyn Expr {
        todo!()
    }
    fn factor(&mut self) -> &dyn Expr {
        todo!()
    }
    fn unary(&mut self) -> &dyn Expr {
        todo!()
    }
    fn primary(&mut self) -> &dyn Expr {
        todo!()
    }
}
