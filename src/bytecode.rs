//! Module storing the building blocks for sequence of `mm` bytecode
use crate::value::{ValueVec, Value};

#[derive(Debug)]
pub enum OpCode {
    // Return from function / call
    Return,
    // Load / produce a constant with the index given by the byte following the opcode
    Constant,
    // Unknown byte, kept for debugging
    Unknown(u8),
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Return,
            1 => Self::Constant,
            _ => Self::Unknown(value),
        }
    }
}

impl TryInto<u8> for OpCode {
    type Error = OpCodeError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Self::Return => Ok(0),
            Self::Constant => Ok(1),
            Self::Unknown(value) => Ok(value),
        }
    }
}

/// A series of bytecode instructions
#[derive(Default)]
pub struct Sequence {
    code: Vec<u8>,
    constants: ValueVec,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn push<T: TryInto<u8>>(&mut self, byte: T) -> Result<(), SequenceError> {
        self.code.push(byte.try_into().map_err(|_e| SequenceError::PushByte)?);
        Ok(())
    }

    pub fn code(&self) -> &[u8] {
        self.code.as_slice()
    }

    pub fn from_slice<P: AsRef<[u8]>>(value: P) -> Self {
        Self {
            code: value.as_ref().to_vec(),
            constants: ValueVec::new(),
        }
    }

    /// Appends a new value to the underlying storage for this byte sequence's constants.
    /// Returns the index where the value was added
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.0.len() - 1
    }
}

#[derive(Debug)]
pub enum SequenceError {
    PushByte,
}

#[derive(Debug)]
pub enum OpCodeError {
}
