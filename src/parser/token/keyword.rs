use nom::{bytes::complete::tag, combinator::map, IResult};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Long,
    Float,
    Double,
    Char,
    Struct,
    Union,
    If,
    Else,
    While,
    For,
    Return,
}

fn parse_keyword_int(input: &str) -> IResult<&str, Token> {
    map(tag("int"), |_| Token::Keyword(Keyword::Int))(input)
}

fn parse_keyword_long(input: &str) -> IResult<&str, Token> {
    map(tag("long"), |_| Token::Keyword(Keyword::Long))(input)
}

fn parse_keyword_float(input: &str) -> IResult<&str, Token> {
    map(tag("float"), |_| Token::Keyword(Keyword::Float))(input)
}

fn parse_keyword_double(input: &str) -> IResult<&str, Token> {
    map(tag("double"), |_| Token::Keyword(Keyword::Double))(input)
}

fn parse_keyword_char(input: &str) -> IResult<&str, Token> {
    map(tag("char"), |_| Token::Keyword(Keyword::Char))(input)
}

fn parse_keyword_struct(input: &str) -> IResult<&str, Token> {
    map(tag("struct"), |_| Token::Keyword(Keyword::Struct))(input)
}

fn parse_keyword_union(input: &str) -> IResult<&str, Token> {
    map(tag("union"), |_| Token::Keyword(Keyword::Union))(input)
}

fn parse_keyword_if(input: &str) -> IResult<&str, Token> {
    map(tag("if"), |_| Token::Keyword(Keyword::If))(input)
}

fn parse_keyword_else(input: &str) -> IResult<&str, Token> {
    map(tag("else"), |_| Token::Keyword(Keyword::Else))(input)
}
fn parse_keyword_while(input: &str) -> IResult<&str, Token> {
    map(tag("while"), |_| Token::Keyword(Keyword::While))(input)
}

fn parse_keyword_for(input: &str) -> IResult<&str, Token> {
    map(tag("for"), |_| Token::Keyword(Keyword::For))(input)
}

fn parse_keyword_return(input: &str) -> IResult<&str, Token> {
    map(tag("return"), |_| Token::Keyword(Keyword::Return))(input)
}