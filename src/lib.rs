mod alloc;
mod bytecode;
mod dis;
mod value;
mod vm;
mod interpret;
mod compiler;
mod scan;


pub use bytecode::{OpCode, Sequence};
pub use dis::Disassembler;
pub use value::Value;
pub use vm::VM;
use interpret::{Interpreter, InterpretError};

use std::{
    io::{self, Write},
    path::Path,
    fs,
};

#[derive(Default)]
pub struct MMalis;

impl MMalis {
    /// Scans, compiles and executes a Malis file found in `path`
    pub fn execute<P: AsRef<Path>>(path: P) -> Result<(), MMalisError> {
        // Create a new `MMalis` object
        let mut malis = Self::default();
        // Read the file from the path
        let source = fs::read(path)?;
        // Run the contents of the file
        malis.run(&source, false)
    }

    // Main, single point running function for executiong of `bytes`
    fn run(&mut self, bytes: &[u8], _is_repl: bool) -> Result<(), MMalisError> {
        let interpreter = Interpreter;
        Ok(interpreter.interpret(bytes)?)
    }

    /// Fires up an interactive command prompt which is capable of executing code one line at
    /// a time.
    // Also known as "REPL", from Lisp:
    // - Read a line of input
    // - Evaluate it
    // - Print the result
    // - Loop and do it all over again
    pub fn interactive() -> Result<(), MMalisError> {
        let mut malis = MMalis::default();
        // Get new handles to the stdin and stdout streams
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        // Create a new buffer to store the input
        let mut buffer = String::new();

        loop {
            // Write the new line identifier
            let _ = stdout.write(b"> ")?;
            // Flush it to make sure we print it
            stdout.flush()?;
            // Read the next line
            let bread = stdin.read_line(&mut buffer)?;

            // If no bytes were read, it means we reached `End-of-File` or `Ctrl-D` was pressed.
            if bread == 0 {
                break;
            }

            match buffer.as_str().trim() {
                "q" | "quit" | "exit" => break,
                _ => {}
            }

            // If a line is invalid, we report the error and go to the next iteration. We also
            // specify the `is_repl` true such that we could evaluate both expressions and
            // statements
            if let Err(err) = malis.run(buffer.as_bytes(), true) {
                println!("Interpreter: {err:?}");
                stdout.flush()?;
            }

            // Make sure to clean the buffer for the next iteration
            buffer.clear();
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum MMalisError {
    StdIO(std::io::Error),
    InterpretError(InterpretError),
}

impl From<std::io::Error> for MMalisError {
    fn from(value: std::io::Error) -> Self {
        Self::StdIO(value)
    }
}

impl From<InterpretError> for MMalisError {
    fn from(value: InterpretError) -> Self {
        Self::InterpretError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_dis() {
        let mut seq = Sequence::new();
        // Create a new constant
        let constant = Value::from(1.2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push return
        // seq.push(OpCode::Return, 13).unwrap();
        // Create a new constant
        let constant = Value::from(13.9);
        // Push the operand for the instruction
        seq.write_constant(constant, 13).unwrap();
        // Push return
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
        /*
        // Check if constant long works
        for i in 0..300u16 {
            // Create a new constant
            let constant = Value::from(f32::from(i));
            // Push the operand for the instruction
            seq.write_constant(constant, 13);
        }*/
        let mut seq2 = Sequence::new();
        // Create a new constant
        let constant = Value::from(1.2);
        // Push the operand for the instruction
        seq2.write_constant(constant, 99).unwrap();
        // Push return
        seq2.push(OpCode::Return, 1119).unwrap();
        // Create a new constant
        let constant = Value::from(1119.9);
        // Push the operand for the instruction
        seq2.write_constant(constant, 1119).unwrap();
        // Push return
        seq2.push(OpCode::Return, 99).unwrap();
        seq2.push(OpCode::Return, 99).unwrap();
        seq2.push(OpCode::Return, 99).unwrap();
        seq2.push(OpCode::Return, 14).unwrap();
        seq2.push(OpCode::Return, 14).unwrap();
        Disassembler::dis_sequence(&seq, "test sequence");
        Disassembler::dis_sequence(&seq2, "test sequence2");
    }

    #[test]
    fn negate() {
        let mut seq = Sequence::new();
        let constant = Value::from(1.2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push negation
        seq.push(OpCode::Negate, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 18).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn add() {
        let mut seq = Sequence::new();
        let constant = Value::from(90.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(10.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push addition
        seq.push(OpCode::Add, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 18).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn add_neg_int() {
        let mut seq = Sequence::new();
        let constant = Value::from(90);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(10);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push addition
        seq.push(OpCode::Add, 17).unwrap();
        // Push another contant
        let constant = Value::from(100);
        // Push the operand for the instruction
        seq.write_constant(constant, 19).unwrap();
        // Push negation
        seq.push(OpCode::Negate, 19).unwrap();
        // Push addition
        seq.push(OpCode::Add, 20).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn sub() {
        let mut seq = Sequence::new();
        let constant = Value::from(110.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(23.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push subtraction
        seq.push(OpCode::Sub, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 18).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn sub_neg_int() {
        let mut seq = Sequence::new();
        let constant = Value::from(30);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(250);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push negation
        seq.push(OpCode::Negate, 17).unwrap();
        // Push subtraction
        seq.push(OpCode::Sub, 20).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn mul() {
        let mut seq = Sequence::new();
        let constant = Value::from(54.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(8.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push subtraction
        seq.push(OpCode::Add, 17).unwrap();
        let constant = Value::from(360.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        seq.push(OpCode::Mul, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 18).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn div() {
        let mut seq = Sequence::new();
        let constant = Value::from(30);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(60);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push negation
        seq.push(OpCode::Div, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn precedence_check1() {
        // Testing: 1 + 2 * 3 (should be 7)
        let mut seq = Sequence::new();
        let constant = Value::from(1);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(3);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push mul
        seq.push(OpCode::Mul, 17).unwrap();
        // Push Add
        seq.push(OpCode::Add, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn precedence_check2() {
        // Testing: 1 + 2 * 3 - 4 / -5 (should be 7.8)
        let mut seq = Sequence::new();
        let constant = Value::from(1);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(3);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push mul
        seq.push(OpCode::Mul, 17).unwrap();
        // Push Add
        seq.push(OpCode::Add, 17).unwrap();
        let constant = -Value::from(4);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = -Value::from(5);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push division
        seq.push(OpCode::Div, 17).unwrap();
        // Push addition
        seq.push(OpCode::Add, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn no_op_negate() {
        // Testing: 4 - 3 * -2 withouth the negate operator
        let mut seq = Sequence::new();
        let constant = Value::from(4);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(3);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push subtract, give -3
        seq.push(OpCode::Sub, 17).unwrap();
        let constant = Value::from(0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push subtract, give -2
        seq.push(OpCode::Sub, 17).unwrap();
        // Push mul for -3 * -2
        seq.push(OpCode::Mul, 17).unwrap();
        // Push add for 4 + (-3 * -2)
        seq.push(OpCode::Add, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn no_op_subtract() {
        // Testing: 4 - 3 * -2 withouth the subtraction operator
        let mut seq = Sequence::new();
        let constant = Value::from(4);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(3);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push negation, give -3
        seq.push(OpCode::Negate, 17).unwrap();
        let constant = Value::from(2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push negation, give -2
        seq.push(OpCode::Negate, 17).unwrap();
        // Push mul for -3 * -2
        seq.push(OpCode::Mul, 17).unwrap();
        // Push add for 4 + (-3 * -2)
        seq.push(OpCode::Add, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 21).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }
}
