use stellang::lang::{lexer::Lexer, parser::Parser, interpreter::Interpreter, exceptions::Exception};

fn eval_code(code: &str) -> Result<stellang::lang::interpreter::Value, Exception> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token();
        if tok == Ok(stellang::lang::lexer::Token::EOF) { break; }
        tokens.push(tok.expect("Lexer error"));
    }
    
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().expect("Parse error").expect("No expression");
    let mut interpreter = Interpreter::new();
    interpreter.eval(&expr)
}

#[test]
fn test_int_arithmetic() {
    assert_eq!(eval_code("1 + 2"), Ok(stellang::lang::interpreter::Value::Int(3)));
    assert_eq!(eval_code("5 - 3"), Ok(stellang::lang::interpreter::Value::Int(2)));
    assert_eq!(eval_code("4 * 2"), Ok(stellang::lang::interpreter::Value::Int(8)));
    assert_eq!(eval_code("10 / 2"), Ok(stellang::lang::interpreter::Value::Float(5.0)));
    assert_eq!(eval_code("10 // 3"), Ok(stellang::lang::interpreter::Value::Int(3)));
    assert_eq!(eval_code("10 % 3"), Ok(stellang::lang::interpreter::Value::Int(1)));
    assert_eq!(eval_code("2 ** 3"), Ok(stellang::lang::interpreter::Value::Float(8.0)));
}

#[test]
fn test_float_arithmetic() {
    assert_eq!(eval_code("1.5 + 2.5"), Ok(stellang::lang::interpreter::Value::Float(4.0)));
    assert_eq!(eval_code("5.0 - 3.0"), Ok(stellang::lang::interpreter::Value::Float(2.0)));
    assert_eq!(eval_code("4.0 * 2.5"), Ok(stellang::lang::interpreter::Value::Float(10.0)));
    assert_eq!(eval_code("10.0 / 4.0"), Ok(stellang::lang::interpreter::Value::Float(2.5)));
    assert_eq!(eval_code("10.0 // 3.0"), Ok(stellang::lang::interpreter::Value::Float(3.0)));
    assert_eq!(eval_code("10.0 % 3.0"), Ok(stellang::lang::interpreter::Value::Float(1.0)));
    assert_eq!(eval_code("2.0 ** 3.0"), Ok(stellang::lang::interpreter::Value::Float(8.0)));
}

#[test]
fn test_mixed_arithmetic() {
    assert_eq!(eval_code("1 + 2.5"), Ok(stellang::lang::interpreter::Value::Float(3.5)));
    assert_eq!(eval_code("5.0 - 3"), Ok(stellang::lang::interpreter::Value::Float(2.0)));
    assert_eq!(eval_code("4 * 2.5"), Ok(stellang::lang::interpreter::Value::Float(10.0)));
    assert_eq!(eval_code("10.0 / 2"), Ok(stellang::lang::interpreter::Value::Float(5.0)));
    assert_eq!(eval_code("10 // 3.0"), Ok(stellang::lang::interpreter::Value::Float(3.0)));
    assert_eq!(eval_code("10.0 % 3"), Ok(stellang::lang::interpreter::Value::Float(1.0)));
    assert_eq!(eval_code("2 ** 3.0"), Ok(stellang::lang::interpreter::Value::Float(8.0)));
}

#[test]
fn test_string_ops() {
    assert_eq!(eval_code("\"hello\" + \"world\""), Ok(stellang::lang::interpreter::Value::Str("helloworld".to_string())));
    assert_eq!(eval_code("\"abc\" * 3"), Ok(stellang::lang::interpreter::Value::Str("abcabcabc".to_string())));
    assert_eq!(eval_code("3 * \"abc\""), Ok(stellang::lang::interpreter::Value::Str("abcabcabc".to_string())));
}

