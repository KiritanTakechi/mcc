use nom::{branch::alt, IResult};

use crate::ast::node::*;

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    //alt((parse_expression_statement, parse_compound_statement))(input)

    todo!()
}

pub fn parse_expression_statement(input: &str) -> IResult<&str, Expression> {
    todo!()
}

pub fn parse_compound_statement(input: &str) -> IResult<&str, CompoundStatement> {
    todo!()
}

pub fn parse_selection_statement(input: &str) -> IResult<&str, SelectionStatement> {
    todo!()
}

pub fn parse_iteration_statement(input: &str) -> IResult<&str, IterationStatement> {
    todo!()
}

pub fn parse_return_statement(input: &str) -> IResult<&str, ReturnStatement> {
    todo!()
}
