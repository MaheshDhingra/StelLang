use std::io::{self, Write};
use std::fs;
use stellang::lang::{lexer::Lexer, parser::Parser, interpreter::Interpreter};
use stellang::lang::lexer::Token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // File mode
        let filename = &args[1];
        let content = std::fs::read_to_string(filename).expect("Failed to read file");
        
        let mut lexer = Lexer::new(&content);
        let mut tokens = Vec::new();
        
        loop {
            let tok = lexer.next_token();
            if tok == Ok(Token::EOF) { break; }
            tokens.push(tok.expect("Lexer error"));
        }
        let mut parser = Parser::new(tokens);
        if let Ok(Some(ast)) = parser.parse() {
            let mut interpreter = Interpreter::new();
            match interpreter.eval(&ast) {
                Ok(result) => println!("{}", result.to_display_string()),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        } else {
            eprintln!("Failed to parse file");
        }
    } else {
        // REPL mode
        println!("StelLang REPL (Press Ctrl+C to exit)");
        
        loop {
            print!(">>> ");
            std::io::stdout().flush().unwrap();
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            
            if input.trim().is_empty() {
                continue;
            }
            
            let mut lexer = Lexer::new(&input);
            let mut tokens = Vec::new();
            
            loop {
                let tok = lexer.next_token();
                if tok == Ok(Token::EOF) { break; }
                tokens.push(tok.expect("Lexer error"));
            }
            let mut parser = Parser::new(tokens);
            if let Ok(Some(expr)) = parser.parse() {
                let mut interpreter = Interpreter::new();
                match interpreter.eval(&expr) {
                    Ok(result) => println!("{}", result.to_display_string()),
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            } else {
                eprintln!("Failed to parse input");
            }
        }
    }
}
