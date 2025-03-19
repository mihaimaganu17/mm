pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, bytes: &[u8]) -> Result<(), InterpretError> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum InterpretError {
    // Reports a static error when compiling the source code
    CompileError,
    // Reports a dynamic error when running the bytecode
    RuntimeError,
    // Stack trying to access and element but it's empty
    StackEmpty,
}
