use self::{keyword::Keyword, operator::Operator, literal::Literal, punctuation::Punctuation};

mod keyword;
mod operator;
mod literal;
mod punctuation;
mod Operator;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Operator(Operator),
    Literal(Literal),
    Identifier(String),
    Punctuation(Punctuation),
    Whitespace,
    Comment(String),
}