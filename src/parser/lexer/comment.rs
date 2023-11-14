use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, map, opt},
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::parser::tokenizer::token::Token;

pub fn parse_comment_line(input: &str) -> IResult<&str, Token> {
    preceded(
        tag("//"),
        map(
            terminated(not_line_ending, opt(alt((line_ending, eof)))),
            |_| Token::Comment,
        ),
    )(input)
}

pub fn parse_comment_block(input: &str) -> IResult<&str, Token> {
    map(delimited(tag("/*"), take_until("*/"), tag("*/")), |_| {
        Token::Comment
    })(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, Token> {
    alt((parse_comment_line, parse_comment_block))(input)
}
