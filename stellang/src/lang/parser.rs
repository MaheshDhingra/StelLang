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
            // Accept any top-level statement, not just blocks
            if let Some(expr) = self.parse_expr() {
                exprs.push(expr);
            } else {
                break;
            }
            // Skip optional semicolons between top-level statements
            while let Token::Semicolon = self.peek() {
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
                } else {
                    // If parse_expr returns None, advance to avoid infinite loop
                    self.advance();
                }
                // Accept optional semicolons between statements
                while let Token::Semicolon = self.peek() {
                    self.advance();
                }
            }
            if let Token::RBrace = self.peek() {
                self.advance();
            }
            return Some(Expr::Block(exprs));
        }
        None
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Const => self.parse_const(),
            Token::Match => self.parse_match(),
            Token::Struct => self.parse_struct(),
            Token::Enum => self.parse_enum(),
            Token::For => self.parse_for(),
            Token::Try => self.parse_try_catch(),
            Token::Throw => self.parse_throw(),
            Token::Import => self.parse_import(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Fn => self.parse_fn_def(),
            Token::Return => self.parse_return(),
            Token::Break => { self.advance(); Some(Expr::Break) },
            Token::Continue => { self.advance(); Some(Expr::Continue) },
            _ => self.parse_logical_or(),
        }
    }

    fn parse_return(&mut self) -> Option<Expr> {
        self.advance(); // consume 'return'
        // Allow return without an expression (for void returns)
        if matches!(self.peek(), Token::Semicolon | Token::RBrace | Token::EOF) {
            Some(Expr::Return(Box::new(Expr::Null)))
        } else {
            let expr = self.parse_expr()?;
            Some(Expr::Return(Box::new(expr)))
        }
    }

    fn parse_import(&mut self) -> Option<Expr> {
        self.advance(); // consume 'import'
        if let Token::String(s) = self.peek() {
            let s = s.clone();
            self.advance();
            Some(Expr::Import(s))
        } else {
            None
        }
    }

    fn parse_let(&mut self) -> Option<Expr> {
        self.advance(); // consume 'let'
        let name = if let Token::Ident(n) = self.peek() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        if let Token::Assign = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let expr = self.parse_expr()?;
        Some(Expr::Let { name, expr: Box::new(expr) })
    }

    fn parse_const(&mut self) -> Option<Expr> {
        self.advance(); // consume 'const'
        let name = if let Token::Ident(n) = self.peek() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        if let Token::Assign = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let expr = self.parse_expr()?;
        Some(Expr::Const { name, expr: Box::new(expr) })
    }

    fn parse_match(&mut self) -> Option<Expr> {
        self.advance(); // consume 'match'
        let expr = self.parse_expr()?;
        if let Token::LBrace = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let mut arms = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::EOF) {
            // Parse pattern
            let pat = self.parse_expr()?;
            if let Token::FatArrow = self.peek() {
                self.advance();
            } else {
                return None;
            }
            let res = self.parse_expr()?;
            arms.push((pat, res));
            if let Token::Comma = self.peek() {
                self.advance();
            }
        }
        if let Token::RBrace = self.peek() {
            self.advance();
        }
        Some(Expr::Match { expr: Box::new(expr), arms })
    }

    fn parse_struct(&mut self) -> Option<Expr> {
        self.advance(); // consume 'struct'
        let name = if let Token::Ident(n) = self.peek() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        if let Token::LBrace = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let mut fields = Vec::new();
        while let Token::Ident(field) = self.peek() {
            fields.push(field.clone());
            self.advance();
            if let Token::Comma = self.peek() {
                self.advance();
            } else {
                break;
            }
        }
        if let Token::RBrace = self.peek() {
            self.advance();
        }
        Some(Expr::StructDef { name, fields })
    }

    fn parse_enum(&mut self) -> Option<Expr> {
        self.advance(); // consume 'enum'
        let name = if let Token::Ident(n) = self.peek() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        if let Token::LBrace = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let mut variants = Vec::new();
        while let Token::Ident(variant) = self.peek() {
            variants.push(variant.clone());
            self.advance();
            if let Token::Comma = self.peek() {
                self.advance();
            } else {
                break;
            }
        }
        if let Token::RBrace = self.peek() {
            self.advance();
        }
        Some(Expr::EnumDef { name, variants })
    }

    fn parse_for(&mut self) -> Option<Expr> {
        self.advance(); // consume 'for'
        let var = if let Token::Ident(n) = self.peek() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        if let Token::In = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let iter = self.parse_expr()?;
        let body = if let Token::LBrace = self.peek() {
            self.parse_block()?
        } else {
            return None;
        };
        Some(Expr::For { var, iter: Box::new(iter), body: Box::new(body) })
    }

    fn parse_try_catch(&mut self) -> Option<Expr> {
        self.advance(); // consume 'try'
        let try_block = self.parse_block()?;
        let mut catch_var = None;
        if let Token::Catch = self.peek() {
            self.advance();
            if let Token::Ident(var) = self.peek() {
                catch_var = Some(var.clone());
                self.advance();
            }
            let catch_block = self.parse_block()?;
            Some(Expr::TryCatch {
                try_block: Box::new(try_block),
                catch_var,
                catch_block: Box::new(catch_block),
            })
        } else {
            None
        }
    }

    fn parse_throw(&mut self) -> Option<Expr> {
        self.advance(); // consume 'throw'
        let expr = self.parse_expr()?;
        Some(Expr::Throw(Box::new(expr)))
    }

    fn parse_if(&mut self) -> Option<Expr> {
        self.advance(); // consume 'if'
        let cond = self.parse_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = if let Token::Else = self.peek() {
            self.advance();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };
        Some(Expr::If {
            cond: Box::new(cond),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Option<Expr> {
        self.advance(); // consume 'while'
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Some(Expr::While {
            cond: Box::new(cond),
            body: Box::new(body),
        })
    }

    fn parse_fn_def(&mut self) -> Option<Expr> {
        self.advance(); // consume 'fn'
        let name = if let Token::Ident(n) = self.peek() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        if let Token::LParen = self.peek() {
            self.advance();
        } else {
            return None;
        }
        let mut params = Vec::new();
        if let Token::RParen = self.peek() {
            self.advance();
        } else {
            loop {
                if let Token::Ident(n) = self.peek() {
                    params.push(n.clone());
                    self.advance();
                } else {
                    return None;
                }
                if let Token::Comma = self.peek() {
                    self.advance();
                } else {
                    break;
                }
            }
            if let Token::RParen = self.peek() {
                self.advance();
            } else {
                return None;
            }
        }
        // Accept optional semicolons before the block
        while let Token::Semicolon = self.peek() {
            self.advance();
        }
        let body = self.parse_block()?;
        Some(Expr::FnDef {
            name,
            params,
            body: Box::new(body),
        })
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
        let mut node = self.parse_comparison()?;
        loop {
            match self.peek() {
                Token::Eq => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "==".into(),
                        right: Box::new(right),
                    };
                }
                Token::NotEq => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "!=".into(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(node)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut node = self.parse_bitwise_or()?;
        loop {
            match self.peek() {
                Token::Lt => {
                    self.advance();
                    let right = self.parse_bitwise_or()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "<".into(),
                        right: Box::new(right),
                    };
                }
                Token::Gt => {
                    self.advance();
                    let right = self.parse_bitwise_or()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: ">".into(),
                        right: Box::new(right),
                    };
                }
                Token::Le => {
                    self.advance();
                    let right = self.parse_bitwise_or()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "<=".into(),
                        right: Box::new(right),
                    };
                }
                Token::Ge => {
                    self.advance();
                    let right = self.parse_bitwise_or()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: ">=".into(),
                        right: Box::new(right),
                    };
                }
                Token::Is => {
                    self.advance();
                    let is_not = if let Some(Token::Not) = self.tokens.get(self.pos) {
                        self.advance();
                        true
                    } else {
                        false
                    };
                    let right = self.parse_bitwise_or()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: if is_not { "is not".into() } else { "is".into() },
                        right: Box::new(right),
                    };
                }
                Token::In => {
                    self.advance();
                    let right = self.parse_bitwise_or()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "in".into(),
                        right: Box::new(right),
                    };
                }
                Token::Not => {
                    if let Some(Token::In) = self.tokens.get(self.pos + 1) {
                        self.advance(); // consume 'not'
                        self.advance(); // consume 'in'
                        let right = self.parse_bitwise_or()?;
                        node = Expr::BinaryOp {
                            left: Box::new(node),
                            op: "not in".into(),
                            right: Box::new(right),
                        };
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Some(node)
    }

    fn parse_bitwise_or(&mut self) -> Option<Expr> {
        let mut node = self.parse_bitwise_xor()?;
        while let Token::BitOr = self.peek() {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            node = Expr::BinaryOp {
                left: Box::new(node),
                op: "|".into(),
                right: Box::new(right),
            };
        }
        Some(node)
    }

    fn parse_bitwise_xor(&mut self) -> Option<Expr> {
        let mut node = self.parse_bitwise_and()?;
        while let Token::BitXor = self.peek() {
            self.advance();
            let right = self.parse_bitwise_and()?;
            node = Expr::BinaryOp {
                left: Box::new(node),
                op: "^".into(),
                right: Box::new(right),
            };
        }
        Some(node)
    }

    fn parse_bitwise_and(&mut self) -> Option<Expr> {
        let mut node = self.parse_shift()?;
        while let Token::BitAnd = self.peek() {
            self.advance();
            let right = self.parse_shift()?;
            node = Expr::BinaryOp {
                left: Box::new(node),
                op: "&".into(),
                right: Box::new(right),
            };
        }
        Some(node)
    }

    fn parse_shift(&mut self) -> Option<Expr> {
        let mut node = self.parse_term()?;
        loop {
            match self.peek() {
                Token::Shl => {
                    self.advance();
                    let right = self.parse_term()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "<<".into(),
                        right: Box::new(right),
                    };
                }
                Token::Shr => {
                    self.advance();
                    let right = self.parse_term()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: ">>".into(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(node)
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
        let mut node = self.parse_power()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_power()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "*".into(),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_power()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "/".into(),
                        right: Box::new(right),
                    };
                }
                Token::Mod => {
                    self.advance();
                    let right = self.parse_power()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "%".into(),
                        right: Box::new(right),
                    };
                }
                Token::FloorDiv => {
                    self.advance();
                    let right = self.parse_power()?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        op: "//".into(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(node)
    }

    fn parse_power(&mut self) -> Option<Expr> {
        let mut node = self.parse_unary()?;
        while let Token::Pow = self.peek() {
            self.advance();
            let right = self.parse_unary()?;
            node = Expr::BinaryOp {
                left: Box::new(node),
                op: "**".into(),
                right: Box::new(right),
            };
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
            Token::BitNot => {
                self.advance();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryOp { op: "~".into(), expr: Box::new(expr) })
            }
            _ => self.parse_call_or_index(),
        }
    }

    fn parse_call_or_index(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                Token::LParen => {
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
                        } else {
                            return None; // Error: expected closing parenthesis
                        }
                    }
                    expr = Expr::FnCall { callable: Box::new(expr), args };
                }
                Token::LBracket => {
                    self.advance();
                    let index_expr = self.parse_expr()?;
                    if let Token::RBracket = self.peek() {
                        self.advance();
                    } else {
                        return None; // Error: expected closing bracket
                    }
                    // Check for assignment to index
                    if let Token::Assign = self.peek() {
                        self.advance();
                        let assign_expr = self.parse_expr()?;
                        expr = Expr::AssignIndex {
                            collection: Box::new(expr),
                            index: Box::new(index_expr),
                            expr: Box::new(assign_expr),
                        };
                    } else {
                        expr = Expr::Index {
                            collection: Box::new(expr),
                            index: Box::new(index_expr),
                        };
                    }
                }
                Token::Dot => {
                    self.advance(); // consume '.'
                    if let Token::Ident(name) = self.peek() {
                        let name = name.clone();
                        self.advance();
                        expr = Expr::GetAttr { object: Box::new(expr), name };
                    } else {
                        return None; // Error: expected identifier after '.'
                    }
                }
                _ => break,
            }
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::LBrace => self.parse_block(),
            Token::LBracket => {
                self.advance();
                let mut items = Vec::new();
                if let Token::RBracket = self.peek() {
                    self.advance();
                    return Some(Expr::ArrayLiteral(items));
                }
                loop {
                    items.push(self.parse_expr()?);
                    if let Token::Comma = self.peek() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                if let Token::RBracket = self.peek() {
                    self.advance();
                } else {
                    // Error: expected closing bracket
                    return None;
                }
                Some(Expr::ArrayLiteral(items))
            }
            Token::Print => {
                self.advance();
                Some(Expr::Ident("print".to_string()))
            }
            Token::Input => {
                self.advance();
                Some(Expr::Ident("input".to_string()))
            }
            Token::True => { self.advance(); Some(Expr::Bool(true)) }
            Token::False => { self.advance(); Some(Expr::Bool(false)) }
            Token::Null => { self.advance(); Some(Expr::Null) }
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
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if let Token::RParen = self.peek() {
                    self.advance();
                    Some(expr)
                } else {
                    None
                }
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                if let Token::Comma = self.peek() {
                    // Destructuring assignment: (a, b) = ...
                    let mut names = vec![name.clone()];
                    while let Token::Comma = self.peek() {
                        self.advance();
                        if let Token::Ident(n) = self.peek() {
                            names.push(n.clone());
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if let Token::Assign = self.peek() {
                        self.advance();
                        let expr = self.parse_expr()?;
                        return Some(Expr::Destructure { names, expr: Box::new(expr) });
                    }
                }
                Some(Expr::Ident(name))
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
        let mut lexer = Lexer::new("x = 1");
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::EOF {
                break;
            }
            tokens.push(tok);
        }
        println!("TOKENS: {:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let ast = parser.parse();
        println!("AST: {:?}", ast);
        let ast = ast.expect("Parser returned None");
        match ast {
            Expr::Assign { ref name, .. } => {
                assert_eq!(name, "x");
            }
            Expr::Block(ref exprs) => {
                assert_eq!(exprs.len(), 1);
                if let Expr::Assign { name, .. } = &exprs[0] {
                    assert_eq!(name, "x");
                } else {
                    panic!("Expected assignment");
                }
            }
            _ => panic!("Expected assignment or block expression"),
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
        println!("TOKENS: {:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let ast = parser.parse();
        println!("AST: {:?}", ast);
        let ast = ast.expect("Parser returned None");
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
                        if let Expr::BinaryOp { ref op, .. } = **expr {
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
