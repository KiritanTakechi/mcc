#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug, PartialEq)]
pub enum TypeSpecifier {
    Int,
    Long,
    Float,
    Double,
    Char,
    Struct,
    Union,
    Void,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub return_type: TypeSpecifier,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: CompoundStatement,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub type_specifier: TypeSpecifier,
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExpressionStatement(Expression),
    CompoundStatement(CompoundStatement),
    SelectionStatement(SelectionStatement),
    IterationStatement(IterationStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    // 表达式标识符
    Identifier(String),

    // 表达式类型
    IntegerLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),

    // 赋值表达式
    AssignmentExpression {
        identifier: String,
        operator: AssignmentOperator,
        value: Box<Expression>,
    },

    // 运算符表达式
    UnaryExpression(UnaryOperator, Box<Expression>),
    BinaryExpression(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    LogicalNot,
    BitwiseNot,
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
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
pub struct CompoundStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct SelectionStatement {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct IterationStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub type_specifier: TypeSpecifier,
    pub identifier: String,
}
