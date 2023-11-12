use nom::{branch::alt, combinator::map_opt, IResult};

use crate::{
    ast::node::*,
    parser::{
        lexer::operator::*,
        utils::{token_to_binary_operator, token_to_unary_operator},
    },
};

pub fn parse_unary_operator(input: &str) -> IResult<&str, UnaryOperator> {
    map_opt(
        alt((
            parse_operator_logical_not,
            parse_operator_bitwise_not,
            parse_operator_increment,
            parse_operator_decrement,
        )),
        token_to_unary_operator,
    )(input)
}

pub fn parse_binary_operator(input: &str) -> IResult<&str, BinaryOperator> {
    map_opt(
        alt((
            parse_operator_plus,
            parse_operator_minus,
            parse_operator_multiply,
            parse_operator_divide,
            parse_operator_modulo,
            parse_operator_equal,
            parse_operator_not_equal,
            parse_operator_less_than,
            parse_operator_greater_than,
            parse_operator_less_than_or_equal,
            parse_operator_greater_than_or_equal,
            parse_operator_logical_and,
            parse_operator_logical_or,
            parse_operator_bitwise_and,
            parse_operator_bitwise_or,
            parse_operator_bitwise_xor,
            parse_operator_left_shift,
            parse_operator_right_shift,
        )),
        token_to_binary_operator,
    )(input)
}
