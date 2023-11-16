use nom::{
    branch::alt,
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

pub fn parse_literal(input: &str) -> IResult<&str, Token> {
    alt((
        parse_integer_literal,
        parse_floating_literal,
        parse_char_literal,
        parse_string_literal,
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::literal::*;

    #[test]
    fn test_parse_integer_literal() {
        let result = parse_integer_literal("12345");
        assert_eq!(result, Ok(("", Token::IntegerLiteral(12345))));
    }

    #[test]
    fn test_parse_floating_literal() {
        let result = parse_floating_literal("123.45");
        assert_eq!(result, Ok(("", Token::FloatingLiteral(123.45))));
    }

    #[test]
    fn test_parse_char_literal() {
        let result = parse_char_literal("'a'");
        assert_eq!(result, Ok(("", Token::CharLiteral('a'))));
    }

    #[test]
    fn test_parse_string_literal() {
        let result = parse_string_literal("\"Hello, World!\"");
        assert_eq!(
            result,
            Ok(("", Token::StringLiteral("Hello, World!".to_string())))
        );
    }
}
