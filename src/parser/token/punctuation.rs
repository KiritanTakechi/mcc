use nom::{bytes::complete::tag, combinator::map, IResult};

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