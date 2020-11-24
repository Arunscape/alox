use crate::token::{Token, TokenType};

pub fn error(token: Token, message: &str) {
    match token.token_type {
        TokenType::EOF => report(token.line, " at end", message),
        _ => report(token.line, &format!(" at '{}'", token.lexeme), message),
    }
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
}
