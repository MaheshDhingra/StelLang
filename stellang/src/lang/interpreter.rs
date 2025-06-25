// Interpreter for StelLang

use super::ast::Expr;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Error(String),
}

pub struct Interpreter {
    pub env: HashMap<String, Value>,
    pub functions: HashMap<String, (Vec<String>, Expr)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: HashMap::new(), functions: HashMap::new() }
    }

    pub fn eval(&mut self, expr: &Expr) -> Value {
        self.eval_inner(expr)
    }

    fn eval_inner(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Integer(n) => Value::Number(*n as f64),
            Expr::Float(f) => Value::Number(*f),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Ident(name) => self.env.get(name).cloned().unwrap_or(Value::Number(0.0)),
            Expr::ArrayLiteral(items) => {
                Value::Array(items.iter().map(|e| self.eval_inner(e)).collect())
            }
            Expr::MapLiteral(pairs) => {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    let key = match self.eval_inner(k) {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        _ => "".to_string(),
                    };
                    let val = self.eval_inner(v);
                    map.insert(key, val);
                }
                Value::Map(map)
            }
            Expr::Index { collection, index } => {
                let coll = self.eval_inner(collection);
                let idx = self.eval_inner(index);
                match (coll, idx) {
                    (Value::Array(arr), Value::Number(n)) => {
                        arr.get(n as usize).cloned().unwrap_or(Value::Number(0.0))
                    }
                    (Value::Map(map), Value::String(s)) => {
                        map.get(&s).cloned().unwrap_or(Value::Number(0.0))
                    }
                    (Value::Map(map), Value::Number(n)) => {
                        map.get(&n.to_string()).cloned().unwrap_or(Value::Number(0.0))
                    }
                    _ => Value::Number(0.0),
                }
            }
            Expr::AssignIndex { collection, index, expr } => {
                let coll = self.eval_inner(collection);
                let idx = self.eval_inner(index);
                let val = self.eval_inner(expr);
                match (coll, idx) {
                    (Value::Array(mut arr), Value::Number(n)) => {
                        let i = n as usize;
                        if i < arr.len() {
                            arr[i] = val.clone();
                        }
                        Value::Array(arr)
                    }
                    (Value::Map(mut map), Value::String(s)) => {
                        map.insert(s, val.clone());
                        Value::Map(map)
                    }
                    (Value::Map(mut map), Value::Number(n)) => {
                        map.insert(n.to_string(), val.clone());
                        Value::Map(map)
                    }
                    _ => Value::Number(0.0),
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let l = self.eval_inner(left);
                let r = self.eval_inner(right);
                match (l, r) {
                    (Value::Number(l), Value::Number(r)) => match op.as_str() {
                        "+" => Value::Number(l + r),
                        "-" => Value::Number(l - r),
                        "*" => Value::Number(l * r),
                        "/" => Value::Number(l / r),
                        "==" => Value::Number((l == r) as i32 as f64),
                        "!=" => Value::Number((l != r) as i32 as f64),
                        "<" => Value::Number((l < r) as i32 as f64),
                        ">" => Value::Number((l > r) as i32 as f64),
                        "<=" => Value::Number((l <= r) as i32 as f64),
                        ">=" => Value::Number((l >= r) as i32 as f64),
                        "and" => Value::Number(((l != 0.0) && (r != 0.0)) as i32 as f64),
                        "or" => Value::Number(((l != 0.0) || (r != 0.0)) as i32 as f64),
                        _ => Value::Number(0.0),
                    },
                    (Value::String(l), Value::String(r)) if op == "+" => Value::String(l + &r),
                    _ => Value::Number(0.0),
                }
            }
            Expr::UnaryOp { op, expr } => {
                let v = self.eval_inner(expr);
                match (op.as_str(), v) {
                    ("-", Value::Number(n)) => Value::Number(-n),
                    ("not", Value::Number(n)) => Value::Number((n == 0.0) as i32 as f64),
                    ("!", Value::Number(n)) => Value::Number((n == 0.0) as i32 as f64),
                    _ => Value::Number(0.0),
                }
            }
            Expr::Assign { name, expr } => {
                let val = self.eval_inner(expr);
                self.env.insert(name.clone(), val.clone());
                val
            }
            Expr::Block(exprs) => {
                let mut last = Value::Number(0.0);
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
                let cond_num = match cond_val { Value::Number(n) => n, _ => 0.0 };
                if cond_num != 0.0 {
                    self.eval_inner(then_branch)
                } else if let Some(else_b) = else_branch {
                    self.eval_inner(else_b)
                } else {
                    Value::Number(0.0)
                }
            }
            Expr::While { cond, body } => {
                let mut last = Value::Number(0.0);
                'outer: while match self.eval_inner(cond) { Value::Number(n) => n != 0.0, _ => false } {
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
                Value::Number(0.0)
            }
            Expr::FnCall { name, args } => {
                if name == "print" {
                    for arg in args {
                        let val = self.eval_inner(arg);
                        match val {
                            Value::Number(n) => println!("{}", n),
                            Value::String(s) => println!("{}", s),
                            Value::Array(arr) => println!("{:?}", arr),
                            Value::Map(map) => println!("{:?}", map),
                            Value::Error(e) => println!("Error: {}", e),
                        }
                    }
                    Value::Number(0.0)
                } else if name == "sqrt" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.sqrt()),
                            _ => Value::Error("sqrt expects a number".to_string()),
                        }
                    } else {
                        Value::Error("sqrt expects 1 argument".to_string())
                    }
                } else if name == "sin" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.sin()),
                            _ => Value::Error("sin expects a number".to_string()),
                        }
                    } else {
                        Value::Error("sin expects 1 argument".to_string())
                    }
                } else if name == "cos" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.cos()),
                            _ => Value::Error("cos expects a number".to_string()),
                        }
                    } else {
                        Value::Error("cos expects 1 argument".to_string())
                    }
                } else if name == "abs" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.abs()),
                            _ => Value::Error("abs expects a number".to_string()),
                        }
                    } else {
                        Value::Error("abs expects 1 argument".to_string())
                    }
                } else if name == "pow" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Number(a), Value::Number(b)) => Value::Number(a.powf(b)),
                            _ => Value::Error("pow expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("pow expects 2 arguments".to_string())
                    }
                } else if name == "len" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Array(arr) => Value::Number(arr.len() as f64),
                            Value::Map(map) => Value::Number(map.len() as f64),
                            Value::String(s) => Value::Number(s.len() as f64),
                            _ => Value::Error("len expects array, map, or string".to_string()),
                        }
                    } else {
                        Value::Error("len expects 1 argument".to_string())
                    }
                } else if name == "type_of" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(_) => Value::String("number".to_string()),
                            Value::String(_) => Value::String("string".to_string()),
                            Value::Array(_) => Value::String("array".to_string()),
                            Value::Map(_) => Value::String("map".to_string()),
                            Value::Error(_) => Value::String("error".to_string()),
                        }
                    } else {
                        Value::Error("type_of expects 1 argument".to_string())
                    }
                } else if name == "to_string" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::String(n.to_string()),
                            Value::String(s) => Value::String(s),
                            Value::Array(arr) => Value::String(format!("{:?}", arr)),
                            Value::Map(map) => Value::String(format!("{:?}", map)),
                            Value::Error(e) => Value::String(format!("Error: {}", e)),
                        }
                    } else {
                        Value::Error("to_string expects 1 argument".to_string())
                    }
                } else if let Some((params, body)) = self.functions.get(name).cloned() {
                    if params.len() != args.len() {
                        return Value::Error("Function argument count mismatch".to_string());
                    }
                    let backup = self.env.clone();
                    for (p, a) in params.iter().zip(args.iter()) {
                        let val = self.eval_inner(a);
                        self.env.insert(p.clone(), val);
                    }
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.eval_inner(&body)));
                    self.env = backup;
                    match result {
                        Ok(v) => v,
                        Err(e) => {
                            if let Some(s) = e.downcast_ref::<String>() {
                                if s.starts_with("__return__") {
                                    let val_str = &s[10..];
                                    // This is a hack: in a real interpreter, use Result/Option for returns
                                    Value::Error(format!("Return: {}", val_str))
                                } else {
                                    Value::Error(format!("Panic: {}", s))
                                }
                            } else {
                                Value::Error("Unknown panic".to_string())
                            }
                        }
                    }
                } else {
                    Value::Error(format!("Unknown function: {}", name))
                }
            }
            Expr::Return(expr) => {
                // Use panic to unwind for return, or use a custom Result type in a real implementation
                let val = self.eval_inner(expr);
                panic!("__return__{:?}", val);
            }
            Expr::Break => Value::Error("__break__".to_string()),
            Expr::Continue => Value::Error("__continue__".to_string()),
        }
    }
}
