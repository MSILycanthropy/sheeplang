pub mod smart;

#[derive(Debug, Clone)]
pub enum Statement {
    LetBinding { name: String, value: Expr },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Lambda { param: String, body: Box<Expr> },
    App { func: Box<Expr>, arg: Box<Expr> },
    Builtin(String),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub main_expr: Option<Expr>,
}

