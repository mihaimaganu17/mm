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

    pub fn next_token(&mut self) -> Option<Result<u8, ScanError>> {
        if self.offset>= self.data.len() {
            None
        } else {
            let curr = self.data[self.offset];
            self.offset = self.offset.checked_add(1)?;
            Some(Ok(curr))
        }
    }
}

#[derive(Debug)]
pub enum ScanError {}
