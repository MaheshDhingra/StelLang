// Parser for StelLang

use super::lexer::Token;
use super::ast::Expr;

/// The Parser struct parses a vector of tokens into an AST expression.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Create a new parser from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Peek at the current token without advancing.
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    /// Advance to the next token and return the previous one.
    fn advance(&mut self) -> &Token {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).unwrap_or(&Token::EOF)
    }

    /// Parse an expression from the token stream.
    pub fn parse(&mut self) -> Option<Expr> {
        let mut exprs = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(expr) = self.parse_block() {
                exprs.push(expr);
            } else {
                break;
            }
            // Skip semicolons between top-level statements
            if let Token::Semicolon = self.peek() {
                self.advance();
            }
        }
        if exprs.len() == 1 {
            Some(exprs.remove(0))
        } else if !exprs.is_empty() {
            Some(Expr::Block(exprs))
        } else {
            None
        }
    }

    fn parse_block(&mut self) -> Option<Expr> {
        let mut exprs = Vec::new();
        if let Token::LBrace = self.peek() {
            self.advance();
            while !matches!(self.peek(), Token::RBrace | Token::EOF) {
                if let Some(expr) = self.parse_expr() {
                    exprs.push(expr);
                }
                if let Token::Semicolon = self.peek() {
                    self.advance();
                }
            }
            if let Token::RBrace = self.peek() {
                self.advance();
            }
            Some(Expr::Block(exprs))
        } else {
            self.parse_expr()
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Option<Expr> {
        let mut node = self.parse_logical_and()?;
        while let Token::Or = self.peek() {
            self.advance();
            let right = self.parse_logical_and()?;
            node = Expr::BinaryOp {
                left: Box::new(node),
                op: "or".into(),
                right: Box::new(right),
            };
        }
        Some(node)
    }

    fn parse_logical_and(&mut self) -> Option<Expr> {
        let mut node = self.parse_equality()?;
        while let Token::And = self.peek() {
            self.advance();
            let right = self.parse_equality()?;
            node = Expr::BinaryOp {
                left: Box::new(node),
                op: "and".into(),
                right: Box::new(right),
            };
        }
        Some(node)
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        // For now, just call parse_assignment (could expand for ==, !=, etc.)
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
        let expr = self.parse_term()?;
        if let Token::Assign = self.peek() {
            self.advance();
            if let Expr::Ident(name) = expr {
                let value = self.parse_expr()?;
                return Some(Expr::Assign {
                    name,
                    expr: Box::new(value),
                });
            }
        }
        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut node = self.parse_factor()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "+".into(),
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "-".into(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(node)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut node = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "*".into(),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "/".into(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(node)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryOp { op: "not".into(), expr: Box::new(expr) })
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryOp { op: "-".into(), expr: Box::new(expr) })
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            if let Token::LParen = self.peek() {
                self.advance();
                let mut args = Vec::new();
                if let Token::RParen = self.peek() {
                    self.advance();
                } else {
                    loop {
                        args.push(self.parse_expr()?);
                        if let Token::Comma = self.peek() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if let Token::RParen = self.peek() {
                        self.advance();
                    }
                }
                if let Expr::Ident(name) = expr {
                    expr = Expr::FnCall { name, args };
                }
            } else {
                break;
            }
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::Integer(n) => {
                let n = *n;
                self.advance();
                Some(Expr::Integer(n))
            }
            Token::Float(f) => {
                let f = *f;
                self.advance();
                Some(Expr::Float(f))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Some(Expr::String(s))
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Some(Expr::Ident(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr();
                if let Token::RParen = self.peek() {
                    self.advance();
                }
                expr
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::lexer::{Lexer, Token};

    #[test]
    fn test_parse_simple_arithmetic() {
        let mut lexer = Lexer::new("1 + 2 * 3");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        // Should parse as 1 + (2 * 3)
        match ast {
            Expr::BinaryOp { op, .. } => assert_eq!(op, "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_assignment() {
        let mut lexer = Lexer::new("x = 42");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        match ast {
            Expr::Assign { name, .. } => assert_eq!(name, "x"),
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_block() {
        let mut lexer = Lexer::new("{ x = 1; y = 2; }");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        match ast {
            Expr::Block(exprs) => {
                assert_eq!(exprs.len(), 2);
                if let Expr::Assign { name, .. } = &exprs[0] {
                    assert_eq!(name, "x");
                } else {
                    panic!("Expected assignment expression");
                }
                if let Expr::Assign { name, .. } = &exprs[1] {
                    assert_eq!(name, "y");
                } else {
                    panic!("Expected assignment expression");
                }
            }
            _ => panic!("Expected block expression"),
        }
    }

    #[test]
    fn test_parse_if() {
        let mut lexer = Lexer::new("if x { y = 1; } else { y = 2; }");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        match ast {
            Expr::If { cond, then_branch, else_branch } => {
                assert_eq!(*cond, Expr::Ident("x".into()));
                if let Expr::Block(exprs) = *then_branch {
                    assert_eq!(exprs.len(), 1);
                    if let Expr::Assign { name, .. } = &exprs[0] {
                        assert_eq!(name, "y");
                    } else {
                        panic!("Expected assignment expression");
                    }
                } else {
                    panic!("Expected block expression");
                }
                if let Some(else_branch) = else_branch {
                    if let Expr::Block(exprs) = *else_branch {
                        assert_eq!(exprs.len(), 1);
                        if let Expr::Assign { name, .. } = &exprs[0] {
                            assert_eq!(name, "y");
                        } else {
                            panic!("Expected assignment expression");
                        }
                    } else {
                        panic!("Expected block expression");
                    }
                } else {
                    panic!("Expected else branch");
                }
            }
            _ => panic!("Expected if expression"),
        }
    }

    #[test]
    fn test_parse_while() {
        let mut lexer = Lexer::new("while x { y = 1; }");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        match ast {
            Expr::While { cond, body } => {
                assert_eq!(*cond, Expr::Ident("x".into()));
                if let Expr::Block(exprs) = *body {
                    assert_eq!(exprs.len(), 1);
                    if let Expr::Assign { name, .. } = &exprs[0] {
                        assert_eq!(name, "y");
                    } else {
                        panic!("Expected assignment expression");
                    }
                } else {
                    panic!("Expected block expression");
                }
            }
            _ => panic!("Expected while expression"),
        }
    }

    #[test]
    fn test_parse_fn_def() {
        let mut lexer = Lexer::new("fn add(x, y) { return x + y; }");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        match ast {
            Expr::FnDef { name, params, body } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], "x");
                assert_eq!(params[1], "y");
                if let Expr::Block(exprs) = *body {
                    assert_eq!(exprs.len(), 1);
                    if let Expr::Return(ref expr) = &exprs[0] {
                        if let Expr::BinaryOp { op, .. } = **expr {
                            assert_eq!(op, "+");
                        } else {
                            panic!("Expected binary operation");
                        }
                    } else {
                        panic!("Expected return expression");
                    }
                } else {
                    panic!("Expected block expression");
                }
            }
            _ => panic!("Expected function definition"),
        }
    }
}
