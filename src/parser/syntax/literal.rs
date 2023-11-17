use std::iter::Peekable;

use crate::{ast::node::Expression, parser::tokenizer::token::Token};

pub fn parse_literal<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens_iter.next() {
        Some(Token::IntegerLiteral(n)) => Ok(Expression::IntegerLiteral(n.to_owned())),
        Some(Token::FloatingLiteral(f)) => Ok(Expression::FloatingLiteral(f.to_owned())),
        Some(Token::CharLiteral(c)) => Ok(Expression::CharLiteral(c.to_owned())),
        Some(Token::StringLiteral(s)) => Ok(Expression::StringLiteral(s.to_owned())),
        _ => Err("Expected a literal".to_string()),
    }
}

#[cfg(test)]
mod tests {}
