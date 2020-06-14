use std::fmt;

#[derive(Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Colon,
    SemiColon,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Arrow,
    Identifier,
    Int,
    Float,
    String,
    If,
    Else,
    For,
    Let,
    Var,
    Fun,
    Return,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub data: String,
    pub row: usize,
    pub col: usize,
}
