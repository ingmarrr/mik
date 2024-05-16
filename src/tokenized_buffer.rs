use super::token::{SourceLocation, Span, Token, TokenInfo, TokenKind};

pub struct TokenizedBuffer<'a> {
    file: Option<&'a str>,
    source: &'a [u8],
    tokens: Vec<Token>,
    kinds: Vec<TokenKind>,
    locations: Vec<Span>,
    spans: Vec<Span>,
    lines: Vec<Span>,
    count: usize,
}

impl<'a> TokenizedBuffer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        let lines = source
            .split(|&c| c == b'\n')
            .scan(0, |pos, line| {
                if *pos >= source.len() {
                    return None;
                }

                let span = Span {
                    start: *pos,
                    end: *pos + line.len(),
                };
                *pos += line.len() + 1;
                Some(span)
            })
            .collect();
        Self {
            file: None,
            source,
            tokens: vec![],
            kinds: vec![],
            locations: vec![],
            spans: vec![],
            lines,
            count: 0,
        }
    }

    pub fn push(&mut self, kind: TokenKind, info: TokenInfo) {
        let next_token = self.next_token();
        self.tokens.push(next_token);
        self.locations.push(Span {
            start: info.row,
            end: info.col,
        });
        self.spans.push(Span {
            start: info.start,
            end: info.end,
        });
        self.kinds.push(kind);
    }

    pub fn get_line(&self, line: usize) -> Option<&'a str> {
        if line >= self.lines.len() {
            return None;
        }
        let span = unsafe { self.lines.get_unchecked(line) };
        std::str::from_utf8(&self.source[span.start..span.end]).ok()
    }

    pub fn span_of(&self, token: &Token) -> Span {
        let index = token.0;
        self.spans[index]
    }

    pub fn kind_of(&self, token: &Token) -> TokenKind {
        let index = token.0;
        self.kinds[index]
    }

    pub fn location_of(&self, token: &Token) -> SourceLocation {
        let index = token.0;
        let span = self.locations[index];
        SourceLocation {
            file: self.file,
            row: span.start,
            col: span.end,
        }
    }

    pub fn str_of(&self, token: &Token) -> &'a str {
        let span = self.span_of(token);
        unsafe { std::str::from_utf8_unchecked(&self.source[span.start..span.end]) }
    }

    fn next_token(&mut self) -> Token {
        self.count += 1;
        Token(self.count - 1)
    }
}

impl std::fmt::Debug for TokenizedBuffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenizedBuffer")
            .field("source", &std::str::from_utf8(self.source).unwrap())
            .field("tokens", &self.tokens)
            .field("locations", &self.locations)
            .field("spans", &self.spans)
            .field("lines", &self.lines)
            .finish()
    }
}

impl Default for TokenizedBuffer<'_> {
    fn default() -> Self {
        TokenizedBuffer {
            file: None,
            source: &[],
            tokens: vec![],
            locations: vec![],
            spans: vec![],
            lines: vec![],
            kinds: vec![],
            count: 0,
        }
    }
}

pub struct TokenizedBufferIter<'a> {
    buffer: &'a TokenizedBuffer<'a>,
    index: usize,
}

impl<'a> TokenizedBufferIter<'a> {
    pub fn new(buffer: &'a TokenizedBuffer<'a>) -> Self {
        Self { buffer, index: 0 }
    }
}

impl<'a> Iterator for TokenizedBufferIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.tokens.len() {
            return None;
        }
        let token = Token(self.index);
        self.index += 1;
        Some(token)
    }
}

impl<'a> IntoIterator for &'a TokenizedBuffer<'a> {
    type Item = Token;
    type IntoIter = TokenizedBufferIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenizedBufferIter::new(self)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_line() {
        let source = "hello\n\nworld\n";
        let buffer = super::TokenizedBuffer::new(source.as_bytes());
        println!("{buffer:#?}");
        assert_eq!(buffer.get_line(0), Some("hello"));
        assert_eq!(buffer.get_line(1), Some(""));
        assert_eq!(buffer.get_line(2), Some("world"));
        assert_eq!(buffer.get_line(3), None);
    }
}
