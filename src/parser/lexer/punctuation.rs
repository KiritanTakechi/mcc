use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::map, IResult};

use crate::parser::tokenizer::token::Token;

pub fn parse_comma(input: &str) -> IResult<&str, Token> {
    map(char(','), |_| Token::Comma)(input)
}

pub fn parse_semicolon(input: &str) -> IResult<&str, Token> {
    map(char(';'), |_| Token::SemiColon)(input)
}

pub fn parse_open_paren(input: &str) -> IResult<&str, Token> {
    map(char('('), |_| Token::OpenParen)(input)
}

pub fn parse_close_paren(input: &str) -> IResult<&str, Token> {
    map(char(')'), |_| Token::CloseParen)(input)
}

pub fn parse_open_brace(input: &str) -> IResult<&str, Token> {
    map(char('{'), |_| Token::OpenBrace)(input)
}

pub fn parse_close_brace(input: &str) -> IResult<&str, Token> {
    map(char('}'), |_| Token::CloseBrace)(input)
}

pub fn parse_open_bracket(input: &str) -> IResult<&str, Token> {
    map(char('['), |_| Token::OpenBracket)(input)
}

pub fn parse_close_bracket(input: &str) -> IResult<&str, Token> {
    map(char(']'), |_| Token::CloseBracket)(input)
}

pub fn parse_dot(input: &str) -> IResult<&str, Token> {
    map(char('.'), |_| Token::Dot)(input)
}

pub fn parse_arrow(input: &str) -> IResult<&str, Token> {
    map(tag("->"), |_| Token::Arrow)(input)
}

pub fn parse_ellipsis(input: &str) -> IResult<&str, Token> {
    map(tag("..."), |_| Token::Ellipsis)(input)
}

pub fn parse_punctuation(input: &str) -> IResult<&str, Token> {
    alt((
        parse_comma,
        parse_semicolon,
        parse_open_paren,
        parse_close_paren,
        parse_open_brace,
        parse_close_brace,
        parse_open_bracket,
        parse_close_bracket,
        parse_dot,
        parse_arrow,
        parse_ellipsis,
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::punctuation::*;

    #[test]
    fn test_parse_comma() {
        let result = parse_comma(",");
        assert_eq!(result, Ok(("", Token::Comma)));
    }

    #[test]
    fn test_parse_semicolon() {
        let result = parse_semicolon(";");
        assert_eq!(result, Ok(("", Token::SemiColon)));
    }

    #[test]
    fn test_parse_open_paren() {
        let result = parse_open_paren("(");
        assert_eq!(result, Ok(("", Token::OpenParen)));
    }

    #[test]
    fn test_parse_close_paren() {
        let result = parse_close_paren(")");
        assert_eq!(result, Ok(("", Token::CloseParen)));
    }

    #[test]
    fn test_parse_open_brace() {
        let result = parse_open_brace("{");
        assert_eq!(result, Ok(("", Token::OpenBrace)));
    }

    #[test]
    fn test_parse_close_brace() {
        let result = parse_close_brace("}");
        assert_eq!(result, Ok(("", Token::CloseBrace)));
    }

    #[test]
    fn test_parse_open_bracket() {
        let result = parse_open_bracket("[");
        assert_eq!(result, Ok(("", Token::OpenBracket)));
    }

    #[test]
    fn test_parse_close_bracket() {
        let result = parse_close_bracket("]");
        assert_eq!(result, Ok(("", Token::CloseBracket)));
    }

    #[test]
    fn test_parse_dot() {
        let result = parse_dot(".");
        assert_eq!(result, Ok(("", Token::Dot)));
    }

    #[test]
    fn test_parse_arrow() {
        let result = parse_arrow("->");
        assert_eq!(result, Ok(("", Token::Arrow)));
    }

    #[test]
    fn test_parse_ellipsis() {
        let result = parse_ellipsis("...");
        assert_eq!(result, Ok(("", Token::Ellipsis)));
    }
}
