use nom::{branch::alt, sequence::tuple, IResult};

use crate::ast::node::Expression;

use super::{literal::*, operator::*};

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_integer_literal,
        parse_floating_literal,
        parse_unary_expression,
        parse_binary_expression,
    ))(input)
}

fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, operator) = parse_unary_operator(input)?;
    let (input, operand) = parse_expression(input)?;
    Ok((
        input,
        Expression::UnaryExpression(operator, Box::new(operand)),
    ))
}

fn parse_binary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, (left, operator, right)) =
        tuple((parse_expression, parse_binary_operator, parse_expression))(input)?;
    Ok((
        input,
        Expression::BinaryExpression(operator, Box::new(left), Box::new(right)),
    ))
}
