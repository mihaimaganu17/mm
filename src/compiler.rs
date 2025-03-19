pub struct Compiler;

impl Compiler {
    pub fn compile(&self, _bytes: &[u8]) -> Result<(), CompileError> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompileError {}
