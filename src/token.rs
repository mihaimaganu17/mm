#[derive(Debug, Default)]
pub struct Token {
    // Token type, `type` is reserved
    t_type: TokenType,
    // Where the token starts as offset into the source bytes
    start: usize,
    // The size of the token
    len: usize,
    // Line on which the token occurs
    line: usize,
}

impl Token {
    // Used to debug the source code
    pub fn new(t_type: TokenType, start: usize, len: usize, line: usize) -> Self {
        Self {
            t_type,
            start,
            len,
            line,
        }
    }
}

impl Default for TokenType {
    fn default() -> Self {
        Self::Eof
    }
}

#[derive(Debug)]
pub enum TokenType {
    SingleChar(SingleChar),
    Comparison(Comparison),
    //Literal(Literal),
    Keyword(Keyword),
    // Only available for debuggin purposes
    DebugByte(u8),
    Ident,
    Ignored,
    Eof,
}

#[derive(Debug)]
pub enum SingleChar {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Colon,
    Slash,
    Star,
    Bang,
    Question,
    Equal,
}

#[derive(Debug)]
pub enum Comparison {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub enum Literal {
    // Because `String` is reserved in Rust
    LitString(String),
    Number([u8; 4]),
}

#[derive(Debug)]
pub enum Keyword {
    And,
    Or,
    Not,
    Class,
    Fun,
    If,
    Else,
    While,
    For,
    True,
    False,
    Nil,
    Var,
    Print,
    Return,
    ClassSelf,
    Super,
}
