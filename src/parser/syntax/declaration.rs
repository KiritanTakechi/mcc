use nom::{
    branch::alt,
    character::complete::{space0, space1},
    combinator::map_opt,
    sequence::tuple,
    IResult,
};

use crate::{
    ast::node::{TypeSpecifier, VariableDeclaration},
    parser::{
        lexer::{identifier::parse_identifier, keyword::*, punctuation::parse_semicolon},
        tokenizer::token::Token,
        utils::token_to_type_specifier,
    },
};

pub fn parse_variable_declaration(input: &str) -> IResult<&str, VariableDeclaration> {
    let (input, (type_specifier, _, identifier_token, _, _)) = tuple((
        parse_type_specifier,
        space1,
        parse_identifier,
        space0,
        parse_semicolon,
    ))(input)?;

    let identifier = match identifier_token {
        Token::Identifier(s) => s,
        _ => panic!("Expected an identifier"),
    };

    Ok((
        input,
        VariableDeclaration {
            type_specifier,
            identifier,
        },
    ))
}

pub fn parse_type_specifier(input: &str) -> IResult<&str, TypeSpecifier> {
    map_opt(
        alt((
            parse_keyword_void,
            parse_keyword_int,
            parse_keyword_long,
            parse_keyword_float,
            parse_keyword_double,
            parse_keyword_char,
            parse_keyword_union,
            parse_keyword_struct,
        )),
        token_to_type_specifier,
    )(input)
}
