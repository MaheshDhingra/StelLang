// AST definitions for StelLang

#[derive(Debug, Clone, PartialEq)]
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
        callable: Box<Expr>,
        args: Vec<Expr>,
    },
    GetAttr {
        object: Box<Expr>,
        name: String,
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
    Break,
    Continue,
    Let {
        name: String,
        expr: Box<Expr>,
    },
    Const {
        name: String,
        expr: Box<Expr>,
    },
    Bool(bool),
    Null,
    // === Pattern matching, structs, enums ===
    Match {
        expr: Box<Expr>,
        arms: Vec<(Expr, Expr)>, // (pattern, result)
    },
    StructDef {
        name: String,
        fields: Vec<String>,
    },
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    EnumDef {
        name: String,
        variants: Vec<String>,
    },
    EnumInit {
        name: String,
        variant: String,
        value: Option<Box<Expr>>,
    },
    For {
        var: String,
        iter: Box<Expr>,
        body: Box<Expr>,
    },
    TryCatch {
        try_block: Box<Expr>,
        catch_var: Option<String>,
        catch_block: Box<Expr>,
    },
    Throw(Box<Expr>),
    TupleLiteral(Vec<Expr>),
    Destructure {
        names: Vec<String>,
        expr: Box<Expr>,
    },
    Import(String),
    LetTyped {
        name: String,
        ty: String,
        expr: Box<Expr>,
    },
    ConstTyped {
        name: String,
        ty: String,
        expr: Box<Expr>,
    },
    Global {
        name: String,
        expr: Box<Expr>,
    },
    Static {
        name: String,
        expr: Box<Expr>,
    },
    Defer(Box<Expr>),
    Switch {
        expr: Box<Expr>,
        cases: Vec<(Expr, Expr)>,
        default: Option<Box<Expr>>,
    },
}
