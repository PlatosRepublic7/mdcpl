use crate::source::SourceLocation;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Identifiers
    Identifier,

    // Numeric
    Integer,
    FloatingPoint,

    // Keywords
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Signed,
    Unsigned,
    Void,
    Auto,
    Const,
    Extern,
    Register,
    Static,
    Volatile,
    Break,
    Case,
    Continue,
    Default,
    Do,
    Else,
    For,
    Goto,
    If,
    Return,
    Switch,
    While,
    Enum,
    Sizeof,
    Struct,
    Typedef,
    Union,

    // Punctuation Literals
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    DoubleQuote,
    SingleQuote,
    Semicolon,
    Colon,
    Comma,
    Point,

    // Operators
    Assignment,         // =
    Addition,
    AddAssign,          // +=
    SubAssign,
    MultAssign,
    DivAssign,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    PrefixIncrement,
    PostfixIncrement,
    Equal,              // ==
    NotEqual,
    Not,
    And,
    Or,
    Xor,
    BitAnd,
    BitOr,
    BitXor,

    // Terminating
    Unknown,
    Eof
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub location: SourceLocation
}

impl Token {
    pub fn new(tk: TokenKind, lex: String, loc: SourceLocation) -> Self {
        Token {
            kind: tk,
            lexeme: lex,
            location: loc
        }
    }
}
