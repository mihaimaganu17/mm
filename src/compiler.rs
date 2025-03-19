use crate::scan::Scanner;

pub struct Compiler;

impl Compiler {
    pub fn compile(&self, bytes: &[u8]) -> Result<(), CompileError> {
        // Currently no line is provided. We want to update this for each token we get
        // let mut line = None;
        let mut scanner = Scanner::new(bytes);
        // Keeps scanning and compiling tokens until the end of `bytes`
        while let Some(maybe_token) = scanner.next_token() {
            println!("{:?}", maybe_token);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompileError {}
