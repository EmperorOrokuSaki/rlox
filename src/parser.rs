use crate::{
    ast::{expr::Expr, stmt::Stmt},
    errors::RLoxError,
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
    fn expression(&mut self) -> Result<Expr, RLoxError> {
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
    fn equality(&mut self) -> Result<Expr, RLoxError> {
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
    fn comparison(&mut self) -> Result<Expr, RLoxError> {
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
    fn term(&mut self) -> Result<Expr, RLoxError> {
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
    fn factor(&mut self) -> Result<Expr, RLoxError> {
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
    fn unary(&mut self) -> Result<Expr, RLoxError> {
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
    fn primary(&mut self) -> Result<Expr, RLoxError> {
        if self.match_token(&vec![TokenType::False]) {
            return Ok(Expr::Literal {
                value: crate::tokens::Object::Boolean(false),
            });
        } else if self.match_token(&vec![TokenType::True]) {
            return Ok(Expr::Literal {
                value: crate::tokens::Object::Boolean(true),
            });
        } else if self.match_token(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: crate::tokens::Object::Nil,
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
        } else if self.match_token(&vec![TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().unwrap(),
            });
        }
        Err(self.parser_error("Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, RLoxError> {
        if self.check(&token_type) {
            let token = self.peek();
            self.advance();
            return Ok(token);
        }
        Err(self.parser_error(message))
    }

    fn parser_error(&self, message: &str) -> RLoxError {
        RLoxError::ParseError(self.peek().line, message.to_string())
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

    fn print_statement(&mut self) -> Result<Stmt, RLoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ; after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, RLoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ; after value.")?;
        Ok(Stmt::Expression { expression: value })
    }

    fn statement(&mut self) -> Result<Stmt, RLoxError> {
        if self.match_token(&vec![TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, RLoxError> {
        let name: Token = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer = Expr::Literal {
            value: crate::tokens::Object::Nil,
        }; // Null by default
        if self.match_token(&vec![TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn declaration(&mut self) -> Result<Stmt, RLoxError> {
        let response: Result<Stmt, RLoxError> = if self.match_token(&vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if response.is_err() {
            self.synchronize();
        }

        response
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, RLoxError> {
        let mut statements = vec![];
        while !self.is_at_end() {
            let response = self.declaration();
            if let Err(err) = response {
                err.print();
                continue;
            }
            statements.push(response.unwrap());
        }
        Ok(statements)
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
    fn resolve<R>(&mut self, mut resolver: R, operators: Vec<TokenType>) -> Result<Expr, RLoxError>
    where
        R: FnMut(&mut Parser) -> Result<Expr, RLoxError>,
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
