use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res},
    number::complete::double,
    sequence::delimited,
    IResult,
};

use crate::parser::tokenizer::token::Token;

pub fn parse_integer_literal(input: &str) -> IResult<&str, Token> {
    map_res(digit1, |s: &str| {
        s.parse::<i64>().map(|n| Token::IntegerLiteral(n))
    })(input)
}

pub fn parse_floating_literal(input: &str) -> IResult<&str, Token> {
    map(double, |f| (Token::FloatingLiteral(f)))(input)
}

pub fn parse_char_literal(input: &str) -> IResult<&str, Token> {
    map(
        delimited(
            char('\''),
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
            char('\''),
        ),
        |c: char| Token::CharLiteral(c),
    )(input)
}

pub fn parse_string_literal(input: &str) -> IResult<&str, Token> {
    map(
        delimited(tag("\""), take_while(|c: char| c != '\"'), tag("\"")),
        |s: &str| Token::StringLiteral(s.to_string()),
    )(input)
}
