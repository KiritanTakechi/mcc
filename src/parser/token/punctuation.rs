use nom::{bytes::complete::tag, character::complete::char, combinator::map, IResult};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Comma,
    SemiColon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Dot,
    Arrow,
    Ellipsis,
}

fn parse_comma(input: &str) -> IResult<&str, Token> {
    map(char(','), |_| Token::Punctuation(Punctuation::Comma))(input)
}

fn parse_semi_colon(input: &str) -> IResult<&str, Token> {
    map(char(';'), |_| Token::Punctuation(Punctuation::SemiColon))(input)
}

fn parse_open_paren(input: &str) -> IResult<&str, Token> {
    map(char('('), |_| Token::Punctuation(Punctuation::OpenParen))(input)
}

fn parse_close_paren(input: &str) -> IResult<&str, Token> {
    map(char(')'), |_| Token::Punctuation(Punctuation::CloseParen))(input)
}

fn parse_open_brace(input: &str) -> IResult<&str, Token> {
    map(char('{'), |_| Token::Punctuation(Punctuation::OpenBrace))(input)
}

fn parse_close_brace(input: &str) -> IResult<&str, Token> {
    map(char('}'), |_| Token::Punctuation(Punctuation::CloseBrace))(input)
}

fn parse_open_bracket(input: &str) -> IResult<&str, Token> {
    map(char('['), |_| Token::Punctuation(Punctuation::OpenBracket))(input)
}

fn parse_close_bracket(input: &str) -> IResult<&str, Token> {
    map(char(']'), |_| Token::Punctuation(Punctuation::CloseBracket))(input)
}

fn parse_dot(input: &str) -> IResult<&str, Token> {
    map(char('.'), |_| Token::Punctuation(Punctuation::Dot))(input)
}

fn parse_arrow(input: &str) -> IResult<&str, Token> {
    map(tag("->"), |_| Token::Punctuation(Punctuation::Arrow))(input)
}

fn parse_ellipsis(input: &str) -> IResult<&str, Token> {
    map(tag("..."), |_| Token::Punctuation(Punctuation::Ellipsis))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comma() {
        let result = parse_comma(",");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::Comma))));
    }

    #[test]
    fn test_parse_semi_colon() {
        let result = parse_semi_colon(";");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::SemiColon))));
    }

    #[test]
    fn test_parse_open_paren() {
        let result = parse_open_paren("(");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::OpenParen))));
    }

    #[test]
    fn test_parse_close_paren() {
        let result = parse_close_paren(")");
        assert_eq!(
            result,
            Ok(("", Token::Punctuation(Punctuation::CloseParen)))
        );
    }

    #[test]
    fn test_parse_open_brace() {
        let result = parse_open_brace("{");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::OpenBrace))));
    }

    #[test]
    fn test_parse_close_brace() {
        let result = parse_close_brace("}");
        assert_eq!(
            result,
            Ok(("", Token::Punctuation(Punctuation::CloseBrace)))
        );
    }

    #[test]
    fn test_parse_open_bracket() {
        let result = parse_open_bracket("[");
        assert_eq!(
            result,
            Ok(("", Token::Punctuation(Punctuation::OpenBracket)))
        );
    }

    #[test]
    fn test_parse_close_bracket() {
        let result = parse_close_bracket("]");
        assert_eq!(
            result,
            Ok(("", Token::Punctuation(Punctuation::CloseBracket)))
        );
    }

    #[test]
    fn test_parse_dot() {
        let result = parse_dot(".");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::Dot))));
    }

    #[test]
    fn test_parse_arrow() {
        let result = parse_arrow("->");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::Arrow))));
    }

    #[test]
    fn test_parse_ellipsis() {
        let result = parse_ellipsis("...");
        assert_eq!(result, Ok(("", Token::Punctuation(Punctuation::Ellipsis))));
    }
}
