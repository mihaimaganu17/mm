use crate::{OpCode, Sequence};

pub struct VM<'vm> {
    // Sequence of bytecode that the VM executes
    sequence: &'vm Sequence,
    // Offset to the byte opcode that needs executing.
    // Note: This type of variable is desired to be kept in a local variable. This is because it
    // gets modified so often during execution that we want the compiler to store it in a register.
    // Note: Allegedly in C, it would be faster to dereference a pointer than look up an element in
    // an array by index. For the Rust case, the compiler makes use of instructions that do pointer
    // math and dereferencing in 1 or 2 cycles (like LEA on x86) so this claim does not hold
    offset: usize,
}

impl<'vm> VM<'vm> {
    pub fn new(sequence: &'vm Sequence) -> Self {
        Self {
            sequence,
            offset: 0,
        }
    }

    // Interprets the sequence of bytes passed to the VM
    pub fn interpret(&mut self, sequence: &'vm Sequence) -> Result<(), InterpretError> {
        self.sequence = sequence;

        let instruction = self.sequence.code()[self.offset].into();
        match instruction {
            OpCode::Return => {
                return Ok(())
            }
            _ => todo!(),
        }
        Ok(())
    }
}

pub enum InterpretError {
    // Reports a static error when compiling the source code
    CompileError,
    // Reports a dynamic error when running the bytecode
    RuntimeError,
}

pub enum Interpret {}
