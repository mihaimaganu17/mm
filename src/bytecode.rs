//! Module storing the building blocks for sequence of `mm` bytecode
use crate::value::{Value, ValueVec};

#[derive(Debug)]
pub enum OpCode {
    // Return from function / call
    Return,
    // Load / produce a constant with the index given by the byte following the opcode. This limits
    // the expressed constant index to be 256 bytes long
    Constant,
    // Load / produce a constant with the index given by the next 3 bytes following the opcode in
    // LittleEndian format
    ConstantLong,
    // Corresponds to the minus `-` operator that negates the succeding operand
    Negate,
    // Corresponds to the plus `+` infix operator that adds 2 values
    Add,
    // Corresponds to the plus `-` infix operator that subtracts 2 values
    Sub,
    // Corresponds to the plus `*` infix operator that multiplies 2 values
    Mul,
    // Corresponds to the plus `/` infix operator that divides 2 values
    Div,
    // Unknown byte, kept for debugging
    Unknown(u8),
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Return,
            1 => Self::Constant,
            2 => Self::ConstantLong,
            3 => Self::Negate,
            4 => Self::Add,
            5 => Self::Sub,
            6 => Self::Mul,
            7 => Self::Div,
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
            Self::ConstantLong => Ok(2),
            Self::Negate => Ok(3),
            Self::Add => Ok(4),
            Self::Sub=> Ok(5),
            Self::Mul=> Ok(6),
            Self::Div=> Ok(7),
            Self::Unknown(value) => Ok(value),
        }
    }
}

/// A series of bytecode instructions
#[derive(Default)]
pub struct Sequence {
    // Stores the entire bytes code sequence
    code: Vec<u8>,
    // Stores lines of the source code, using a run-length encoding algorithm where successive
    // opcodes stored on the same line only store the line number and the number of successive
    // occurences of that line number. Information is stored as 2 integers:
    // 1. First is the line number
    // 2. Second is the number of occurences of the line
    lines: Vec<(u32, u32)>,
    // Stores constant values, referred to by their index
    constants: ValueVec,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn push<T: TryInto<u8>>(&mut self, byte: T, line: u32) -> Result<(), SequenceError> {
        self.code
            .push(byte.try_into().map_err(|_e| SequenceError::PushByte)?);
        if !self.lines.is_empty() {
            let last_line_idx = self.lines.len() - 1;
            let last_line = self.lines[last_line_idx];
            if line == last_line.0 {
                // If the `line` is the same as the last line pushed, we only increase the number of
                // occurences of the last already existing entry in lines
                self.lines[last_line_idx] = (last_line.0, last_line.1 + 1);
            } else {
                // Otherwise we add a new entry
                self.lines.push((line, 1));
            }
        } else {
            // Otherwise we add a new entry
            self.lines.push((line, 1));
        }
        Ok(())
    }

    pub fn code(&self) -> &[u8] {
        self.code.as_slice()
    }

    pub fn line(&self, idx: usize) -> u32 {
        // Computes the index of the last opcode having the current line number
        let mut run_length_index = 0;
        // For each occurence of line's run-length
        for (line, occurences) in self.lines.iter() {
            // Add the number of occurences
            run_length_index += occurences;
            // If our number of occurences goes past the desired index, we are in the desired range
            if (idx as u32) < run_length_index {
                return *line;
            }
        }
        0
    }

    pub fn constant(&self, idx: usize) -> &Value {
        &self.constants.values()[idx]
    }

    pub fn read_constant(&self, offset: usize) -> &Value {
        let idx = self.code()[offset];
        self.constant(usize::from(idx))
    }

    pub fn from_slice<P: AsRef<[u8]>>(value: P) -> Self {
        Self {
            code: value.as_ref().to_vec(),
            ..Default::default()
        }
    }

    /// Appends a new value to the underlying storage for this byte sequence's constants.
    /// Returns the index where the value was added
    pub fn add_constant(&mut self, value: Value) -> usize {
        // Push the value in the constants pool
        self.constants.push(value);
        // Get the index where the constant was pushed
        self.constants.0.len() - 1
    }

    /// Writes a constant's index as a one-byte or 3-byte form
    pub fn write_constant(&mut self, value: Value, line: u32) -> Result<(), SequenceError> {
        let idx = self.add_constant(value);

        // Check the size of our index value. If the value exceeds 255.
        if idx > 0xff {
            // Push the opcode
            self.push(OpCode::ConstantLong, line)?;
            // We decide to store its index as a 3-byte value in Little Endian
            let bytes = idx.to_le_bytes();
            for byte in bytes.iter().take(3) {
                self.push(*byte, line)?;
            }
        } else {
            // Push the opcode
            self.push(OpCode::Constant, line)?;
            // Otherwise, we just write the 1-byte value
            self.push(idx, line)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SequenceError {
    PushByte,
}

#[derive(Debug)]
pub enum OpCodeError {}
