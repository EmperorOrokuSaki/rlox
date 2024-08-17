use crate::{
    ast::expr::Expr,
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
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            tokens: token_stream,
            current: 0,
        }
    }

    /// Returns true if the current type is EOF
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Returns the current token
    fn peek(&self) -> Token {
        self.tokens[self.current as usize]
    }

    /// Returns true if the current token has the `token_type` type.
    /// Does not move the `current` field's value.
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.tokens[self.current as usize].token_type == token_type
    }

    /// Returns the previous token if in range
    fn previous(&self) -> Option<Token> {
        if self.current == 0 {
            return None;
        }
        Some(self.tokens[self.current as usize - 1])
    }

    /// Advances the `current` field's value by one
    fn advance(&mut self) {
        if self.is_at_end() {
            return;
        }
        self.current += 1;
    }

    /// Takes a vector of TokenType values and advances once one of them are encountered
    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                // the token type matches the value
                self.advance();
                return true;
            }
        }
        false
    }

    /// Handles the expression rule
    fn expression(&mut self) -> Expr {
        // our lowest priority rule is equality (== OR !=)
        self.equality()
    }

    /// Handles the equality rule by passing the current value to the [`equality()`] function, until eq-eq or bang-eq is reached. Then passes everything on the other side to the [`equality()`] function again.
    ///
    /// Example:
    /// ```
    ///                                 1 < 2 == 2 > 1
    /// resolve via comparison          -----
    ///                                 true  == 2 > 1
    /// next token is ==, advance             --
    ///                                 true  == 2 > 1
    /// resolve via comparison                   -----
    ///                                 true  ==  true
    /// return the resolved state       --------------
    /// ```
    fn equality(&mut self) -> Expr {
        self.resolve(
            |parser| parser.comparison(),
            vec![TokenType::EqualEqual, TokenType::BangEqual],
        )
    }

    /// Handles the comparison rule by passing the current value to the [`equality()`] function, until eq-eq or bang-eq is reached. Then passes everything on the other side to the [`equality()`] function again.
    ///
    /// Example:
    /// ```
    ///                                 1 < 2 == 2 > 1
    /// resolve via comparison          -----
    ///                                 true  == 2 > 1
    /// next token is ==, advance             --
    ///                                 true  == 2 > 1
    /// resolve via comparison                   -----
    ///                                 true  ==  true
    /// return the resolved state       --------------
    /// ```
    fn comparison(&mut self) -> Expr {
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

    /// Handles the comparison rule by passing the current value to the [`equality()`] function, until eq-eq or bang-eq is reached. Then passes everything on the other side to the [`equality()`] function again.
    ///
    /// Example:
    /// ```
    ///                                 1 < 2 == 2 > 1
    /// resolve via comparison          -----
    ///                                 true  == 2 > 1
    /// next token is ==, advance             --
    ///                                 true  == 2 > 1
    /// resolve via comparison                   -----
    ///                                 true  ==  true
    /// return the resolved state       --------------
    /// ```
    fn term(&mut self) -> Expr {
        self.resolve(
            |parser| parser.factor(),
            vec![TokenType::Minus, TokenType::Plus],
        )
    }

    /// Handles the comparison rule by passing the current value to the [`equality()`] function, until eq-eq or bang-eq is reached. Then passes everything on the other side to the [`equality()`] function again.
    ///
    /// Example:
    /// ```
    ///                                 1 < 2 == 2 > 1
    /// resolve via comparison          -----
    ///                                 true  == 2 > 1
    /// next token is ==, advance             --
    ///                                 true  == 2 > 1
    /// resolve via comparison                   -----
    ///                                 true  ==  true
    /// return the resolved state       --------------
    /// ```
    fn factor(&mut self) -> Expr {
        self.resolve(
            |parser| parser.unary(),
            vec![TokenType::Slash, TokenType::Star],
        )
    }

    /// Handles the comparison rule by passing the current value to the [`equality()`] function, until eq-eq or bang-eq is reached. Then passes everything on the other side to the [`equality()`] function again.
    ///
    /// Example:
    /// ```
    ///                                 1 < 2 == 2 > 1
    /// resolve via comparison          -----
    ///                                 true  == 2 > 1
    /// next token is ==, advance             --
    ///                                 true  == 2 > 1
    /// resolve via comparison                   -----
    ///                                 true  ==  true
    /// return the resolved state       --------------
    /// ```
    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    /// Handles the comparison rule by passing the current value to the [`equality()`] function, until eq-eq or bang-eq is reached. Then passes everything on the other side to the [`equality()`] function again.
    ///
    /// Example:
    /// ```
    ///                                 1 < 2 == 2 > 1
    /// resolve via comparison          -----
    ///                                 true  == 2 > 1
    /// next token is ==, advance             --
    ///                                 true  == 2 > 1
    /// resolve via comparison                   -----
    ///                                 true  ==  true
    /// return the resolved state       --------------
    /// ```
    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::False]) {
            return Expr::Literal {
                value: "false".to_string(),
            };
        } else if self.match_token(vec![TokenType::True]) {
            return Expr::Literal {
                value: "true".to_string(),
            };
        } else if self.match_token(vec![TokenType::Nil]) {
            return Expr::Literal {
                value: "nil".to_string(),
            };
        } else if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal {
                value: self.previous().unwrap().literal.unwrap(),
            };
        } else if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        panic!("Nothing to do!!");
    }

    fn resolve<R>(&mut self, mut resolver: R, operators: Vec<TokenType>) -> Expr
    where
        R: FnMut(&mut Parser) -> Expr,
    {
        // we resolve the left-hand side expression.
        let mut expr: Expr = resolver(self);

        while self.match_token(operators) {
            // because the line above advanced by one, the operator is now behind us by one.
            // we can safely unwrap because the match_token function definitely advanced. if it hadn't, the while loop wouldn't have run.
            let operator = self.previous().unwrap();
            // we resolve the right-hand side expression
            let right = resolver(self);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }
}
