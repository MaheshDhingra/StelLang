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
fn test_str_len() {
    assert_eq!(eval_code("\"hello\".len()"), Ok(stellang::lang::interpreter::Value::Int(5)));
    assert_eq!(eval_code("\"\".len()"), Ok(stellang::lang::interpreter::Value::Int(0)));
    assert_eq!(eval_code("\"你好\".len()"), Ok(stellang::lang::interpreter::Value::Int(6))); // Byte length, not char count
}

#[test]
fn test_str_upper() {
    assert_eq!(eval_code("\"hello\".upper()"), Ok(stellang::lang::interpreter::Value::Str("HELLO".to_string())));
    assert_eq!(eval_code("\"Hello World\".upper()"), Ok(stellang::lang::interpreter::Value::Str("HELLO WORLD".to_string())));
    assert_eq!(eval_code("\"你好\".upper()"), Ok(stellang::lang::interpreter::Value::Str("你好".to_string()))); // Non-ASCII chars unchanged
}

#[test]
fn test_str_lower() {
    assert_eq!(eval_code("\"HELLO\".lower()"), Ok(stellang::lang::interpreter::Value::Str("hello".to_string())));
    assert_eq!(eval_code("\"Hello World\".lower()"), Ok(stellang::lang::interpreter::Value::Str("hello world".to_string())));
    assert_eq!(eval_code("\"你好\".lower()"), Ok(stellang::lang::interpreter::Value::Str("你好".to_string()))); // Non-ASCII chars unchanged
}

#[test]
fn test_str_strip() {
    assert_eq!(eval_code("\"  hello  \".strip()"), Ok(stellang::lang::interpreter::Value::Str("hello".to_string())));
    assert_eq!(eval_code("\"\\n\\tworld\\n\".strip()"), Ok(stellang::lang::interpreter::Value::Str("world".to_string())));
    assert_eq!(eval_code("\"hello\".strip()"), Ok(stellang::lang::interpreter::Value::Str("hello".to_string())));
}

#[test]
fn test_str_split() {
    assert_eq!(eval_code("\"a,b,c\".split(\",\")"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Str("a".to_string()), stellang::lang::interpreter::Value::Str("b".to_string()), stellang::lang::interpreter::Value::Str("c".to_string())])));
    assert_eq!(eval_code("\"a b c\".split()"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Str("a".to_string()), stellang::lang::interpreter::Value::Str("b".to_string()), stellang::lang::interpreter::Value::Str("c".to_string())])));
    assert_eq!(eval_code("\"a  b   c\".split()"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Str("a".to_string()), stellang::lang::interpreter::Value::Str("b".to_string()), stellang::lang::interpreter::Value::Str("c".to_string())])));
    assert_eq!(eval_code("\"a,b,\".split(\",\")"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Str("a".to_string()), stellang::lang::interpreter::Value::Str("b".to_string()), stellang::lang::interpreter::Value::Str("".to_string())])));
    assert_eq!(eval_code("\"\".split(\",\")"), Ok(stellang::lang::interpreter::Value::List(vec![stellang::lang::interpreter::Value::Str("".to_string())])));
}

#[test]
fn test_str_join() {
    assert_eq!(eval_code("\",\".join([\"a\", \"b\", \"c\"])"), Ok(stellang::lang::interpreter::Value::Str("a,b,c".to_string())));
    assert_eq!(eval_code("\"-\".join([\"1\", \"2\"])"), Ok(stellang::lang::interpreter::Value::Str("1-2".to_string())));
    assert_eq!(eval_code("\"\".join([\"a\", \"b\"])"), Ok(stellang::lang::interpreter::Value::Str("ab".to_string())));
    assert_eq!(eval_code("\",\".join([])"), Ok(stellang::lang::interpreter::Value::Str("".to_string())));
}

#[test]
fn test_str_replace() {
    assert_eq!(eval_code("\"hello world\".replace(\"world\", \"rust\")"), Ok(stellang::lang::interpreter::Value::Str("hello rust".to_string())));
    assert_eq!(eval_code("\"aaaaa\".replace(\"a\", \"b\", 2)"), Ok(stellang::lang::interpreter::Value::Str("bbaaa".to_string())));
    assert_eq!(eval_code("\"aaaaa\".replace(\"a\", \"b\")"), Ok(stellang::lang::interpreter::Value::Str("bbbbb".to_string())));
    assert_eq!(eval_code("\"abcabc\".replace(\"a\", \"x\")"), Ok(stellang::lang::interpreter::Value::Str("xbcxbc".to_string())));
}

#[test]
fn test_str_find() {
    assert_eq!(eval_code("\"hello world\".find(\"world\")"), Ok(stellang::lang::interpreter::Value::Int(6)));
    assert_eq!(eval_code("\"hello world\".find(\"foo\")"), Ok(stellang::lang::interpreter::Value::Int(-1)));
    assert_eq!(eval_code("\"aaaaa\".find(\"a\")"), Ok(stellang::lang::interpreter::Value::Int(0)));
    assert_eq!(eval_code("\"\".find(\"a\")"), Ok(stellang::lang::interpreter::Value::Int(-1)));
}

#[test]
fn test_str_count() {
    assert_eq!(eval_code("\"hello world\".count(\"o\")"), Ok(stellang::lang::interpreter::Value::Int(2)));
    assert_eq!(eval_code("\"aaaaa\".count(\"a\")"), Ok(stellang::lang::interpreter::Value::Int(5)));
    assert_eq!(eval_code("\"aaaaa\".count(\"aa\")"), Ok(stellang::lang::interpreter::Value::Int(2)));
    assert_eq!(eval_code("\"\".count(\"a\")"), Ok(stellang::lang::interpreter::Value::Int(0)));
}

#[test]
fn test_str_startswith() {
    assert_eq!(eval_code("\"hello world\".startswith(\"hello\")"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"hello world\".startswith(\"world\")"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".startswith(\"\")"), Ok(stellang::lang::interpreter::Value::Bool(true)));
}

#[test]
fn test_str_endswith() {
    assert_eq!(eval_code("\"hello world\".endswith(\"world\")"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"hello world\".endswith(\"hello\")"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".endswith(\"\")"), Ok(stellang::lang::interpreter::Value::Bool(true)));
}

#[test]
fn test_str_isalnum() {
    assert_eq!(eval_code("\"abc123\".isalnum()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"abc\".isalnum()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"123\".isalnum()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"abc!\".isalnum()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".isalnum()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_str_isalpha() {
    assert_eq!(eval_code("\"abc\".isalpha()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"abc1\".isalpha()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".isalpha()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_str_isdigit() {
    assert_eq!(eval_code("\"123\".isdigit()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"123a\".isdigit()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".isdigit()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_str_islower() {
    assert_eq!(eval_code("\"abc\".islower()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"Abc\".islower()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".islower()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_str_isupper() {
    assert_eq!(eval_code("\"ABC\".isupper()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"ABc\".isupper()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".isupper()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_str_isspace() {
    assert_eq!(eval_code("\"   \".isspace()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\" \\n\\t\".isspace()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\" a \".isspace()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".isspace()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
}

#[test]
fn test_str_istitle() {
    assert_eq!(eval_code("\"Hello World\".istitle()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"hello world\".istitle()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"Hello world\".istitle()"), Ok(stellang::lang::interpreter::Value::Bool(true)));
    assert_eq!(eval_code("\"HELLO WORLD\".istitle()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
    assert_eq!(eval_code("\"\".istitle()"), Ok(stellang::lang::interpreter::Value::Bool(false)));
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
