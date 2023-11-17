use std::iter::Peekable;

use crate::{ast::node::Expression, parser::tokenizer::token::Token};

use super::{identifier::parse_identifier, literal::parse_literal, operator::*};

pub fn parse_expression<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    todo!()
}

pub fn parse_primary_expression<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    todo!()
}

pub fn parse_unary_expression<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    todo!()
}

pub fn parse_binary_expression<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    todo!()
}

pub fn parse_assignment_expression<'a, I>(
    tokens_iter: &mut Peekable<I>,
) -> Result<Expression, String>
where
    I: Iterator<Item = &'a Token>,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::node::*,
        parser::{syntax::expression::*, tokenizer::token::Token},
    };

    #[test]
    fn test_parse_expression() {}

    #[test]
    fn test_primary_expression() {
        let tokens = vec![Token::Identifier("x".to_string())];
        let mut tokens_iter = tokens.iter().peekable();
        let result = parse_unary_expression(&mut tokens_iter);
        assert_eq!(result, Ok(Expression::Identifier("x".to_string())));
    }

    #[test]
    fn test_parse_unary_expression() {
        let tokens = vec![Token::LogicalNot, Token::Identifier("x".to_string())];
        let mut tokens_iter = tokens.iter().peekable();
        let result = parse_unary_expression(&mut tokens_iter);
        assert_eq!(
            result,
            Ok(Expression::UnaryExpression(
                UnaryOperator::LogicalNot,
                Box::new(Expression::Identifier("x".to_string()))
            ))
        );
    }

    #[test]
    fn test_parse_binary_expression() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::IntegerLiteral(12345),
        ];
        let mut tokens_iter = tokens.iter().peekable();
        let result = parse_binary_expression(&mut tokens_iter);
        assert_eq!(
            result,
            Ok(Expression::BinaryExpression(
                BinaryOperator::Plus,
                Box::new(Expression::Identifier("x".to_string())),
                Box::new(Expression::IntegerLiteral(12345))
            ))
        );
    }

    #[test]
    fn test_parse_assignment_expression() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::AddAssign,
            Token::IntegerLiteral(114),
        ];
        let mut tokens_iter = tokens.iter().peekable();
        let result = parse_binary_expression(&mut tokens_iter);
        assert_eq!(
            result,
            Ok(Expression::AssignmentExpression {
                identifier: "x".to_string(),
                operator: AssignmentOperator::AddAssign,
                value: Box::new(Expression::IntegerLiteral(114))
            })
        );
    }
}
