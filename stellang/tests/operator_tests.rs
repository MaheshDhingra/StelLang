use stellang::lang::interpreter::{Interpreter, Value};
use stellang::lang::lexer::Lexer;
use stellang::lang::parser::Parser;
use stellang::lang::exceptions::ExceptionKind;

fn eval_code(code: &str) -> Value {
    let mut lexer = Lexer::new(code);
    let tokens = (0..).map(|_| lexer.next_token()).take_while(|t| t != &stellang::lang::lexer::Token::EOF).collect();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().expect("Failed to parse expression");
    let mut interpreter = Interpreter::new();
    interpreter.eval(&expr)
}

#[test]
fn test_int_arithmetic() {
    assert_eq!(eval_code("1 + 2"), Value::Int(3));
    assert_eq!(eval_code("5 - 3"), Value::Int(2));
    assert_eq!(eval_code("4 * 2"), Value::Int(8));
    assert_eq!(eval_code("10 / 2"), Value::Float(5.0));
    assert_eq!(eval_code("10 // 3"), Value::Int(3));
    assert_eq!(eval_code("10 % 3"), Value::Int(1));
    assert_eq!(eval_code("2 ** 3"), Value::Float(8.0));
}

#[test]
fn test_float_arithmetic() {
    assert_eq!(eval_code("1.5 + 2.5"), Value::Float(4.0));
    assert_eq!(eval_code("5.0 - 3.0"), Value::Float(2.0));
    assert_eq!(eval_code("4.0 * 2.5"), Value::Float(10.0));
    assert_eq!(eval_code("10.0 / 4.0"), Value::Float(2.5));
    assert_eq!(eval_code("10.0 // 3.0"), Value::Float(3.0));
    assert_eq!(eval_code("10.0 % 3.0"), Value::Float(1.0));
    assert_eq!(eval_code("2.0 ** 3.0"), Value::Float(8.0));
}

#[test]
fn test_mixed_arithmetic() {
    assert_eq!(eval_code("1 + 2.5"), Value::Float(3.5));
    assert_eq!(eval_code("5.0 - 3"), Value::Float(2.0));
    assert_eq!(eval_code("4 * 2.5"), Value::Float(10.0));
    assert_eq!(eval_code("10.0 / 2"), Value::Float(5.0));
    assert_eq!(eval_code("10 // 3.0"), Value::Float(3.0));
    assert_eq!(eval_code("10.0 % 3"), Value::Float(1.0));
    assert_eq!(eval_code("2 ** 3.0"), Value::Float(8.0));
}

#[test]
fn test_string_ops() {
    assert_eq!(eval_code("\"hello\" + \"world\""), Value::Str("helloworld".to_string()));
    assert_eq!(eval_code("\"abc\" * 3"), Value::Str("abcabcabc".to_string()));
    assert_eq!(eval_code("3 * \"abc\""), Value::Str("abcabcabc".to_string()));
}

#[test]
fn test_comparison_ops() {
    assert_eq!(eval_code("1 == 1"), Value::Bool(true));
    assert_eq!(eval_code("1 != 2"), Value::Bool(true));
    assert_eq!(eval_code("1 < 2"), Value::Bool(true));
    assert_eq!(eval_code("2 > 1"), Value::Bool(true));
    assert_eq!(eval_code("1 <= 1"), Value::Bool(true));
    assert_eq!(eval_code("2 >= 2"), Value::Bool(true));

    assert_eq!(eval_code("1.0 == 1.0"), Value::Bool(true));
    assert_eq!(eval_code("1.0 != 2.0"), Value::Bool(true));
    assert_eq!(eval_code("1.0 < 2.0"), Value::Bool(true));
    assert_eq!(eval_code("2.0 > 1.0"), Value::Bool(true));
    assert_eq!(eval_code("1.0 <= 1.0"), Value::Bool(true));
    assert_eq!(eval_code("2.0 >= 2.0"), Value::Bool(true));

    assert_eq!(eval_code("\"a\" == \"a\""), Value::Bool(true));
    assert_eq!(eval_code("\"a\" != \"b\""), Value::Bool(true));
    assert_eq!(eval_code("\"a\" < \"b\""), Value::Bool(true));
    assert_eq!(eval_code("\"b\" > \"a\""), Value::Bool(true));
    assert_eq!(eval_code("\"a\" <= \"a\""), Value::Bool(true));
    assert_eq!(eval_code("\"b\" >= \"b\""), Value::Bool(true));
}

#[test]
fn test_logical_ops() {
    assert_eq!(eval_code("true and false"), Value::Bool(false));
    assert_eq!(eval_code("true or false"), Value::Bool(true));
    assert_eq!(eval_code("not true"), Value::Bool(false));
    assert_eq!(eval_code("not false"), Value::Bool(true));

    assert_eq!(eval_code("1 and 0"), Value::Bool(false));
    assert_eq!(eval_code("1 or 0"), Value::Bool(true));
    assert_eq!(eval_code("not 0"), Value::Bool(true));
    assert_eq!(eval_code("not 1"), Value::Bool(false));
}

