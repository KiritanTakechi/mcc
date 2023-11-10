use nom::{bytes::complete::tag, combinator::map, IResult};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Increment,
    Decrement,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    LeftShift,
    RightShift,
    Assignment,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
}

fn parse_operator_plus(input: &str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Operator(Operator::Plus))(input)
}