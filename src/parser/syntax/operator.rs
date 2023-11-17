use std::iter::Peekable;

use crate::{ast::node::*, parser::tokenizer::token::Token};

pub fn parse_unary_operator<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<UnaryOperator, String>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens_iter.next() {
        Some(Token::LogicalNot) => Ok(UnaryOperator::LogicalNot),
        Some(Token::BitwiseNot) => Ok(UnaryOperator::BitwiseNot),
        Some(Token::Increment) => Ok(UnaryOperator::Increment),
        Some(Token::Decrement) => Ok(UnaryOperator::Decrement),
        _ => Err("Expected a unary operator".to_string()),
    }
}

pub fn parse_binary_operator<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<BinaryOperator, String>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens_iter.next() {
        Some(Token::Plus) => Ok(BinaryOperator::Plus),
        Some(Token::Minus) => Ok(BinaryOperator::Minus),
        Some(Token::Multiply) => Ok(BinaryOperator::Multiply),
        Some(Token::Divide) => Ok(BinaryOperator::Divide),
        Some(Token::Modulo) => Ok(BinaryOperator::Modulo),
        Some(Token::Equal) => Ok(BinaryOperator::Equal),
        Some(Token::NotEqual) => Ok(BinaryOperator::NotEqual),
        Some(Token::LessThan) => Ok(BinaryOperator::LessThan),
        Some(Token::GreaterThan) => Ok(BinaryOperator::GreaterThan),
        Some(Token::LessThanOrEqual) => Ok(BinaryOperator::LessThanOrEqual),
        Some(Token::GreaterThanOrEqual) => Ok(BinaryOperator::GreaterThanOrEqual),
        Some(Token::LogicalAnd) => Ok(BinaryOperator::LogicalAnd),
        Some(Token::LogicalOr) => Ok(BinaryOperator::LogicalOr),
        Some(Token::BitwiseAnd) => Ok(BinaryOperator::BitwiseAnd),
        Some(Token::BitwiseOr) => Ok(BinaryOperator::BitwiseOr),
        Some(Token::BitwiseXor) => Ok(BinaryOperator::BitwiseXor),
        Some(Token::LeftShift) => Ok(BinaryOperator::LeftShift),
        Some(Token::RightShift) => Ok(BinaryOperator::RightShift),
        _ => Err("Expected a binary operator".to_string()),
    }
}

pub fn parse_assignment_operator<'a, I>(
    tokens_iter: &mut Peekable<I>,
) -> Result<AssignmentOperator, String>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens_iter.next() {
        Some(Token::Assignment) => Ok(AssignmentOperator::Assignment),
        Some(Token::AddAssign) => Ok(AssignmentOperator::AddAssign),
        Some(Token::SubtractAssign) => Ok(AssignmentOperator::SubtractAssign),
        Some(Token::MultiplyAssign) => Ok(AssignmentOperator::MultiplyAssign),
        Some(Token::DivideAssign) => Ok(AssignmentOperator::DivideAssign),
        Some(Token::ModuloAssign) => Ok(AssignmentOperator::ModuloAssign),
        Some(Token::AndAssign) => Ok(AssignmentOperator::AndAssign),
        Some(Token::OrAssign) => Ok(AssignmentOperator::OrAssign),
        Some(Token::XorAssign) => Ok(AssignmentOperator::XorAssign),
        Some(Token::LeftShiftAssign) => Ok(AssignmentOperator::LeftShiftAssign),
        Some(Token::RightShiftAssign) => Ok(AssignmentOperator::RightShiftAssign),
        _ => Err("Expected a assignment operator".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{syntax::operator::*, tokenizer::token::Token};

    #[test]
    fn test_parse_unary_operator() {
        let input = vec![Token::LogicalNot];
        let mut input_iter = input.iter().peekable();
        let result = parse_unary_operator(&mut input_iter);
        let expected = UnaryOperator::LogicalNot;
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_binary_operator() {
        let input = vec![Token::LessThanOrEqual];
        let mut input_iter = input.iter().peekable();
        let result = parse_binary_operator(&mut input_iter);
        let expected = BinaryOperator::LessThanOrEqual;
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_assignment_operator() {
        let input = vec![Token::ModuloAssign];
        let mut input_iter = input.iter().peekable();
        let result = parse_assignment_operator(&mut input_iter);
        let expected = AssignmentOperator::ModuloAssign;
        assert_eq!(result, Ok(expected));
    }
}
