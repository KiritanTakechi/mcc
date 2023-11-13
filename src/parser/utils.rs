use crate::ast::{node::*, traits::IntoStatement};

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

pub fn token_to_type_specifier(token: Token) -> Option<TypeSpecifier> {
    match token {
        Token::Void => Some(TypeSpecifier::Void),
        Token::Int => Some(TypeSpecifier::Int),
        Token::Long => Some(TypeSpecifier::Long),
        Token::Float => Some(TypeSpecifier::Float),
        Token::Double => Some(TypeSpecifier::Double),
        Token::Char => Some(TypeSpecifier::Char),
        Token::Union => Some(TypeSpecifier::Union),
        Token::Struct => Some(TypeSpecifier::Struct),
        _ => None,
    }
}

pub fn token_to_assignment_operator(token: Token) -> Option<AssignmentOperator> {
    match token {
        Token::Assignment => Some(AssignmentOperator::Assignment),
        Token::AddAssign => Some(AssignmentOperator::AddAssign),
        Token::SubtractAssign => Some(AssignmentOperator::SubtractAssign),
        Token::MultiplyAssign => Some(AssignmentOperator::MultiplyAssign),
        Token::DivideAssign => Some(AssignmentOperator::DivideAssign),
        Token::ModuloAssign => Some(AssignmentOperator::ModuloAssign),
        Token::AndAssign => Some(AssignmentOperator::AndAssign),
        Token::OrAssign => Some(AssignmentOperator::OrAssign),
        Token::XorAssign => Some(AssignmentOperator::XorAssign),
        Token::LeftShiftAssign => Some(AssignmentOperator::LeftShiftAssign),
        Token::RightShiftAssign => Some(AssignmentOperator::RightShiftAssign),
        _ => None,
    }
}

pub fn statement_inverse_enum<T: IntoStatement>(statement: T) -> Option<Statement> {
    Some(statement.into_statement())
}
