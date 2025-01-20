use crate::{
    ast::{Expr, Literal, LiteralType, Binary},
    token::{Token, TokenType, Comparison},
    error::ParserError,
};

/// Parses the tokens according to the `malis.cfg` context-free grammar
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        // We first check for the first comparison of the production rule
        let mut expr = self.comparison()?;
        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule
        let bang_equal = TokenType::Comparison(Comparison::BangEqual);
        let equal_equal = TokenType::Comparison(Comparison::EqualEqual);

        // Then we have a compound of any number of `!=` or `==` followed by another comparison
        while self.any(&[&bang_equal, &equal_equal])? {
            // The operator if the `Token` that we matched above
            let operator = self.advance()?.clone();
            // After the operator, the expression is the next comparison
            let right_expr = self.comparison()?;
            // We create a new `Binary` expression using the two
            expr = Expr::Binary(Binary::new(expr, operator, right_expr));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let expr = Expr::Literal(Literal { l_type: LiteralType::True });
        Ok(expr)
    }

    // Given the list of `t_types` token types, we check if the current token matches any of the
    // desired ones.
    fn any(&mut self, t_types: &[&TokenType]) -> Result<bool, ParserError> {
        for t_type in t_types {
            if self.check(t_type)? { return Ok(true); }
        }
        Ok(false)
    }

    // Returns whether there are more tokens to be parsed
    fn tokens_left(&self) -> Result<bool, ParserError> {
        let token = self.peek()?;

        Ok(token.t_type.get().ok_or(ParserError::NoTokenType)? != &TokenType::EOF)
    }

    // Returns the token at the `current` index
    fn peek(&self) -> Result<&Token, ParserError> {
        self.tokens.get(self.current).ok_or(ParserError::InvalidIdx(self.current))
    }

    // Returns the token that preceded `current` indexed token
    fn previous(&self) -> Result<&Token, ParserError> {
        if self.current != 0 {
            self.tokens.get(self.current).ok_or(ParserError::InvalidIdx(self.current))
        } else {
            Err(ParserError::NegativeIdx)
        }
    }

    // Returns the `Token` at the `current` index and moves the index forward
    fn advance(&mut self) -> Result<&Token, ParserError> {
        if self.tokens_left()? {
            self.current += 1;
        }
        self.previous()
    }

    // Returns whether the `Token` at the `current` index is of desired `t_type`
    fn check(&self, t_type: &TokenType) -> Result<bool, ParserError> {
        let check = if self.tokens_left()? {
            self.peek()?.t_type.get().ok_or(ParserError::NoTokenType)? == t_type
        } else {
            false
        };
        Ok(check)
    }
}
