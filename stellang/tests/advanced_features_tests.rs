use stellang::lang::interpreter::Interpreter;
use stellang::lang::lexer::Lexer;
use stellang::lang::parser::Parser;
use stellang::lang::exceptions::Exception;

fn eval_code(code: &str) -> Result<stellang::lang::interpreter::Value, Exception> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(stellang::lang::lexer::Token::EOF) => break,
            Ok(token) => tokens.push(token),
            Err(e) => return Err(e),
        }
    }

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    
    match expr {
        Some(expr) => {
            let mut interpreter = Interpreter::new();
            interpreter.eval(&expr)
        }
        None => Ok(stellang::lang::interpreter::Value::None),
    }
}

#[test]
fn test_function_definition_and_call() {
    let code = r#"
        fn add(a, b) {
            return a + b
        }
        
        let result = add(5, 3)
        result
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Int(n)) = result {
        assert_eq!(n, 8);
    } else {
        panic!("Expected integer result");
    }
}

#[test]
fn test_function_with_multiple_statements() {
    let code = r#"
        fn factorial(n) {
            if n <= 1 {
                return 1
            } else {
                return n * factorial(n - 1)
            }
        }
        
        factorial(5)
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Int(n)) = result {
        assert_eq!(n, 120);
    } else {
        panic!("Expected integer result");
    }
}

#[test]
fn test_class_definition_and_instantiation() {
    let code = r#"
        class Person {
            name = "Unknown"
            age = 0
            
            fn __init__(self, name, age) {
                self.name = name
                self.age = age
            }
            
            fn greet(self) {
                return "Hello, I'm " + self.name
            }
        }
        
        let person = Person("Alice", 30)
        person.greet()
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Str(s)) = result {
        assert_eq!(s, "Hello, I'm Alice");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_class_inheritance() {
    let code = r#"
        class Animal {
            name = "Unknown"
            
            fn __init__(self, name) {
                self.name = name
            }
            
            fn speak(self) {
                return "Some sound"
            }
        }
        
        class Dog extends Animal {
            fn speak(self) {
                return "Woof! I'm " + self.name
            }
        }
        
        let dog = Dog("Buddy")
        dog.speak()
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Str(s)) = result {
        assert_eq!(s, "Woof! I'm Buddy");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_module_import() {
    let code = r#"
        import math
        
        let pi = 3.14159
        let radius = 5
        let area = pi * radius * radius
        area
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Float(f)) = result {
        assert!((f - 78.53975).abs() < 0.001);
    } else {
        panic!("Expected float result");
    }
}

#[test]
fn test_error_handling() {
    let code = r#"
        fn divide(a, b) {
            if b == 0 {
                throw "Division by zero"
            }
            return a / b
        }
        
        try {
            divide(10, 0)
        } catch error {
            "Error: " + error
        }
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Str(s)) = result {
        assert_eq!(s, "Error: Division by zero");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_pattern_matching() {
    let code = r#"
        fn describe(value) {
            match value {
                0 => "Zero"
                1 => "One"
                "hello" => "Greeting"
                _ => "Unknown"
            }
        }
        
        describe(1)
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Str(s)) = result {
        assert_eq!(s, "One");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_list_comprehension() {
    let code = r#"
        let numbers = [1, 2, 3, 4, 5]
        let squares = [x * x for x in numbers]
        squares
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::List(items)) = result {
        assert_eq!(items.len(), 5);
        if let stellang::lang::interpreter::Value::Int(n) = items[0] {
            assert_eq!(n, 1);
        }
        if let stellang::lang::interpreter::Value::Int(n) = items[1] {
            assert_eq!(n, 4);
        }
    } else {
        panic!("Expected list result");
    }
}

#[test]
fn test_decorators() {
    let code = r#"
        fn log_calls(fn) {
            return fn
        }
        
        @log_calls
        fn hello(name) {
            return "Hello, " + name
        }
        
        hello("World")
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Str(s)) = result {
        assert_eq!(s, "Hello, World");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_async_await() {
    let code = r#"
        async fn fetch_data() {
            return "Data from server"
        }
        
        async fn main() {
            let data = await fetch_data()
            return data
        }
        
        main()
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Str(s)) = result {
        assert_eq!(s, "Data from server");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_type_annotations() {
    let code = r#"
        fn add(a: int, b: int) -> int {
            return a + b
        }
        
        let result: int = add(5, 3)
        result
    "#;
    
    let result = eval_code(code);
    assert!(result.is_ok());
    if let Ok(stellang::lang::interpreter::Value::Int(n)) = result {
        assert_eq!(n, 8);
    } else {
        panic!("Expected integer result");
    }
} 