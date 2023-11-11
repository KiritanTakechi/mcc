use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res},
    number::complete::double,
    sequence::delimited,
    IResult,
};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Literal {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),
}

fn parse_integer_literal(input: &str) -> IResult<&str, Token> {
    map_res(digit1, |s: &str| {
        s.parse::<i64>()
            .map(|n| Token::Literal(Literal::IntegerLiteral(n)))
    })(input)
}

fn parse_floating_literal(input: &str) -> IResult<&str, Token> {
    map(double, |f| (Token::Literal(Literal::FloatingLiteral(f))))(input)
}

fn parse_char_literal(input: &str) -> IResult<&str, Token> {
    map(
        delimited(
            char('\''),
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
            char('\''),
        ),
        |c: char| Token::Literal(Literal::CharLiteral(c)),
    )(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, Token> {
    map(
        delimited(tag("\""), take_while(|c: char| c != '\"'), tag("\"")),
        |s: &str| Token::Literal(Literal::StringLiteral(s.to_string())),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_literal() {
        let result = parse_integer_literal("1234");
        assert_eq!(
            result,
            Ok(("", Token::Literal(Literal::IntegerLiteral(1234))))
        );
    }

    #[test]
    fn test_parse_floating_literal() {
        let result = parse_floating_literal("1234.56");
        assert_eq!(
            result,
            Ok(("", Token::Literal(Literal::FloatingLiteral(1234.56))))
        );
    }

    #[test]
    fn test_parse_char_literal() {
        let result = parse_char_literal("'a'");
        assert_eq!(result, Ok(("", Token::Literal(Literal::CharLiteral('a')))));
    }

    #[test]
    fn test_parse_string_literal() {
        let result = parse_string_literal("\"hello\"");
        assert_eq!(
            result,
            Ok((
                "",
                Token::Literal(Literal::StringLiteral("hello".to_string()))
            ))
        );
    }
}
