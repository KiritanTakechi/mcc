use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
    combinator::map,
    IResult, Parser,
};

use self::{
    comment::parse_comment, identifier::parse_identifier, keyword::parse_keyword,
    literal::parse_literal, operator::parse_operator, punctuation::parse_punctuation,
};

use super::tokenizer::token::Token;

pub mod comment;
pub mod identifier;
pub mod illegal;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod punctuation;

// 主词法分析器
fn lexer(input: &str) -> IResult<&str, Vec<Token>> {
    let mut tokens = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        let (next_input, token) = alt((
            map(parse_comment, |_| None),
            map(parse_keyword, Some),
            map(parse_literal, Some),
            map(parse_operator, Some),
            map(parse_punctuation, Some),
            map(parse_identifier, Some),
            map(multispace0, |_| None),
            //map(parse_illegal, Some),
        ))
        .parse(remaining)?;

        if let Some(token) = token {
            tokens.push(token);
        }

        if remaining == next_input {
            break;
        }

        remaining = next_input;
    }

    Ok((remaining, tokens))
}

#[cfg(test)]
mod tests {
    use super::super::tokenizer::token::Token;
    use super::*;

    #[test]
    fn test_lexer_simple_expression() {
        let input = "int x = 5;";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier(String::from("x")),
                Token::Assignment,
                Token::IntegerLiteral(5),
                Token::SemiColon,
            ]
        );
    }

    #[test]
    fn test_lexer_function_declaration() {
        let input = "void my_function(int a, int b) {}";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Void,
                Token::Identifier(String::from("my_function")),
                Token::OpenParen,
                Token::Int,
                Token::Identifier(String::from("a")),
                Token::Comma,
                Token::Int,
                Token::Identifier(String::from("b")),
                Token::CloseParen,
                Token::OpenBrace,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_variable_declaration() {
        let input = "int myVar = 100;";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier(String::from("myVar")),
                Token::Assignment,
                Token::IntegerLiteral(100),
                Token::SemiColon,
            ]
        );
    }

    #[test]
    fn test_if_statement() {
        let input = "if (x > 10) { x = 10; }";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::OpenParen,
                Token::Identifier(String::from("x")),
                Token::GreaterThan,
                Token::IntegerLiteral(10),
                Token::CloseParen,
                Token::OpenBrace,
                Token::Identifier(String::from("x")),
                Token::Assignment,
                Token::IntegerLiteral(10),
                Token::SemiColon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let input = "doSomething(5, 10);";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Identifier(String::from("doSomething")),
                Token::OpenParen,
                Token::IntegerLiteral(5),
                Token::Comma,
                Token::IntegerLiteral(10),
                Token::CloseParen,
                Token::SemiColon,
            ]
        );
    }

    #[test]
    fn test_complex_expression() {
        let input = "result = (a + b) * c;";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Identifier(String::from("result")),
                Token::Assignment,
                Token::OpenParen,
                Token::Identifier(String::from("a")),
                Token::Plus,
                Token::Identifier(String::from("b")),
                Token::CloseParen,
                Token::Multiply,
                Token::Identifier(String::from("c")),
                Token::SemiColon,
            ]
        );
    }

    #[test]
    fn test_single_line_comment() {
        let input = "int x = 10; // This is a comment";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier(String::from("x")),
                Token::Assignment,
                Token::IntegerLiteral(10),
                Token::SemiColon,
            ]
        );
    }

    #[test]
    fn test_multi_line_comment() {
        let input = "/* Multi-line\n comment */int y = 20;";
        let tokens = lexer(input).unwrap().1;
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier(String::from("y")),
                Token::Assignment,
                Token::IntegerLiteral(20),
                Token::SemiColon,
            ]
        );
    }
}
