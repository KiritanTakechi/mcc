use std::{iter::Peekable, slice::Iter};

use crate::{ast::node::Expression, parser::tokenizer::token::Token};

pub fn parse_literal(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    match tokens_iter.next() {
        Some(Token::IntegerLiteral(n)) => Ok(Expression::IntegerLiteral(n.to_owned())),
        Some(Token::FloatingLiteral(f)) => Ok(Expression::FloatingLiteral(f.to_owned())),
        Some(Token::CharLiteral(c)) => Ok(Expression::CharLiteral(c.to_owned())),
        Some(Token::StringLiteral(s)) => Ok(Expression::StringLiteral(s.to_owned())),
        _ => Err("Expected a literal".to_string()),
    }
}
