use nom::{character::complete::multispace0, combinator::map, IResult};

use crate::parser::tokenizer::token::Token;

pub fn parse_whitespace(input: &str) -> IResult<&str, Token> {
    map(multispace0, |_| Token::Whitespace)(input)
}
