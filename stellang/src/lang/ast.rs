// AST definitions for StelLang

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Ident(String),
    String(String),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    Block(Vec<Expr>),
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    FnDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    FnCall {
        name: String,
        args: Vec<Expr>,
    },
    // === Added for arrays, maps, indexing, unary, and return ===
    ArrayLiteral(Vec<Expr>),
    MapLiteral(Vec<(Expr, Expr)>),
    Index {
        collection: Box<Expr>,
        index: Box<Expr>,
    },
    AssignIndex {
        collection: Box<Expr>,
        index: Box<Expr>,
        expr: Box<Expr>,
    },
    UnaryOp {
        op: String,
        expr: Box<Expr>,
    },
    Return(Box<Expr>),
    // === Control flow ===
    Break,
    Continue,
}
