use nom::{combinator::map, sequence::tuple, IResult};

use crate::{ast::node::FunctionDefinition, parser::lexer::identifier::parse_identifier};

// fn parse_function_definition(input: &str) -> IResult<&str, FunctionDefinition> {
//     map(
//         tuple((
//             parse_type_specifier,
//             parse_identifier,
//             parse_parameters,
//             parse_compound_statement,
//         )),
//         |(return_type, name, parameters, body)| FunctionDefinition {
//             return_type,
//             name,
//             parameters,
//             body,
//         },
//     )(input)
// }
