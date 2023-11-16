use std::{iter::Peekable, slice::Iter};

use nom::IResult;

use crate::{
    ast::node::{TypeSpecifier, VariableDeclaration},
    parser::tokenizer::token::Token,
};

pub fn parse_variable_declaration(input: &str) -> IResult<&str, VariableDeclaration> {
    todo!()
}

fn parse_type_specifier(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<TypeSpecifier, String> {
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
