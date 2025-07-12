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
fn test_str_len() {
    assert_eq!(eval_code("\"hello\".len()"), Value::Int(5));
    assert_eq!(eval_code("\"\".len()"), Value::Int(0));
    assert_eq!(eval_code("\"你好\".len()"), Value::Int(6)); // Byte length, not char count
}

#[test]
fn test_str_upper() {
    assert_eq!(eval_code("\"hello\".upper()"), Value::Str("HELLO".to_string()));
    assert_eq!(eval_code("\"Hello World\".upper()"), Value::Str("HELLO WORLD".to_string()));
    assert_eq!(eval_code("\"你好\".upper()"), Value::Str("你好".to_string())); // Non-ASCII chars unchanged
}

#[test]
fn test_str_lower() {
    assert_eq!(eval_code("\"HELLO\".lower()"), Value::Str("hello".to_string()));
    assert_eq!(eval_code("\"Hello World\".lower()"), Value::Str("hello world".to_string()));
    assert_eq!(eval_code("\"你好\".lower()"), Value::Str("你好".to_string())); // Non-ASCII chars unchanged
}

#[test]
fn test_str_strip() {
    assert_eq!(eval_code("\"  hello  \".strip()"), Value::Str("hello".to_string()));
    assert_eq!(eval_code("\"\\n\\tworld\\n\".strip()"), Value::Str("world".to_string()));
    assert_eq!(eval_code("\"hello\".strip()"), Value::Str("hello".to_string()));
}

#[test]
fn test_str_split() {
    assert_eq!(eval_code("\"a,b,c\".split(\",\")"), Value::List(vec![Value::Str("a".to_string()), Value::Str("b".to_string()), Value::Str("c".to_string())]));
    assert_eq!(eval_code("\"a b c\".split()"), Value::List(vec![Value::Str("a".to_string()), Value::Str("b".to_string()), Value::Str("c".to_string())]));
    assert_eq!(eval_code("\"a  b   c\".split()"), Value::List(vec![Value::Str("a".to_string()), Value::Str("b".to_string()), Value::Str("c".to_string())]));
    assert_eq!(eval_code("\"a,b,\".split(\",\")"), Value::List(vec![Value::Str("a".to_string()), Value::Str("b".to_string()), Value::Str("".to_string())]));
    assert_eq!(eval_code("\"\".split(\",\")"), Value::List(vec![Value::Str("".to_string())]));
}

#[test]
fn test_str_join() {
    assert_eq!(eval_code("\",\".join([\"a\", \"b\", \"c\"])"), Value::Str("a,b,c".to_string()));
    assert_eq!(eval_code("\"-\".join([\"1\", \"2\"])"), Value::Str("1-2".to_string()));
    assert_eq!(eval_code("\"\".join([\"a\", \"b\"])"), Value::Str("ab".to_string()));
    assert_eq!(eval_code("\",\".join([])"), Value::Str("".to_string()));
}

#[test]
fn test_str_replace() {
    assert_eq!(eval_code("\"hello world\".replace(\"world\", \"rust\")"), Value::Str("hello rust".to_string()));
    assert_eq!(eval_code("\"aaaaa\".replace(\"a\", \"b\", 2)"), Value::Str("bbaaa".to_string()));
    assert_eq!(eval_code("\"aaaaa\".replace(\"a\", \"b\")"), Value::Str("bbbbb".to_string()));
    assert_eq!(eval_code("\"abcabc\".replace(\"a\", \"x\")"), Value::Str("xbcxbc".to_string()));
}

#[test]
fn test_str_find() {
    assert_eq!(eval_code("\"hello world\".find(\"world\")"), Value::Int(6));
    assert_eq!(eval_code("\"hello world\".find(\"foo\")"), Value::Int(-1));
    assert_eq!(eval_code("\"aaaaa\".find(\"a\")"), Value::Int(0));
    assert_eq!(eval_code("\"\".find(\"a\")"), Value::Int(-1));
}

#[test]
fn test_str_count() {
    assert_eq!(eval_code("\"hello world\".count(\"o\")"), Value::Int(2));
    assert_eq!(eval_code("\"aaaaa\".count(\"a\")"), Value::Int(5));
    assert_eq!(eval_code("\"aaaaa\".count(\"aa\")"), Value::Int(2));
    assert_eq!(eval_code("\"\".count(\"a\")"), Value::Int(0));
}

#[test]
fn test_str_startswith() {
    assert_eq!(eval_code("\"hello world\".startswith(\"hello\")"), Value::Bool(true));
    assert_eq!(eval_code("\"hello world\".startswith(\"world\")"), Value::Bool(false));
    assert_eq!(eval_code("\"\".startswith(\"\")"), Value::Bool(true));
}

#[test]
fn test_str_endswith() {
    assert_eq!(eval_code("\"hello world\".endswith(\"world\")"), Value::Bool(true));
    assert_eq!(eval_code("\"hello world\".endswith(\"hello\")"), Value::Bool(false));
    assert_eq!(eval_code("\"\".endswith(\"\")"), Value::Bool(true));
}

#[test]
fn test_str_isalnum() {
    assert_eq!(eval_code("\"abc123\".isalnum()"), Value::Bool(true));
    assert_eq!(eval_code("\"abc\".isalnum()"), Value::Bool(true));
    assert_eq!(eval_code("\"123\".isalnum()"), Value::Bool(true));
    assert_eq!(eval_code("\"abc!\".isalnum()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".isalnum()"), Value::Bool(false));
}

#[test]
fn test_str_isalpha() {
    assert_eq!(eval_code("\"abc\".isalpha()"), Value::Bool(true));
    assert_eq!(eval_code("\"abc1\".isalpha()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".isalpha()"), Value::Bool(false));
}

#[test]
fn test_str_isdigit() {
    assert_eq!(eval_code("\"123\".isdigit()"), Value::Bool(true));
    assert_eq!(eval_code("\"123a\".isdigit()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".isdigit()"), Value::Bool(false));
}

#[test]
fn test_str_islower() {
    assert_eq!(eval_code("\"abc\".islower()"), Value::Bool(true));
    assert_eq!(eval_code("\"Abc\".islower()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".islower()"), Value::Bool(false));
}

#[test]
fn test_str_isupper() {
    assert_eq!(eval_code("\"ABC\".isupper()"), Value::Bool(true));
    assert_eq!(eval_code("\"ABc\".isupper()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".isupper()"), Value::Bool(false));
}

#[test]
fn test_str_isspace() {
    assert_eq!(eval_code("\"   \".isspace()"), Value::Bool(true));
    assert_eq!(eval_code("\" \\n\\t\".isspace()"), Value::Bool(true));
    assert_eq!(eval_code("\" a \".isspace()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".isspace()"), Value::Bool(false));
}

#[test]
fn test_str_istitle() {
    assert_eq!(eval_code("\"Hello World\".istitle()"), Value::Bool(true));
    assert_eq!(eval_code("\"hello world\".istitle()"), Value::Bool(false));
    assert_eq!(eval_code("\"Hello world\".istitle()"), Value::Bool(true));
    assert_eq!(eval_code("\"HELLO WORLD\".istitle()"), Value::Bool(false));
    assert_eq!(eval_code("\"\".istitle()"), Value::Bool(false));
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
