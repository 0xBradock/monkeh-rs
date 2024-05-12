#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub line: usize,
    pub literal: EToken,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EToken {
    // One token
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two tokens
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(String),

    // Keywords
    And,
    Or,
    If,
    Else,
    For,
    While,
    True,
    False,
    Nil,
    Print,
    Return,
    Super,
    This,
    Var,
}

pub struct Scanner {
    pub contents: String,
}

impl Scanner {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    pub fn scan(&self) -> Vec<Token> {
        vec![Token {
            literal: EToken::Dot,
            line: 0,
            token_type: String::from("type"),
        }]
    }
}
