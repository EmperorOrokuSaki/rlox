#[derive(Debug, Clone)]
pub enum Object {
    Nil,            // Represents nil
    Boolean(bool),  // Represents a boolean
    Number(f64),    // Represents a number (using f64 as an example)
    String(String), // Represents a string
}

impl Object {
    pub fn print(self) {
        match self {
            Object::Nil => println!(""),
            Object::Boolean(boolean) => println!("{}", boolean),
            Object::Number(number) => println!("{}", number.round()),
            Object::String(string) => println!("{}", string),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: u64,
}

impl Token {
    /// Returns type + lexeme + literal as a formatted string
    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        format!(
            "{:?} {} {:?}",
            self.token_type,
            self.lexeme,
            self.literal
                .clone()
                .unwrap_or(Object::String("".to_string()))
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
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
