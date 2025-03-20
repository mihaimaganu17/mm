use crate::scan::{ScanError, Scanner};

pub struct Compiler;

impl Compiler {
    pub fn compile(&self, bytes: &[u8]) -> Result<(), CompileError> {
        // Currently no line is provided. We want to update this for each token we get
        let mut line = None;
        let mut scanner = Scanner::new(bytes);
        // Keeps scanning and compiling tokens until the end of `bytes`. Usually the last token
        // would be `Eof` and after that the `iterator` would return `None`
        while let Some(maybe_token) = scanner.next_token() {
            // Get the next token
            let token = maybe_token?;
            // If we have a line assigned from the previous token
            if let Some(line) = &mut line {
                // If it differs from the current token's line
                if *line != token.line() {
                    // Print the new line
                    print!("{:04} ", token.line());
                    // Replace it with the line of the current token
                    *line = token.line();
                } else {
                    // If it does not differ, print a bar to show continuation of the previous line
                    print!("   | ");
                }
            } else {
                // Print the new line
                print!("{:04} ", token.line());
                // Replace it with the line of the current token
                line = Some(token.line())
            }
            // Get the representation of the token from the bytes
            let token_str = std::str::from_utf8(
                bytes
                    .get(token.start()..token.end())
                    .ok_or(CompileError::ScanOutOfBounds(token.start(), token.end()))?,
            )?;
            println!("{:?} {}", token.t_type(), token_str);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompileError {
    ScanError(ScanError),
    Utf8Error(core::str::Utf8Error),
    ScanOutOfBounds(usize, usize),
}

crate::impl_from_err!(ScanError, CompileError, ScanError);
crate::impl_from_err!(core::str::Utf8Error, CompileError, Utf8Error);
