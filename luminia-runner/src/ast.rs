use core::fmt::Display;

#[derive(Clone, Debug)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Bool(bool),
    Variable(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Gt,
}

impl Op {
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Eq => "==",
            Self::Lt => "<",
            Self::Gt => ">",
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assign(String, Expr),
    If(Expr, Vec<Statement>, Option<Vec<Statement>>),
    While(Expr, Vec<Statement>),
    Def(String, Vec<String>, Vec<Statement>),
    Return(Expr),
    ExprStatement(Expr),
}
