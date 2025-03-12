//! Module storing the building blocks for sequence of `mm` bytecode

#[derive(Debug)]
pub enum OpCode {
    // Return from function / call
    Return,
    // Unknown byte, kept for debugging
    Unknown(u8),
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Return,
            _ => Self::Unknown(value),
        }
    }
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

    pub fn code(&self) -> &[u8] {
        self.code.as_slice()
    }

    pub fn from_slice<P: AsRef<[u8]>>(value: P) -> Self {
        Self { code: value.as_ref().to_vec() }
    }
}
