use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::char,
    combinator::{map, recognize},
    sequence::{delimited, preceded, terminated},
    IResult,
};

use crate::parser::tokenizer::token::Token;

pub fn parse_comment_line(input: &str) -> IResult<&str, Token> {
    map(
        terminated(
            recognize(preceded(tag("//"), take_while(|c| c != '\n'))),
            char('\n'),
        ),
        |s: &str| Token::CommentLine(s.to_string()),
    )(input)
}

pub fn parse_comment_block(input: &str) -> IResult<&str, Token> {
    map(
        delimited(tag("/*"), take_until("*/"), tag("*/")),
        |s: &str| Token::CommentBlock(s.to_string()),
    )(input)
}
