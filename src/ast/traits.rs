use super::node::*;

// 特性
pub trait IntoStatement {
    fn into_statement(self) -> Statement;
}

// 实现
impl IntoStatement for Expression {
    fn into_statement(self) -> Statement {
        Statement::ExpressionStatement(self)
    }
}

impl IntoStatement for CompoundStatement {
    fn into_statement(self) -> Statement {
        Statement::CompoundStatement(self)
    }
}

impl IntoStatement for SelectionStatement {
    fn into_statement(self) -> Statement {
        Statement::SelectionStatement(self)
    }
}

impl IntoStatement for IterationStatement {
    fn into_statement(self) -> Statement {
        Statement::IterationStatement(self)
    }
}

impl IntoStatement for ReturnStatement {
    fn into_statement(self) -> Statement {
        Statement::ReturnStatement(self)
    }
}
