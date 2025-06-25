// Lexer for StelLang

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
    Mod,
    Pow,
    Eq,
    NotEq,
    Lt,
    Gt,
    Le,
    Ge,
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
    In,
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
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Shl,
    Shr,
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
    Dot,
    DoubleDot,
    TripleDot,
    Semi,
    CommaTok,
    LBracket,
    RBracket,
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

    fn read_number(&mut self) -> Token {
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
            Token::Float(num.parse().unwrap())
        } else {
            Token::Integer(num.parse().unwrap())
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
            _ => Token::Ident(ident),
        }
    }

    fn read_string(&mut self) -> Token {
        let mut s = String::new();
        self.advance(); // skip opening quote
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                break;
            }
            s.push(ch);
            self.advance();
        }
        Token::String(s)
    }

    pub fn next_token(&mut self) -> Token {
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
            Some('=') => { self.advance(); Token::Assign },
            Some('+') => { self.advance(); Token::Plus },
            Some('-') => { self.advance(); Token::Minus },
            Some('*') => { self.advance(); Token::Star },
            Some('/') => { self.advance(); Token::Slash },
            Some('(') => { self.advance(); Token::LParen },
            Some(')') => { self.advance(); Token::RParen },
            Some('{') => { self.advance(); Token::LBrace },
            Some('}') => { self.advance(); Token::RBrace },
            Some(',') => { self.advance(); Token::Comma },
            Some(';') => { self.advance(); Token::Semicolon },
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(ch) if ch.is_alphabetic() || ch == '_' => self.read_ident(),
            Some(_) => { self.advance(); self.next_token() },
            None => Token::EOF,
        }
    }
}
