#![allow(dead_code)]
use crate::smallvec::SmallVec;

pub struct Fifo<E: Copy> {
    fifo: SmallVec<E, 20>,
}

impl<E: Copy> Fifo<E> {
    pub fn new() -> Self {
        Self {
            fifo: SmallVec::new(),
        }
    }

    pub fn pop(&mut self) -> crate::Result<E> {
        if self.fifo.is_empty() {
            return Err("Trying to pop from an empty FIFO".into());
        }

        Ok(self.fifo.remove(0))
    }

    pub fn push(&mut self, e: E) {
        self.fifo.push(e);
    }

    pub fn is_empty(&self) -> bool {
        self.fifo.is_empty()
    }
}
