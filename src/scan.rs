use crate::token::{Comparison, Literal, SingleChar, Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    data: &'a [u8],
    // Offset that marks the beginning of the current lexeme being scanned
    start: usize,
    // Current offset in the `data` field
    offset: usize,
    // The line the cursor is on
    line: usize,
}

impl<'a> Scanner<'a> {
    // Creates a new scanner from the given bytes
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            start: 0,
            offset: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Result<Token, ScanError>> {
        if self.start >= self.data.len() {
            None
        } else {
            // Skip whitespaces and comments first
            self.skip_non_tokens()?;
            // Start from where we left off at the previous token
            self.start = self.offset;

            let token_type = match self.next_byte()? {
                b'(' => TokenType::SingleChar(SingleChar::LeftParen),
                b')' => TokenType::SingleChar(SingleChar::RightParen),
                b'{' => TokenType::SingleChar(SingleChar::LeftBrace),
                b'}' => TokenType::SingleChar(SingleChar::RightBrace),
                b',' => TokenType::SingleChar(SingleChar::Comma),
                b'.' => TokenType::SingleChar(SingleChar::Dot),
                b'-' => TokenType::SingleChar(SingleChar::Minus),
                b'+' => TokenType::SingleChar(SingleChar::Plus),
                b';' => TokenType::SingleChar(SingleChar::SemiColon),
                b':' => TokenType::SingleChar(SingleChar::Colon),
                b'/' => TokenType::SingleChar(SingleChar::Slash),
                b'*' => TokenType::SingleChar(SingleChar::Star),
                b'?' => TokenType::SingleChar(SingleChar::Question),
                b'!' => {
                    if let Some(b'=') = self.next_byte() {
                        TokenType::Comparison(Comparison::BangEqual)
                    } else {
                        TokenType::SingleChar(SingleChar::Bang)
                    }
                }
                b'=' => {
                    if let Some(b'=') = self.next_byte() {
                        TokenType::Comparison(Comparison::EqualEqual)
                    } else {
                        TokenType::SingleChar(SingleChar::Equal)
                    }
                }
                b'<' => {
                    if let Some(b'=') = self.next_byte() {
                        TokenType::Comparison(Comparison::LessEqual)
                    } else {
                        TokenType::SingleChar(SingleChar::Less)
                    }
                }
                b'>' => {
                    if let Some(b'=') = self.next_byte() {
                        TokenType::Comparison(Comparison::GreaterEqual)
                    } else {
                        TokenType::SingleChar(SingleChar::Greater)
                    }
                }
                b'"' => {
                    // A single double quote specifies a string
                    match self.string() {
                        Ok(token) => token,
                        Err(e) => return Some(Err(e)),
                    }
                }
                _ => TokenType::Eof,
            };
            // Create a new debug token
            let token = Token::new(token_type, self.start, 1, self.line);
            Some(Ok(token))
        }
    }

    fn next_byte(&mut self) -> Option<&u8> {
        let b = self.data.get(self.offset);
        self.offset = self.offset.saturating_add(1);
        b
    }

    fn peek_next(&mut self) -> Option<&u8> {
        let b = self.data.get(self.offset);
        b
    }

    fn string(&mut self) -> Result<TokenType, ScanError> {
        // Peek the next byte
        while let Some(byte) = self.peek_next() {
            // Until we find the closing double quote
            if *byte == b'\"' {
                break;
            }
            // If we encounter a newline
            if *byte == b'\n' {
                // We tell the scanner we are at the next line
                self.line += 1;
            }
            self.next_byte().ok_or(ScanError::CannotConsumeByte)?;
        }
        // If we are at the end (meaning we did not yet consume the ending double quote) we return
        // an error
        if self.is_at_end() {
            return Err(ScanError::UnterminatedString(self.start, self.offset));
        }
        // Consume the closing quote
        self.next_byte().ok_or(ScanError::CannotConsumeByte)?;
        // Return a new string token
        Ok(TokenType::Literal(Literal::LitString))
    }

    fn _bytes_left(&self) -> usize {
        self.data.len() - self.offset
    }

    fn is_at_end(&self) -> bool {
        self.data.len() == self.offset
    }

    fn skip_non_tokens(&mut self) -> Option<()> {
        while match self.peek_next()? {
            b' ' | b'\r' | b'\t' => true,
            b'\n' => {
                self.line = self.line.saturating_add(1);
                true
            }
            b'/' => {
                if let Some(b'/') = self.peek_next() {
                    // A comment goes until end of line
                    while self.peek_next()? != &b'\n' {
                        self.next_byte()?;
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        } {
            self.next_byte()?;
        }
        Some(())
    }
}

#[derive(Debug)]
pub enum ScanError {
    UnterminatedString(usize, usize),
    CannotConsumeByte,
}
