use std::str::Chars;

use crate::{
    errors::rlox_error,
    tokens::{Token, TokenType},
};

pub struct Scanner {
    pub start: u64,
    pub current: u64,
    pub line: u64,
    pub source: String,
    pub chars: Chars<'static>,
    pub tokens: Vec<Token>,
    pub count: u64,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = Box::leak(source.clone().into_boxed_str()).chars();
        Self {
            start: 0,
            current: 0,
            line: 1,
            chars: chars.clone(),
            source,
            count: chars.count() as u64,
            tokens: vec![],
        }
    }

    fn is_at_end(&self) -> bool {
        self.count <= self.current
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.chars.nth(self.current as usize);
        self.current += 1;
        character
    }

    fn expected<'a>(&mut self, expectation: &'a str) -> bool {
        // Get the character from the source
        if let Some(source_char) = self.chars.nth(self.current as usize) {
            // Compare it with the first character of the expectation string
            if let Some(expected_char) = expectation.chars().next() {
                if source_char == expected_char {
                    self.current += 1;
                    return true;
                }
            }
        }

        false
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
            '!' => {
                if self.expected("=") {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            },
            '=' => {
                if self.expected("=") {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            },
            '<' => {
                if self.expected("=") {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            },
            '>' => {
                if self.expected("=") {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            },
            _ => {
                rlox_error(self.line, &format!("Unexpected character {}", character));
                return;
            }
        };
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token: TokenType, literal: Option<String>) {
        let lexeme = self.source[self.start as usize..self.current as usize].to_string();
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
