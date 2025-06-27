// Interpreter for StelLang

use super::ast::Expr;
use std::collections::{HashMap, HashSet};
use std::ops::Range as StdRange;

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
    Error(String),
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
                        arr.get(n as usize).cloned().unwrap_or(Value::Int(0))
                    }
                    (Value::Dict(map), Value::Str(s)) => {
                        map.get(&s).cloned().unwrap_or(Value::Int(0))
                    }
                    (Value::Dict(map), Value::Int(n)) => {
                        map.get(&n.to_string()).cloned().unwrap_or(Value::Int(0))
                    }
                    _ => Value::Int(0),
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
                    Value::Error("Assignment to constant is not allowed".to_string())
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
                if name == "print" {
                    if args.is_empty() {
                        println!("");
                    } else {
                        for arg in args {
                            let val = self.eval_inner(arg);
                            println!("{}", val.to_display_string());
                        }
                    }
                    Value::Null // Return Null instead of Number(0.0)
                } else if name == "clear" || name == "cls" {
                    // Clear the terminal screen (Windows and Unix)
                    #[cfg(windows)] { std::process::Command::new("cmd").args(["/C", "cls"]).status().ok(); }
                    #[cfg(not(windows))] { std::process::Command::new("clear").status().ok(); }
                    Value::Null
                } else if name == "sqrt" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Float((n as f64).sqrt()),
                            Value::Float(n) => Value::Float(n.sqrt()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for sqrt".to_string()),
                        }
                    } else {
                        Value::Error("sqrt expects 1 argument".to_string())
                    }
                } else if name == "sin" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Float((n as f64).sin()),
                            Value::Float(n) => Value::Float(n.sin()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for sin".to_string()),
                        }
                    } else {
                        Value::Error("sin expects 1 argument".to_string())
                    }
                } else if name == "cos" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Float((n as f64).cos()),
                            Value::Float(n) => Value::Float(n.cos()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for cos".to_string()),
                        }
                    } else {
                        Value::Error("cos expects 1 argument".to_string())
                    }
                } else if name == "abs" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Int(n.abs()),
                            Value::Float(n) => Value::Float(n.abs()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for abs".to_string()),
                        }
                    } else {
                        Value::Error("abs expects 1 argument".to_string())
                    }
                } else if name == "pow" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as f64)),
                            (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
                            _ => Value::Error("pow expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("pow expects 2 arguments".to_string())
                    }
                } else if name == "len" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::Int(arr.len() as i64),
                            Value::Dict(map) => Value::Int(map.len() as i64),
                            Value::Str(s) => Value::Int(s.len() as i64),
                            _ => Value::Error("len expects array, map, or string".to_string()),
                        }
                    } else {
                        Value::Error("len expects 1 argument".to_string())
                    }
                } else if name == "type_of" || name == "type" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => {
                                if n == 0 {
                                    Value::String("integer".to_string())
                                } else {
                                    Value::String("decimal".to_string())
                                }
                            },
                            Value::Str(_) => Value::String("string".to_string()),
                            Value::List(_) => Value::String("array".to_string()),
                            Value::Dict(_) => Value::String("map".to_string()),
                            Value::Error(_) => Value::String("error".to_string()),
                            Value::Bool(_) => Value::String("bool".to_string()),
                            Value::Null => Value::String("null".to_string()),
                        }
                    } else {
                        Value::Error("type expects 1 argument".to_string())
                    }
                } else if name == "round" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Int(n),
                            Value::Float(n) => Value::Int(n.round() as i64),
                            _ => Value::Error("round expects a number".to_string()),
                        }
                    } else {
                        Value::Error("round expects 1 argument".to_string())
                    }
                } else if name == "floor" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Int(n),
                            Value::Float(n) => Value::Int(n.floor() as i64),
                            _ => Value::Error("floor expects a number".to_string()),
                        }
                    } else {
                        Value::Error("floor expects 1 argument".to_string())
                    }
                } else if name == "ceil" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Int(n),
                            Value::Float(n) => Value::Int(n.ceil() as i64),
                            _ => Value::Error("ceil expects a number".to_string()),
                        }
                    } else {
                        Value::Error("ceil expects 1 argument".to_string())
                    }
                } else if name == "int" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Int(n),
                            Value::Float(n) => Value::Int(n.trunc() as i64),
                            Value::Str(s) => s.parse::<i64>().map(Value::Int).unwrap_or(Value::Error("invalid string for int".to_string())),
                            _ => Value::Error("int expects a number or string".to_string()),
                        }
                    } else {
                        Value::Error("int expects 1 argument".to_string())
                    }
                } else if name == "float" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Float(n as f64),
                            Value::Float(n) => Value::Float(n),
                            Value::Str(s) => s.parse::<f64>().map(Value::Float).unwrap_or(Value::Error("invalid string for float".to_string())),
                            _ => Value::Error("float expects a number or string".to_string()),
                        }
                    } else {
                        Value::Error("float expects 1 argument".to_string())
                    }
                } else if name == "str" {
                    if args.len() == 1 {
                        let v = self.eval_inner(&args[0]);
                        Value::String(v.to_display_string())
                    } else {
                        Value::Error("str expects 1 argument".to_string())
                    }
                } else if name == "to_string" {
                    if args.len() == 1 {
                        let v = self.eval_inner(&args[0]);
                        Value::String(v.to_display_string())
                    } else {
                        Value::Error("to_string expects 1 argument".to_string())
                    }
                } else if name == "help" {
                    println!("Built-in functions: print, input, sqrt, sin, cos, abs, pow, len, type, type_of, to_string, str, int, float, round, floor, ceil, min, max, sum, range, map, filter, find, reduce, zip, map_keys, map_values, array_contains, array_index_of, help");
                    println!("Usage: print(x), type(x), int(x), float(x), str(x), round(x), floor(x), ceil(x), etc.");
                    Value::Null
                } else if name == "min" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a.min(b)),
                            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(b)),
                            _ => Value::Error("min expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("min expects 2 arguments".to_string())
                    }
                } else if name == "max" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a.max(b)),
                            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(b)),
                            _ => Value::Error("max expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("max expects 2 arguments".to_string())
                    }
                } else if name == "sum" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::Int(arr.iter().filter_map(|v| if let Value::Int(n) = v { Some(*n) } else { None }).sum()),
                            _ => Value::Error("sum expects an array of numbers".to_string()),
                        }
                    } else {
                        Value::Error("sum expects 1 argument".to_string())
                    }
                } else if name == "range" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Int(start), Value::Int(end)) => {
                                let arr = (start..end).map(|n| Value::Int(n)).collect();
                                Value::List(arr)
                            }
                            _ => Value::Error("range expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("range expects 2 arguments".to_string())
                    }
                } else if name == "map" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), &args[1]) {
                            (Value::List(arr), Expr::FnDef { name, params, body }) => {
                                let mut result = Vec::new();
                                for v in arr {
                                    let mut local_env = self.env.clone();
                                    if let Some(param) = params.get(0) {
                                        local_env.insert(param.clone(), v);
                                    }
                                    let mut sub_interpreter = Interpreter {
                                        env: local_env,
                                        functions: self.functions.clone(),
                                    };
                                    result.push(sub_interpreter.eval_inner(body));
                                }
                                Value::List(result)
                            }
                            _ => Value::Error("map expects an array and a function".to_string()),
                        }
                    } else {
                        Value::Error("map expects 2 arguments".to_string())
                    }
                } else if name == "filter" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), &args[1]) {
                            (Value::List(arr), Expr::FnDef { name, params, body }) => {
                                let mut result = Vec::new();
                                for v in arr {
                                    let mut local_env = self.env.clone();
                                    if let Some(param) = params.get(0) {
                                        local_env.insert(param.clone(), v.clone());
                                    }
                                    let mut sub_interpreter = Interpreter {
                                        env: local_env,
                                        functions: self.functions.clone(),
                                    };
                                    let cond = sub_interpreter.eval_inner(body);
                                    if let Value::Int(n) = cond {
                                        if n != 0 {
                                            result.push(v);
                                        }
                                    }
                                }
                                Value::List(result)
                            }
                            _ => Value::Error("filter expects an array and a function".to_string()),
                        }
                    } else {
                        Value::Error("filter expects 2 arguments".to_string())
                    }
                } else if name == "find" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), &args[1]) {
                            (Value::List(arr), Expr::FnDef { name, params, body }) => {
                                for v in arr {
                                    let mut local_env = self.env.clone();
                                    if let Some(param) = params.get(0) {
                                        local_env.insert(param.clone(), v.clone());
                                    }
                                    let mut sub_interpreter = Interpreter {
                                        env: local_env,
                                        functions: self.functions.clone(),
                                    };
                                    let cond = sub_interpreter.eval_inner(body);
                                    if let Value::Int(n) = cond {
                                        if n != 0 {
                                            return v;
                                        }
                                    }
                                }
                                Value::Null
                            }
                            _ => Value::Error("find expects an array and a function".to_string()),
                        }
                    } else {
                        Value::Error("find expects 2 arguments".to_string())
                    }
                } else if name == "reduce" {
                    if args.len() == 3 {
                        match (self.eval_inner(&args[0]), &args[1], self.eval_inner(&args[2])) {
                            (Value::List(arr), Expr::FnDef { name, params, body }, init) => {
                                let mut acc = init;
                                for v in arr {
                                    let mut local_env = self.env.clone();
                                    if let Some(p0) = params.get(0) {
                                        local_env.insert(p0.clone(), acc.clone());
                                    }
                                    if let Some(p1) = params.get(1) {
                                        local_env.insert(p1.clone(), v.clone());
                                    }
                                    let mut sub_interpreter = Interpreter {
                                        env: local_env,
                                        functions: self.functions.clone(),
                                    };
                                    acc = sub_interpreter.eval_inner(body);
                                }
                                acc
                            }
                            _ => Value::Error("reduce expects array, function, and initial value".to_string()),
                        }
                    } else {
                        Value::Error("reduce expects 3 arguments".to_string())
                    }
                } else if name == "zip" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::List(a), Value::List(b)) => {
                                let arr = a.into_iter().zip(b.into_iter()).map(|(x, y)| Value::List(vec![x, y])).collect();
                                Value::List(arr)
                            }
                            _ => Value::Error("zip expects two arrays".to_string()),
                        }
                    } else {
                        Value::Error("zip expects 2 arguments".to_string())
                    }
                } else if name == "map_keys" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Dict(map) => Value::List(map.keys().cloned().collect()),
                            _ => Value::Error("map_keys expects a map".to_string()),
                        }
                    } else {
                        Value::Error("map_keys expects 1 argument".to_string())
                    }
                } else if name == "map_values" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Dict(map) => Value::List(map.values().cloned().collect()),
                            _ => Value::Error("map_values expects a map".to_string()),
                        }
                    } else {
                        Value::Error("map_values expects 1 argument".to_string())
                    }
                } else if name == "array_contains" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::List(arr), v) => Value::Int(arr.contains(&v) as i32),
                            _ => Value::Error("array_contains expects array and value".to_string()),
                        }
                    } else {
                        Value::Error("array_contains expects 2 arguments".to_string())
                    }
                } else if name == "array_index_of" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::List(arr), v) => Value::Int(arr.iter().position(|x| x == &v).map(|i| i as i32).unwrap_or(-1)),
                            _ => Value::Error("array_index_of expects array and value".to_string()),
                        }
                    } else {
                        Value::Error("array_index_of expects 2 arguments".to_string())
                    }
                } else if name == "all" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::Bool(arr.iter().all(|v| match v { Value::Bool(b) => *b, Value::Int(n) => *n != 0, _ => true })),
                            _ => Value::Error("all expects an array".to_string()),
                        }
                    } else {
                        Value::Error("all expects 1 argument".to_string())
                    }
                } else if name == "any" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::Bool(arr.iter().any(|v| match v { Value::Bool(b) => *b, Value::Int(n) => *n != 0, _ => false })),
                            _ => Value::Error("any expects an array".to_string()),
                        }
                    } else {
                        Value::Error("any expects 1 argument".to_string())
                    }
                } else if name == "ascii" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Str(s) => Value::Str(s.chars().map(|c| if c.is_ascii() { c } else { '?' }).collect()),
                            _ => Value::Error("ascii expects a string".to_string()),
                        }
                    } else {
                        Value::Error("ascii expects 1 argument".to_string())
                    }
                } else if name == "bin" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Str(format!("0b{:b}", n)),
                            _ => Value::Error("bin expects a number".to_string()),
                        }
                    } else {
                        Value::Error("bin expects 1 argument".to_string())
                    }
                } else if name == "bool" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Bool(b) => Value::Bool(b),
                            Value::Int(n) => Value::Bool(n != 0),
                            Value::Null => Value::Bool(false),
                            Value::Str(s) => Value::Bool(!s.is_empty()),
                            Value::List(arr) => Value::Bool(!arr.is_empty()),
                            Value::Dict(map) => Value::Bool(!map.is_empty()),
                            _ => Value::Bool(false),
                        }
                    } else {
                        Value::Error("bool expects 1 argument".to_string())
                    }
                } else if name == "bytes" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Str(s) => Value::Bytes(s.bytes().collect()),
                            Value::List(arr) => Value::Bytes(arr.iter().filter_map(|v| if let Value::Int(n) = v { Some(*n as u8) } else { None }).collect()),
                            _ => Value::Error("bytes expects a string or array of numbers".to_string()),
                        }
                    } else {
                        Value::Error("bytes expects 1 argument".to_string())
                    }
                } else if name == "callable" {
                    if args.len() == 1 {
                        match &args[0] {
                            Expr::Ident(id) => Value::Bool(self.functions.contains_key(id)),
                            _ => Value::Bool(false),
                        }
                    } else {
                        Value::Error("callable expects 1 argument".to_string())
                    }
                } else if name == "chr" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => std::char::from_u32(n as u32).map(|c| Value::Str(c.to_string())).unwrap_or(Value::Error("invalid code point".to_string())),
                            _ => Value::Error("chr expects a number".to_string()),
                        }
                    } else {
                        Value::Error("chr expects 1 argument".to_string())
                    }
                } else if name == "complex" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Int(a), Value::Int(b)) => Value::String(format!("{}+{}j", a, b)),
                            (Value::Float(a), Value::Float(b)) => Value::String(format!("{}+{}j", a, b)),
                            _ => Value::Error("complex expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("complex expects 2 arguments".to_string())
                    }
                } else if name == "dict" {
                    if args.is_empty() {
                        Value::Dict(std::collections::HashMap::new())
                    } else if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => {
                                let mut map = std::collections::HashMap::new();
                                for v in arr {
                                    if let Value::List(pair) = v {
                                        if pair.len() == 2 {
                                            if let Value::Str(k) = &pair[0] {
                                                map.insert(k.clone(), pair[1].clone());
                                            }
                                        }
                                    }
                                }
                                Value::Dict(map)
                            }
                            _ => Value::Error("dict expects an array of [key, value] pairs".to_string()),
                        }
                    } else {
                        Value::Error("dict expects 0 or 1 argument".to_string())
                    }
                } else if name == "dir" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Dict(map) => Value::List(map.keys().cloned().collect()),
                            Value::List(_) => Value::List(vec![Value::Str("length".to_string())]),
                            Value::Str(_) => Value::List(vec![Value::Str("length".to_string())]),
                            _ => Value::Error("dir expects a map, array, or string".to_string()),
                        }
                    } else {
                        Value::Error("dir expects 1 argument".to_string())
                    }
                } else if name == "divmod" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Int(a), Value::Int(b)) if b != 0 => Value::List(vec![Value::Int((a / b)), Value::Int(a % b)]),
                            (Value::Float(a), Value::Float(b)) if b != 0.0 => Value::List(vec![Value::Float((a / b).trunc()), Value::Float(a % b)]),
                            _ => Value::Error("divmod expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("divmod expects 2 arguments".to_string())
                    }
                } else if name == "enumerate" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::List(arr.into_iter().enumerate().map(|(i, v)| Value::List(vec![Value::Int(i as i64), v])).collect()),
                            _ => Value::Error("enumerate expects an array".to_string()),
                        }
                    } else {
                        Value::Error("enumerate expects 1 argument".to_string())
                    }
                } else if name == "float" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Float(n as f64),
                            Value::Float(n) => Value::Float(n),
                            Value::Str(s) => s.parse::<f64>().map(Value::Float).unwrap_or(Value::Error("invalid string for float".to_string())),
                            _ => Value::Error("float expects a number or string".to_string()),
                        }
                    } else {
                        Value::Error("float expects 1 argument".to_string())
                    }
                } else if name == "format" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Str(fmt), Value::List(vals)) => {
                                let mut out = fmt;
                                for (i, v) in vals.iter().enumerate() {
                                    out = out.replace(&format!("{{{}}}", i), &v.to_display_string());
                                }
                                Value::Str(out)
                            }
                            _ => Value::Error("format expects a string and an array".to_string()),
                        }
                    } else {
                        Value::Error("format expects 2 arguments".to_string())
                    }
                } else if name == "frozenset" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => {
                                use std::collections::HashSet;
                                let set: HashSet<_> = arr.into_iter().collect();
                                Value::List(set.into_iter().collect())
                            }
                            _ => Value::Error("frozenset expects an array".to_string()),
                        }
                    } else {
                        Value::Error("frozenset expects 1 argument".to_string())
                    }
                } else if name == "getattr" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Dict(map), Value::Str(attr)) => map.get(&attr).cloned().unwrap_or(Value::Null),
                            _ => Value::Error("getattr expects a map and a string".to_string()),
                        }
                    } else {
                        Value::Error("getattr expects 2 arguments".to_string())
                    }
                } else if name == "hasattr" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Dict(map), Value::Str(attr)) => Value::Bool(map.contains_key(&attr)),
                            _ => Value::Error("hasattr expects a map and a string".to_string()),
                        }
                    } else {
                        Value::Error("hasattr expects 2 arguments".to_string())
                    }
                } else if name == "hash" {
                    if args.len() == 1 {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let v = self.eval_inner(&args[0]);
                        let mut hasher = DefaultHasher::new();
                        match &v {
                            Value::Int(n) => n.hash(&mut hasher),
                            Value::Float(n) => n.to_bits().hash(&mut hasher),
                            Value::Str(s) => s.hash(&mut hasher),
                            Value::Bool(b) => b.hash(&mut hasher),
                            _ => (),
                        }
                        Value::Int(hasher.finish() as i64)
                    } else {
                        Value::Error("hash expects 1 argument".to_string())
                    }
                } else if name == "hex" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Str(format!("0x{:x}", n)),
                            _ => Value::Error("hex expects a number".to_string()),
                        }
                    } else {
                        Value::Error("hex expects 1 argument".to_string())
                    }
                } else if name == "id" {
                    if args.len() == 1 {
                        let addr = &args[0] as *const _ as usize;
                        Value::Int(addr as i64)
                    } else {
                        Value::Error("id expects 1 argument".to_string())
                    }
                } else if name == "input" {
                    use std::io::{self, Write};
                    if args.is_empty() {
                        let mut s = String::new();
                        io::stdout().flush().ok();
                        io::stdin().read_line(&mut s).ok();
                        Value::String(s.trim_end().to_string())
                    } else if args.len() == 1 {
                        let prompt = self.eval_inner(&args[0]).to_display_string();
                        print!("{}", prompt);
                        io::stdout().flush().ok();
                        let mut s = String::new();
                        io::stdin().read_line(&mut s).ok();
                        Value::String(s.trim_end().to_string())
                    } else {
                        Value::Error("input expects 0 or 1 argument".to_string())
                    }
                } else if name == "int" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Int(n),
                            Value::Float(n) => Value::Int(n.trunc() as i64),
                            Value::Str(s) => s.parse::<i64>().map(Value::Int).unwrap_or(Value::Error("invalid string for int".to_string())),
                            _ => Value::Error("int expects a number or string".to_string()),
                        }
                    } else {
                        Value::Error("int expects 1 argument".to_string())
                    }
                } else if name == "isinstance" {
                    if args.len() == 2 {
                        let v = self.eval_inner(&args[0]);
                        let t = self.eval_inner(&args[1]);
                        let vtype = match v {
                            Value::Int(n) => if n == 0 { "integer" } else { "decimal" },
                            Value::Str(_) => "string",
                            Value::List(_) => "array",
                            Value::Dict(_) => "map",
                            Value::Bool(_) => "bool",
                            Value::Null => "null",
                            _ => "unknown",
                        };
                        match t {
                            Value::Str(s) => Value::Bool(vtype == s),
                            _ => Value::Error("isinstance expects a value and a type string".to_string()),
                        }
                    } else {
                        Value::Error("isinstance expects 2 arguments".to_string())
                    }
                } else if name == "issubclass" {
                    // Not implemented: no class hierarchy
                    Value::Error("issubclass not supported".to_string())
                } else if name == "iter" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::List(arr),
                            Value::Str(s) => Value::List(s.chars().map(|c| Value::Str(c.to_string())).collect()),
                            _ => Value::Error("iter expects an array or string".to_string()),
                        }
                    } else {
                        Value::Error("iter expects 1 argument".to_string())
                    }
                } else if name == "list" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::List(arr),
                            Value::Str(s) => Value::List(s.chars().map(|c| Value::Str(c.to_string())).collect()),
                            _ => Value::Error("list expects an array or string".to_string()),
                        }
                    } else {
                        Value::Error("list expects 1 argument".to_string())
                    }
                } else if name == "locals" {
                    // Not implemented: would require stack frame tracking
                    Value::Error("locals not supported".to_string())
                } else if name == "object" {
                    Value::Dict(std::collections::HashMap::new())
                } else if name == "oct" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Int(n) => Value::Str(format!("0o{:o}", n)),
                            _ => Value::Error("oct expects a number".to_string()),
                        }
                    } else {
                        Value::Error("oct expects 1 argument".to_string())
                    }
                } else if name == "ord" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Str(s) => s.chars().next().map(|c| Value::Int(c as u32 as i64)).unwrap_or(Value::Error("ord expects a non-empty string".to_string())),
                            _ => Value::Error("ord expects a string".to_string()),
                        }
                    } else {
                        Value::Error("ord expects 1 argument".to_string())
                    }
                } else if name == "repr" {
                    if args.len() == 1 {
                        let v = self.eval_inner(&args[0]);
                        Value::String(format!("{:?}", v))
                    } else {
                        Value::Error("repr expects 1 argument".to_string())
                    }
                } else if name == "reversed" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(mut arr) => { arr.reverse(); Value::List(arr) },
                            Value::Str(s) => Value::Str(s.chars().rev().collect()),
                            _ => Value::Error("reversed expects an array or string".to_string()),
                        }
                    } else {
                        Value::Error("reversed expects 1 argument".to_string())
                    }
                } else if name == "set" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => {
                                use std::collections::HashSet;
                                let set: HashSet<_> = arr.into_iter().collect();
                                Value::List(set.into_iter().collect())
                            }
                            _ => Value::Error("set expects an array".to_string()),
                        }
                    } else {
                        Value::Error("set expects 1 argument".to_string())
                    }
                } else if name == "slice" {
                    if args.len() == 3 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1]), self.eval_inner(&args[2])) {
                            (Value::List(arr), Value::Int(start), Value::Int(stop)) => {
                                let s = start as usize;
                                let e = stop as usize;
                                Value::List(arr[s..e.min(arr.len())].to_vec())
                            }
                            (Value::Str(s), Value::Int(start), Value::Int(stop)) => {
                                let sidx = start as usize;
                                let eidx = stop as usize;
                                Value::Str(s.chars().skip(sidx).take(eidx - sidx).collect())
                            }
                            _ => Value::Error("slice expects array/string, start, stop".to_string()),
                        }
                    } else {
                        Value::Error("slice expects 3 arguments".to_string())
                    }
                } else if name == "sorted" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(mut arr) => { arr.sort_by(|a, b| a.to_display_string().cmp(&b.to_display_string())); Value::List(arr) },
                            _ => Value::Error("sorted expects an array".to_string()),
                        }
                    } else {
                        Value::Error("sorted expects 1 argument".to_string())
                    }
                } else if name == "staticmethod" {
                    Value::Error("staticmethod not supported".to_string())
                } else if name == "str" {
                    if args.len() == 1 {
                        let v = self.eval_inner(&args[0]);
                        Value::String(v.to_display_string())
                    } else {
                        Value::Error("str expects 1 argument".to_string())
                    }
                } else if name == "sum" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::Int(arr.iter().filter_map(|v| if let Value::Int(n) = v { Some(*n) } else { None }).sum()),
                            _ => Value::Error("sum expects an array of numbers".to_string()),
                        }
                    } else {
                        Value::Error("sum expects 1 argument".to_string())
                    }
                } else if name == "super" {
                    Value::Error("super not supported".to_string())
                } else if name == "tuple" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(arr) => Value::List(arr),
                            _ => Value::Error("tuple expects an array".to_string()),
                        }
                    } else {
                        Value::Error("tuple expects 1 argument".to_string())
                    }
                } else if name == "vars" {
                    let arr = self.env.iter().map(|(k, v)| Value::List(vec![Value::Str(k.clone()), v.clone()])).collect();
                    Value::List(arr)
                } else if name == "delattr" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Dict(mut map), Value::Str(attr)) => { map.remove(&attr); Value::Dict(map) },
                            _ => Value::Error("delattr expects a map and a string".to_string()),
                        }
                    } else {
                        Value::Error("delattr expects 2 arguments".to_string())
                    }
                } else if name == "setattr" {
                    if args.len() == 3 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1]), self.eval_inner(&args[2])) {
                            (Value::Dict(mut map), Value::Str(attr), v) => { map.insert(attr, v); Value::Dict(map) },
                            _ => Value::Error("setattr expects a map, string, and value".to_string()),
                        }
                    } else {
                        Value::Error("setattr expects 3 arguments".to_string())
                    }
                } else if name == "next" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::List(mut arr) => if arr.is_empty() { Value::Null } else { arr.remove(0) },
                            _ => Value::Error("next expects an array".to_string()),
                        }
                    } else {
                        Value::Error("next expects 1 argument".to_string())
                    }
                } else if name == "breakpoint" {
                    Value::Error("breakpoint not supported".to_string())
                } else if name == "compile" {
                    Value::Error("compile not supported".to_string())
                } else if name == "eval" {
                    Value::Error("eval not supported".to_string())
                } else if name == "exec" {
                    Value::Error("exec not supported".to_string())
                } else if name == "open" {
                    Value::Error("open not supported".to_string())
                } else if name == "property" {
                    Value::Error("property not supported".to_string())
                } else if name == "classmethod" {
                    Value::Error("classmethod not supported".to_string())
                } else if name == "globals" {
                    Value::Error("globals not supported".to_string())
                } else if name == "memoryview" {
                    Value::Error("memoryview not supported".to_string())
                } else if name == "__import__" {
                    Value::Error("__import__ not supported".to_string())
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
