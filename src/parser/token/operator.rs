use nom::{character::complete::char, combinator::map, sequence::tuple, IResult};

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
    map(char('+'), |_| Token::Operator(Operator::Plus))(input)
}

fn parse_operator_minus(input: &str) -> IResult<&str, Token> {
    map(char('-'), |_| Token::Operator(Operator::Minus))(input)
}

fn parse_operator_multiply(input: &str) -> IResult<&str, Token> {
    map(char('*'), |_| Token::Operator(Operator::Multiply))(input)
}

fn parse_operator_divide(input: &str) -> IResult<&str, Token> {
    map(char('/'), |_| Token::Operator(Operator::Divide))(input)
}

fn parse_operator_modulo(input: &str) -> IResult<&str, Token> {
    map(char('%'), |_| Token::Operator(Operator::Modulo))(input)
}
fn parse_operator_increment(input: &str) -> IResult<&str, Token> {
    map(tuple((char('+'), char('+'))), |_| {
        Token::Operator(Operator::Increment)
    })(input)
}

fn parse_operator_decrement(input: &str) -> IResult<&str, Token> {
    map(tuple((char('-'), char('-'))), |_| {
        Token::Operator(Operator::Increment)
    })(input)
}

fn parse_operator_equal(input: &str) -> IResult<&str, Token> {
    map(tuple((char('='), char('='))), |_| {
        Token::Operator(Operator::Equal)
    })(input)
}

fn parse_operator_not_equal(input: &str) -> IResult<&str, Token> {
    map(tuple((char('!'), char('='))), |_| {
        Token::Operator(Operator::NotEqual)
    })(input)
}

fn parse_operator_less_than(input: &str) -> IResult<&str, Token> {
    map(char('<'), |_| Token::Operator(Operator::LessThan))(input)
}

fn parse_operator_greater_than(input: &str) -> IResult<&str, Token> {
    map(char('>'), |_| Token::Operator(Operator::GreaterThan))(input)
}

fn parse_operator_less_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tuple((char('<'), char('='))), |_| {
        Token::Operator(Operator::LessThanOrEqual)
    })(input)
}

fn parse_operator_greater_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tuple((char('>'), char('='))), |_| {
        Token::Operator(Operator::GreaterThanOrEqual)
    })(input)
}

fn parse_operator_logical_and(input: &str) -> IResult<&str, Token> {
    map(tuple((char('&'), char('&'))), |_| {
        Token::Operator(Operator::LogicalAnd)
    })(input)
}

fn parse_operator_logical_or(input: &str) -> IResult<&str, Token> {
    map(tuple((char('|'), char('|'))), |_| {
        Token::Operator(Operator::LogicalOr)
    })(input)
}

fn parse_operator_logical_not(input: &str) -> IResult<&str, Token> {
    map(char('!'), |_| Token::Operator(Operator::LogicalNot))(input)
}

fn parse_operator_bitwise_and(input: &str) -> IResult<&str, Token> {
    map(char('&'), |_| Token::Operator(Operator::BitwiseAnd))(input)
}

fn parse_operator_bitwise_or(input: &str) -> IResult<&str, Token> {
    map(char('|'), |_| Token::Operator(Operator::BitwiseOr))(input)
}

fn parse_operator_bitwise_not(input: &str) -> IResult<&str, Token> {
    map(char('~'), |_| Token::Operator(Operator::BitwiseNot))(input)
}

fn parse_operator_bitwise_xor(input: &str) -> IResult<&str, Token> {
    map(char('^'), |_| Token::Operator(Operator::BitwiseXor))(input)
}

fn parse_operator_left_shift(input: &str) -> IResult<&str, Token> {
    map(tuple((char('<'), char('<'))), |_| {
        Token::Operator(Operator::LeftShift)
    })(input)
}

fn parse_operator_right_shift(input: &str) -> IResult<&str, Token> {
    map(tuple((char('>'), char('>'))), |_| {
        Token::Operator(Operator::RightShift)
    })(input)
}

fn parse_operator_assignment(input: &str) -> IResult<&str, Token> {
    map(char('='), |_| Token::Operator(Operator::Assignment))(input)
}
