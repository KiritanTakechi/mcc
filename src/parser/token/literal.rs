use nom::{bytes::complete::tag, combinator::map, IResult};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Literal {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),
}
