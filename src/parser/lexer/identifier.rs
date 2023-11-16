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

#[cfg(test)]
mod tests {
    use crate::parser::lexer::identifier::*;

    #[test]
    fn test_parse_identifier() {
        let result = parse_identifier("myVariable123");
        assert_eq!(
            result,
            Ok(("", Token::Identifier("myVariable123".to_string())))
        );
    }

    #[test]
    fn test_parse_identifier_with_underscore() {
        let result = parse_identifier("_myVariable_123");
        assert_eq!(
            result,
            Ok(("", Token::Identifier("_myVariable_123".to_string())))
        );
    }

    #[test]
    fn test_parse_identifier_all_underscore() {
        let result = parse_identifier("___");
        assert_eq!(result, Ok(("", Token::Identifier("___".to_string()))));
    }

    #[test]
    fn test_parse_identifier_starts_with_underscore() {
        let result = parse_identifier("_var");
        assert_eq!(result, Ok(("", Token::Identifier("_var".to_string()))));
    }

    #[test]
    fn test_parse_identifier_starts_with_letter() {
        let result = parse_identifier("var123");
        assert_eq!(result, Ok(("", Token::Identifier("var123".to_string()))));
    }
}
