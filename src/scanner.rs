use std::iter::Peekable;
use std::str::Chars;

use crate::errors::rlox_error;
use crate::keywords::KEYWORDS;
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
    /// Generates a new Scanner.
    pub fn new(source: String) -> Self {
        let chars = Box::leak(source.clone().into_boxed_str())
            .chars()
            .peekable();
        Self {
            start: 0,
            current: 0,
            line: 1,
            chars,
            source: source,
            tokens: vec![],
        }
    }

    /// Returns `true` if there are no more characters
    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none() // Peek to check if we're at the end
    }

    /// Consumes the next character and advances the iterator
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next() // Use .next() to advance the iterator
    }

    /// Returns `true` if the next character is the same as `expectation`.
    /// Advances by one if the result is `true`;
    fn expected(&mut self, expectation: &str) -> bool {
        if let Some(&next_char) = self.chars.peek() {
            if next_char == expectation.chars().next().unwrap() {
                self.advance(); // Only advance if the expectation is met
                return true;
            }
        }
        false
    }

    /// Returns the character that is one index ahead
    /// Use .peek() to look at the next character without advancing
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    // Returns the character that is two indexes ahead
    /// Use .peek() to look at the second next character without advancing
    fn peek_next(&mut self) -> Option<char> {
        let mut cloned_chars = self.chars.clone();
        cloned_chars.next();
        cloned_chars.next()
    }

    fn identify_string(&mut self) {
        // we continue advancing until the next character is the closing double quotation mark
        while self.peek() != Some('"') && !self.is_at_end() {
            // supporting multi-line strings.
            if self.peek() == Some(char::from('\n')) {
                self.line += 1;
            }
            self.advance();
        }

        // we did not reach a double quotation mark but the file ended.
        if self.is_at_end() {
            rlox_error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        self.add_token(
            TokenType::String,
            Some(self.source[self.start as usize + 1..self.current as usize - 1].to_string()),
        );
    }

    fn identify_number(&mut self) {
        // we continue advancing until the next character is not a digit anymore
        loop {
            if self.is_at_end() || !self.peek().unwrap().is_digit(10) {
                break;
            } else {
                self.advance();
            }
        }

        // we did not reach a double quotation mark but the file ended.
        if self.peek() == Some('.') {
            if let Some(peek_next_character) = self.peek_next() {
                if peek_next_character.is_digit(10) {
                    self.advance();
                    while !self.is_at_end() {
                        if self.peek().unwrap().is_digit(10) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        self.add_token(
            TokenType::Number,
            Some(self.source[self.start as usize..self.current as usize].to_string()),
        );
    }

    fn scan_identifier(&mut self) {
        // Loop until we find a non-alphanumeric or non-underscore character.
        while self
            .peek()
            .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
        }

        // Now we collect the identifier text.
        let text = &self.source[self.start as usize..self.current as usize];

        // Check if the identifier is a keyword.
        let token_type = KEYWORDS.get(text).copied().unwrap_or(TokenType::Identifier);

        // Add the token.
        self.add_token(token_type, None);
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
            '"' => {
                self.identify_string();
                return;
            }
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
                if self.expected("*") {
                    let mut counter = 1;
                    while let Some(c) = self.peek() {
                        if self.is_at_end() {
                            break;
                        }

                        self.advance();

                        if c == '/' && self.expected("*") {
                            counter += 1;
                        } else if c == '*' && self.expected("/") {
                            counter -= 1;
                            if counter == 0 {
                                break;
                            }
                        }
                    }
                    return;
                } else if self.expected("/") {
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
                if character.is_digit(10) {
                    // it is a base10 digit!
                    self.identify_number();
                } else if character.is_ascii_alphabetic() || character == '_' {
                    self.scan_identifier();
                } else {
                    rlox_error(self.line, &format!("Unexpected character {}", character));
                }
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
