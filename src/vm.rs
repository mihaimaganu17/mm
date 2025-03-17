use crate::{Disassembler, OpCode, Value, Sequence};
use std::collections::LinkedList;

// Flag enabling/disabling VM execution tracing for debugging
const DEBUG_TRACE_EXECUTION: bool = false;

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
    // Stack that holds the operators needed to perform any of the VM's operations. It does not
    // hold the actual operands or a reference to them, but rather, their index into the VM's
    // storage.
    stack: LinkedList<usize>,
}

impl<'vm> VM<'vm> {
    pub fn new(sequence: &'vm Sequence) -> Self {
        Self {
            sequence,
            offset: 0,
            stack: LinkedList::new(),
        }
    }

    // Interprets the sequence of bytes passed to the VM
    pub fn interpret(&mut self, sequence: &'vm Sequence) -> Result<(), InterpretError> {
        self.sequence = sequence;

        loop {
            // If we reached the end of the sequence, break
            if self.offset == self.sequence.code().len() {
                break;
            }
            // If we want to trace the debugging
            if DEBUG_TRACE_EXECUTION {
                // Headline for the stack
                println!("== Stack conttents ==");
                // Print the stack contents
                for value_idx in self.stack.iter() {
                    println!("[{}]", self.sequence.constant(*value_idx));
                }
                println!("== Current instruction ==");
                // We disassemble the instruction at the current point
                Disassembler::dis_instruction(sequence, self.offset);
            }
            // Get the instruction opcode
            let instruction = self.sequence.code()[self.offset].into();
            // Get past the opcode
            self.offset += 1;
            // Dispatch the instruction
            match instruction {
                OpCode::Return => {
                    return Ok(())
                }
                OpCode::Constant => {
                    let value = &self.sequence.read_constant(self.offset);
                    // Go past the constant
                    self.offset += 1;
                    println!("{value}");
                }
                _ => todo!(),
            }
        }
        Ok(())
    }

    // Empties the VM's stack
    pub fn reset_stack(&mut self) {
        // Pop elements from the stack until is is empty
        while let Some(_) = self.stack.pop_back() {}
    }
}

#[derive(Debug)]
pub enum InterpretError {
    // Reports a static error when compiling the source code
    CompileError,
    // Reports a dynamic error when running the bytecode
    RuntimeError,
}

pub enum Interpret {}
