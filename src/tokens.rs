pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u64,
}

impl Token {
    /// Returns type + lexeme + literal
    pub fn as_string(self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // End of file.
    Eof,
}
