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

#[cfg(test)]
mod tests {
    use crate::parser::lexer::comment::*;

    #[test]
    fn test_parse_comment_line() {
        let input = "// This is a single line comment\nNext line";
        let result = parse_comment_line(input);
        assert_eq!(result, Ok(("Next line", Token::Comment)));
    }

    #[test]
    fn test_parse_comment_block() {
        let input = "/* This is a\nmulti-line comment */Next part";
        let result = parse_comment_block(input);
        assert_eq!(result, Ok(("Next part", Token::Comment)));
    }
}
