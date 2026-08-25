#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Dot,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    Identifier(String),
    Array(Vec<Expr>),
    SliceAccess {
        target: String,
        start: Box<Expr>,
        end: Option<Box<Expr>>,
    },
    BinaryOp {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>,
    },
    FunctionCall {
        name: String,
        arg: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        target: String,
        expr: Expr,
    },
    Expression(Expr),
    IfElse {
        condition: Expr,
        true_block: Vec<Statement>,
        false_block: Option<Vec<Statement>>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
