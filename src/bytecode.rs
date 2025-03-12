//! Module storing the building blocks for sequence of `mm` bytecode

#[derive(Debug)]
pub enum OpCode {
    // Return from function / call
    Return,
}

/// A series of bytecode instructions
pub struct Sequence {
    code: Vec<u8>,
}

impl Sequence {
    pub fn new() -> Self {
        Self { code: vec![] }
    }

    pub fn push(&mut self, byte: u8) {
        self.code.push(byte)
    }
}
