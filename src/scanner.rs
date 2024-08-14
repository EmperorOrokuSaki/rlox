use crate::tokens::{Token, TokenType};

pub struct Scanner {
    pub start: u64,
    pub current: u64,
    pub line: u64,
    pub source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 0,
            source,
            tokens: vec![],
        }
    }

    fn is_at_end(&self) -> bool {
        self.source.len() <= self.current as usize
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current as usize)
    }

    fn scan_token(&mut self) {
        let character = if let Some(character_unwrapped) = self.advance() {
            character_unwrapped
        } else {
            return;
        };
        let token_type = match character {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            _ => panic!("Unrecognizable character: {}", character),
        };
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token: TokenType, literal: Option<String>) {
        let start_byte = self
            .source
            .char_indices()
            .nth(self.start as usize)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap_or(self.source.len());

        let end_byte = self
            .source
            .char_indices()
            .nth(self.current as usize)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap_or(self.source.len());

        let lexeme = self.source[start_byte..end_byte].to_string();
        self.tokens.push(Token {
            token_type: token,
            lexeme,
            literal,
            line: self.line,
        });
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // we are the beginning of the lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::default(),
            literal: None,
            line: self.line,
        });
    }
}
