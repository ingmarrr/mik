use crate::tokenized_buffer::TokenizedBuffer;

pub struct Parser<'a> {
    buf: &'a TokenizedBuffer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a TokenizedBuffer<'a>) -> Self {
        Self { buf }
    }
}