#[test]
fn test_bitwise_ops() {
    assert_eq!(eval_code("5 & 3"), Value::Int(1)); // 101 & 011 = 001
    assert_eq!(eval_code("5 | 3"), Value::Int(7)); // 101 | 011 = 111
    assert_eq!(eval_code("5 ^ 3"), Value::Int(6)); // 101 ^ 011 = 110
    assert_eq!(eval_code("~5"), Value::Int(-6)); // ~000...101 = 111...010 (-6 in 2's complement)
    assert_eq!(eval_code("1 << 2"), Value::Int(4)); // 001 << 2 = 100
    assert_eq!(eval_code("4 >> 1"), Value::Int(2)); // 100 >> 1 = 010
}

#[test]
fn test_identity_ops() {
    assert_eq!(eval_code("1 is 1"), Value::Bool(true));
    assert_eq!(eval_code("1 is not 2"), Value::Bool(true));
    assert_eq!(eval_code("null is null"), Value::Bool(true));
    assert_eq!(eval_code("null is not 1"), Value::Bool(true));
    assert_eq!(eval_code("\"a\" is \"a\""), Value::Bool(true));
    assert_eq!(eval_code("\"a\" is not \"b\""), Value::Bool(true));
}

#[test]
fn test_membership_ops() {
    assert_eq!(eval_code("\"a\" in \"abc\""), Value::Bool(true));
    assert_eq!(eval_code("\"d\" not in \"abc\""), Value::Bool(true));
    assert_eq!(eval_code("1 in [1, 2, 3]"), Value::Bool(true));
    assert_eq!(eval_code("4 not in [1, 2, 3]"), Value::Bool(true));
}

#[test]
fn test_division_by_zero() {
    let result = eval_code("10 / 0");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }

    let result = eval_code("10 // 0");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }

    let result = eval_code("10 % 0");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }

    let result = eval_code("10.0 / 0.0");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::ZeroDivisionError);
    } else {
        panic!("Expected ZeroDivisionError, got {:?}", result);
    }
}

#[test]
fn test_unsupported_operations() {
    let result = eval_code("\"a\" + 1");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }

    let result = eval_code("1 + \"a\"");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }

    let result = eval_code("true + 1");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }

    let result = eval_code("~true");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::TypeError);
    } else {
        panic!("Expected TypeError, got {:?}", result);
    }
}

#[test]
fn test_negative_repetition() {
    let result = eval_code("\"abc\" * -1");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::ValueError);
    } else {
        panic!("Expected ValueError, got {:?}", result);
    }

    let result = eval_code("[-1, -2] * -1");
    if let Value::Exception(e) = result {
        assert_eq!(e.kind, ExceptionKind::ValueError);
    } else {
        panic!("Expected ValueError, got {:?}", result);
    }
}

#[test]
fn test_list_concatenation_repetition() {
    assert_eq!(eval_code("[1, 2] + [3, 4]"), Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)]));
    assert_eq!(eval_code("[1, 2] * 3"), Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(1), Value::Int(2), Value::Int(1), Value::Int(2)]));
    assert_eq!(eval_code("3 * [1, 2]"), Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(1), Value::Int(2), Value::Int(1), Value::Int(2)]));
}

#[test]
fn test_list_membership() {
    assert_eq!(eval_code("1 in [1, 2, 3]"), Value::Bool(true));
    assert_eq!(eval_code("4 in [1, 2, 3]"), Value::Bool(false));
    assert_eq!(eval_code("1 not in [1, 2, 3]"), Value::Bool(false));
    assert_eq!(eval_code("4 not in [1, 2, 3]"), Value::Bool(true));
}

#[test]
fn test_index_assignment() {
    let mut interpreter = Interpreter::new();
    interpreter.eval(&Parser::new(Lexer::new("let my_list = [1, 2, 3]").next_token_stream()).parse().unwrap());
    interpreter.eval(&Parser::new(Lexer::new("my_list[0] = 10").next_token_stream()).parse().unwrap());
    assert_eq!(interpreter.env.get("my_list").unwrap().clone(), Value::List(vec![Value::Int(10), Value::Int(2), Value::Int(3)]));

    let mut interpreter = Interpreter::new();
    interpreter.eval(&Parser::new(Lexer::new("let my_dict = {\"a\": 1, \"b\": 2}").next_token_stream()).parse().unwrap());
    interpreter.eval(&Parser::new(Lexer::new("my_dict[\"a\"] = 10").next_token_stream()).parse().unwrap());
    assert_eq!(interpreter.env.get("my_dict").unwrap().clone(), Value::Dict(vec![("a".to_string(), Value::Int(10)), ("b".to_string(), Value::Int(2))].into_iter().map(|(k,v)| (Value::Str(k),v)).collect()));
}

// Helper to convert Lexer output to Vec<Token>
trait LexerExt {
    fn next_token_stream(&mut self) -> Vec<stellang::lang::lexer::Token>;
}

impl LexerExt for Lexer {
    fn next_token_stream(&mut self) -> Vec<stellang::lang::lexer::Token> {
        (0..).map(|_| self.next_token()).take_while(|t| t != &stellang::lang::lexer::Token::EOF).collect()
    }
}
