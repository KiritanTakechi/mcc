use crate::ast::node::{BinaryOperator, UnaryOperator};

use super::tokenizer::token::Token;

pub fn token_to_unary_operator(token: Token) -> Option<UnaryOperator> {
    match token {
        Token::LogicalNot => Some(UnaryOperator::LogicalNot),
        Token::BitwiseNot => Some(UnaryOperator::BitwiseNot),
        Token::Increment => Some(UnaryOperator::Increment),
        Token::Decrement => Some(UnaryOperator::Decrement),
        _ => None,
    }
}

pub fn token_to_binary_operator(token: Token) -> Option<BinaryOperator> {
    match token {
        Token::Plus => Some(BinaryOperator::Plus),
        Token::Minus => Some(BinaryOperator::Minus),
        Token::Multiply => Some(BinaryOperator::Multiply),
        Token::Divide => Some(BinaryOperator::Divide),
        Token::Modulo => Some(BinaryOperator::Modulo),
        Token::Equal => Some(BinaryOperator::Equal),
        Token::NotEqual => Some(BinaryOperator::NotEqual),
        Token::LessThan => Some(BinaryOperator::LessThan),
        Token::GreaterThan => Some(BinaryOperator::GreaterThan),
        Token::LessThanOrEqual => Some(BinaryOperator::LessThanOrEqual),
        Token::GreaterThanOrEqual => Some(BinaryOperator::GreaterThanOrEqual),
        Token::LogicalAnd => Some(BinaryOperator::LogicalAnd),
        Token::LogicalOr => Some(BinaryOperator::LogicalOr),
        Token::BitwiseAnd => Some(BinaryOperator::BitwiseAnd),
        Token::BitwiseOr => Some(BinaryOperator::BitwiseOr),
        Token::BitwiseXor => Some(BinaryOperator::BitwiseXor),
        Token::LeftShift => Some(BinaryOperator::LeftShift),
        Token::RightShift => Some(BinaryOperator::RightShift),
        _ => None,
    }
}
