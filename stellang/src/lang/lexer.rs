// Lexer for StelLang

use super::exceptions::{Exception, ExceptionKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    Ident(String),
    String(String),
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    If,
    Else,
    While,
    Fn,
    Return,
    Break,
    Continue,
    And,
    Or,
    Not,
    EOF,
    // --- Extended tokens for future features, stdlib, and language expansion ---
    Mod,      // %
    Pow,      // **
    Eq,       // ==
    NotEq,    // !=
    Lt,       // <
    Gt,       // >
    Le,       // <=
    Ge,       // >=
    FloorDiv, // //
    BitAnd,   // &
    BitOr,    // |
    BitXor,   // ^
    BitNot,   // ~
    Shl,      // <<
    Shr,      // >>
    Is,       // is
    In,       // in
    True,
    False,
    Null,
    Print,
    Input,
    Let,
    Const,
    Struct,
    Enum,
    Match,
    Case,
    Import,
    Export,
    As,
    From,
    Use,
    Pub,
    Static,
    Mut,
    Ref,
    SelfValue,
    Super,
    This,
    New,
    Class,
    Interface,
    Implements,
    Extends,
    Trait,
    Where,
    Type,
    Dyn,
    Async,
    Await,
    Yield,
    Throw,
    Try,
    Catch,
    Finally,
    With,
    Do,
    For,
    Foreach,
    Of,
    Range,
    Step,
    ContinueOuter,
    BreakOuter,
    Goto,
    Label,
    Macro,
    Include,
    Define,
    Undef,
    Line,
    File,
    Column,
    Debug,
    Trace,
    Warn,
    ErrorTok,
    Fatal,
    Todo,
    Unreachable,
    Deprecated,
    Override,
    Abstract,
    Virtual,
    Final,
    Synchronized,
    Volatile,
    Transient,
    Native,
    Package,
    Module,
    Library,
    Extern,
    Unsafe,
    Operator,
    Overload,
    Inline,
    NoInline,
    Register,
    Restrict,
    ThreadLocal,
    Alignas,
    Alignof,
    StaticAssert,
    Concept,
    Requires,
    Default,
    Explicit,
    Friend,
    Template,
    Typename,
    Namespace,
    Using,
    Question,
    Colon,
    DoubleColon,
    Arrow,
    FatArrow,
    Backtick,
    Dollar,
    At,
    Hash,
    Pipe,
    Backslash,
    Underscore,
    Tilde,
    Percent,
    Caret,
    Dot,        // .
    DoubleDot,  // ..
    TripleDot,  // ...
    Semi,
    CommaTok,
    LBracket,  // [
    RBracket,  // ]
    LAngle,
    RAngle,
    LCurly,
    RCurly,
    LParenTok,
    RParenTok,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<Token, Exception> {
        let mut num = String::new();
        let mut is_float = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if is_float {
            num.parse::<f64>().map(Token::Float).map_err(|e| Exception::new(ExceptionKind::ValueError, vec![format!("Invalid float literal: {}", e)]))
        } else {
            num.parse::<i64>().map(Token::Integer).map_err(|e| Exception::new(ExceptionKind::ValueError, vec![format!("Invalid integer literal: {}", e)]))
        }
    }

    fn read_ident(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "let" => Token::Let,
            "const" => Token::Const,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "print" => Token::Print,
            "input" => Token::Input,
            "match" => Token::Match,
            "case" => Token::Case,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "for" => Token::For,
            "in" => Token::In,
            "is" => Token::Is,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "throw" => Token::Throw,
            "import" => Token::Import,
            _ => Token::Ident(ident),
        }
    }

    fn read_string(&mut self) -> Result<Token, Exception> {
        let mut s = String::new();
        self.advance(); // skip opening quote
        let mut closed = false;
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                closed = true;
                break;
            }
            s.push(ch);
            self.advance();
        }
        if closed {
            Ok(Token::String(s))
        } else {
            Err(Exception::new(ExceptionKind::SyntaxError, vec!["Unterminated string literal".to_string()]))
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Exception> {
        self.skip_whitespace();
        // Skip comments
        if let Some('#') = self.peek() {
            while let Some(ch) = self.peek() {
                if ch == '\n' { break; }
                self.advance();
            }
            self.skip_whitespace();
        }
        match self.peek() {
            Some('"') => self.read_string(),
            Some('=') => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Ok(Token::Eq)
                } else {
                    Ok(Token::Assign)
                }
            },
            Some('!') => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Ok(Token::NotEq)
                } else {
                    Ok(Token::Not)
                }
            },
            Some('<') => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Ok(Token::Le)
                } else if let Some('<') = self.peek() {
                    self.advance();
                    Ok(Token::Shl)
                } else {
                    Ok(Token::Lt)
                }
            },
            Some('>') => {
                self.advance();
                if let Some('=') = self.peek() {
                    self.advance();
                    Ok(Token::Ge)
                } else if let Some('>') = self.peek() {
                    self.advance();
                    Ok(Token::Shr)
                } else {
                    Ok(Token::Gt)
                }
            },
            Some('+') => { self.advance(); Ok(Token::Plus) },
            Some('-') => { self.advance(); Ok(Token::Minus) },
            Some('*') => {
                self.advance();
                if let Some('*') = self.peek() {
                    self.advance();
                    Ok(Token::Pow)
                } else {
                    Ok(Token::Star)
                }
            },
            Some('/') => {
                self.advance();
                if let Some('/') = self.peek() {
                    self.advance();
                    Ok(Token::FloorDiv)
                } else {
                    Ok(Token::Slash)
                }
            },
            Some('%') => { self.advance(); Ok(Token::Mod) },
            Some('&') => { self.advance(); Ok(Token::BitAnd) },
            Some('|') => { self.advance(); Ok(Token::BitOr) },
            Some('^') => { self.advance(); Ok(Token::BitXor) },
            Some('~') => { self.advance(); Ok(Token::BitNot) },
            Some('(') => { self.advance(); Ok(Token::LParen) },
            Some(')') => { self.advance(); Ok(Token::RParen) },
            Some('[') => { self.advance(); Ok(Token::LBracket) },
            Some(']') => { self.advance(); Ok(Token::RBracket) },
            Some('{') => { self.advance(); Ok(Token::LBrace) },
            Some('}') => { self.advance(); Ok(Token::RBrace) },
            Some(',') => { self.advance(); Ok(Token::Comma) },
            Some(';') => { self.advance(); Ok(Token::Semicolon) },
            Some('.') => { self.advance(); Ok(Token::Dot) }, // Added for attribute access
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(ch) if ch.is_alphabetic() || ch == '_' => Ok(self.read_ident()),
            Some(ch) => Err(Exception::new(ExceptionKind::SyntaxError, vec![format!("Unexpected character: {}", ch)])),
            None => Ok(Token::EOF),
        }
    }
}
