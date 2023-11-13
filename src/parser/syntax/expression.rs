use nom::{branch::alt, sequence::tuple, IResult};

use crate::{
    ast::node::Expression,
    parser::{
        lexer::{identifier::parse_identifier, operator::parse_operator_assignment},
        tokenizer::token::Token,
        utils::token_to_assignment_operator,
    },
};

use super::{literal::*, operator::*};

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_assignment_expression,
        parse_integer_literal,
        parse_unary_expression,
        parse_binary_expression,
    ))(input)
}

pub fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, operator) = parse_unary_operator(input)?;
    let (input, operand) = parse_expression(input)?;

    Ok((
        input,
        Expression::UnaryExpression(operator, Box::new(operand)),
    ))
}

pub fn parse_binary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, (left, operator, right)) =
        tuple((parse_expression, parse_binary_operator, parse_expression))(input)?;

    Ok((
        input,
        Expression::BinaryExpression(operator, Box::new(left), Box::new(right)),
    ))
}
pub fn parse_assignment_expression(input: &str) -> IResult<&str, Expression> {
    let (input, (identifier_token, operator_token, expr)) = tuple((
        parse_identifier,
        parse_operator_assignment,
        parse_expression,
    ))(input)?;

    let identifier = match identifier_token {
        Token::Identifier(s) => s,
        _ => panic!("Expected an identifier"),
    };

    let operator = match token_to_assignment_operator(operator_token) {
        Some(s) => s,
        _ => panic!("Expected an identifier"),
    };

    Ok((
        input,
        Expression::AssignmentExpression {
            identifier,
            operator,
            value: Box::new(expr),
        },
    ))
}
