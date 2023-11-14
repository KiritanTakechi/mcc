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
