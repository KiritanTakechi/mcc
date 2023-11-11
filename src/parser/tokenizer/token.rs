#[derive(Debug, PartialEq)]
pub enum Token {
    // Keyword
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

    // Operator
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

    // Literal
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),

    // Identifier
    Identifier(String),

    // Punctuation
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

    // Whitespace
    Whitespace,

    // Comment
    CommentLine(String),
    CommentBlock(String),
}
