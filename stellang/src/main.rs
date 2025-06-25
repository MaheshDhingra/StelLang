mod lang {
    pub mod lexer;
    pub mod parser;
    pub mod ast;
    pub mod interpreter;
}

use std::io::{self, Write};
use std::fs;
use lang::lexer::Lexer;
use lang::parser::Parser;
use lang::interpreter::Interpreter;
use lang::lexer::Token;

fn main() {
    println!("Welcome to StelLang!");
    let mut interpreter = Interpreter::new();
    // Run src/main.stl if it exists
    if let Ok(source) = fs::read_to_string("src/main.stl") {
        let mut lexer = Lexer::new(&source);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF { break; }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        if let Some(ast) = parser.parse() {
            let result = interpreter.eval(&ast);
            println!("[main.stl] = {:?}", result);
        } else {
            println!("[main.stl] Parse error");
        }
    }
    // Start REPL
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }
        if input.trim() == "exit" {
            break;
        }
        let mut lexer = Lexer::new(&input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF { break; }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        if let Some(expr) = parser.parse() {
            let result = interpreter.eval(&expr);
            println!("= {:?}", result);
        } else {
            println!("Parse error");
        }
    }
}
