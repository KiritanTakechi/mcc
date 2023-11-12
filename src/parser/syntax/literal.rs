use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res},
    number::complete::double,
    sequence::delimited,
    IResult,
};

use crate::ast::node::Expression;

pub fn parse_integer_literal(input: &str) -> IResult<&str, Expression> {
    map_res(digit1, |s: &str| {
        s.parse::<i64>().map(|n| Expression::IntegerLiteral(n))
    })(input)
}

pub fn parse_floating_literal(input: &str) -> IResult<&str, Expression> {
    map(double, |f| (Expression::FloatingLiteral(f)))(input)
}

pub fn parse_char_literal(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            char('\''),
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
            char('\''),
        ),
        |c: char| Expression::CharLiteral(c),
    )(input)
}

pub fn parse_string_literal(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(tag("\""), take_while(|c: char| c != '\"'), tag("\"")),
        |s: &str| Expression::StringLiteral(s.to_string()),
    )(input)
}
