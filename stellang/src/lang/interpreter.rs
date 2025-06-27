// Interpreter for StelLang

use super::ast::Expr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Error(String),
    Bool(bool),
    Null,
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
                val // Return the assigned value, not just the first element
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
            Expr::Bool(b) => Value::Number(if *b { 1.0 } else { 0.0 }),
            Expr::Null => Value::Number(0.0),
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
                            Value::Number(n) => Value::Number(n.sqrt()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for sqrt".to_string()),
                        }
                    } else {
                        Value::Error("sqrt expects 1 argument".to_string())
                    }
                } else if name == "sin" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.sin()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for sin".to_string()),
                        }
                    } else {
                        Value::Error("sin expects 1 argument".to_string())
                    }
                } else if name == "cos" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.cos()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for cos".to_string()),
                        }
                    } else {
                        Value::Error("cos expects 1 argument".to_string())
                    }
                } else if name == "abs" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.abs()),
                            Value::Error(_) => Value::String("error".to_string()),
                            _ => Value::Error("Invalid type for abs".to_string()),
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
                } else if name == "type_of" || name == "type" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => {
                                if n.fract() == 0.0 {
                                    Value::String("integer".to_string())
                                } else {
                                    Value::String("decimal".to_string())
                                }
                            },
                            Value::String(_) => Value::String("string".to_string()),
                            Value::Array(_) => Value::String("array".to_string()),
                            Value::Map(_) => Value::String("map".to_string()),
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
                            Value::Number(n) => Value::Number(n.round()),
                            _ => Value::Error("round expects a number".to_string()),
                        }
                    } else {
                        Value::Error("round expects 1 argument".to_string())
                    }
                } else if name == "floor" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.floor()),
                            _ => Value::Error("floor expects a number".to_string()),
                        }
                    } else {
                        Value::Error("floor expects 1 argument".to_string())
                    }
                } else if name == "ceil" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.ceil()),
                            _ => Value::Error("ceil expects a number".to_string()),
                        }
                    } else {
                        Value::Error("ceil expects 1 argument".to_string())
                    }
                } else if name == "int" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n.trunc()),
                            Value::String(s) => s.parse::<f64>().map(|n| Value::Number(n.trunc())).unwrap_or(Value::Error("invalid string for int".to_string())),
                            _ => Value::Error("int expects a number or string".to_string()),
                        }
                    } else {
                        Value::Error("int expects 1 argument".to_string())
                    }
                } else if name == "float" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Number(n) => Value::Number(n),
                            Value::String(s) => s.parse::<f64>().map(Value::Number).unwrap_or(Value::Error("invalid string for float".to_string())),
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
                            (Value::Number(a), Value::Number(b)) => Value::Number(a.min(b)),
                            _ => Value::Error("min expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("min expects 2 arguments".to_string())
                    }
                } else if name == "max" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Number(a), Value::Number(b)) => Value::Number(a.max(b)),
                            _ => Value::Error("max expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("max expects 2 arguments".to_string())
                    }
                } else if name == "sum" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Array(arr) => Value::Number(arr.iter().filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None }).sum()),
                            _ => Value::Error("sum expects an array of numbers".to_string()),
                        }
                    } else {
                        Value::Error("sum expects 1 argument".to_string())
                    }
                } else if name == "range" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Number(start), Value::Number(end)) => {
                                let arr = (start as i64..end as i64).map(|n| Value::Number(n as f64)).collect();
                                Value::Array(arr)
                            }
                            _ => Value::Error("range expects two numbers".to_string()),
                        }
                    } else {
                        Value::Error("range expects 2 arguments".to_string())
                    }
                } else if name == "map" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), &args[1]) {
                            (Value::Array(arr), Expr::FnDef { name, params, body }) => {
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
                                Value::Array(result)
                            }
                            _ => Value::Error("map expects an array and a function".to_string()),
                        }
                    } else {
                        Value::Error("map expects 2 arguments".to_string())
                    }
                } else if name == "filter" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), &args[1]) {
                            (Value::Array(arr), Expr::FnDef { name, params, body }) => {
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
                                    if let Value::Number(n) = cond {
                                        if n != 0.0 {
                                            result.push(v);
                                        }
                                    }
                                }
                                Value::Array(result)
                            }
                            _ => Value::Error("filter expects an array and a function".to_string()),
                        }
                    } else {
                        Value::Error("filter expects 2 arguments".to_string())
                    }
                } else if name == "find" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), &args[1]) {
                            (Value::Array(arr), Expr::FnDef { name, params, body }) => {
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
                                    if let Value::Number(n) = cond {
                                        if n != 0.0 {
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
                            (Value::Array(arr), Expr::FnDef { name, params, body }, init) => {
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
                            (Value::Array(a), Value::Array(b)) => {
                                let arr = a.into_iter().zip(b.into_iter()).map(|(x, y)| Value::Array(vec![x, y])).collect();
                                Value::Array(arr)
                            }
                            _ => Value::Error("zip expects two arrays".to_string()),
                        }
                    } else {
                        Value::Error("zip expects 2 arguments".to_string())
                    }
                } else if name == "map_keys" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Map(map) => Value::Array(map.keys().map(|k| Value::String(k.clone())).collect()),
                            _ => Value::Error("map_keys expects a map".to_string()),
                        }
                    } else {
                        Value::Error("map_keys expects 1 argument".to_string())
                    }
                } else if name == "map_values" {
                    if args.len() == 1 {
                        match self.eval_inner(&args[0]) {
                            Value::Map(map) => Value::Array(map.values().cloned().collect()),
                            _ => Value::Error("map_values expects a map".to_string()),
                        }
                    } else {
                        Value::Error("map_values expects 1 argument".to_string())
                    }
                } else if name == "array_contains" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Array(arr), v) => Value::Number(arr.contains(&v) as i32 as f64),
                            _ => Value::Error("array_contains expects array and value".to_string()),
                        }
                    } else {
                        Value::Error("array_contains expects 2 arguments".to_string())
                    }
                } else if name == "array_index_of" {
                    if args.len() == 2 {
                        match (self.eval_inner(&args[0]), self.eval_inner(&args[1])) {
                            (Value::Array(arr), v) => Value::Number(arr.iter().position(|x| x == &v).map(|i| i as f64).unwrap_or(-1.0)),
                            _ => Value::Error("array_index_of expects array and value".to_string()),
                        }
                    } else {
                        Value::Error("array_index_of expects 2 arguments".to_string())
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
                self.env.insert(format!("__struct__{}", name), Value::Array(fields.iter().map(|f| Value::String(f.clone())).collect()));
                Value::Null
            }
            Expr::StructInit { name, fields } => {
                // Create a map with struct fields
                let mut map = std::collections::HashMap::new();
                for (k, v) in fields {
                    map.insert(k.clone(), self.eval_inner(v));
                }
                map.insert("__struct_name__".to_string(), Value::String(name.clone()));
                Value::Map(map)
            }
            Expr::EnumDef { name, variants } => {
                // Store enum definition in env as a marker
                self.env.insert(format!("__enum__{}", name), Value::Array(variants.iter().map(|v| Value::String(v.clone())).collect()));
                Value::Null
            }
            Expr::EnumInit { name, variant, value } => {
                let mut map = std::collections::HashMap::new();
                map.insert("__enum_name__".to_string(), Value::String(name.clone()));
                map.insert("__variant__".to_string(), Value::String(variant.clone()));
                if let Some(val) = value {
                    map.insert("value".to_string(), self.eval_inner(val));
                }
                Value::Map(map)
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
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            // Wildcard pattern: _
            (_, Value::String(s)) if s == "_" => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn to_display_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_display_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Map(map) => {
                let items: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k, v.to_display_string())).collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Error(e) => format!("Error: {}", e),
            Value::Bool(b) => format!("{}", b),
            Value::Null => "null".to_string(),
        }
    }
}