#[test]
fn test_comparison_ops() {
    assert_eq!(eval_code("1 == 1"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1 != 2"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1 < 2"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("2 > 1"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1 <= 1"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("2 >= 2"), Ok(stellang::lang::interpreter::Value::Bool(true)));

    assert_eq!(eval_code("1.0 == 1.0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1.0 != 2.0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1.0 < 2.0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("2.0 > 1.0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1.0 <= 1.0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("2.0 >= 2.0"), Ok(stellang::lang::interpreter::Value::Bool(true)));

    assert_eq!(eval_code("\"a\" == \"a\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"a\" != \"b\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"a\" < \"b\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"b\" > \"a\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"a\" <= \"a\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"b\" >= \"b\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
}

#[test]
fn test_logical_ops() {
    assert_eq!(eval_code("true and false"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("true or false"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("not true"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("not false"), Ok(stellang::lang::interpreter::Value::Bool(true)));

    assert_eq!(eval_code("1 and 0"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("1 or 0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("not 0"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("not 1"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_bitwise_ops() {
    assert_eq!(eval_code("5 & 3"), Ok(stellang::lang::interpreter::Value::Int(1))); // 101 & 011 = 001
    assert_eq!(eval_code("5 | 3"), Ok(stellang::lang::interpreter::Value::Int(7))); // 101 | 011 = 111
    assert_eq!(eval_code("5 ^ 3"), Ok(stellang::lang::interpreter::Value::Int(6))); // 101 ^ 011 = 110
    assert_eq!(eval_code("~5"), Ok(stellang::lang::interpreter::Value::Int(-6))); // ~000...101 = 111...010 (-6 in 2's complement)
    assert_eq!(eval_code("1 << 2"), Ok(stellang::lang::interpreter::Value::Int(4))); // 001 << 2 = 100
    assert_eq!(eval_code("4 >> 1"), Ok(stellang::lang::interpreter::Value::Int(2))); // 100 >> 1 = 010
}

#[test]
fn test_identity_ops() {
    assert_eq!(eval_code("1 is 1"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1 is not 2"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("null is null"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("null is not 1"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"a\" is \"a\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"a\" is not \"b\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
}

#[test]
fn test_membership_ops() {
    assert_eq!(eval_code("\"a\" in \"abc\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"d\" not in \"abc\""), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("1 in [1, 2, 3]"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("4 not in [1, 2, 3]"), Ok(stellang::lang::interpreter::Value::Bool(true)));
}

#[test]
fn test_division_by_zero() {
    let result = eval_code("10 / 0");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }

    let result = eval_code("10 // 0");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }

    let result = eval_code("10 % 0");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }

    let result = eval_code("10.0 / 0.0");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }
}

#[test]
fn test_unsupported_operations() {
    let result = eval_code("\"a\" + 1");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }

    let result = eval_code("1 + \"a\"");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }

    let result = eval_code("true + 1");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }

    let result = eval_code("~true");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }
}

#[test]
fn test_negative_repetition() {
    let result = eval_code("\"abc\" * -1");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::ValueError);
    } else {
        panic!("Expected ValueError, got {:?}", result);
    }

    let result = eval_code("[-1, -2] * -1");
    if let Ok(stellang::lang::interpreter::Value::Exception(e)) = result {
        assert_eq!(e.kind, stellang::lang::exceptions::ExceptionKind::ValueError);
    } else {
        panic!("Expected ValueError, got {:?}", result);
    }
}

#[test]
fn test_list_concatenation_repetition() {
    assert_eq!(eval_code("[1, 2] + [3, 4]"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2), stellang::lang::interpreter::Value::Int(3), stellang::lang::interpreter::Value::Int(4)])));
    assert_eq!(eval_code("[1, 2] * 3"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2), stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2), stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2)])));
    assert_eq!(eval_code("3 * [1, 2]"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2), stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2), stellang::lang::interpreter::Value::Int(1), stellang::lang::interpreter::Value::Int(2)])));
}

#[test]
fn test_list_membership() {
    assert_eq!(eval_code("1 in [1, 2, 3]"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("4 in [1, 2, 3]"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("1 not in [1, 2, 3]"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("4 not in [1, 2, 3]"), Ok(stellang::lang::interpreter::Value::Bool(true)));
}

#[test]
fn test_index_assignment() {
    let mut interpreter = Interpreter::new();
    interpreter.eval(&Parser::new(Lexer::new("let my_list = [1, 2, 3]").next_token_stream()).parse().unwrap().unwrap());
    interpreter.eval(&Parser::new(Lexer::new("my_list[0] = 10").next_token_stream()).parse().unwrap().unwrap());
    assert_eq!(interpreter.env.get("my_list").unwrap().clone(), stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Int(10), stellang::lang::interpreter::Value::Int(2), stellang::lang::interpreter::Value::Int(3)]));

    let mut interpreter = Interpreter::new();
    interpreter.eval(&Parser::new(Lexer::new("let my_dict = {\"a\": 1, \"b\": 2}").next_token_stream()).parse().unwrap().unwrap());
    interpreter.eval(&Parser::new(Lexer::new("my_dict[\"a\"] = 10").next_token_stream()).parse().unwrap().unwrap());
    assert_eq!(interpreter.env.get("my_dict").unwrap().clone(), stellang::lang::interpreter::Value::Dict(vec![("a".to_string(), stellang::lang::interpreter::Value::Int(10)), ("b".to_string(), stellang::lang::interpreter::Value::Int(2))].into_iter().map(|(k,v)| (stellang::lang::interpreter::Value::Str(k),v)).collect()));
}

// Helper to convert Lexer output to Vec<Token>
trait LexerExt {
    fn next_token_stream(&mut self) -> Vec<stellang::lang::lexer::Token>;
}

impl LexerExt for Lexer {
    fn next_token_stream(&mut self) -> Vec<stellang::lang::lexer::Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Ok(stellang::lang::lexer::Token::EOF) { break; }
            tokens.push(tok.expect("Lexer error"));
        }
        tokens
    }
}
