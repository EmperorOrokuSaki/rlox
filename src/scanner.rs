use std::str::Chars;
use std::iter::Peekable;

use crate::errors::rlox_error;
use crate::tokens::{Token, TokenType};

pub struct Scanner {
    pub start: u64,
    pub current: u64,
    pub line: u64,
    pub source: String,
    pub chars: Peekable<Chars<'static>>, // Use Peekable iterator
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = Box::leak(source.clone().into_boxed_str()).chars().peekable();
        Self {
            start: 0,
            current: 0,
            line: 1,
            chars,
            source: source,
            tokens: vec![],
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none() // Peek to check if we're at the end
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next() // Use .next() to advance the iterator
    }

    fn expected(&mut self, expectation: &str) -> bool {
        if let Some(&next_char) = self.chars.peek() {
            if next_char == expectation.chars().next().unwrap() {
                self.advance(); // Only advance if the expectation is met
                return true;
            }
        }
        false
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied() // Use .peek() to look at the next character without advancing
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
            }
            '=' => {
                if self.expected("=") {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.expected("=") {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.expected("=") {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.expected("/") {
                    while let Some(c) = self.peek() {
                        if c == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                    return;
                } else {
                    TokenType::Slash
                }
            }
            '\r' | '\t' | ' ' => return, // Ignore whitespace characters
            '\n' => {
                self.line += 1;
                return;
            }
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
            // Start of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        // Push EOF token
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::default(),
            literal: None,
            line: self.line,
        });
    }
}
