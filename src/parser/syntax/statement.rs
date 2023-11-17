use nom::{
    branch::alt,
    combinator::{map_opt, opt},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{
    ast::node::*,
    parser::{
        lexer::{keyword::*, punctuation::*},
        utils::statement_inverse_enum,
    },
};

use super::expression::parse_expression;

// fn parse_statement(input: &str) -> IResult<&str, Statement> {
//     alt((
//         map_opt(parse_expression_statement, statement_inverse_enum),
//         map_opt(parse_compound_statement, statement_inverse_enum),
//         map_opt(parse_selection_statement, statement_inverse_enum),
//         map_opt(parse_iteration_statement, statement_inverse_enum),
//         map_opt(parse_return_statement, statement_inverse_enum),
//     ))(input)
// }

// pub fn parse_expression_statement(input: &str) -> IResult<&str, Expression> {
//     let (input, expr) = parse_expression(input)?;
//     let (input, _) = parse_semicolon(input)?;

//     Ok((input, expr))
// }

// pub fn parse_compound_statement(input: &str) -> IResult<&str, CompoundStatement> {
//     let (input, statements) =
//         delimited(parse_open_brace, many0(parse_statement), parse_close_brace)(input)?;

//     Ok((input, CompoundStatement { statements }))
// }

// pub fn parse_selection_statement(input: &str) -> IResult<&str, SelectionStatement> {
//     let (input, _) = parse_keyword_if(input)?;
//     let (input, _) = parse_open_paren(input)?;
//     let (input, condition) = parse_expression(input)?;
//     let (input, _) = parse_close_paren(input)?;
//     let (input, then_branch) = parse_statement(input)?;
//     let (input, else_branch) = opt(preceded(parse_keyword_else, parse_statement))(input)?;

//     Ok((
//         input,
//         SelectionStatement {
//             condition,
//             then_branch: Box::new(then_branch),
//             else_branch: else_branch.map(Box::new),
//         },
//     ))
// }

// pub fn parse_iteration_statement(input: &str) -> IResult<&str, IterationStatement> {
//     let (input, _) = parse_keyword_while(input)?;
//     let (input, _) = parse_open_paren(input)?;
//     let (input, condition) = parse_expression(input)?;
//     let (input, _) = parse_close_paren(input)?;
//     let (input, body) = parse_statement(input)?;

//     Ok((
//         input,
//         IterationStatement {
//             condition,
//             body: Box::new(body),
//         },
//     ))
// }

// pub fn parse_return_statement(input: &str) -> IResult<&str, ReturnStatement> {
//     let (input, _) = parse_keyword_return(input)?;
//     let (input, value) = opt(parse_expression)(input)?;
//     let (input, _) = parse_semicolon(input)?;

//     Ok((input, ReturnStatement { value }))
// }

#[cfg(test)]
mod tests {}
