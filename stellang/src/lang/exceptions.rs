// Python-style exception hierarchy for StelLang
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExceptionKind {
    BaseException,
    Exception,
    ArithmeticError,
    AssertionError,
    AttributeError,
    BufferError,
    EOFError,
    FloatingPointError,
    GeneratorExit,
    ImportError,
    ModuleNotFoundError,
    IndexError,
    KeyError,
    KeyboardInterrupt,
    MemoryError,
    NameError,
    NotImplementedError,
    OSError,
    OverflowError,
    RecursionError,
    ReferenceError,
    RuntimeError,
    StopIteration,
    StopAsyncIteration,
    SyntaxError,
    IndentationError,
    TabError,
    SystemError,
    SystemExit,
    TypeError,
    UnboundLocalError,
    UnicodeError,
    UnicodeEncodeError,
    UnicodeDecodeError,
    UnicodeTranslateError,
    ValueError,
    ZeroDivisionError,
    Warning,
    UserWarning,
    DeprecationWarning,
    PendingDeprecationWarning,
    SyntaxWarning,
    RuntimeWarning,
    FutureWarning,
    ImportWarning,
    UnicodeWarning,
    BytesWarning,
    ResourceWarning,
    EncodingWarning,
    BlockingIOError,
    ChildProcessError,
    ConnectionError,
    BrokenPipeError,
    ConnectionAbortedError,
    ConnectionRefusedError,
    ConnectionResetError,
    FileExistsError,
    FileNotFoundError,
    InterruptedError,
    IsADirectoryError,
    NotADirectoryError,
    PermissionError,
    ProcessLookupError,
    TimeoutError,
    // ...add more as needed
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Exception {
    pub kind: ExceptionKind,
    pub args: Vec<String>,
    pub context: Option<Box<Exception>>,
    pub cause: Option<Box<Exception>>,
    pub suppress_context: bool,
    pub notes: Vec<String>,
}

impl Exception {
    pub fn new(kind: ExceptionKind, args: Vec<String>) -> Self {
        Exception {
            kind,
            args,
            context: None,
            cause: None,
            suppress_context: false,
            notes: vec![],
        }
    }
    pub fn with_context(mut self, ctx: Exception) -> Self {
        self.context = Some(Box::new(ctx));
        self
    }
    pub fn with_cause(mut self, cause: Exception) -> Self {
        self.cause = Some(Box::new(cause));
        self.suppress_context = true;
        self
    }
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }
}
