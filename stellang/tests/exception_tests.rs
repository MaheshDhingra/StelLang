// Exception system tests for StelLang

use stellang::lang::{lexer::Lexer, parser::Parser, interpreter::Interpreter, exceptions::{Exception, ExceptionKind}};

#[test]
fn test_division_by_zero_exception() {
    let mut lexer = Lexer::new("10 / 0");
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token();
        if tok == Ok(stellang::lang::lexer::Token::EOF) { break; }
        tokens.push(tok.expect("Lexer error"));
    }
    
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().expect("Parse error").expect("No expression");
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&expr);
    
    match result {
        Ok(stellang::lang::interpreter::Value::Exception(e)) => {
            assert_eq!(e.kind, ExceptionKind::ZeroDivisionError);
        },
        _ => panic!("Expected ZeroDivisionError exception"),
    }
}

#[test]
fn test_exception_hierarchy() {
    let base = Exception::new(ExceptionKind::BaseException, vec!["base".to_string()]);
    let syntax = Exception::new(ExceptionKind::SyntaxError, vec!["syntax error".to_string()]);
    let value = Exception::new(ExceptionKind::ValueError, vec!["value error".to_string()]);
    assert_eq!(base.kind, ExceptionKind::BaseException);
    assert_eq!(syntax.kind, ExceptionKind::SyntaxError);
    assert_eq!(value.kind, ExceptionKind::ValueError);
}

#[test]
fn test_custom_exception_creation() {
    let mut custom = Exception::new(ExceptionKind::UserWarning, vec!["custom warning".to_string()]);
    custom.add_note("extra info".to_string());
    assert_eq!(custom.kind, ExceptionKind::UserWarning);
    assert_eq!(custom.args[0], "custom warning");
    assert_eq!(custom.notes[0], "extra info");
}

#[test]
fn test_exception_chaining() {
    let cause = Exception::new(ExceptionKind::ValueError, vec!["bad value".to_string()]);
    let main = Exception::new(ExceptionKind::RuntimeError, vec!["runtime failed".to_string()]).with_cause(cause.clone());
    assert!(main.suppress_context);
    assert!(main.cause.is_some());
    assert_eq!(main.cause.unwrap().kind, ExceptionKind::ValueError);
}
