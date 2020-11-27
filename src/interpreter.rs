use crate::{parser::Expr, token::Literal, token::Token, token::TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn visit_grouping_expr(&self, expr: Expr) -> Literal {
        match expr {
            Expr::Grouping(_) => self.evaluate(expr),
            _ => panic!("Expr must be Expr::Grouping variant"),
        }
    }

    pub fn visit_unary_expr(&self, expr: Expr) -> Literal {
        match expr {
            Expr::Unary { operator, right } => {
                let right = self.evaluate(*right);
                match operator.token_type {
                    TokenType::Minus => return -right,
                    TokenType::Bang => return !Self::is_truthy(right),
                    _ => {}
                }

                unreachable!()
            }
            _ => panic!("Expr must be Expr::Unary variant"),
        }
    }

    // TODO restrict the types
    pub fn visit_binary_expr(&self, expr: Expr) -> Literal {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left);
                let right = self.evaluate(*right);
                match operator.token_type {
                    TokenType::Plus => left + right,
                    TokenType::Minus => left - right,
                    TokenType::Slash => left / right,
                    TokenType::Star => left * right,
                    TokenType::Greater => left > right,
                    TokenType::GreaterEqual => left >= right,
                    TokenType::Less => left < right,
                    TokenType::LessEqual => left <= right,
                    TokenType::BangEqual => left != right,
                    TokenType::EqualEqual => left == right,
                    _ => unreachable!(),
                }
            }
            _ => panic!("Expr must be Expr::Binary variant"),
        }
    }

    fn evaluate(&self, expr: Expr) -> Literal {
        todo!();
        // expr.accept(&self)
    }

    fn is_truthy(literal: Literal) -> Literal {
        match literal {
            Literal::Nil => Literal::Boolean(false),
            Literal::Boolean(_) => literal,
            _ => Literal::Boolean(true),
        }
    }
}
