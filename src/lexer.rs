use std::{cell::RefCell, rc::Rc};

use crate::diagnostics::{self, Diagnostic, DiagnosticEmitter};
use crate::fifo::Fifo;

use super::{
    token::{self, TokenInfo},
    tokenized_buffer::TokenizedBuffer,
};

pub struct Lexer<'a> {
    diagnostics: Rc<RefCell<dyn DiagnosticEmitter>>,
    file_name: Option<&'a str>,
    source: &'a [u8],
    context: Context,
    lookahead: Option<Context>,
    tokens: TokenizedBuffer<'a>,
}

pub struct Context {
    row: usize,
    col: usize,
    pos: usize,
    pending: Fifo<u8>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            pos: 0,
            pending: Fifo::new(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let bytes = source.as_bytes();
        Lexer {
            diagnostics: Rc::new(RefCell::new(diagnostics::DefaultEmitter)),
            file_name: None,
            source: bytes,
            context: Context::new(),
            lookahead: None,
            tokens: TokenizedBuffer::new(bytes),
        }
    }

    pub fn new_file(file_name: &'a str, source: &'a str) -> Self {
        let bytes = source.as_bytes();
        Lexer {
            diagnostics: Rc::new(RefCell::new(diagnostics::DefaultEmitter)),
            file_name: Some(file_name),
            source: bytes,
            context: Context::new(),
            lookahead: None,
            tokens: TokenizedBuffer::new(bytes),
        }
    }

    pub fn with_file_name(mut self, file_name: &'a str) -> Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn tokenize(mut self) -> TokenizedBuffer<'a> {
        while let Some(byte) = self.peek() {
            match byte {
                b' ' | b'\t' => self.visit_horizontal_whitespace(),
                b'\r' | b'\n' => self.visit_vertical_whitespace(),
                // b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.visit_identifier(),
                b'0'..=b'9' => self.visit_number(),
                b'+' | b'-' | b'*' | b'/' => self.visit_operator(),
                b'\0' => {
                    self.take();
                    self.push(
                        token::TokenKind::Eof,
                        token::Span::new(self.context.pos, self.context.pos),
                    );
                    break;
                }
                _ => {
                    self.take();
                    self.push(
                        token::TokenKind::Invalid,
                        token::Span::new(self.context.pos, self.context.pos),
                    );
                }
            };
        }
        return self.tokens;
    }

    fn visit_operator(&mut self) {
        let start = self.context.pos;
        let op = self.take();
        assert!(op.is_some());
        self.push(
            match op.unwrap() {
                b'+' => token::TokenKind::Plus,
                b'-' => token::TokenKind::Minus,
                b'*' => token::TokenKind::Star,
                b'/' => token::TokenKind::Slash,
                _ => unreachable!(),
            },
            token::Span::new(start, self.context.pos),
        );
    }

    fn visit_number(&mut self) {
        let initial_pos = self.context.pos;
        let mut is_float = false;
        while let Some(byte) = self.peek() {
            match byte {
                b'0'..=b'9' => self.take(),
                _ => break,
            };
        }

        match self.peek() {
            Some(b'.') => {
                self.take();
                while let Some(byte) = self.peek() {
                    match byte {
                        b'0'..=b'9' => self.take(),
                        _ => break,
                    };
                }
                is_float = true;
            }
            _ => {}
        }

        match self.peek() {
            Some(b'e') | Some(b'E') => {
                self.take();
                match self.peek() {
                    Some(b'+') | Some(b'-') => {
                        self.take();
                    }
                    _ => {}
                };
                while let Some(byte) = self.peek() {
                    match byte {
                        b'0'..=b'9' => self.take(),
                        _ => break,
                    };
                }
            }
            _ => {}
        }

        self.push(
            if is_float {
                token::TokenKind::RealLiteral
            } else {
                token::TokenKind::IntLiteral
            },
            token::Span::new(initial_pos, self.context.pos),
        );
    }

    fn visit_horizontal_whitespace(&mut self) {
        let initial_pos = self.context.pos;
        while let Some(byte) = self.peek() {
            match byte {
                b' ' | b'\t' => self.take(),
                _ => break,
            };
        }
        self.push(
            token::TokenKind::HorizontalWhitespace,
            token::Span::new(initial_pos, self.context.pos),
        );
    }

    fn visit_vertical_whitespace(&mut self) {
        let initial_pos = self.context.pos;
        while let Some(byte) = self.peek() {
            match byte {
                b'\r' | b'\n' => self.take(),
                _ => break,
            };
        }
        self.push(
            token::TokenKind::VerticalWhitespace,
            token::Span::new(initial_pos, self.context.pos),
        );
    }

    fn push(&mut self, kind: token::TokenKind, span: token::Span) {
        self.tokens.push(
            kind,
            TokenInfo {
                row: self.context.row,
                col: self.context.col,
                start: span.start,
                end: span.end,
            },
        );
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.context.pos).copied()
    }

    fn take(&mut self) -> Option<u8> {
        let byte = self.peek();
        if byte.is_some() {
            self.context.pos += 1;
            if byte == Some(b'\n') {
                self.context.row += 1;
                self.context.col = 0;
            } else {
                self.context.col += 1;
            }
        }
        byte
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.borrow_mut().emit(diagnostic);
    }

    fn emit_error(&mut self, message: &str) {
        self.emit(Diagnostic {
            message,
            level: diagnostics::Level::Error,
            location: token::SourceLocation::new(
                self.file_name,
                self.context.row,
                self.context.col,
            ),
        });
    }
}
