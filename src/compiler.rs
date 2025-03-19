use crate::scan::{ScanError, Scanner};

pub struct Compiler;

impl Compiler {
    pub fn compile(&self, bytes: &[u8]) -> Result<(), CompileError> {
        // Currently no line is provided. We want to update this for each token we get
        let mut line = None;
        let mut scanner = Scanner::new(bytes);
        // Keeps scanning and compiling tokens until the end of `bytes`
        while let Some(maybe_token) = scanner.next_token() {
            let token = maybe_token?;
            if let Some(line) = &mut line {
                if *line != token.line() {
                    *line = token.line();
                }
            } else {
                line = Some(token.line())
            }
            println!("{token:?}");
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompileError {
    ScanError(ScanError),
}

impl From<ScanError> for CompileError {
    fn from(value: ScanError) -> CompileError {
        Self::ScanError(value)
    }
}
