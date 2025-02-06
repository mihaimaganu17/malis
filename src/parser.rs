use crate::{
    ast::{Binary, Expr, Group, Literal, Stmt, Ternary, Unary, VarStmt, IfStmt},
    error::ParserError,
    token::{Comparison, Keyword, SingleChar, Token, TokenType},
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

    pub fn reset(&mut self) {
        self.current = 0
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = vec![];
        while self.tokens_left()? {
            if let Some(declaration) = self.declaration()? {
                statements.push(declaration);
            }
        }
        Ok(statements)
    }

    // Parses a Malis Declaration, which is in fact a node of statement
    fn declaration(&mut self) -> Result<Option<Stmt>, ParserError> {
        // We could have 1 type of declaration as a statemen: variable declration
        let var_token = TokenType::Keyword(Keyword::Var);

        let maybe_declaration = if self.any(&[&var_token])? {
            self.advance()?;
            self.var_declaration()
        } else {
            self.statement()
        };

        if maybe_declaration.is_err() {
            self.synchronize()?;
            return Ok(None);
        }
        maybe_declaration.map(Some)
    }

    // Parses a Malis Variable Declaration, which is in fact a node of statement
    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        // At this point we have a `var` keyword and we need to consume the Identifier that follows
        // it
        let ident = TokenType::Ident;
        let var_name = self
            .consume(&ident, "Expected a variable name".to_string())?
            .clone();

        // We now have an indetifier and we optionally need to bind it to a value using equal `=`
        let equal = TokenType::SingleChar(SingleChar::Equal);

        let maybe_binded = if self.any(&[&equal])? {
            self.advance()?;
            Some(self.ternary()?)
        } else {
            None
        };

        // We need to consume the `;` in order to parse a proper declaration statement
        let semi_colon = TokenType::SingleChar(SingleChar::SemiColon);
        self.consume(&semi_colon, "Expect ';' after expression".to_string())?;
        Ok(Stmt::Var(VarStmt::new(var_name, maybe_binded)))
    }

    // Parses a Malis Statement
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        // We could have 3 type of statements: expression statement, block statements (inside a new
        // scoped block) and print statements

        // Print statements are identified by the keyword `print`
        let print_token = TokenType::Keyword(Keyword::Print);

        if self.any(&[&print_token])? {
            // Consume the print
            let _ = self.advance()?;
            return self.print_statement();
        }

        // If statements are identified by the keyword `if`
        let if_token = TokenType::Keyword(Keyword::If);

        if self.any(&[&if_token])? {
            // Consume the `if` token
            let _ = self.advance()?;
            return self.if_statement();
        }

        // Block statements are starting with a left curly brace
        let left_brace = TokenType::SingleChar(SingleChar::LeftBrace);

        if self.any(&[&left_brace])? {
            // Consume the left brace
            let _ = self.advance()?;
            return self.block_statement();
        }

        self.expr_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        // Parse the expression in the statement
        let expr = self.separator()?;
        let semi_colon = TokenType::SingleChar(SingleChar::SemiColon);
        // We need to consume the `;` in order to parse a proper statement
        self.consume(&semi_colon, "Expect ';' after expression".to_string())?;
        Ok(Stmt::Print(expr))
    }

    // A block statement is a block definining a new scope, which contains several statements.
    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        // In an if statement, we first parse the condition which is an `expression` surrounded by
        // parenthesis.
        let left_paren = TokenType::SingleChar(SingleChar::LeftParen);

        // While we did not reach the ending right brace
        let (condition, then_branch) = if self.any(&[&left_paren])? {
            // Consume the left paren
            let _ = self.advance()?;
            // Consume the condition
            let condition = self.separator()?;
            // Consume the right parenthesis
            let right_paren = TokenType::SingleChar(SingleChar::RightParen);
            // We need to consume the `;` in order to parse a proper statement
            self.consume(&right_paren, "Expect ')' after `if` condition".to_string())?;
            // Now we parse the statement for the `true` then-branch of the condition evaluation
            let then_branch = self.statement()?;

            (condition, then_branch)
        } else {
            return Err(ParserError::InvalidIfStmt(
                    format!("Expected `if` followed by `(` condition `)`, found {:?}", self.peek()?)));
        };

        // At this point we have the following `if (condition) statement` logic parsed.
        // Now, we have to also check the else keyword and branch
        let else_token = TokenType::Keyword(Keyword::Else);

        let else_branch = if self.any(&[&else_token])? {
            // Consume the `else`
            let _ = self.advance()?;
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::If(IfStmt::new(condition, then_branch, else_branch)))
    }

    // A block statement is a block definining a new scope, which contains several statements.
    fn block_statement(&mut self) -> Result<Stmt, ParserError> {
        // Prepare a new vector that will hold the statements in this block
        let mut statements = vec![];

        let right_brace = TokenType::SingleChar(SingleChar::RightBrace);

        // While we did not reach the ending right brace
        while !self.any(&[&right_brace])? {
            // Consume the next declaration
            if let Some(declaration) = self.declaration()? {
                // Add it to the list of statements
                statements.push(declaration);
            }
        }

        // We need to consume the right brace `}` which ends the block
        self.consume(
            &right_brace,
            "Expect '}' to close the block scope".to_string(),
        )?;

        Ok(Stmt::Block(statements))
    }

    fn expr_statement(&mut self) -> Result<Stmt, ParserError> {
        // Parse the expression in the statement
        let expr = self.separator()?;
        let semi_colon = TokenType::SingleChar(SingleChar::SemiColon);
        // We need to consume the `;` in order to parse a proper statement
        self.consume(&semi_colon, "Expect ';' after expression".to_string())?;
        Ok(Stmt::Expr(expr))
    }

    pub fn separator(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.assignment()?;
        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule. In this case, we want to match comma which could be used in C to chain expressions
        // together similar to how a block chains statements
        let comma = TokenType::SingleChar(SingleChar::Comma);

        // Then we have a compound of any number of `!=` or `==` followed by another comparison
        while self.any(&[&comma])? {
            // The operator if the `Token` that we matched above
            let operator = self.advance()?.clone();
            // After the operator, the expression is the next comparison
            let right_expr = self.assignment()?;
            // We create a new `Binary` expression using the two
            expr = Expr::Binary(Binary::new(expr, operator, right_expr));
        }
        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        // Try and parse a normal ternary expression
        let expr = self.ternary()?;
        // If we have an equal afterwards
        let equal = TokenType::SingleChar(SingleChar::Equal);
        if self.any(&[&equal])? {
            // Move past the equal
            let equals = self.advance()?.clone();
            // Get the next value
            let value = self.assignment()?;
            // If the top expression that we parsed, is actualy a variable name
            if let Expr::Var(var) = expr {
                // We return a new assign expression with that variable name and the value
                Ok(Expr::Assign(var, Box::new(value)))
            } else {
                Err(ParserError::PanicMode(
                    "Invalid assignment target".to_string(),
                    equals,
                ))
            }
        } else {
            Ok(expr)
        }
    }

    fn ternary(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.expression()?;
        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule. In this case, we want to match question mark first and then colon
        let question_mark = TokenType::SingleChar(SingleChar::Question);
        let colon = TokenType::SingleChar(SingleChar::Colon);

        // Then we have a compound of any number of `!=` or `==` followed by another comparison
        while self.any(&[&question_mark])? {
            // The operator if the `Token` that we matched above
            let operator1 = self.advance()?.clone();
            // After the operator, the expression is the value to be returned if the condition
            // is true
            let variant1 = self.expression()?;
            // At this point, we need to consume the colon to have a valid ternary condition
            let operator2 = if self
                .consume(&colon, "Expect ':' after expression".to_string())
                .is_err()
            {
                return Err(ParserError::MissingColon);
            } else {
                self.previous()?.clone()
            };
            let variant2 = self.expression()?;

            // We create a new `ternary` expression using the two
            expr = Expr::Ternary(Ternary::new(expr, operator1, variant1, operator2, variant2));
        }
        Ok(expr)
    }

    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality()?;
        Ok(expr)
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
        let bang = TokenType::SingleChar(SingleChar::Bang);
        let minus = TokenType::SingleChar(SingleChar::Minus);

        // Unary is either formed by an unary operator followed by its operand
        let expr = if self.any(&[&bang, &minus])? {
            let operator = self.advance()?.clone();
            let expr = self.unary()?;
            Expr::Unary(Unary::new(operator, expr))
        } else {
            // Or a single primary production rule
            self.primary()?
        };

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule
        if let Ok(literal) = Literal::new(self.peek()?) {
            self.advance()?;
            Ok(Expr::Literal(literal))
        } else {
            match self.peek_type()? {
                TokenType::SingleChar(SingleChar::LeftParen) => {
                    // Move past the left parenthesis
                    self.advance()?;
                    // Parse the expression following if possible
                    let expr = self.separator()?;
                    // Consume the closing parenthesis
                    let right_paren = TokenType::SingleChar(SingleChar::RightParen);
                    if self
                        .consume(&right_paren, "Expect ')' after expression".to_string())
                        .is_ok()
                    {
                        Ok(Expr::Group(Group::new(expr)))
                    } else {
                        Err(ParserError::MissingClosingParen)
                    }
                }
                TokenType::Ident => {
                    let token = self.advance()?.clone();
                    Ok(Expr::Var(token))
                }
                _ => {
                    self.error()?;
                    Err(ParserError::NoPrimaryProduction)
                }
            }
        }
    }

    fn error(&mut self) -> Result<(), ParserError> {
        // Prepare the `TokenType`s we want to match against for the operators of this production
        // rule. In this case, we want to match comma which could be used in C to chain expressions
        // together similar to how a block chains statements
        let comma = TokenType::SingleChar(SingleChar::Comma);
        let bang_equal = TokenType::Comparison(Comparison::BangEqual);
        let equal_equal = TokenType::Comparison(Comparison::EqualEqual);
        let greater = TokenType::Comparison(Comparison::Greater);
        let greater_equal = TokenType::Comparison(Comparison::GreaterEqual);
        let less = TokenType::Comparison(Comparison::Less);
        let less_equal = TokenType::Comparison(Comparison::LessEqual);
        let minus = TokenType::SingleChar(SingleChar::Minus);
        let plus = TokenType::SingleChar(SingleChar::Plus);
        let slash = TokenType::SingleChar(SingleChar::Slash);
        let star = TokenType::SingleChar(SingleChar::Star);

        // Then we have a compound of any number of `!=` or `==` followed by another comparison
        if self.any(&[
            &comma,
            &bang_equal,
            &equal_equal,
            &greater,
            &greater_equal,
            &less,
            &less_equal,
            &minus,
            &plus,
            &slash,
            &star,
        ])? {
            // The operator if the `Token` that we matched above
            let operator = self.advance()?.clone();
            // After the operator, the expression is the next comparison
            let right_expr = self.ternary()?;

            let message = format!(
                "Found binary operator {} with only right operand {}",
                operator,
                crate::AstPrinter.print_expr(&right_expr)
            );

            Err(ParserError::PanicMode(message, operator))
        } else {
            Ok(())
        }
    }

    // Given the list of `t_types` token types, we check if the current token matches any of the
    // desired ones.
    fn any(&self, t_types: &[&TokenType]) -> Result<bool, ParserError> {
        for t_type in t_types {
            if self.check(t_type)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn consume(&mut self, t_type: &TokenType, message: String) -> Result<&Token, ParserError> {
        if self.any(&[t_type])? {
            self.advance()
        } else {
            Err(ParserError::PanicMode(message, self.peek()?.clone()))
        }
    }

    // Returns whether there are more tokens to be parsed
    fn tokens_left(&self) -> Result<bool, ParserError> {
        let token = self.peek()?;

        Ok(token.t_type.get().ok_or(ParserError::NoTokenType)? != &TokenType::Eof)
    }

    // Returns the token at the `current` index
    fn peek(&self) -> Result<&Token, ParserError> {
        self.tokens
            .get(self.current)
            .ok_or(ParserError::InvalidIdx(self.current))
    }

    // Returns the token type at the `current` index, without further advancing the cursor
    fn peek_type(&self) -> Result<&TokenType, ParserError> {
        self.peek()?.t_type.get().ok_or(ParserError::NoTokenType)
    }

    // Returns the token that preceded `current` indexed token
    fn previous(&self) -> Result<&Token, ParserError> {
        if self.current != 0 {
            self.tokens
                .get(self.current - 1)
                .ok_or(ParserError::InvalidIdx(self.current - 1))
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

    // Synchronizes the recursive descent parser which entered the panic mode due to an unxpected
    // token and tries to get the parser back to a safe state for further parsing the remaining
    // of the code or script. This entails the following: unwinding the call stack, such that we
    // clear any tokens owned by the current faulty statement and finding the start of the next
    // statement
    fn synchronize(&mut self) -> Result<(), ParserError> {
        self.advance()?;
        // while we are not at the end of the code
        while self.tokens_left()? {
            // If we are at a semicolon, this means the current statement ended and we just need
            // to go past it and return in order to synchronize
            // P
            if self.check(&TokenType::SingleChar(SingleChar::SemiColon))? {
                // Go past the faulty token which issued the panic mode
                self.advance()?;
                return Ok(());
            }

            if let TokenType::Keyword(
                Keyword::Class
                | Keyword::Fun
                | Keyword::Var
                | Keyword::For
                | Keyword::If
                | Keyword::While
                | Keyword::Print
                | Keyword::Return,
            ) = self.peek_type()?
            {
                // We (likely) are at the start of a new statement
                return Ok(());
            }
            // If we reach this point, we must munch tokens forward until we find the start of
            // another statement
            self.advance()?;
        }
        Ok(())
    }
}
