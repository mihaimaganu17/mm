#[derive(Debug)]
pub struct Scanner<'a> {
    data: &'a [u8],
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
            offset: 0,
            line: 1,
        }
    }
}

#[derive(Debug)]
pub enum ScanError {}
