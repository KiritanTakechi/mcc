use std::iter::Peekable;

use crate::{
    ast::node::{TypeSpecifier, VariableDeclaration},
    parser::tokenizer::token::Token,
};

use super::identifier::parse_identifier;

pub fn parse_variable_declaration<'a, I>(
    tokens_iter: &mut Peekable<I>,
) -> Result<VariableDeclaration, String>
where
    I: Iterator<Item = &'a Token>,
{
    let type_specifier = parse_type_specifier(tokens_iter)?;
    let identifier = parse_identifier(tokens_iter)?;

    if tokens_iter.next() != Some(&Token::SemiColon) {
        return Err("Expected semicolon".to_string());
    }

    Ok(VariableDeclaration {
        type_specifier,
        identifier,
    })
}

fn parse_type_specifier<'a, I>(tokens_iter: &mut Peekable<I>) -> Result<TypeSpecifier, String>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens_iter.next() {
        Some(Token::Void) => Ok(TypeSpecifier::Void),
        Some(Token::Int) => Ok(TypeSpecifier::Int),
        Some(Token::Long) => Ok(TypeSpecifier::Long),
        Some(Token::Float) => Ok(TypeSpecifier::Float),
        Some(Token::Double) => Ok(TypeSpecifier::Double),
        Some(Token::Char) => Ok(TypeSpecifier::Char),
        Some(Token::Struct) => Ok(TypeSpecifier::Struct),
        Some(Token::Union) => Ok(TypeSpecifier::Union),
        _ => Err("Expected a type specifier".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::syntax::declaration::*;

    #[test]
    fn test_parse_variable_declaration() {
        let input = vec![
            Token::Int,
            Token::Identifier("x".to_string()),
            Token::SemiColon,
            Token::Identifier("x".to_string()),
        ];
        let mut input_iter = input.iter().peekable();
        let result = parse_variable_declaration(&mut input_iter);
        let expected = VariableDeclaration {
            type_specifier: TypeSpecifier::Int,
            identifier: "x".to_string(),
        };
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_type_specifier() {
        let input = vec![Token::Int, Token::Identifier("x".to_string())];
        let mut input_iter = input.iter().peekable();
        let result = parse_type_specifier(&mut input_iter);
        let expected = TypeSpecifier::Int;
        assert_eq!(result, Ok(expected));
    }
}
