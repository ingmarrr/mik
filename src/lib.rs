pub mod diagnostics;
pub mod lexer;
pub mod parse_tree;
pub mod parser;
pub mod token;
pub mod tokenized_buffer;

pub(crate) mod fifo;
pub(crate) mod smallvec;

pub type Result<T> = ::core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;
