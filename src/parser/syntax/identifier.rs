use std::iter::Peekable;

use crate::parser::tokenizer::token::Token;

pub fn parse_identifier<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<String, String>
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(Token::Identifier(name)) = tokens_iter.next() {
        Ok(name.to_owned())
    } else {
        Err("Expected an identifier".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{syntax::identifier::*, tokenizer::token::Token};

    #[test]
    fn test_parse_identifier() {
        let input = vec![Token::Identifier("x".to_string())];
        let mut input_iter = input.iter().peekable();
        let result = parse_identifier(&mut input_iter);
        let expected = "x".to_string();
        assert_eq!(result, Ok(expected));
    }
}
