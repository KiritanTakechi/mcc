#[derive(Debug, PartialEq)]
pub enum Token {
    // 关键字
    Void,
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

    // 操作符
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

    // 标识符
    Identifier(String),

    // 字面量
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),

    // 标点符号
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

    // 错误Token
    Illegal(String),

    // 注释
    Comment,
}
