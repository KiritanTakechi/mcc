use nom::{bytes::complete::tag, combinator::map, IResult};

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

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Long,
    Float,
    Double,
    Char,
    Struct,
    Union,
    If,
    Else,
    While,
    For,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Increment,
    Decrement,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    LeftShift,
    RightShift,
    Assignment,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Comma,
    SemiColon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Dot,
    Arrow,
    Ellipsis,
}

fn parse_keyword_int(input: &str) -> IResult<&str, Token> {
    map(tag("int"), |_| Token::Keyword(Keyword::Int))(input)
}

fn parse_keyword_long(input: &str) -> IResult<&str, Token> {
    map(tag("long"), |_| Token::Keyword(Keyword::Long))(input)
}

fn parse_keyword_float(input: &str) -> IResult<&str, Token> {
    map(tag("float"), |_| Token::Keyword(Keyword::Float))(input)
}

fn parse_keyword_double(input: &str) -> IResult<&str, Token> {
    map(tag("double"), |_| Token::Keyword(Keyword::Double))(input)
}

fn parse_keyword_char(input: &str) -> IResult<&str, Token> {
    map(tag("char"), |_| Token::Keyword(Keyword::Char))(input)
}

fn parse_keyword_struct(input: &str) -> IResult<&str, Token> {
    map(tag("struct"), |_| Token::Keyword(Keyword::Struct))(input)
}

fn parse_keyword_union(input: &str) -> IResult<&str, Token> {
    map(tag("union"), |_| Token::Keyword(Keyword::Union))(input)
}
