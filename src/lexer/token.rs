use crate::source::SourceLocation;

pub enum TokenKind {
    Identifier,
    Constant,
    If,
    While,
    For,
    Return,
    Void,
    Int,
    Char,
    Float,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Unknown,
    Eof
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    location: SourceLocation
}
