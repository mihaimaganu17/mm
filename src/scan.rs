use crate::token::{Token, TokenType, SingleChar, Comparison};

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
}

#[derive(Debug)]
pub enum ScanError {}
