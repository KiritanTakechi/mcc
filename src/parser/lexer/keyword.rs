use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::parser::tokenizer::token::Token;

pub fn parse_keyword_void(input: &str) -> IResult<&str, Token> {
    map(tag("void"), |_| Token::Void)(input)
}

pub fn parse_keyword_int(input: &str) -> IResult<&str, Token> {
    map(tag("int"), |_| Token::Int)(input)
}

pub fn parse_keyword_long(input: &str) -> IResult<&str, Token> {
    map(tag("long"), |_| Token::Long)(input)
}

pub fn parse_keyword_float(input: &str) -> IResult<&str, Token> {
    map(tag("float"), |_| Token::Float)(input)
}

pub fn parse_keyword_double(input: &str) -> IResult<&str, Token> {
    map(tag("double"), |_| Token::Double)(input)
}

pub fn parse_keyword_char(input: &str) -> IResult<&str, Token> {
    map(tag("char"), |_| Token::Char)(input)
}

pub fn parse_keyword_struct(input: &str) -> IResult<&str, Token> {
    map(tag("struct"), |_| Token::Struct)(input)
}

pub fn parse_keyword_union(input: &str) -> IResult<&str, Token> {
    map(tag("union"), |_| Token::Union)(input)
}

pub fn parse_keyword_if(input: &str) -> IResult<&str, Token> {
    map(tag("if"), |_| Token::If)(input)
}

pub fn parse_keyword_else(input: &str) -> IResult<&str, Token> {
    map(tag("else"), |_| Token::Else)(input)
}
pub fn parse_keyword_while(input: &str) -> IResult<&str, Token> {
    map(tag("while"), |_| Token::While)(input)
}

pub fn parse_keyword_for(input: &str) -> IResult<&str, Token> {
    map(tag("for"), |_| Token::For)(input)
}

pub fn parse_keyword_return(input: &str) -> IResult<&str, Token> {
    map(tag("return"), |_| Token::Return)(input)
}

pub fn parse_keyword(input: &str) -> IResult<&str, Token> {
    alt((
        parse_keyword_void,
        parse_keyword_int,
        parse_keyword_long,
        parse_keyword_float,
        parse_keyword_double,
        parse_keyword_char,
        parse_keyword_struct,
        parse_keyword_union,
        parse_keyword_if,
        parse_keyword_else,
        parse_keyword_while,
        parse_keyword_for,
        parse_keyword_return,
    ))(input)
}
