use std::ops::Deref;

#[derive(Debug)]
pub struct Token(pub usize);

impl Deref for Token {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SourceLocation<'a> {
    pub file: Option<&'a str>,
    pub row: usize,
    pub col: usize,
}

impl<'a> SourceLocation<'a> {
    pub fn new(file: Option<&'a str>, row: usize, col: usize) -> Self {
        Self { file, row, col }
    }
}

impl<'a> From<(usize, usize)> for SourceLocation<'a> {
    fn from((row, col): (usize, usize)) -> Self {
        Self {
            file: None,
            row,
            col,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
pub struct TokenInfo {
    pub row: usize,
    pub col: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Ident,

    IntLiteral,
    RealLiteral,

    FuncKeyword,
    TypeKeyword,
    ModuleKeyword,

    HorizontalWhitespace,
    VerticalWhitespace,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    Plus,
    Minus,
    Star,
    Slash,

    Eof,
    Invalid,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Ident => write!(f, "Ident"),

            TokenKind::IntLiteral => write!(f, "IntLiteral"),
            TokenKind::RealLiteral => write!(f, "RealLiteral"),

            TokenKind::FuncKeyword => write!(f, "FuncKeyword"),
            TokenKind::TypeKeyword => write!(f, "TypeKeyword"),
            TokenKind::ModuleKeyword => write!(f, "ModuleKeyword"),

            TokenKind::HorizontalWhitespace => write!(f, "HorizontalWhitespace"),
            TokenKind::VerticalWhitespace => write!(f, "VerticalWhitespace"),

            TokenKind::OpenParen => write!(f, "OpenParen"),
            TokenKind::CloseParen => write!(f, "CloseParen"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::OpenBracket => write!(f, "OpenBracket"),
            TokenKind::CloseBracket => write!(f, "CloseBracket"),

            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Star => write!(f, "Star"),
            TokenKind::Slash => write!(f, "Slash"),

            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Invalid => write!(f, "Invalid"),
        }
    }
}
