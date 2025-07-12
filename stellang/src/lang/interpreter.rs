use super::ast::Expr;
use std::collections::HashMap;
use crate::lang::exceptions::{Exception, ExceptionKind};

#[derive(Debug)] // Removed Clone derive
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
    Set(std::collections::HashSet<Value>),
    FrozenSet(std::collections::HashSet<Value>),
    Dict(std::collections::HashMap<Value, Value>),
    // Iterator(Box<dyn std::any::Any>), // Removed due to Clone trait issue
    // Generator(Box<dyn std::any::Any>), // Removed due to Clone trait issue
    None,
    NotImplemented,
    Ellipsis,
    Exception(exceptions::Exception),
    BuiltinMethod {
        object: Box<Value>,
        method_name: String,
    },
}

// Manual implementation of Clone for Value
impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Int(i) => Value::Int(*i),
            Value::Float(f) => Value::Float(*f),
            Value::Complex(r, i) => Value::Complex(*r, *i),
            Value::Bool(b) => Value::Bool(*b),
            Value::Str(s) => Value::Str(s.clone()),
            Value::Bytes(b) => Value::Bytes(b.clone()),
            Value::ByteArray(b) => Value::ByteArray(b.clone()),
            Value::MemoryView(b) => Value::MemoryView(b.clone()),
            Value::List(l) => Value::List(l.clone()),
            Value::Tuple(t) => Value::Tuple(t.clone()),
            Value::Range(r) => Value::Range(r.clone()),
            Value::Set(s) => Value::Set(s.clone()),
            Value::FrozenSet(s) => Value::FrozenSet(s.clone()),
            Value::Dict(d) => Value::Dict(d.clone()),
            Value::None => Value::None,
            Value::NotImplemented => Value::NotImplemented,
            Value::Ellipsis => Value::Ellipsis,
            Value::Exception(e) => Value::Exception(e.clone()),
            Value::BuiltinMethod { object, method_name } => Value::BuiltinMethod {
                object: object.clone(),
                method_name: method_name.clone(),
            },
        }
    }
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
        env.insert("None".to_string(), Value::None);
        env.insert("NotImplemented".to_string(), Value::NotImplemented);
        env.insert("Ellipsis".to_string(), Value::Ellipsis);
        env.insert("__debug__".to_string(), Value::Bool(true));
        // Interactive shell constants (printable objects)
        env.insert("quit".to_string(), Value::Str("Use quit() or Ctrl-D (i.e. EOF) to exit".to_string()));
        env.insert("exit".to_string(), Value::Str("Use exit() or Ctrl-D (i.e. EOF) to exit".to_string()));
        env.insert("help".to_string(), Value::Str("Type help() for interactive help, or help(object) for help about object.".to_string()));
        env.insert("copyright".to_string(), Value::Str("Copyright (c) StelLang contributors".to_string()));
        env.insert("credits".to_string(), Value::Str("Thanks to all StelLang contributors!".to_string()));
        env.insert("license".to_string(), Value::Str("Type license() to see the full license text".to_string()));
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
            Expr::Ident(name) => self.env.get(name).cloned().unwrap_or(Value::None), // Changed default to None
            Expr::ArrayLiteral(items) => {
                Value::List(items.iter().map(|e| self.eval_inner(e)).collect())
            }
            Expr::MapLiteral(pairs) => {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    let key = self.eval_inner(k);
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
                    (Value::Dict(map), key) => {
                        map.get(&key).cloned().unwrap_or(
                            Value::Exception(Exception::new(ExceptionKind::KeyError, vec![key.to_display_string()]))
                        )
                    }
                    (Value::Str(s), Value::Int(n)) => {
                        if n < 0 || n as usize >= s.len() {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("string index {} out of range", n)]))
                        } else {
                            s.chars().nth(n as usize).map(|c| Value::Str(c.to_string())).unwrap_or(Value::None)
                        }
                    }
                    (Value::Bytes(b), Value::Int(n)) => {
                        if n < 0 || n as usize >= b.len() {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("bytes index {} out of range", n)]))
                        } else {
                            b.get(n as usize).map(|&byte| Value::Int(byte as i64)).unwrap_or(Value::None)
                        }
                    }
                    (Value::ByteArray(b), Value::Int(n)) => {
                        if n < 0 || n as usize >= b.len() {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("bytearray index {} out of range", n)]))
                        } else {
                            b.get(n as usize).map(|&byte| Value::Int(byte as i64)).unwrap_or(Value::None)
                        }
                    }
                    (Value::Tuple(t), Value::Int(n)) => {
                        if n < 0 || n as usize >= t.len() {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("tuple index {} out of range", n)]))
                        } else {
                            t.get(n as usize).cloned().unwrap_or(Value::None)
                        }
                    }
                    _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("'{}' object is not subscriptable", coll.type_name())]))
                }
            }
            Expr::AssignIndex { collection, index, expr } => {
                let mut coll = self.eval_inner(collection);
                let idx = self.eval_inner(index);
                let val = self.eval_inner(expr);
                match (&mut coll, idx) {
                    (Value::List(arr), Value::Int(n)) => {
                        let i = n as usize;
                        if i < arr.len() {
                            arr[i] = val.clone();
                            coll
                        } else {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("list assignment index {} out of range", n)]))
                        }
                    }
                    (Value::Dict(map), key) => {
                        map.insert(key, val.clone());
                        coll
                    }
                    (Value::ByteArray(arr), Value::Int(n)) => {
                        let i = n as usize;
                        if i < arr.len() {
                            if let Value::Int(byte_val) = val {
                                if byte_val >= 0 && byte_val <= 255 {
                                    arr[i] = byte_val as u8;
                                    coll
                                } else {
                                    Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["byte must be in range(0, 256)".to_string()]))
                                }
                            } else {
                                Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["bytearray assignment must be an integer".to_string()]))
                            }
                        } else {
                            Value::Exception(Exception::new(ExceptionKind::IndexError, vec![format!("bytearray assignment index {} out of range", n)]))
                        }
                    }
                    _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("'{}' object does not support item assignment", coll.type_name())]))
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
                        "/" => {
                            if r == 0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["division by zero".to_string()]));
                            }
                            Value::Float((l as f64) / (r as f64))
                        },
                        "//" => {
                            if r == 0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["integer division by zero".to_string()]));
                            }
                            Value::Int(l / r)
                        },
                        "%" => {
                            if r == 0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["modulo by zero".to_string()]));
                            }
                            Value::Int(l % r)
                        },
                        "**" => Value::Float((l as f64).powf(r as f64)),
                        "&" => Value::Int(l & r),
                        "|" => Value::Int(l | r),
                        "^" => Value::Int(l ^ r),
                        "<<" => Value::Int(l << r),
                        ">>" => Value::Int(l >> r),
                        "==" => Value::Bool(l == r),
                        "!=" => Value::Bool(l != r),
                        "<" => Value::Bool(l < r),
                        ">" => Value::Bool(l > r),
                        "<=" => Value::Bool(l <= r),
                        ">=" => Value::Bool(l >= r),
                        "and" => Value::Bool((l != 0) && (r != 0)),
                        "or" => Value::Bool((l != 0) || (r != 0)),
                        "is" => Value::Bool(l == r), // For primitive types, 'is' is value equality
                        "is not" => Value::Bool(l != r),
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: 'int' and 'int'", op)])),
                    },
                    (Value::Float(l), Value::Float(r)) => match op.as_str() {
                        "+" => Value::Float(l + r),
                        "-" => Value::Float(l - r),
                        "*" => Value::Float(l * r),
                        "/" => {
                            if r == 0.0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["division by zero".to_string()]));
                            }
                            Value::Float(l / r)
                        },
                        "//" => {
                            if r == 0.0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["float floor division by zero".to_string()]));
                            }
                            Value::Float((l / r).floor())
                        },
                        "%" => {
                            if r == 0.0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["float modulo by zero".to_string()]));
                            }
                            Value::Float(l % r)
                        },
                        "**" => Value::Float(l.powf(r)),
                        "==" => Value::Bool(l == r),
                        "!=" => Value::Bool(l != r),
                        "<" => Value::Bool(l < r),
                        ">" => Value::Bool(l > r),
                        "<=" => Value::Bool(l <= r),
                        ">=" => Value::Bool(l >= r),
                        "and" => Value::Bool((l != 0.0) && (r != 0.0)),
                        "or" => Value::Bool((l != 0.0) || (r != 0.0)),
                        "is" => Value::Bool(l == r),
                        "is not" => Value::Bool(l != r),
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: 'float' and 'float'", op)])),
                    },
                    (Value::Int(l), Value::Float(r)) => match op.as_str() {
                        "+" => Value::Float((l as f64) + r),
                        "-" => Value::Float((l as f64) - r),
                        "*" => Value::Float((l as f64) * r),
                        "/" => {
                            if r == 0.0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["division by zero".to_string()]));
                            }
                            Value::Float((l as f64) / r)
                        },
                        "//" => {
                            if r == 0.0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["float floor division by zero".to_string()]));
                            }
                            Value::Float(((l as f64) / r).floor())
                        },
                        "%" => {
                            if r == 0.0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["float modulo by zero".to_string()]));
                            }
                            Value::Float((l as f64) % r)
                        },
                        "**" => Value::Float((l as f64).powf(r)),
                        "==" => Value::Bool((l as f64) == r),
                        "!=" => Value::Bool((l as f64) != r),
                        "<" => Value::Bool((l as f64) < r),
                        ">" => Value::Bool((l as f64) > r),
                        "<=" => Value::Bool((l as f64) <= r),
                        ">=" => Value::Bool((l as f64) >= r),
                        "and" => Value::Bool(((l != 0) && (r != 0.0))),
                        "or" => Value::Bool(((l != 0) || (r != 0.0))),
                        "is" => Value::Bool((l as f64) == r),
                        "is not" => Value::Bool((l as f64) != r),
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: 'int' and 'float'", op)])),
                    },
                    (Value::Float(l), Value::Int(r)) => match op.as_str() {
                        "+" => Value::Float(l + (r as f64)),
                        "-" => Value::Float(l - (r as f64)),
                        "*" => Value::Float(l * (r as f64)),
                        "/" => {
                            if r == 0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["division by zero".to_string()]));
                            }
                            Value::Float(l / (r as f64))
                        },
                        "//" => {
                            if r == 0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["float floor division by zero".to_string()]));
                            }
                            Value::Float((l / (r as f64)).floor())
                        },
                        "%" => {
                            if r == 0 {
                                return Value::Exception(Exception::new(ExceptionKind::ZeroDivisionError, vec!["float modulo by zero".to_string()]));
                            }
                            Value::Float(l % (r as f64))
                        },
                        "**" => Value::Float(l.powf(r as f64)),
                        "==" => Value::Bool(l == (r as f64)),
                        "!=" => Value::Bool(l != (r as f64)),
                        "<" => Value::Bool(l < (r as f64)),
                        ">" => Value::Bool(l > (r as f64)),
                        "<=" => Value::Bool(l <= (r as f64)),
                        ">=" => Value::Bool(l >= (r as f64)),
                        "and" => Value::Bool(((l != 0.0) && (r != 0))),
                        "or" => Value::Bool(((l != 0.0) || (r != 0))),
                        "is" => Value::Bool(l == (r as f64)),
                        "is not" => Value::Bool(l != (r as f64)),
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: 'float' and 'int'", op)])),
                    },
                    (Value::Str(l), Value::Str(r)) => match op.as_str() {
                        "+" => Value::Str(l + &r),
                        "==" => Value::Bool(l == r),
                        "!=" => Value::Bool(l != r),
                        "<" => Value::Bool(l < r),
                        ">" => Value::Bool(l > r),
                        "<=" => Value::Bool(l <= r),
                        ">=" => Value::Bool(l >= r),
                        "is" => Value::Bool(l == r),
                        "is not" => Value::Bool(l != r),
                        "in" => Value::Bool(r.contains(l)),
                        "not in" => Value::Bool(!r.contains(l)),
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: 'str' and 'str'", op)])),
                    },
                    (Value::Str(l), Value::Int(r)) if op == "*" => {
                        if r < 0 {
                            return Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["negative repetition count".to_string()]));
                        }
                        Value::Str(l.repeat(r as usize))
                    },
                    (Value::Int(l), Value::Str(r)) if op == "*" => {
                        if l < 0 {
                            return Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["negative repetition count".to_string()]));
                        }
                        Value::Str(r.repeat(l as usize))
                    },
                    (Value::Bool(l), Value::Bool(r)) => match op.as_str() {
                        "and" => Value::Bool(l && r),
                        "or" => Value::Bool(l || r),
                        "==" => Value::Bool(l == r),
                        "!=" => Value::Bool(l != r),
                        "is" => Value::Bool(l == r),
                        "is not" => Value::Bool(l != r),
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: 'bool' and 'bool'", op)])),
                    },
                    (Value::List(l), Value::List(r)) if op == "+" => {
                        let mut new_list = l.clone();
                        new_list.extend(r.clone());
                        Value::List(new_list)
                    },
                    (Value::List(l), Value::Int(r)) if op == "*" => {
                        if r < 0 {
                            return Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["negative repetition count".to_string()]));
                        }
                        let mut new_list = Vec::new();
                        for _ in 0..(r as usize) {
                            new_list.extend_from_slice(&l);
                        }
                        Value::List(new_list)
                    },
                    (Value::Int(l), Value::List(r)) if op == "*" => {
                        if l < 0 {
                            return Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["negative repetition count".to_string()]));
                        }
                        let mut new_list = Vec::new();
                        for _ in 0..(l as usize) {
                            new_list.extend_from_slice(&r);
                        }
                        Value::List(new_list)
                    },
                    (Value::List(l), r_val) if op == "in" => {
                        Value::Bool(l.contains(&r_val))
                    },
                    (Value::List(l), r_val) if op == "not in" => {
                        Value::Bool(!l.contains(&r_val))
                    },
                    (Value::None, Value::None) if op == "is" => Value::Bool(true),
                    (Value::None, Value::None) if op == "is not" => Value::Bool(false),
                    (Value::None, _) if op == "is" => Value::Bool(false),
                    (Value::None, _) if op == "is not" => Value::Bool(true),
                    (_, Value::None) if op == "is" => Value::Bool(false),
                    (_, Value::None) if op == "is not" => Value::Bool(true),
                    (l_val, r_val) if op == "is" => Value::Bool(l_val == r_val), // Fallback for other types
                    (l_val, r_val) if op == "is not" => Value::Bool(l_val != r_val), // Fallback for other types
                    _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("unsupported operand type(s) for {}: '{}' and '{}'", op, l.type_name(), r.type_name())])),
                }
            }
            Expr::UnaryOp { op, expr } => {
                let v = self.eval_inner(expr);
                match (op.as_str(), v) {
                    ("-", Value::Int(n)) => Value::Int(-n),
                    ("-", Value::Float(n)) => Value::Float(-n),
                    ("not", Value::Bool(b)) => Value::Bool(!b),
                    ("not", Value::Int(n)) => Value::Bool(n == 0),
                    ("~", Value::Int(n)) => Value::Int(!n),
                    _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("bad operand type for unary {}: '{}'", op, v.type_name())])),
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
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Null => Value::None,
            Expr::Block(exprs) => {
                let mut last = Value::None;
                for e in exprs {
                    match self.eval_inner(e) {
                        Value::Exception(ref exc) if exc.kind == ExceptionKind::Break || exc.kind == ExceptionKind::Continue => return Value::Exception(exc.clone()),
                        v => last = v,
                    }
                }
                last
            }
            Expr::If { cond, then_branch, else_branch } => {
                let cond_val = self.eval_inner(cond);
                let cond_bool = cond_val.is_truthy();
                if cond_bool {
                    self.eval_inner(then_branch)
                } else if let Some(else_b) = else_branch {
                    self.eval_inner(else_b)
                } else {
                    Value::None
                }
            }
            Expr::While { cond, body } => {
                let mut last = Value::None;
                'outer: while self.eval_inner(cond).is_truthy() {
                    match self.eval_inner(body) {
                        Value::Exception(ref exc) if exc.kind == ExceptionKind::Break => break 'outer,
                        Value::Exception(ref exc) if exc.kind == ExceptionKind::Continue => continue 'outer,
                        v => last = v,
                    }
                }
                last
            }
            Expr::FnDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), *body.clone()));
                Value::None
            }
            Expr::FnCall { name, args } => {
                // --- Built-in functions (e.g., print, input) ---
                match name.as_str() {
                    "print" => {
                        let mut output = String::new();
                        for (i, arg) in args.iter().enumerate() {
                            output.push_str(&self.eval_inner(arg).to_display_string());
                            if i < args.len() - 1 {
                                output.push(' ');
                            }
                        }
                        println!("{}", output);
                        return Value::None;
                    }
                    "input" => {
                        let prompt = if !args.is_empty() {
                            self.eval_inner(&args[0]).to_display_string()
                        } else {
                            "".to_string()
                        };
                        print!("{}", prompt);
                        use std::io::{self, Write};
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        return Value::Str(input.trim_end_matches(&['\r', '\n'][..]).to_string());
                    }
                    _ => { /* continue to check for bytes/bytearray methods or user-defined functions */ }
                }
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
                        _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Not a bytes or bytearray object".to_string()])),
                    };
                    match name.as_str() {
                        "isalnum" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isalnum = !b.is_empty() && b.iter().all(|c| (b'a' <= *c && *c <= b'z') || (b'A' <= *c && *c <= b'Z') || (b'0' <= *c && *c <= b'9'));
                                Value::Bool(isalnum)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isalnum() on non-bytes/bytearray object".to_string()])) }
                        }
                        "isalpha" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isalpha = !b.is_empty() && b.iter().all(|c| (b'a' <= *c && *c <= b'z') || (b'A' <= *c && *c <= b'Z'));
                                Value::Bool(isalpha)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isalpha() on non-bytes/bytearray object".to_string()])) }
                        }
                        "isascii" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isascii = b.iter().all(|c| *c <= 0x7F);
                                Value::Bool(isascii)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isascii() on non-bytes/bytearray object".to_string()])) }
                        }
                        "isdigit" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isdigit = !b.is_empty() && b.iter().all(|c| b'0' <= *c && *c <= b'9');
                                Value::Bool(isdigit)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isdigit() on non-bytes/bytearray object".to_string()])) }
                        }
                        "islower" => {
                            if let Some(b) = as_bytes(&obj) {
                                let has_lower = b.iter().any(|c| b'a' <= *c && *c <= b'z');
                                let has_upper = b.iter().any(|c| b'A' <= *c && *c <= b'Z');
                                Value::Bool(has_lower && !has_upper)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["islower() on non-bytes/bytearray object".to_string()])) }
                        }
                        "isspace" => {
                            if let Some(b) = as_bytes(&obj) {
                                let isspace = !b.is_empty() && b.iter().all(|c| matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c));
                                Value::Bool(isspace)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isspace() on non-bytes/bytearray object".to_string()])) }
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
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["istitle() on non-bytes/bytearray object".to_string()])) }
                        }
                        "isupper" => {
                            if let Some(b) = as_bytes(&obj) {
                                let has_upper = b.iter().any(|c| b'A' <= *c && *c <= b'Z');
                                let has_lower = b.iter().any(|c| b'a' <= *c && *c <= b'z');
                                Value::Bool(has_upper && !has_lower)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isupper() on non-bytes/bytearray object".to_string()])) }
                        }
                        "lower" => {
                            if let Some(b) = as_bytes(&obj) {
                                let out = b.iter().map(|c| if b'A' <= *c && *c <= b'Z' { *c + 32 } else { *c }).collect();
                                make_result(&obj, out)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["lower() on non-bytes/bytearray object".to_string()])) }
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
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["splitlines() on non-bytes/bytearray object".to_string()])) }
                        }
                        "swapcase" => {
                            if let Some(b) = as_bytes(&obj) {
                                let out = b.iter().map(|c| {
                                    if b'a' <= *c && *c <= b'z' { *c - 32 }
                                    else if b'A' <= *c && *c <= b'Z' { *c + 32 }
                                    else { *c }
                                }).collect();
                                make_result(&obj, out)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["swapcase() on non-bytes/bytearray object".to_string()])) }
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
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["title() on non-bytes/bytearray object".to_string()])) }
                        }
                        "upper" => {
                            if let Some(b) = as_bytes(&obj) {
                                let out = b.iter().map(|c| if b'a' <= *c && *c <= b'z' { *c - 32 } else { *c }).collect();
                                make_result(&obj, out)
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["upper() on non-bytes/bytearray object".to_string()])) }
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
                                    Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["zfill() expects width argument".to_string()]))
                                }
                            } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["zfill() on non-bytes/bytearray object".to_string()])) }
                        }
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'bytes' object has no attribute '{}'", name)])),
                    }
                }
                // ...existing code for user-defined functions...
            }
            Expr::Return(expr) => {
                // Use panic to unwind for return, or use a custom Result type in a real implementation
                let val = self.eval_inner(expr);
                panic!("__return__{:?}", val);
            }
            Expr::Break => Value::Exception(Exception::new(ExceptionKind::Break, vec![])),
            Expr::Continue => Value::Exception(Exception::new(ExceptionKind::Continue, vec![])),
            Expr::Match { expr, arms } => {
                let val = self.eval_inner(expr);
                for (pat, res) in arms {
                    let pat_val = self.eval_inner(pat);
                    if Self::pattern_match(&val, &pat_val) {
                        return self.eval_inner(res);
                    }
                }
                Value::None
            }
            Expr::StructDef { name, fields } => {
                // Store struct definition in env as a marker
                self.env.insert(format!("__struct__{}", name), Value::List(fields.iter().map(|f| Value::Str(f.clone())).collect()));
                Value::None
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
                Value::None
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
                            if s.starts_with("__exception__") {
                                // Extract the Exception from the panic message
                                let exception_str = &s["__exception__".len()..];
                                // This is a hacky way to deserialize the Exception from its Debug format.
                                // In a real scenario, you'd want a proper serialization/deserialization mechanism.
                                // For now, we'll just create a generic RuntimeError.
                                Value::Exception(Exception::new(ExceptionKind::RuntimeError, vec![format!("Caught exception: {}", exception_str)]))
                            } else {
                                Value::Exception(Exception::new(ExceptionKind::RuntimeError, vec![s.clone()]))
                            }
                        } else if let Some(s) = e.downcast_ref::<&str>() {
                            if s.starts_with("__exception__") {
                                let exception_str = &s["__exception__".len()..];
                                Value::Exception(Exception::new(ExceptionKind::RuntimeError, vec![format!("Caught exception: {}", exception_str)]))
                            } else {
                                Value::Exception(Exception::new(ExceptionKind::RuntimeError, vec![s.to_string()]))
                            }
                        } else {
                            Value::Exception(Exception::new(ExceptionKind::RuntimeError, vec!["Unknown error".to_string()]))
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
                    Value::Exception(e) => panic!("__exception__{:?}", e),
                    Value::Str(s) => panic!("__exception__{:?}", Exception::new(ExceptionKind::Exception, vec![s])),
                    _ => panic!("__exception__{:?}", Exception::new(ExceptionKind::Exception, vec!["thrown value".to_string()])),
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
                    Value::Exception(Exception::new(ExceptionKind::ImportError, vec![format!("Failed to import {}", path)]))
                }
            }
            Expr::GetAttr { object, name } => {
                let obj_val = self.eval_inner(object);
                match obj_val {
                    Value::Str(s) => match name.as_str() {
                        "len" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_len".to_string() },
                        "upper" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_upper".to_string() },
                        "lower" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_lower".to_string() },
                        "strip" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_strip".to_string() },
                        "split" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_split".to_string() },
                        "join" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_join".to_string() },
                        "replace" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_replace".to_string() },
                        "find" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_find".to_string() },
                        "count" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_count".to_string() },
                        "startswith" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_startswith".to_string() },
                        "endswith" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_endswith".to_string() },
                        "isalnum" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_isalnum".to_string() },
                        "isalpha" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_isalpha".to_string() },
                        "isdigit" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_isdigit".to_string() },
                        "islower" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_islower".to_string() },
                        "isupper" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_isupper".to_string() },
                        "isspace" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_isspace".to_string() },
                        "istitle" => Value::BuiltinMethod { object: Box::new(Value::Str(s)), method_name: "str_istitle".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'str' object has no attribute '{}'", name)])),
                    },
                    Value::List(l) => match name.as_str() {
                        "append" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_append".to_string() },
                        "pop" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_pop".to_string() },
                        "extend" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_extend".to_string() },
                        "insert" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_insert".to_string() },
                        "remove" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_remove".to_string() },
                        "index" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_index".to_string() },
                        "count" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_count".to_string() },
                        "sort" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_sort".to_string() },
                        "reverse" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_reverse".to_string() },
                        "clear" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_clear".to_string() },
                        "copy" => Value::BuiltinMethod { object: Box::new(Value::List(l)), method_name: "list_copy".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'list' object has no attribute '{}'", name)])),
                    },
                    Value::Dict(d) => match name.as_str() {
                        "keys" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_keys".to_string() },
                        "values" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_values".to_string() },
                        "items" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_items".to_string() },
                        "get" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_get".to_string() },
                        "pop" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_pop".to_string() },
                        "update" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_update".to_string() },
                        "clear" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_clear".to_string() },
                        "copy" => Value::BuiltinMethod { object: Box::new(Value::Dict(d)), method_name: "dict_copy".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'dict' object has no attribute '{}'", name)])),
                    },
                    Value::Set(s) => match name.as_str() {
                        "add" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_add".to_string() },
                        "remove" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_remove".to_string() },
                        "discard" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_discard".to_string() },
                        "pop" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_pop".to_string() },
                        "clear" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_clear".to_string() },
                        "union" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_union".to_string() },
                        "intersection" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_intersection".to_string() },
                        "difference" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_difference".to_string() },
                        "symmetric_difference" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_symmetric_difference".to_string() },
                        "issubset" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_issubset".to_string() },
                        "issuperset" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_issuperset".to_string() },
                        "isdisjoint" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_isdisjoint".to_string() },
                        "copy" => Value::BuiltinMethod { object: Box::new(Value::Set(s)), method_name: "set_copy".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'set' object has no attribute '{}'", name)])),
                    },
                    Value::FrozenSet(s) => match name.as_str() {
                        "union" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_union".to_string() },
                        "intersection" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_intersection".to_string() },
                        "difference" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_difference".to_string() },
                        "symmetric_difference" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_symmetric_difference".to_string() },
                        "issubset" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_issubset".to_string() },
                        "issuperset" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_issuperset".to_string() },
                        "isdisjoint" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_isdisjoint".to_string() },
                        "copy" => Value::BuiltinMethod { object: Box::new(Value::FrozenSet(s)), method_name: "frozenset_copy".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'frozenset' object has no attribute '{}'", name)])),
                    },
                    Value::Bytes(b) => match name.as_str() {
                        "len" => Value::BuiltinMethod { object: Box::new(Value::Bytes(b)), method_name: "bytes_len".to_string() },
                        "hex" => Value::BuiltinMethod { object: Box::new(Value::Bytes(b)), method_name: "bytes_hex".to_string() },
                        "decode" => Value::BuiltinMethod { object: Box::new(Value::Bytes(b)), method_name: "bytes_decode".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'bytes' object has no attribute '{}'", name)])),
                    },
                    Value::ByteArray(b) => match name.as_str() {
                        "len" => Value::BuiltinMethod { object: Box::new(Value::ByteArray(b)), method_name: "bytearray_len".to_string() },
                        "hex" => Value::BuiltinMethod { object: Box::new(Value::ByteArray(b)), method_name: "bytearray_hex".to_string() },
                        "decode" => Value::BuiltinMethod { object: Box::new(Value::ByteArray(b)), method_name: "bytearray_decode".to_string() },
                        "append" => Value::BuiltinMethod { object: Box::new(Value::ByteArray(b)), method_name: "bytearray_append".to_string() },
                        "pop" => Value::BuiltinMethod { object: Box::new(Value::ByteArray(b)), method_name: "bytearray_pop".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'bytearray' object has no attribute '{}'", name)])),
                    },
                    Value::Tuple(t) => match name.as_str() {
                        "count" => Value::BuiltinMethod { object: Box::new(Value::Tuple(t)), method_name: "tuple_count".to_string() },
                        "index" => Value::BuiltinMethod { object: Box::new(Value::Tuple(t)), method_name: "tuple_index".to_string() },
                        _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'tuple' object has no attribute '{}'", name)])),
                    },
                    _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("'{}' object has no attribute '{}'", obj_val.type_name(), name)])),
                }
            }
            Expr::FnCall { callable, args } => {
                let callable_val = self.eval_inner(callable);
                let evaluated_args: Vec<Value> = args.iter().map(|arg| self.eval_inner(arg)).collect();

                match callable_val {
                    Value::BuiltinMethod { object, method_name } => {
                        match method_name.as_str() {
                            // String methods
                            "str_len" => {
                                if let Value::Str(s) = *object { Value::Int(s.len() as i64) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_upper" => {
                                if let Value::Str(s) = *object { Value::Str(s.to_uppercase()) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_lower" => {
                                if let Value::Str(s) = *object { Value::Str(s.to_lowercase()) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_strip" => {
                                if let Value::Str(s) = *object { Value::Str(s.trim().to_string()) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_split" => {
                                if let Value::Str(s) = *object {
                                    let delimiter = if evaluated_args.is_empty() {
                                        None
                                    } else if let Value::Str(d) = &evaluated_args[0] {
                                        Some(d.as_str())
                                    } else {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["split() delimiter must be string".to_string()]));
                                    };
                                    let parts: Vec<Value> = if let Some(d) = delimiter {
                                        s.split(d).map(|p| Value::Str(p.to_string())).collect()
                                    } else {
                                        s.split_whitespace().map(|p| Value::Str(p.to_string())).collect()
                                    };
                                    Value::List(parts)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_join" => {
                                if let Value::Str(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["join() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::List(list_to_join) = &evaluated_args[0] {
                                        let mut result = String::new();
                                        for (i, item) in list_to_join.iter().enumerate() {
                                            if let Value::Str(item_s) = item {
                                                result.push_str(item_s);
                                                if i < list_to_join.len() - 1 {
                                                    result.push_str(&s);
                                                }
                                            } else {
                                                return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["sequence item 0: expected str instance, {} found".to_string()]));
                                            }
                                        }
                                        Value::Str(result)
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["join() argument must be a list of strings".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_replace" => {
                                if let Value::Str(s) = *object {
                                    if evaluated_args.len() < 2 || evaluated_args.len() > 3 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["replace() takes 2 or 3 arguments".to_string()]));
                                    }
                                    let old_val = if let Value::Str(v) = &evaluated_args[0] { v } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["replace() argument 1 must be string".to_string()])); };
                                    let new_val = if let Value::Str(v) = &evaluated_args[1] { v } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["replace() argument 2 must be string".to_string()])); };
                                    let count = if evaluated_args.len() == 3 {
                                        if let Value::Int(c) = &evaluated_args[2] { Some(*c as usize) } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["replace() argument 3 must be int".to_string()])); }
                                    } else { None };

                                    let result = if let Some(c) = count {
                                        s.replacen(old_val, new_val, c)
                                    } else {
                                        s.replace(old_val, new_val)
                                    };
                                    Value::Str(result)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_find" => {
                                if let Value::Str(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["find() takes exactly one argument".to_string()]));
                                    }
                                    let sub = if let Value::Str(v) = &evaluated_args[0] { v } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["find() argument must be string".to_string()])); };
                                    Value::Int(s.find(sub).map_or(-1, |idx| idx as i64))
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_count" => {
                                if let Value::Str(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["count() takes exactly one argument".to_string()]));
                                    }
                                    let sub = if let Value::Str(v) = &evaluated_args[0] { v } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["count() argument must be string".to_string()])); };
                                    Value::Int(s.matches(sub).count() as i64)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_startswith" => {
                                if let Value::Str(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["startswith() takes exactly one argument".to_string()]));
                                    }
                                    let prefix = if let Value::Str(v) = &evaluated_args[0] { v } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["startswith() argument must be string".to_string()])); };
                                    Value::Bool(s.starts_with(prefix))
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_endswith" => {
                                if let Value::Str(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["endswith() takes exactly one argument".to_string()]));
                                    }
                                    let suffix = if let Value::Str(v) = &evaluated_args[0] { v } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["endswith() argument must be string".to_string()])); };
                                    Value::Bool(s.ends_with(suffix))
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_isalnum" => {
                                if let Value::Str(s) = *object { Value::Bool(s.chars().all(char::is_alphanumeric)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_isalpha" => {
                                if let Value::Str(s) = *object { Value::Bool(s.chars().all(char::is_alphabetic)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_isdigit" => {
                                if let Value::Str(s) = *object { Value::Bool(s.chars().all(char::is_ascii_digit)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_islower" => {
                                if let Value::Str(s) = *object { Value::Bool(s.chars().all(char::is_lowercase)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_isupper" => {
                                if let Value::Str(s) = *object { Value::Bool(s.chars().all(char::is_uppercase)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_isspace" => {
                                if let Value::Str(s) = *object { Value::Bool(s.chars().all(char::is_whitespace)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            "str_istitle" => {
                                if let Value::Str(s) = *object {
                                    let mut prev_is_space = true;
                                    let mut is_title = true;
                                    for c in s.chars() {
                                        if c.is_uppercase() {
                                            if !prev_is_space { is_title = false; break; }
                                            prev_is_space = false;
                                        } else if c.is_lowercase() {
                                            if prev_is_space { is_title = false; break; }
                                            prev_is_space = false;
                                        } else {
                                            prev_is_space = c.is_whitespace();
                                        }
                                    }
                                    Value::Bool(is_title)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected string object".to_string()])) }
                            },
                            // List methods
                            "list_append" => {
                                if let Value::List(mut l) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["append() takes exactly one argument".to_string()]));
                                    }
                                    l.push(evaluated_args[0].clone());
                                    // Update the environment with the modified list if it was a variable
                                    // This is a simplification; proper object mutation would be more complex
                                    // For now, we return None as Python list.append() does
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_pop" => {
                                if let Value::List(mut l) = *object {
                                    if !evaluated_args.is_empty() {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["pop() takes no arguments".to_string()]));
                                    }
                                    l.pop().unwrap_or(Value::Exception(Exception::new(ExceptionKind::IndexError, vec!["pop from empty list".to_string()])))
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_extend" => {
                                if let Value::List(mut l) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["extend() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::List(other) = &evaluated_args[0] {
                                        l.extend(other.clone());
                                        Value::None
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["extend() argument must be a list".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_insert" => {
                                if let Value::List(mut l) = *object {
                                    if evaluated_args.len() != 2 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["insert() takes exactly two arguments".to_string()]));
                                    }
                                    let index = if let Value::Int(i) = &evaluated_args[0] { *i as usize } else { return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["insert() index must be an integer".to_string()])); };
                                    let value = evaluated_args[1].clone();
                                    if index > l.len() {
                                        l.push(value); // Python appends if index is too large
                                    } else {
                                        l.insert(index, value);
                                    }
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_remove" => {
                                if let Value::List(mut l) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["remove() takes exactly one argument".to_string()]));
                                    }
                                    let value_to_remove = &evaluated_args[0];
                                    if let Some(pos) = l.iter().position(|x| x == value_to_remove) {
                                        l.remove(pos);
                                        Value::None
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["list.remove(x): x not in list".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_index" => {
                                if let Value::List(l) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["index() takes exactly one argument".to_string()]));
                                    }
                                    let value_to_find = &evaluated_args[0];
                                    if let Some(pos) = l.iter().position(|x| x == value_to_find) {
                                        Value::Int(pos as i64)
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["'{}' is not in list".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_count" => {
                                if let Value::List(l) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["count() takes exactly one argument".to_string()]));
                                    }
                                    let value_to_count = &evaluated_args[0];
                                    Value::Int(l.iter().filter(|x| *x == value_to_count).count() as i64)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_sort" => {
                                if let Value::List(mut l) = *object {
                                    // This is a simplified sort. For full Python compatibility,
                                    // it would need 'key' and 'reverse' arguments, and a way to compare arbitrary Value types.
                                    // For now, only works if elements are comparable (e.g., Int, Float, Str)
                                    let mut sortable = true;
                                    for i in 0..l.len() {
                                        for j in (i + 1)..l.len() {
                                            if !l[i].eq(&l[j]) { // Check if elements are comparable
                                                // This is a very basic check, a proper comparison would be needed
                                            }
                                        }
                                    }
                                    if sortable {
                                        // Requires Value to implement Ord, which it currently doesn't fully.
                                        // For now, this will panic if elements are not naturally sortable.
                                        // A proper solution would involve implementing Ord for Value or a custom comparator.
                                        // l.sort(); // This line would require Value to be Ord
                                        Value::Exception(Exception::new(ExceptionKind::NotImplementedError, vec!["list.sort() for arbitrary types not yet implemented".to_string()]))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["'<' not supported between instances of different types".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_reverse" => {
                                if let Value::List(mut l) = *object {
                                    l.reverse();
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_clear" => {
                                if let Value::List(mut l) = *object {
                                    l.clear();
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            "list_copy" => {
                                if let Value::List(l) = *object {
                                    Value::List(l.clone())
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected list object".to_string()])) }
                            },
                            // Dict methods
                            "dict_keys" => {
                                if let Value::Dict(d) = *object {
                                    Value::List(d.keys().cloned().collect())
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_values" => {
                                if let Value::Dict(d) = *object {
                                    Value::List(d.values().cloned().collect())
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_items" => {
                                if let Value::Dict(d) = *object {
                                    let items: Vec<Value> = d.iter().map(|(k, v)| Value::Tuple(vec![k.clone(), v.clone()])).collect();
                                    Value::List(items)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_get" => {
                                if let Value::Dict(d) = *object {
                                    if evaluated_args.is_empty() || evaluated_args.len() > 2 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["get() takes 1 or 2 arguments".to_string()]));
                                    }
                                    let key = &evaluated_args[0];
                                    d.get(key).cloned().unwrap_or_else(|| {
                                        if evaluated_args.len() == 2 {
                                            evaluated_args[1].clone()
                                        } else {
                                            Value::None
                                        }
                                    })
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_pop" => {
                                if let Value::Dict(mut d) = *object {
                                    if evaluated_args.is_empty() || evaluated_args.len() > 2 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["pop() takes 1 or 2 arguments".to_string()]));
                                    }
                                    let key = &evaluated_args[0];
                                    d.remove(key).unwrap_or_else(|| {
                                        if evaluated_args.len() == 2 {
                                            evaluated_args[1].clone()
                                        } else {
                                            Value::Exception(Exception::new(ExceptionKind::KeyError, vec![key.to_display_string()]))
                                        }
                                    })
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_update" => {
                                if let Value::Dict(mut d) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["update() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Dict(other) = &evaluated_args[0] {
                                        d.extend(other.clone());
                                        Value::None
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["update() argument must be a dictionary".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_clear" => {
                                if let Value::Dict(mut d) = *object {
                                    d.clear();
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            "dict_copy" => {
                                if let Value::Dict(d) = *object {
                                    Value::Dict(d.clone())
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected dict object".to_string()])) }
                            },
                            // Set methods
                            "set_add" => {
                                if let Value::Set(mut s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["add() takes exactly one argument".to_string()]));
                                    }
                                    s.insert(evaluated_args[0].clone());
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_remove" => {
                                if let Value::Set(mut s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["remove() takes exactly one argument".to_string()]));
                                    }
                                    if s.remove(&evaluated_args[0]) {
                                        Value::None
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::KeyError, vec![evaluated_args[0].to_display_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_discard" => {
                                if let Value::Set(mut s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["discard() takes exactly one argument".to_string()]));
                                    }
                                    s.remove(&evaluated_args[0]);
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_pop" => {
                                if let Value::Set(mut s) = *object {
                                    if !evaluated_args.is_empty() {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["pop() takes no arguments".to_string()]));
                                    }
                                    s.drain().next().unwrap_or(Value::Exception(Exception::new(ExceptionKind::KeyError, vec!["pop from an empty set".to_string()])))
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_clear" => {
                                if let Value::Set(mut s) = *object {
                                    s.clear();
                                    Value::None
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_union" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["union() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Set(s.union(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["union() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_intersection" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["intersection() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Set(s.intersection(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["intersection() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_difference" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["difference() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Set(s.difference(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["difference() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_symmetric_difference" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["symmetric_difference() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Set(s.symmetric_difference(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["symmetric_difference() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_issubset" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issubset() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Bool(s.is_subset(other))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issubset() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_issuperset" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issuperset() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Bool(s.is_superset(other))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issuperset() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_isdisjoint" => {
                                if let Value::Set(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isdisjoint() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Set(other) = &evaluated_args[0] {
                                        Value::Bool(s.is_disjoint(other))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isdisjoint() argument must be a set".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            "set_copy" => {
                                if let Value::Set(s) = *object {
                                    Value::Set(s.clone())
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected set object".to_string()])) }
                            },
                            // FrozenSet methods (similar to set, but immutable)
                            "frozenset_union" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["union() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::FrozenSet(s.union(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["union() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_intersection" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["intersection() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::FrozenSet(s.intersection(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["intersection() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_difference" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["difference() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::FrozenSet(s.difference(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["difference() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_symmetric_difference" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["symmetric_difference() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::FrozenSet(s.symmetric_difference(other).cloned().collect())
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["symmetric_difference() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_issubset" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issubset() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::Bool(s.is_subset(other))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issubset() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_issuperset" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issuperset() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::Bool(s.is_superset(other))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["issuperset() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_isdisjoint" => {
                                if let Value::FrozenSet(s) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isdisjoint() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::FrozenSet(other) = &evaluated_args[0] {
                                        Value::Bool(s.is_disjoint(other))
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["isdisjoint() argument must be a frozenset".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            "frozenset_copy" => {
                                if let Value::FrozenSet(s) = *object {
                                    Value::FrozenSet(s.clone())
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected frozenset object".to_string()])) }
                            },
                            // Bytes methods
                            "bytes_len" => {
                                if let Value::Bytes(b) = *object { Value::Int(b.len() as i64) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytes object".to_string()])) }
                            },
                            "bytes_hex" => {
                                if let Value::Bytes(b) = *object { Value::Str(hex::encode(b)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytes object".to_string()])) }
                            },
                            "bytes_decode" => {
                                if let Value::Bytes(b) = *object {
                                    let encoding = if evaluated_args.is_empty() {
                                        "utf-8".to_string()
                                    } else if let Value::Str(e) = &evaluated_args[0] {
                                        e.clone()
                                    } else {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["decode() encoding must be string".to_string()]));
                                    };
                                    match encoding.as_str() {
                                        "utf-8" => {
                                            String::from_utf8(b).map_or_else(
                                                |e| Value::Exception(Exception::new(ExceptionKind::UnicodeDecodeError, vec![format!("'utf-8' codec can't decode byte: {}", e)])),
                                                |s| Value::Str(s)
                                            )
                                        },
                                        _ => Value::Exception(Exception::new(ExceptionKind::LookupError, vec![format!("unknown encoding: {}", encoding)])),
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytes object".to_string()])) }
                            },
                            // ByteArray methods
                            "bytearray_len" => {
                                if let Value::ByteArray(b) = *object { Value::Int(b.len() as i64) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytearray object".to_string()])) }
                            },
                            "bytearray_hex" => {
                                if let Value::ByteArray(b) = *object { Value::Str(hex::encode(b)) }
                                else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytearray object".to_string()])) }
                            },
                            "bytearray_decode" => {
                                if let Value::ByteArray(b) = *object {
                                    let encoding = if evaluated_args.is_empty() {
                                        "utf-8".to_string()
                                    } else if let Value::Str(e) = &evaluated_args[0] {
                                        e.clone()
                                    } else {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["decode() encoding must be string".to_string()]));
                                    };
                                    match encoding.as_str() {
                                        "utf-8" => {
                                            String::from_utf8(b).map_or_else(
                                                |e| Value::Exception(Exception::new(ExceptionKind::UnicodeDecodeError, vec![format!("'utf-8' codec can't decode byte: {}", e)])),
                                                |s| Value::Str(s)
                                            )
                                        },
                                        _ => Value::Exception(Exception::new(ExceptionKind::LookupError, vec![format!("unknown encoding: {}", encoding)])),
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytearray object".to_string()])) }
                            },
                            "bytearray_append" => {
                                if let Value::ByteArray(mut b) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["append() takes exactly one argument".to_string()]));
                                    }
                                    if let Value::Int(byte_val) = &evaluated_args[0] {
                                        if *byte_val >= 0 && *byte_val <= 255 {
                                            b.push(*byte_val as u8);
                                            Value::None
                                        } else {
                                            Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["byte must be in range(0, 256)".to_string()]))
                                        }
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["an integer is required (got type {})".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytearray object".to_string()])) }
                            },
                            "bytearray_pop" => {
                                if let Value::ByteArray(mut b) = *object {
                                    if !evaluated_args.is_empty() {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["pop() takes no arguments".to_string()]));
                                    }
                                    b.pop().map_or(Value::Exception(Exception::new(ExceptionKind::IndexError, vec!["pop from empty bytearray".to_string()])), |byte| Value::Int(byte as i64))
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected bytearray object".to_string()])) }
                            },
                            // Tuple methods
                            "tuple_count" => {
                                if let Value::Tuple(t) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["count() takes exactly one argument".to_string()]));
                                    }
                                    let value_to_count = &evaluated_args[0];
                                    Value::Int(t.iter().filter(|x| *x == value_to_count).count() as i64)
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected tuple object".to_string()])) }
                            },
                            "tuple_index" => {
                                if let Value::Tuple(t) = *object {
                                    if evaluated_args.len() != 1 {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["index() takes exactly one argument".to_string()]));
                                    }
                                    let value_to_find = &evaluated_args[0];
                                    if let Some(pos) = t.iter().position(|x| x == value_to_find) {
                                        Value::Int(pos as i64)
                                    } else {
                                        Value::Exception(Exception::new(ExceptionKind::ValueError, vec!["'{}' is not in tuple".to_string()]))
                                    }
                                } else { Value::Exception(Exception::new(ExceptionKind::TypeError, vec!["Expected tuple object".to_string()])) }
                            },
                            _ => Value::Exception(Exception::new(ExceptionKind::AttributeError, vec![format!("Unknown builtin method: {}", method_name)])),
                        }
                    },
                    Value::Ident(name) => { // Global function call
                        match name.as_str() {
                            "print" => {
                                let mut output = String::new();
                                for (i, arg) in evaluated_args.iter().enumerate() {
                                    output.push_str(&arg.to_display_string());
                                    if i < evaluated_args.len() - 1 {
                                        output.push(' ');
                                    }
                                }
                                println!("{}", output);
                                Value::None
                            }
                            "input" => {
                                let prompt = if !evaluated_args.is_empty() {
                                    evaluated_args[0].to_display_string()
                                } else {
                                    "".to_string()
                                };
                                print!("{}", prompt);
                                use std::io::{self, Write};
                                io::stdout().flush().unwrap();
                                let mut input = String::new();
                                io::stdin().read_line(&mut input).expect("Failed to read line");
                                Value::Str(input.trim_end_matches(&['\r', '\n'][..]).to_string())
                            }
                            _ => { // User-defined function
                                if let Some((params, body)) = self.functions.get(&name).cloned() {
                                    if params.len() != evaluated_args.len() {
                                        return Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("{}() takes {} arguments but {} were given", name, params.len(), evaluated_args.len())]));
                                    }
                                    let old_env = self.env.clone();
                                    for (param_name, arg_val) in params.into_iter().zip(evaluated_args.into_iter()) {
                                        self.env.insert(param_name, arg_val);
                                    }
                                    let result = self.eval_inner(&body);
                                    self.env = old_env; // Restore environment
                                    result
                                } else {
                                    Value::Exception(Exception::new(ExceptionKind::NameError, vec![format!("name '{}' is not defined", name)]))
                                }
                            }
                        }
                    },
                    _ => Value::Exception(Exception::new(ExceptionKind::TypeError, vec![format!("'{}' object is not callable", callable_val.type_name())])),
                }
            }
            _ => Value::Exception(Exception::new(ExceptionKind::NotImplementedError, vec![format!("Expression not implemented: {:?}", expr)])),
        }
    }

    // Helper for pattern matching
    fn pattern_match(val: &Value, pat: &Value) -> bool {
        match (val, pat) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::None, Value::None) => true,
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
            Value::Bool(b) => format!("{}", b),
            Value::None => "None".to_string(),
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
            Value::Iterator(_) => "<iterator object>".to_string(),
            Value::Generator(_) => "<generator object>".to_string(),
            Value::NotImplemented => "NotImplemented".to_string(),
            Value::Ellipsis => "Ellipsis".to_string(),
            Value::Complex(r, i) => format!("({}{}{}j)", r, if i >= 0.0 { "+" } else { "" }, i),
            Value::Tuple(t) => {
                let items: Vec<String> = t.iter().map(|v| v.to_display_string()).collect();
                format!("({})", items.join(", "))
            }
            Value::Exception(e) => format!("<Exception: {:?}>", e), // More detailed exception display
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Complex(_, _) => "complex",
            Value::Bool(_) => "bool",
            Value::Str(_) => "str",
            Value::Bytes(_) => "bytes",
            Value::ByteArray(_) => "bytearray",
            Value::MemoryView(_) => "memoryview",
            Value::List(_) => "list",
            Value::Tuple(_) => "tuple",
            Value::Range(_) => "range",
            Value::Set(_) => "set",
            Value::FrozenSet(_) => "frozenset",
            Value::Dict(_) => "dict",
            Value::Iterator(_) => "iterator",
            Value::Generator(_) => "generator",
            Value::None => "NoneType",
            Value::NotImplemented => "NotImplementedType",
            Value::Ellipsis => "EllipsisType",
            Value::Exception(_) => "Exception",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0 && !f.is_nan(),
            Value::Str(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Tuple(t) => !t.is_empty(),
            Value::Dict(d) => !d.is_empty(),
            Value::Set(s) => !s.is_empty(),
            Value::FrozenSet(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            Value::None => false,
            _ => true, // Other types are considered truthy for now
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Complex(ar, ai), Value::Complex(br, bi)) => ar == br && ai == bi,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bytes(a), Value::Bytes(b)) => a == b,
            (Value::ByteArray(a), Value::ByteArray(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Range(a), Value::Range(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a == b,
            (Value::FrozenSet(a), Value::FrozenSet(b)) => a == b,
            (Value::Dict(a), Value::Dict(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::NotImplemented, Value::NotImplemented) => true,
            (Value::Ellipsis, Value::Ellipsis) => true,
            // Allow comparison between Int and Float
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            // Allow comparison between Int/Float and Bool
            (Value::Int(a), Value::Bool(b)) => (*a != 0) == *b,
            (Value::Bool(a), Value::Int(b)) => *a == (*b != 0),
            (Value::Float(a), Value::Bool(b)) => (*a != 0.0) == *b,
            (Value::Bool(a), Value::Float(b)) => *a == (*b != 0.0),
            _ => false, // Different types are not equal by default
        }
    }
}

impl Eq for Value {}

use std::hash::{Hash, Hasher};
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Int(i) => i.hash(state),
            Value::Float(f) => f.to_bits().hash(state), // Hash float bits
            Value::Complex(r, i) => {
                r.to_bits().hash(state);
                i.to_bits().hash(state);
            },
            Value::Bool(b) => b.hash(state),
            Value::Str(s) => s.hash(state),
            Value::Bytes(b) => b.hash(state),
            Value::ByteArray(b) => b.hash(state),
            Value::List(l) => l.iter().for_each(|v| v.hash(state)), // Hash each element
            Value::Tuple(t) => t.iter().for_each(|v| v.hash(state)), // Hash each element
            Value::Range(r) => r.hash(state),
            Value::Set(s) => {
                let mut sorted_elements: Vec<&Value> = s.iter().collect();
                // Sorting by display string is a hack; a proper solution would require Value to be Ord
                sorted_elements.sort_by_key(|v| v.to_display_string());
                sorted_elements.iter().for_each(|v| v.hash(state));
            },
            Value::FrozenSet(s) => {
                let mut sorted_elements: Vec<&Value> = s.iter().collect();
                sorted_elements.sort_by_key(|v| v.to_display_string());
                sorted_elements.iter().for_each(|v| v.hash(state));
            },
            Value::Dict(d) => {
                let mut sorted_pairs: Vec<(&Value, &Value)> = d.iter().collect();
                sorted_pairs.sort_by_key(|(k, _)| k.to_display_string());
                sorted_pairs.iter().for_each(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
            },
            Value::None => 0.hash(state),
            Value::NotImplemented => 1.hash(state),
            Value::Ellipsis => 2.hash(state),
            Value::Exception(e) => e.hash(state),
            Value::Iterator(_) => "iterator".hash(state), // Hash type name for now
            Value::Generator(_) => "generator".hash(state), // Hash type name for now
        }
    }
}
