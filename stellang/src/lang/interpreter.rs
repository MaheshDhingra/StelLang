// Interpreter for StelLang

use super::ast::Expr;
use std::collections::{HashMap, HashSet};
use std::ops::Range as StdRange;
mod exceptions;
use exceptions::{Exception, ExceptionKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i64),
    Float(f64),
    Complex(f64, f64),
    Bool(bool),
    Str(String),
    Bytes(Vec<u8>),
    ByteArray(Vec<u8>),
    MemoryView(Vec<u8>), // Placeholder, could be a wrapper struct
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Range(RangeData),
    Set(HashSet<Value>),
    FrozenSet(HashSet<Value>),
    Dict(HashMap<Value, Value>),
    Iterator(Box<dyn std::any::Any>), // Placeholder for trait object
    Generator(Box<dyn std::any::Any>), // Placeholder for trait object
    None,
    NotImplemented,
    Ellipsis,
    Exception(Exception),
    Error(String), // Deprecated: use Exception
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeData {
    pub start: i64,
    pub stop: i64,
    pub step: i64,
}

pub struct Interpreter {
    pub env: HashMap<String, Value>,
    pub functions: HashMap<String, (Vec<String>, Expr)>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = HashMap::new();
        // Built-in constants
        env.insert("True".to_string(), Value::Bool(true));
        env.insert("False".to_string(), Value::Bool(false));
        env.insert("None".to_string(), Value::Null);
        env.insert("NotImplemented".to_string(), Value::String("NotImplemented".to_string()));
        env.insert("Ellipsis".to_string(), Value::String("...".to_string()));
        env.insert("__debug__".to_string(), Value::Bool(true));
        // Interactive shell constants (printable objects)
        env.insert("quit".to_string(), Value::String("Use quit() or Ctrl-D (i.e. EOF) to exit".to_string()));
        env.insert("exit".to_string(), Value::String("Use exit() or Ctrl-D (i.e. EOF) to exit".to_string()));
        env.insert("help".to_string(), Value::String("Type help() for interactive help, or help(object) for help about object.".to_string()));
        env.insert("copyright".to_string(), Value::String("Copyright (c) StelLang contributors".to_string()));
        env.insert("credits".to_string(), Value::String("Thanks to all StelLang contributors!".to_string()));
        env.insert("license".to_string(), Value::String("Type license() to see the full license text".to_string()));
        Self { env, functions: HashMap::new() }
    }

    pub fn eval(&mut self, expr: &Expr) -> Value {
        self.eval_inner(expr)
    }

    fn eval_inner(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Integer(n) => Value::Int(*n),
            Expr::Float(f) => Value::Float(*f),
            Expr::String(s) => Value::Str(s.clone()),
            Expr::Ident(name) => self.env.get(name).cloned().unwrap_or(Value::Int(0)),
            Expr::ArrayLiteral(items) => {
                Value::List(items.iter().map(|e| self.eval_inner(e)).collect())
            }
            Expr::MapLiteral(pairs) => {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    let key = match self.eval_inner(k) {
                        Value::Str(s) => s,
                        Value::Int(n) => n.to_string(),
                        _ => "".to_string(),
                    };
                    let val = self.eval_inner(v);
                    map.insert(key, val);
                }
                Value::Dict(map)
            }
            Expr::Index { collection, index } => {
                let coll = self.eval_inner(collection);
                let idx = self.eval_inner(index);
                match (coll, idx) {
                    (Value::List(arr), Value::Int(n)) => {
                        if n < 0 || n as usize >= arr.len() {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("list index {} out of range", n)]))
                        } else {
                            arr.get(n as usize).cloned().unwrap_or(Value::None)
                        }
                    }
                    (Value::Dict(map), Value::Str(s)) => {
                        map.get(&s).cloned().unwrap_or(Value::Exception(Exception::new(ExceptionKind::KeyError, vec![s])))
                    }
                    (Value::Dict(map), Value::Int(n)) => {
                        let key = n.to_string();
                        map.get(&key).cloned().unwrap_or(Value::Exception(Exception::new(ExceptionKind::KeyError, vec![key])))
                    }
                    _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Invalid index operation".to_string()]))
                }
            }
            Expr::AssignIndex { collection, index, expr } => {
                let coll = self.eval_inner(collection);
                let idx = self.eval_inner(index);
                let val = self.eval_inner(expr);
                match (coll, idx) {
                    (Value::List(mut arr), Value::Int(n)) => {
                        let i = n as usize;
                        if i < arr.len() {
                            arr[i] = val.clone();
                        }
                        Value::List(arr)
                    }
                    (Value::Dict(mut map), Value::Str(s)) => {
                        map.insert(s, val.clone());
                        Value::Dict(map)
                    }
                    (Value::Dict(mut map), Value::Int(n)) => {
                        map.insert(n.to_string(), val.clone());
                        Value::Dict(map)
                    }
                    _ => Value::Int(0),
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let l = self.eval_inner(left);
                let r = self.eval_inner(right);
                match (l, r) {
                    (Value::Int(l), Value::Int(r)) => match op.as_str() {
                        "+" => Value::Int(l + r),
                        "-" => Value::Int(l - r),
                        "*" => Value::Int(l * r),
                        "/" => Value::Float((l as f64) / (r as f64)),
                        "==" => Value::Int((l == r) as i32),
                        "!=" => Value::Int((l != r) as i32),
                        "<" => Value::Int((l < r) as i32),
                        ">" => Value::Int((l > r) as i32),
                        "<=" => Value::Int((l <= r) as i32),
                        ">=" => Value::Int((l >= r) as i32),
                        "and" => Value::Int(((l != 0) && (r != 0)) as i32),
                        "or" => Value::Int(((l != 0) || (r != 0)) as i32),
                        _ => Value::Int(0),
                    },
                    (Value::Float(l), Value::Float(r)) => match op.as_str() {
                        "+" => Value::Float(l + r),
                        "-" => Value::Float(l - r),
                        "*" => Value::Float(l * r),
                        "/" => Value::Float(l / r),
                        "==" => Value::Int((l == r) as i32 as i64),
                        "!=" => Value::Int((l != r) as i32 as i64),
                        "<" => Value::Int((l < r) as i32 as i64),
                        ">" => Value::Int((l > r) as i32 as i64),
                        "<=" => Value::Int((l <= r) as i32 as i64),
                        ">=" => Value::Int((l >= r) as i32 as i64),
                        "and" => Value::Int(((l != 0.0) && (r != 0.0)) as i32 as i64),
                        "or" => Value::Int(((l != 0.0) || (r != 0.0)) as i32 as i64),
                        _ => Value::Int(0),
                    },
                    (Value::Int(l), Value::Float(r)) => match op.as_str() {
                        "+" => Value::Float((l as f64) + r),
                        "-" => Value::Float((l as f64) - r),
                        "*" => Value::Float((l as f64) * r),
                        "/" => Value::Float((l as f64) / r),
                        "==" => Value::Int((l == r as i64) as i32),
                        "!=" => Value::Int((l != r as i64) as i32),
                        "<" => Value::Int((l < r as i64) as i32),
                        ">" => Value::Int((l > r as i64) as i32),
                        "<=" => Value::Int((l <= r as i64) as i32),
                        ">=" => Value::Int((l >= r as i64) as i32),
                        "and" => Value::Int(((l != 0) && (r != 0.0)) as i32),
                        "or" => Value::Int(((l != 0) || (r != 0.0)) as i32),
                        _ => Value::Int(0),
                    },
                    (Value::Float(l), Value::Int(r)) => match op.as_str() {
                        "+" => Value::Float(l + (r as f64)),
                        "-" => Value::Float(l - (r as f64)),
                        "*" => Value::Float(l * (r as f64)),
                        "/" => Value::Float(l / (r as f64)),
                        "==" => Value::Int((l as i64 == r) as i32),
                        "!=" => Value::Int((l as i64 != r) as i32),
                        "<" => Value::Int((l as i64 < r) as i32),
                        ">" => Value::Int((l as i64 > r) as i32),
                        "<=" => Value::Int((l as i64 <= r) as i32),
                        ">=" => Value::Int((l as i64 >= r) as i32),
                        "and" => Value::Int(((l != 0.0) && (r != 0)) as i32),
                        "or" => Value::Int(((l != 0.0) || (r != 0)) as i32),
                        _ => Value::Int(0),
                    },
                    (Value::Str(l), Value::Str(r)) if op == "+" => Value::Str(l + &r),
                    _ => Value::Int(0),
                }
            }
            Expr::UnaryOp { op, expr } => {
                let v = self.eval_inner(expr);
                match (op.as_str(), v) {
                    ("-", Value::Int(n)) => Value::Int(-n),
                    ("-", Value::Float(n)) => Value::Float(-n),
                    ("not", Value::Int(n)) => Value::Int((n == 0) as i32),
                    ("!", Value::Int(n)) => Value::Int((n == 0) as i32),
                    _ => Value::Int(0),
                }
            }
            Expr::Assign { name, expr } => {
                if name == "True" || name == "False" || name == "None" || name == "__debug__" {
                    Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Assignment to constant is not allowed".to_string()]))
                } else {
                    let val = self.eval_inner(expr);
                    self.env.insert(name.clone(), val.clone());
                    val
                }
            }
            Expr::Let { name, expr } => {
                let val = self.eval_inner(expr);
                self.env.insert(name.clone(), val.clone());
                val
            }
            Expr::Const { name, expr } => {
                let val = self.eval_inner(expr);
                // For now, treat like let (no immutability enforcement yet)
                self.env.insert(name.clone(), val.clone());
                val
            }
            Expr::Bool(b) => Value::Int(if *b { 1 } else { 0 }),
            Expr::Null => Value::Int(0),
            Expr::Block(exprs) => {
                let mut last = Value::Int(0);
                for e in exprs {
                    match self.eval_inner(e) {
                        Value::Error(ref s) if s == "__break__" || s == "__continue__" => return Value::Error(s.clone()),
                        v => last = v,
                    }
                }
                last
            }
            Expr::If { cond, then_branch, else_branch } => {
                let cond_val = self.eval_inner(cond);
                let cond_num = match cond_val { Value::Int(n) => n, _ => 0 };
                if cond_num != 0 {
                    self.eval_inner(then_branch)
                } else if let Some(else_b) = else_branch {
                    self.eval_inner(else_b)
                } else {
                    Value::Int(0)
                }
            }
            Expr::While { cond, body } => {
                let mut last = Value::Int(0);
                'outer: while match self.eval_inner(cond) { Value::Int(n) => n != 0, _ => false } {
                    match self.eval_inner(body) {
                        Value::Error(ref s) if s == "__break__" => break 'outer,
                        Value::Error(ref s) if s == "__continue__" => continue 'outer,
                        v => last = v,
                    }
                }
                last
            }
            Expr::FnDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), *body.clone()));
                Value::Int(0)
            }
            Expr::FnCall { name, args } => {
                // --- Bytes and ByteArray methods ---
                let is_bytes_method = [
                    "isalnum", "isalpha", "isascii", "isdigit", "islower", "isspace", "istitle", "isupper", "lower", "splitlines", "swapcase", "title", "upper", "zfill"
                ].contains(&name.as_str());
                if is_bytes_method && args.len() >= 1 {
                    let (obj, rest) = (self.eval_inner(&args[0]), &args[1..]);
                    let as_bytes = |v: &Value| match v {
                        Value::Bytes(b) | Value::ByteArray(b) => Some(b),
                        _ => None,
                    };
                    let make_result = |orig: &Value, b: Vec<u8>| match orig {
                        Value::Bytes(_) => Value::Bytes(b),
                        Value::ByteArray(_) => Value::ByteArray(b),
                        _ => Value::Error("Not a bytes or bytearray object".to_string()),
                    };
                    match name.as_str() {
                        "isalnum" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isalnum = !b.is_empty() && b.iter().all(|c| (b'a' <= *c && *c <= b'z') || (b'A' <= *c && *c <= b'Z') || (b'0' <= *c && *c <= b'9'));
                                Value::Bool(isalnum)
                            } else { Value::Error("isalnum: not bytes/bytearray".to_string()) }
                        }
                        "isalpha" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isalpha = !b.is_empty() && b.iter().all(|c| (b'a' <= *c && *c <= b'z') || (b'A' <= *c && *c <= b'Z'));
                                Value::Bool(isalpha)
                            } else { Value::Error("isalpha: not bytes/bytearray".to_string()) }
                        }
                        "isascii" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isascii = b.iter().all(|c| *c <= 0x7F);
                                Value::Bool(isascii)
                            } else { Value::Error("isascii: not bytes/bytearray".to_string()) }
                        }
                        "isdigit" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isdigit = !b.is_empty() && b.iter().all(|c| b'0' <= *c && *c <= b'9');
                                Value::Bool(isdigit)
                            } else { Value::Error("isdigit: not bytes/bytearray".to_string()) }
                        }
                        "islower" => {
                            if let Some(b) = as_bytes(&obj) {
                                let has_lower = b.iter().any(|c| b'a' <= *c && *c <= b'z');
                                let has_upper = b.iter().any(|c| b'A' <= *c && *c <= b'Z');
                                Value::Bool(has_lower && !has_upper)
                            } else { Value::Error("islower: not bytes/bytearray".to_string()) }
                        }
                        "isspace" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isspace = !b.is_empty() && b.iter().all(|c| matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c));
                                Value::Bool(isspace)
                            } else { Value::Error("isspace: not bytes/bytearray".to_string()) }
                        }
                        "istitle" => {
                            if let Some(b) = as_bytes(&obj) {
                                let mut prev_cased = false;
                                let mut is_title = false;
                                let mut seen = false;
                                for &c in b.iter() {
                                    let is_upper = b'A' <= c && c <= b'Z';
                                    let is_lower = b'a' <= c && c <= b'z';
                                    if is_upper {
                                        if prev_cased { is_title = false; break; }
                                        is_title = true;
                                        prev_cased = true;
                                        seen = true;
                                    } else if is_lower {
                                        if !prev_cased { is_title = false; break; }
                                        prev_cased = true;
                                        seen = true;
                                    } else {
                                        prev_cased = false;
                                    }
                                }
                                Value::Bool(seen && is_title)
                            } else { Value::Error("istitle: not bytes/bytearray".to_string()) }
                        }
                        "isupper" => {
                            if let Some(b) = as_bytes(&obj) {
                                let has_upper = b.iter().any(|c| b'A' <= *c && *c <= b'Z');
                                let has_lower = b.iter().any(|c| b'a' <= *c && *c <= b'z');
                                Value::Bool(has_upper && !has_lower)
                            } else { Value::Error("isupper: not bytes/bytearray".to_string()) }
                        }
                        "lower" => {
                            if let Some(b) = as_bytes(&obj) {
                                let out = b.iter().map(|c| if b'A' <= *c && *c <= b'Z' { *c + 32 } else { *c }).collect();
                                make_result(&obj, out)
                            } else { Value::Error("lower: not bytes/bytearray".to_string()) }
                        }
                        "splitlines" => {
                            let keepends = rest.get(0).map(|v| self.eval_inner(v)).map(|v| matches!(v, Value::Bool(true) | Value::Int(1))).unwrap_or(false);
                            if let Some(b) = as_bytes(&obj) {
                                let mut lines = Vec::new();
                                let mut start = 0;
                                let mut i = 0;
                                while i < b.len() {
                                    let c = b[i];
                                    let line_end = match c {
                                        b'\n' => Some(i+1),
                                        b'\r' => {
                                            if i+1 < b.len() && b[i+1] == b'\n' { Some(i+2) } else { Some(i+1) }
                                        },
                                        _ => None,
                                    };
                                    if let Some(end) = line_end {
                                        let mut line = b[start..end].to_vec();
                                        if !keepends {
                                            if line.ends_with(&[b'\n']) { line.pop(); }
                                            if line.ends_with(&[b'\r']) { line.pop(); }
                                        }
                                        lines.push(make_result(&obj, line));
                                        start = end;
                                        i = end;
                                    } else {
                                        i += 1;
                                    }
                                }
                                if start < b.len() {
                                    lines.push(make_result(&obj, b[start..].to_vec()));
                                }
                                Value::List(lines)
                            } else { Value::Error("splitlines: not bytes/bytearray".to_string()) }
                        }
                        "swapcase" => {
                            if let Some(b) = as_bytes(&obj) {
                                let out = b.iter().map(|c| {
                                    if b'a' <= *c && *c <= b'z' { *c - 32 }
                                    else if b'A' <= *c && *c <= b'Z' { *c + 32 }
                                    else { *c }
                                }).collect();
                                make_result(&obj, out)
                            } else { Value::Error("swapcase: not bytes/bytearray".to_string()) }
                        }
                        "title" => {
                            if let Some(b) = as_bytes(&obj) {
                                let mut out = Vec::with_capacity(b.len());
                                let mut prev_cased = false;
                                for &c in b.iter() {
                                    let is_alpha = (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z');
                                    if is_alpha {
                                        if !prev_cased {
                                            if b'a' <= c && c <= b'z' { out.push(c - 32); } else { out.push(c); }
                                            prev_cased = true;
                                        } else {
                                            if b'A' <= c && c <= b'Z' { out.push(c + 32); } else { out.push(c); }
                                        }
                                    } else {
                                        out.push(c);
                                        prev_cased = false;
                                    }
                                }
                                make_result(&obj, out)
                            } else { Value::Error("title: not bytes/bytearray".to_string()) }
                        }
                        "upper" => {
                            if let Some(b) = as_bytes(&obj) {
                                let out = b.iter().map(|c| if b'a' <= *c && *c <= b'z' { *c - 32 } else { *c }).collect();
                                make_result(&obj, out)
                            } else { Value::Error("upper: not bytes/bytearray".to_string()) }
                        }
                        "zfill" => {
                            if let Some(b) = as_bytes(&obj) {
                                if let Some(Value::Int(width)) = rest.get(0) {
                                    let width = *width as usize;
                                    let mut out = Vec::new();
                                    let mut sign = 0;
                                    if !b.is_empty() && (b[0] == b'+' as u8 || b[0] == b'-' as u8) {
                                        out.push(b[0]);
                                        sign = 1;
                                    }
                                    let pad = width.saturating_sub(b.len());
                                    for _ in 0..pad { out.push(b'0'); }
                                    out.extend_from_slice(&b[sign..]);
                                    make_result(&obj, out)
                                } else {
                                    Value::Error("zfill expects width argument".to_string())
                                }
                            } else { Value::Error("zfill: not bytes/bytearray".to_string()) }
                        }
                        _ => Value::Error("Unknown bytes/bytearray method".to_string()),
                    }
                }
                // ...existing code for user-defined functions...
            }
            Expr::Return(expr) => {
                // Use panic to unwind for return, or use a custom Result type in a real implementation
                let val = self.eval_inner(expr);
                panic!("__return__{:?}", val);
            }
            Expr::Break => Value::Error("__break__".to_string()),
            Expr::Continue => Value::Error("__continue__".to_string()),
            Expr::Match { expr, arms } => {
                let val = self.eval_inner(expr);
                for (pat, res) in arms {
                    let pat_val = self.eval_inner(pat);
                    if Self::pattern_match(&val, &pat_val) {
                        return self.eval_inner(res);
                    }
                }
                Value::Null
            }
            Expr::StructDef { name, fields } => {
                // Store struct definition in env as a marker
                self.env.insert(format!("__struct__{}", name), Value::List(fields.iter().map(|f| Value::Str(f.clone())).collect()));
                Value::Null
            }
            Expr::StructInit { name, fields } => {
                // Create a map with struct fields
                let mut map = std::collections::HashMap::new();
                for (k, v) in fields {
                    map.insert(k.clone(), self.eval_inner(v));
                }
                map.insert("__struct_name__".to_string(), Value::Str(name.clone()));
                Value::Dict(map)
            }
            Expr::EnumDef { name, variants } => {
                // Store enum definition in env as a marker
                self.env.insert(format!("__enum__{}", name), Value::List(variants.iter().map(|v| Value::Str(v.clone())).collect()));
                Value::Null
            }
            Expr::EnumInit { name, variant, value } => {
                let mut map = std::collections::HashMap::new();
                map.insert("__enum_name__".to_string(), Value::Str(name.clone()));
                map.insert("__variant__".to_string(), Value::Str(variant.clone()));
                if let Some(val) = value {
                    map.insert("value".to_string(), self.eval_inner(val));
                }
                Value::Dict(map)
            }
            Expr::TryCatch { try_block, catch_var, catch_block } => {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.eval_inner(try_block)));
                match result {
                    Ok(v) => v,
                    Err(e) => {
                        // Try to extract the error value from panic
                        let err_val = if let Some(s) = e.downcast_ref::<String>() {
                            if s.starts_with("Error: ") {
                                Value::Error(s[7..].to_string())
                            } else {
                                Value::Error(s.clone())
                            }
                        } else if let Some(s) = e.downcast_ref::<&str>() {
                            if s.starts_with("Error: ") {
                                Value::Error(s[7..].to_string())
                            } else {
                                Value::Error(s.to_string())
                            }
                        } else {
                            Value::Error("Unknown error".to_string())
                        };
                        if let Some(var) = catch_var {
                            self.env.insert(var.clone(), err_val.clone());
                        }
                        self.eval_inner(catch_block)
                    }
                }
            }
            Expr::Throw(expr) => {
                let val = self.eval_inner(expr);
                match val {
                    Value::Error(e) => panic!("Error: {}", e),
                    Value::String(s) => panic!("Error: {}", s),
                    _ => panic!("Error: thrown value"),
                }
            }
            Expr::Import(path) => {
                use std::fs;
                let code = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());
                let mut lexer = super::lexer::Lexer::new(&code);
                let mut tokens = Vec::new();
                loop {
                    let tok = lexer.next_token();
                    if tok == super::lexer::Token::EOF { break; }
                    tokens.push(tok);
                }
                let mut parser = super::parser::Parser::new(tokens);
                if let Some(expr) = parser.parse() {
                    let mut sub_interpreter = Interpreter {
                        env: self.env.clone(),
                        functions: self.functions.clone(),
                    };
                    let result = sub_interpreter.eval(&expr);
                    // Merge imported env and functions
                    self.env.extend(sub_interpreter.env);
                    self.functions.extend(sub_interpreter.functions);
                    result
                } else {
                    Value::Error(format!("Failed to import {}", path))
                }
            }
            _ => Value::Error("Not implemented".to_string()),
        }
    }

    // Helper for pattern matching
    fn pattern_match(val: &Value, pat: &Value) -> bool {
        match (val, pat) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Dict(a), Value::Dict(b)) => a == b,
            // Wildcard pattern: _
            (_, Value::Str(s)) if s == "_" => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn to_display_string(&self) -> String {
        match self {
            Value::Int(n) => {
                format!("{}", *n)
            }
            Value::Float(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Str(s) => s.clone(),
            Value::List(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_display_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Dict(map) => {
                let items: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k.to_display_string(), v.to_display_string())).collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Error(e) => format!("Error: {}", e),
            Value::Bool(b) => format!("{}", b),
            Value::Null => "null".to_string(),
            Value::Bytes(b) => format!("b{:?}", b),
            Value::ByteArray(b) => format!("bytearray({:?})", b),
            Value::MemoryView(b) => format!("memoryview({:?})", b),
            Value::Range(r) => format!("range({}, {}, {})", r.start, r.stop, r.step),
            Value::Set(s) => {
                let items: Vec<String> = s.iter().map(|v| v.to_display_string()).collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::FrozenSet(s) => {
                let items: Vec<String> = s.iter().map(|v| v.to_display_string()).collect();
                format!("frozenset({{{}}})", items.join(", "))
            }
            Value::Iterator(_) => "[iterator]".to_string(),
            Value::Generator(_) => "[generator]".to_string(),
        }
    }
}
