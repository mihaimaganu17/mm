use crate::compiler::{CompileError, Compiler};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, bytes: &[u8]) -> Result<(), InterpretError> {
        let compiler = Compiler;
        Ok(compiler.compile(bytes)?)
    }
}

#[derive(Debug)]
pub enum InterpretError {
    // Reports a static error when compiling the source code
    CompileError(CompileError),
    // Reports a dynamic error when running the bytecode
    RuntimeError,
    // Stack trying to access and element but it's empty
    StackEmpty,
}

impl From<CompileError> for InterpretError {
    fn from(value: CompileError) -> Self {
        Self::CompileError(value)
    }
}
