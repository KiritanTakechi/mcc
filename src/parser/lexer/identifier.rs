use nom::{
    character::complete::satisfy,
    combinator::{map, recognize},
    multi::many0,
    sequence::pair,
    IResult,
};

use crate::parser::tokenizer::token::Token;

pub fn parse_identifier(input: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            satisfy(|c| c.is_alphabetic() || c == '_'),
            many0(satisfy(|c| c.is_alphanumeric() || c == '_')),
        )),
        |s: &str| Token::Identifier(s.to_string()),
    )(input)
}
