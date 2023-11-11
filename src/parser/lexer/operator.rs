use nom::{bytes::complete::tag, character::complete::char, combinator::map, IResult};

use crate::parser::tokenizer::token::Token;

pub fn parse_operator_plus(input: &str) -> IResult<&str, Token> {
    map(char('+'), |_| Token::Plus)(input)
}

pub fn parse_operator_minus(input: &str) -> IResult<&str, Token> {
    map(char('-'), |_| Token::Minus)(input)
}

pub fn parse_operator_multiply(input: &str) -> IResult<&str, Token> {
    map(char('*'), |_| Token::Multiply)(input)
}

pub fn parse_operator_divide(input: &str) -> IResult<&str, Token> {
    map(char('/'), |_| Token::Divide)(input)
}

pub fn parse_operator_modulo(input: &str) -> IResult<&str, Token> {
    map(char('%'), |_| Token::Modulo)(input)
}

pub fn parse_operator_increment(input: &str) -> IResult<&str, Token> {
    map(tag("++"), |_| Token::Increment)(input)
}

pub fn parse_operator_decrement(input: &str) -> IResult<&str, Token> {
    map(tag("--"), |_| Token::Decrement)(input)
}

pub fn parse_operator_equal(input: &str) -> IResult<&str, Token> {
    map(tag("=="), |_| Token::Equal)(input)
}

pub fn parse_operator_not_equal(input: &str) -> IResult<&str, Token> {
    map(tag("!="), |_| Token::NotEqual)(input)
}

pub fn parse_operator_less_than(input: &str) -> IResult<&str, Token> {
    map(char('<'), |_| Token::LessThan)(input)
}

pub fn parse_operator_greater_than(input: &str) -> IResult<&str, Token> {
    map(char('>'), |_| Token::GreaterThan)(input)
}

pub fn parse_operator_less_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tag("<="), |_| Token::LessThanOrEqual)(input)
}

pub fn parse_operator_greater_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tag(">="), |_| Token::GreaterThanOrEqual)(input)
}

pub fn parse_operator_logical_and(input: &str) -> IResult<&str, Token> {
    map(tag("&&"), |_| Token::LogicalAnd)(input)
}

pub fn parse_operator_logical_or(input: &str) -> IResult<&str, Token> {
    map(tag("||"), |_| Token::LogicalOr)(input)
}

pub fn parse_operator_logical_not(input: &str) -> IResult<&str, Token> {
    map(char('!'), |_| Token::LogicalNot)(input)
}

pub fn parse_operator_bitwise_and(input: &str) -> IResult<&str, Token> {
    map(char('&'), |_| Token::BitwiseAnd)(input)
}

pub fn parse_operator_bitwise_or(input: &str) -> IResult<&str, Token> {
    map(char('|'), |_| Token::BitwiseOr)(input)
}

pub fn parse_operator_bitwise_not(input: &str) -> IResult<&str, Token> {
    map(char('~'), |_| Token::BitwiseNot)(input)
}

pub fn parse_operator_bitwise_xor(input: &str) -> IResult<&str, Token> {
    map(char('^'), |_| Token::BitwiseXor)(input)
}

pub fn parse_operator_left_shift(input: &str) -> IResult<&str, Token> {
    map(tag("<<"), |_| Token::LeftShift)(input)
}

pub fn parse_operator_right_shift(input: &str) -> IResult<&str, Token> {
    map(tag(">>"), |_| Token::RightShift)(input)
}

pub fn parse_operator_assignment(input: &str) -> IResult<&str, Token> {
    map(char('='), |_| Token::Assignment)(input)
}

pub fn parse_operator_add_assign(input: &str) -> IResult<&str, Token> {
    map(tag("+="), |_| Token::AddAssign)(input)
}

pub fn parse_operator_subtract_assign(input: &str) -> IResult<&str, Token> {
    map(tag("-="), |_| Token::SubtractAssign)(input)
}

pub fn parse_operator_multiply_assign(input: &str) -> IResult<&str, Token> {
    map(tag("*="), |_| Token::MultiplyAssign)(input)
}

pub fn parse_operator_divide_assign(input: &str) -> IResult<&str, Token> {
    map(tag("/="), |_| Token::DivideAssign)(input)
}

pub fn parse_operator_modulo_assign(input: &str) -> IResult<&str, Token> {
    map(tag("%="), |_| Token::ModuloAssign)(input)
}

pub fn parse_operator_and_assign(input: &str) -> IResult<&str, Token> {
    map(tag("&="), |_| Token::AndAssign)(input)
}

pub fn parse_operator_or_assign(input: &str) -> IResult<&str, Token> {
    map(tag("~="), |_| Token::OrAssign)(input)
}

pub fn parse_operator_xor_assign(input: &str) -> IResult<&str, Token> {
    map(tag("^="), |_| Token::XorAssign)(input)
}

pub fn parse_operator_left_shift_assign(input: &str) -> IResult<&str, Token> {
    map(tag("<<="), |_| Token::LeftShiftAssign)(input)
}

pub fn parse_operator_right_shift_assign(input: &str) -> IResult<&str, Token> {
    map(tag(">>="), |_| Token::RightShiftAssign)(input)
}
