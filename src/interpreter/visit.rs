use super::{Interpreter, MalisCallable, MalisObject, UserFunction};
use crate::{
    ast::{
        Binary, Call, Expr, FunctionDeclaration, Group, IfStmt, Literal, LiteralType, Logical,
        ReturnStmt, Stmt, Ternary, Unary, VarStmt, WhileStmt,
    },
    error::RuntimeError,
    token::{Comparison, Keyword, SingleChar, Token, TokenType},
    visit::{ExprVisitor, StmtVisitor},
};

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expr_stmt(&mut self, stmt: &Expr) -> Result<(), RuntimeError> {
        self.evaluate(stmt)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Expr) -> Result<(), RuntimeError> {
        let expr = self.evaluate(stmt)?;
        println!("{expr}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), RuntimeError> {
        let value = if let Some(expr) = stmt.expr() {
            self.evaluate(expr)?
        } else {
            MalisObject::Nil
        };
        let name = stmt.identifier().lexeme();
        let _ = self
            .environment
            .borrow_mut()
            .define(name.to_string(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        self.execute_block(stmts, self.environment.clone())
    }

    fn visit_if_stmt(&mut self, if_stmt: &IfStmt) -> Result<(), RuntimeError> {
        let cond = self.evaluate(&if_stmt.condition)?;

        if cond.is_truthy() {
            self.execute(&if_stmt.then_branch)?;
        } else if let Some(branch) = &if_stmt.else_branch {
            self.execute(branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, while_stmt: &WhileStmt) -> Result<(), RuntimeError> {
        while self.evaluate(&while_stmt.condition)?.is_truthy() {
            self.execute(&while_stmt.stmt)?;
        }

        Ok(())
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt) -> Result<(), RuntimeError> {
        let expr = if let Some(expr) = return_stmt.expr() {
            self.evaluate(expr)?
        } else {
            MalisObject::Nil
        };

        // We are using the error system in `Rust` to return from a function in Malis. This is used
        // in order to clean all the needed context
        Err(RuntimeError::Return(expr))
    }

    fn visit_function(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Result<(), RuntimeError> {
        // Get the function name
        let func_name = function_declaration.name.lexeme().to_string();
        // Since we want the closure environment to remain a snapshot of the scope this current
        // function declaration is in, we need to do a complete clone of the `Environment` object.
        // This is because cloning the `Rc` alone would just give us a reference that could change
        // after exiting this function due to other statements.
        use std::cell::RefCell;
        use std::rc::Rc;
        let closure_env = Rc::new(RefCell::new(self.environment.borrow().clone()));
        // We define the function with the environment present at the time of declaration
        self.environment.borrow_mut().define(
            func_name,
            MalisObject::UserFunction(UserFunction::new(
                function_declaration.clone(),
                closure_env.clone(),
            )),
        )?;
        closure_env.replace(self.environment.borrow().clone());
        Ok(())
    }
}

impl ExprVisitor<Result<MalisObject, RuntimeError>> for Interpreter {
    fn visit_unary(&mut self, unary: &Unary) -> Result<MalisObject, RuntimeError> {
        // We first evaluate the operand expression;
        let right_malis_object = unary.right.walk(self)?;
        // Our interpreter is doing a post-order traversal - each node evaluates its children
        // before doing its own work. As such we first evaluated the underlying expression above
        // and now we are evaluating the operator of our current value
        match unary.operator.t_type() {
            TokenType::SingleChar(SingleChar::Minus) => -right_malis_object,
            TokenType::SingleChar(SingleChar::Bang) => Ok(!right_malis_object),
            _ => Err(RuntimeError::UnaryEvaluation(format!(
                "Invalid unary operator {:?}",
                unary.operator
            ))),
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> Result<MalisObject, RuntimeError> {
        // In a binary expression, we evaluate the operand from left to right and then evaulte
        // the binary expression itself
        let left_object = binary.left.walk(self)?;
        let right_object = binary.right.walk(self)?;

        match binary.operator.t_type() {
            TokenType::SingleChar(SingleChar::Plus) => left_object + right_object,
            TokenType::SingleChar(SingleChar::Minus) => left_object - right_object,
            TokenType::SingleChar(SingleChar::Slash) => left_object / right_object,
            TokenType::SingleChar(SingleChar::Star) => left_object * right_object,
            TokenType::Comparison(Comparison::Greater) => {
                Ok(MalisObject::Boolean(left_object.gt(&right_object)))
            }
            TokenType::Comparison(Comparison::GreaterEqual) => {
                Ok(MalisObject::Boolean(left_object.ge(&right_object)))
            }
            TokenType::Comparison(Comparison::Less) => {
                Ok(MalisObject::Boolean(left_object.lt(&right_object)))
            }
            TokenType::Comparison(Comparison::LessEqual) => {
                Ok(MalisObject::Boolean(left_object.le(&right_object)))
            }
            TokenType::Comparison(Comparison::BangEqual) => {
                Ok(MalisObject::Boolean(left_object.ne(&right_object)))
            }
            TokenType::Comparison(Comparison::EqualEqual) => {
                Ok(MalisObject::Boolean(left_object.eq(&right_object)))
            }
            // When we have the comma separator, separating multiple expressions, similar to C,
            // the return value is the result of the last expression
            TokenType::SingleChar(SingleChar::Comma) => Ok(right_object),
            _ => Err(RuntimeError::BinaryEvaluation(format!(
                "Invalid binary operator {:?}",
                binary.operator
            ))),
        }
    }
    fn visit_ternary(&mut self, ternary: &Ternary) -> Result<MalisObject, RuntimeError> {
        let cond = self.evaluate(&ternary.first)?;

        if cond.is_truthy() {
            ternary.second.walk(self)
        } else {
            ternary.third.walk(self)
        }
    }
    // Evaluating literals. A literal is a `bit of syntax` that produces a vlue. A literal
    // always appears somewhere in the user's source code. Lots of values are produces by
    // computation and don't exist anywhere in the code itself, but those are not literals.
    // A literal comes from the parser's domain. Values are an interpreter concept, part of the
    // runtime's world.
    fn visit_literal(&mut self, literal: &Literal) -> Result<MalisObject, RuntimeError> {
        let malis_object = match &literal.l_type {
            LiteralType::Number(n) => MalisObject::Number(f32::from_le_bytes(*n)),
            LiteralType::LitString(s) => MalisObject::StringValue(s.to_string()),
            LiteralType::True => MalisObject::Boolean(true),
            LiteralType::False => MalisObject::Boolean(false),
            LiteralType::Nil => MalisObject::Nil,
        };
        Ok(malis_object)
    }
    // Grouping is an expression surrounded by parenthesis. To evaluate the grouping expression
    // itself, we recursively evaluate the subexpression contained and return it.
    fn visit_group(&mut self, group: &Group) -> Result<MalisObject, RuntimeError> {
        group.expr.walk(self)
    }

    // One type of expression is accessing a variable, previously declared, using it's identifier.
    // We do that by accessing the interpreters environment
    fn visit_variable(&mut self, var: &Token) -> Result<MalisObject, RuntimeError> {
        Ok(self.lookup_variable(var)?)
    }

    // Assignment is treated as an expression and not a variable. As such, we need a previously
    // defined identifier which mutates state to the new value
    fn visit_assign(&mut self, ident: &Token, expr: &Expr) -> Result<MalisObject, RuntimeError> {
        let malis_object = expr.walk(self)?;

        // If there is a distance, it means the variable was in an specific environment
        let object = if let Some(distance) = self.locals.get(&format!("{:p}", expr)) {
            // We traverse `distance` environments in order to get the value
            self.environment
                .borrow_mut()
                .insert_at(*distance, ident.lexeme(), malis_object)?
        } else {
            self._globals
                .borrow_mut()
                .insert(ident.lexeme(), malis_object)?
        };

        Ok(object)
    }

    fn visit_logical(&mut self, logical: &Logical) -> Result<MalisObject, RuntimeError> {
        // In a logical expression, we evaluate the operand from left to right and then evaulte
        // the logical expression itself. The logical expression operators `or` and `and`
        // short-circuit. This means that:
        // - for the `or` operator, if the first operand evaluates to `true` we do not have to
        // evaulate the second operand
        // - for the `and` operator, if the first operand evaluates to `false` we do not have to
        // evaulate the second operand
        let left_object = logical.left.walk(self)?;
        let left_object_is_true = left_object.is_truthy();

        match logical.operator.t_type() {
            TokenType::Keyword(Keyword::Or) => {
                if left_object_is_true {
                    return Ok(left_object);
                }
            }
            TokenType::Keyword(Keyword::And) => {
                if !left_object_is_true {
                    return Ok(left_object);
                }
            }
            _ => unreachable!(),
        }

        let right_object = logical.right.walk(self)?;
        Ok(right_object)
    }

    fn visit_call(&mut self, call: &Call) -> Result<MalisObject, RuntimeError> {
        // First we evaluate the callee
        let callee = self.evaluate(&call.callee)?;
        // Next we evaluate each of the arguments
        let mut arguments = vec![];

        for arg in call.arguments.iter() {
            arguments.push(self.evaluate(arg)?);
        }

        if !callee.is_callable() {
            return Err(RuntimeError::NotCallable(format!(
                "[{:?}] Object {} is not callable.",
                call.paren, callee
            )));
        }
        // Check if the number of arguments matches the function's arity
        if arguments.len() != callee.arity()? {
            return Err(RuntimeError::InvalidArgumentsNumber(format!(
                "[{:?}] Expected {} arguments but got {}.",
                call.paren,
                callee.arity()?,
                arguments.len()
            )));
        }
        callee.call(self, arguments)
    }
}
