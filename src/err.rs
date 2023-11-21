#[derive(Debug, thiserror::Error)]
pub enum LexErr {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

#[derive(Debug, thiserror::Error)]
pub enum RunErr {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
