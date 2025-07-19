// AST definitions for StelLang

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Integer(i64),
    Float(f64), // f64 cannot implement Eq or Hash directly, will need manual impl for Expr
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
    ClassDef {
        name: String,
        bases: Vec<Expr>,
        body: Vec<Expr>,
    },
    ClassInit {
        class_name: String,
        args: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
}

use std::hash::{Hash, Hasher};

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Expr::Integer(i) => i.hash(state),
            Expr::Float(f) => f.to_bits().hash(state), // Hash float bits
            Expr::Ident(s) => s.hash(state),
            Expr::String(s) => s.hash(state),
            Expr::BinaryOp { left, op, right } => {
                left.hash(state);
                op.hash(state);
                right.hash(state);
            },
            Expr::Assign { name, expr } => {
                name.hash(state);
                expr.hash(state);
            },
            Expr::Block(exprs) => exprs.hash(state),
            Expr::If { cond, then_branch, else_branch } => {
                cond.hash(state);
                then_branch.hash(state);
                else_branch.hash(state);
            },
            Expr::While { cond, body } => {
                cond.hash(state);
                body.hash(state);
            },
            Expr::FnDef { name, params, body } => {
                name.hash(state);
                params.hash(state);
                body.hash(state);
            },
            Expr::FnCall { callable, args } => {
                callable.hash(state);
                args.hash(state);
            },
            Expr::GetAttr { object, name } => {
                object.hash(state);
                name.hash(state);
            },
            Expr::ArrayLiteral(items) => items.hash(state),
            Expr::MapLiteral(pairs) => {
                for (k, v) in pairs {
                    k.hash(state);
                    v.hash(state);
                }
            },
            Expr::Index { collection, index } => {
                collection.hash(state);
                index.hash(state);
            },
            Expr::AssignIndex { collection, index, expr } => {
                collection.hash(state);
                index.hash(state);
                expr.hash(state);
            },
            Expr::UnaryOp { op, expr } => {
                op.hash(state);
                expr.hash(state);
            },
            Expr::Return(expr) => expr.hash(state),
            Expr::Break => "Break".hash(state),
            Expr::Continue => "Continue".hash(state),
            Expr::Let { name, expr } => {
                name.hash(state);
                expr.hash(state);
            },
            Expr::Const { name, expr } => {
                name.hash(state);
                expr.hash(state);
            },
            Expr::Bool(b) => b.hash(state),
            Expr::Null => "Null".hash(state),
            Expr::Match { expr, arms } => {
                expr.hash(state);
                for (pat, res) in arms {
                    pat.hash(state);
                    res.hash(state);
                }
            },
            Expr::StructDef { name, fields } => {
                name.hash(state);
                fields.hash(state);
            },
            Expr::StructInit { name, fields } => {
                name.hash(state);
                for (f_name, f_expr) in fields {
                    f_name.hash(state);
                    f_expr.hash(state);
                }
            },
            Expr::EnumDef { name, variants } => {
                name.hash(state);
                variants.hash(state);
            },
            Expr::EnumInit { name, variant, value } => {
                name.hash(state);
                variant.hash(state);
                value.hash(state);
            },
            Expr::For { var, iter, body } => {
                var.hash(state);
                iter.hash(state);
                body.hash(state);
            },
            Expr::TryCatch { try_block, catch_var, catch_block } => {
                try_block.hash(state);
                catch_var.hash(state);
                catch_block.hash(state);
            },
            Expr::Throw(expr) => expr.hash(state),
            Expr::TupleLiteral(items) => items.hash(state),
            Expr::Destructure { names, expr } => {
                names.hash(state);
                expr.hash(state);
            },
            Expr::Import(s) => s.hash(state),
            Expr::LetTyped { name, ty, expr } => {
                name.hash(state);
                ty.hash(state);
                expr.hash(state);
            },
            Expr::ConstTyped { name, ty, expr } => {
                name.hash(state);
                ty.hash(state);
                expr.hash(state);
            },
            Expr::Global { name, expr } => {
                name.hash(state);
                expr.hash(state);
            },
            Expr::Static { name, expr } => {
                name.hash(state);
                expr.hash(state);
            },
            Expr::Defer(expr) => expr.hash(state),
            Expr::Switch { expr, cases, default } => {
                expr.hash(state);
                for (case_expr, case_body) in cases {
                    case_expr.hash(state);
                    case_body.hash(state);
                }
                default.hash(state);
            },
            Expr::ClassDef { name, bases, body } => {
                name.hash(state);
                bases.hash(state);
                body.hash(state);
            },
            Expr::ClassInit { class_name, args } => {
                class_name.hash(state);
                args.hash(state);
            },
            Expr::MethodCall { object, method, args } => {
                object.hash(state);
                method.hash(state);
                args.hash(state);
            },
            Expr::FieldAccess { object, field } => {
                object.hash(state);
                field.hash(state);
            },
        }
    }
}
