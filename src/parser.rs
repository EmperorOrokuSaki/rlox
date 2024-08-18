use crate::{
    ast::expr::Expr,
    errors::{rlox_error, RuntimeError},
    tokens::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: u64,
}
impl Parser {
    /// Generates a new Parser with the given token stream.
    ///
    /// Uses the Recursive Descent Parsing (top to bottom) algorithm.
    ///
    /// # Example
    /// ```
    /// let tokens = vec![Token { token_type: TokenType::Number, .. }];
    /// let mut parser = Parser::new(tokens);
    /// ```
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            tokens: token_stream,
            current: 0,
        }
    }

    /// Returns true if the current token is EOF
    ///
    /// # Example
    /// ```
    /// let eof_token = Token { token_type: TokenType::Eof, .. };
    /// let mut parser = Parser::new(vec![eof_token]);
    /// assert!(parser.is_at_end());
    /// ```
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Returns the current token
    ///
    /// # Example
    /// ```
    /// let token = Token { token_type: TokenType::Number, .. };
    /// let mut parser = Parser::new(vec![token.clone()]);
    /// assert_eq!(parser.peek(), token);
    /// ```
    fn peek(&self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    /// Returns true if the current token has the `token_type` type.
    /// Does not move the `current` field's value.
    ///
    /// # Example
    /// ```
    /// let token = Token { token_type: TokenType::Number, .. };
    /// let mut parser = Parser::new(vec![token.clone()]);
    /// assert!(parser.check(TokenType::Number));
    /// ```
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.tokens[self.current as usize].token_type == token_type
    }

    /// Returns the previous token if in range
    ///
    /// # Example
    /// ```
    /// let tokens = vec![
    ///     Token { token_type: TokenType::Number, .. },
    ///     Token { token_type: TokenType::Plus, .. },
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// parser.advance();
    /// assert_eq!(parser.previous().unwrap().token_type, TokenType::Number);
    /// ```
    fn previous(&self) -> Option<Token> {
        if self.current == 0 {
            return None;
        }
        let token = &self.tokens[self.current as usize - 1];
        Some(token.clone())
    }

    /// Advances the `current` field's value by one.
    ///
    /// # Example
    /// ```
    /// let tokens = vec![Token { token_type: TokenType::Number, .. }];
    /// let mut parser = Parser::new(tokens);
    /// parser.advance();
    /// assert!(parser.is_at_end());
    /// ```
    fn advance(&mut self) {
        if self.is_at_end() {
            return;
        }
        self.current += 1;
    }

    /// Takes a vector of `TokenType` values and advances once one of them is encountered.
    ///
    /// # Example
    /// ```
    /// let tokens = vec![
    ///     Token { token_type: TokenType::Number, .. },
    ///     Token { token_type: TokenType::Plus, .. },
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// assert!(parser.match_token(vec![TokenType::Number]));
    /// assert_eq!(parser.peek().token_type, TokenType::Plus);
    /// ```
    fn match_token(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Handles the expression rule.
    ///
    /// # Example
    /// ```
    ///                                 1 == 1 != 0
    /// resolve via equality            -----
    ///                                 true != 0
    /// next token is !=, advance             --
    ///                                 true != 0
    /// resolve via equality                       -----
    ///                                 true != false
    /// return the resolved state       ----------------
    /// ```
    fn expression(&mut self) -> Result<Expr, RuntimeError> {
        self.equality()
    }

    /// Handles the equality rule by passing the current value to the [`comparison()`] function, until `==` or `!=` is reached.
    ///
    /// # Example
    /// ```
    ///                                 1 == 2 != 1 == 2
    /// resolve via comparison          -----
    ///                                 false != 1 == 2
    /// next token is !=, advance              --
    ///                                 false != 1 == 2
    /// resolve via comparison                     -----
    ///                                 false != false
    /// return the resolved state       ----------------
    /// ```
    fn equality(&mut self) -> Result<Expr, RuntimeError> {
        self.resolve(
            |parser| parser.comparison(),
            vec![TokenType::EqualEqual, TokenType::BangEqual],
        )
    }

    /// Handles the comparison rule by passing the current value to the [`term()`] function, until `>`, `<`, `>=`, or `<=` is reached.
    ///
    /// # Example
    /// ```
    ///                                 1 + 2 >= 2 + 1
    /// resolve via term                -----
    ///                                 3     >= 2 + 1
    /// next token is >=, advance             --
    ///                                 3     >= 2 + 1
    /// resolve via term                         -----
    ///                                 3     >=     3
    /// return the resolved state       --------------
    /// ```
    fn comparison(&mut self) -> Result<Expr, RuntimeError> {
        self.resolve(
            |parser| parser.term(),
            vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    /// Handles the term rule by passing the current value to the [`factor()`] function, until `+` or `-` is reached.
    ///
    /// # Example
    /// ```
    ///                                 1 + 2 - 3
    /// resolve via factor              -----
    ///                                 3     - 3
    /// next token is -, advance              --
    ///                                 3     - 3
    /// resolve via factor                     -----
    ///                                 3     -     3
    /// return the resolved state       --------------
    /// ```
    fn term(&mut self) -> Result<Expr, RuntimeError> {
        self.resolve(
            |parser| parser.factor(),
            vec![TokenType::Minus, TokenType::Plus],
        )
    }

    /// Handles the factor rule by passing the current value to the [`unary()`] function, until `*` or `/` is reached.
    ///
    /// # Example
    /// ```
    ///                                 6 / 2 * 3
    /// resolve via unary               -----
    ///                                 3     * 3
    /// next token is *, advance              --
    ///                                 3     * 3
    /// resolve via unary                      -----
    ///                                 3     *  3
    /// return the resolved state       --------------
    /// ```
    fn factor(&mut self) -> Result<Expr, RuntimeError> {
        self.resolve(
            |parser| parser.unary(),
            vec![TokenType::Slash, TokenType::Star],
        )
    }

    /// Handles the unary rule by checking for `!` or `-` and recursively calling itself for further unary operators or primary expressions.
    ///
    /// # Example
    /// ```
    ///                                 -1 + 2
    /// resolve via unary               --
    ///                                 -1     + 2
    /// resolve via term                      -----
    ///                                 -1     + 2
    /// return the resolved state       --------------
    /// ```
    fn unary(&mut self) -> Result<Expr, RuntimeError> {
        if self.match_token(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    /// Handles the primary rule, which is the most basic unit of an expression (e.g., literals, grouping, or variable access).
    ///
    /// # Example
    /// ```
    ///                                 (1 + 2) * 3
    /// resolve via grouping            -----
    ///                                 ( 3 ) * 3
    /// resolve via factor                    -----
    ///                                 3     * 3
    /// return the resolved state       --------------
    /// ```
    fn primary(&mut self) -> Result<Expr, RuntimeError> {
        if self.match_token(&vec![TokenType::False]) {
            return Ok(Expr::Literal {
                value: "false".to_string(),
            });
        } else if self.match_token(&vec![TokenType::True]) {
            return Ok(Expr::Literal {
                value: "true".to_string(),
            });
        } else if self.match_token(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: "nil".to_string(),
            });
        } else if self.match_token(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: self.previous().unwrap().literal.unwrap(),
            });
        } else if self.match_token(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        Err(self.parser_error("Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, RuntimeError> {
        if self.check(&token_type) {
            self.advance();
            return Ok(self.peek());
        }
        Err(self.parser_error(message))
    }

    fn parser_error(&self, message: &str) -> RuntimeError {
        rlox_error(self.peek().line, message);
        RuntimeError::ParseError
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if let Some(prev_token) = self.previous() {
                if prev_token.token_type == TokenType::Semicolon {
                    return;
                }

                match prev_token.token_type {
                    TokenType::Class => {}
                    TokenType::Fun => {}
                    TokenType::For => {}
                    TokenType::Var => {}
                    TokenType::If => {}
                    TokenType::While => {}
                    TokenType::Print => {}
                    TokenType::Return => {}
                    _ => {}
                }

                self.advance();
            }
        }
    }

    pub fn parse(&mut self) -> Result<Expr, RuntimeError> {
        self.expression()
    }

    /// Resolves binary expressions by taking an operator and a resolver function.
    ///
    /// # Example
    /// ```
    /// let expr = parser.resolve(
    ///     |parser| parser.term(),
    ///     vec![TokenType::Plus, TokenType::Minus]
    /// );
    /// ```
    fn resolve<R>(
        &mut self,
        mut resolver: R,
        operators: Vec<TokenType>,
    ) -> Result<Expr, RuntimeError>
    where
        R: FnMut(&mut Parser) -> Result<Expr, RuntimeError>,
    {
        let mut expr = resolver(self)?;

        while self.match_token(&operators) {
            let operator = self.previous().unwrap();
            let right = resolver(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
}
