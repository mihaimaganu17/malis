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
        // We first check for the first `term` according to the production rule
        let mut expr = self.term()?;

        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule
        let greater = TokenType::Comparison(Comparison::Greater);
        let greater_equal = TokenType::Comparison(Comparison::GreaterEqual);
        let less = TokenType::Comparison(Comparison::Less);
        let less_equal = TokenType::Comparison(Comparison::LessEqual);

        while self.any(&[&greater, &greater_equal, &less, &less_equal])? {
            // The operator if the `Token` that we matched above
            let operator = self.advance()?.clone();
            // After the operator, the expression is the next term
            let right_expr = self.term()?;
            // We create a new `Binary` expression using the two
            expr = Expr::Binary(Binary::new(expr, operator, right_expr));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        // We first check for the first `factor` according to the production rule
        let mut expr = self.factor()?;

        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule
        let minus = TokenType::SingleChar(SingleChar::Minus);
        let plus = TokenType::SingleChar(SingleChar::Plus);

        while self.any(&[&minus, &plus])? {
            // The operator if the `Token` that we matched above
            let operator = self.advance()?.clone();
            // After the operator, the expression is the next factor
            let right_expr = self.factor()?;
            // We create a new `Binary` expression using the two
            expr = Expr::Binary(Binary::new(expr, operator, right_expr));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        // We first check for the first `unary` according to the production rule
        let mut expr = self.unary()?;

        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule
        let slash = TokenType::SingleChar(SingleChar::Slash);
        let star = TokenType::SingleChar(SingleChar::Star);

        while self.any(&[&slash, &star])? {
            // The operator if the `Token` that we matched above
            let operator = self.advance()?.clone();
            // After the operator, the expression is the next factor
            let right_expr = self.unary()?;
            // We create a new `Binary` expression using the two
            expr = Expr::Binary(Binary::new(expr, operator, right_expr));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule
        let bang = TokenType::Comparison(SingleChar::Comparison);
        let minus = TokenType::SingleChar(SingleChar::Minus);

        // Unary is either formed by an unary operator followed by its operand
        let expr = if self.any(&[&bang, &minus])? {
            let operator = self.advance()?.clone();
            let mut expr = self.unary()?;
            Expr::Unary::new(operator, expr)
        } else {
            // Or a single primary production rule
            self.primary()?
        };

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        todo!()
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
