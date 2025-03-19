use crate::scan::Scanner;

pub struct Compiler;

impl Compiler {
    pub fn compile(&self, bytes: &[u8]) -> Result<(), CompileError> {
        let _scanner = Scanner::new(bytes);
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompileError {}
