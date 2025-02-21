use crate::Interpreter;
use crate::{
    ast::{
        Binary, Call, Expr, FunctionDeclaration, Group, IfStmt, Literal, Logical, ReturnStmt, Stmt,
        Ternary, Unary, VarStmt, WhileStmt,
    },
    error::ResolverError,
    token::Token,
    visit::{ExprVisitor, StmtVisitor},
};
use std::collections::{HashMap, LinkedList};

// The resolver visits every node in the syntax tree and could perform the following actions:
// - Define a new scope
// - Append to an existing scope
// - Remove a scope
// The ultimate goal is to find a resolution for each variable access, according to the scope it is
// defined in. As such, the following nodes are of interest:
// - A block statement introduces a new scope for the statements it contains.
// - A function declaration introduces a new scope for its body and binds its parameters in that
// scope.
// - A variable declaration adds a new variable to the current scope.
// - Variable and assignment expressions need to have their variables resolved.
pub struct Resolver {
    interpreter: Interpreter,
    scopes: LinkedList<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: LinkedList::new(),
        }
    }

    fn resolve(&mut self, stmts: &[Stmt]) -> Result<(), ResolverError> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolverError> {
        stmt.walk(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolverError> {
        expr.walk(self)
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), ResolverError> {
        // Iterate through all the scopes from the innermost (top of the stack) to the outer most
        // (bottom of the stack)
        for (idx, scope) in self.scopes.iter().rev().enumerate() {
            // If we find the variable in one of the scopes
            if scope.get(name.lexeme()).is_some() {
                // We resolve it, passing in the number of scopes between the current innermost
                // scope and the scope where the varaible was found.
                self.interpreter.resolve(expr, idx)?;
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, function: &FunctionDeclaration) -> Result<(), ResolverError> {
        // Each function declaration creates a new scope
        self.begin_scope();

        // We first declare and define each of the function's parameters
        for param in function.parameters.iter() {
            self.declare(param);
            self.define(param);
        }

        // Afterards, we resolve the function body
        self.resolve(&function.body)?;

        self.end_scope();
        // Each function exit, end a scope
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push_back(HashMap::new());
    }

    fn declare(&mut self, name: &Token) {
        // We get a mutable reference to the top stack scope. This way the variable will be
        // declared in the the innermost scope and will shadow any other existing variable with the
        // same name
        if let Some(current_scope) = self.scopes.back_mut() {
            // And insert the new declaration in this scope. Because we did not resolve the variable
            // yet, we insert it with a `false` flag in the scopes `HashMap`.
            current_scope.insert(name.lexeme().to_string(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        // At this point, initializer for the variable represented by name should have been run
        // and we mark it as such in the scope
        if let Some(current_scope) = self.scopes.back_mut() {
            current_scope.insert(name.lexeme().to_string(), true);
        }
    }

    fn end_scope(&mut self) {
        self.scopes.pop_back();
    }
}

impl ExprVisitor<Result<(), ResolverError>> for Resolver {
    fn visit_unary(&mut self, unary: &Unary) -> Result<(), ResolverError> {
        self.resolve_expr(&unary.right)
    }

    fn visit_binary(&mut self, binary: &Binary) -> Result<(), ResolverError> {
        self.resolve_expr(&binary.left)?;
        self.resolve_expr(&binary.right)
    }

    fn visit_ternary(&mut self, ternary: &Ternary) -> Result<(), ResolverError> {
        self.resolve_expr(&ternary.first)?;
        self.resolve_expr(&ternary.second)?;
        self.resolve_expr(&ternary.third)
    }

    fn visit_literal(&mut self, _literal: &Literal) -> Result<(), ResolverError> {
        // Literals could only resolve to themselves
        Ok(())
    }

    fn visit_group(&mut self, group: &Group) -> Result<(), ResolverError> {
        self.resolve_expr(&group.expr)
    }

    fn visit_variable(&mut self, variable: &Token) -> Result<(), ResolverError> {
        // We read the scope map and check whether the variable is defined in the current scope.
        if let Some(current_scope) = self.scopes.back() {
            // If the variable is in this scope but it's initializer flag is false, it means it
            // was declared but not defined yet. We consider this an error and we report it.
            if current_scope.get(variable.lexeme()) == Some(&false) {
                return Err(ResolverError::NotInitialized(format!(
                    "Can't access local variable {} in it own initializer.",
                    variable
                )));
            }
        }
        // At this point, we know we should have a value for the variable and we resolve it
        self.resolve_local(&Expr::Var(variable.clone()), variable)?;
        Ok(())
    }

    fn visit_assign(&mut self, ident: &Token, expr: &Expr) -> Result<(), ResolverError> {
        self.resolve_expr(expr)?;
        self.resolve_local(expr, ident)?;
        Ok(())
    }

    fn visit_logical(&mut self, logical: &Logical) -> Result<(), ResolverError> {
        self.resolve_expr(&logical.left)?;
        self.resolve_expr(&logical.right)
    }

    fn visit_call(&mut self, call: &Call) -> Result<(), ResolverError> {
        self.resolve_expr(&call.callee)?;

        for arg in call.arguments.iter() {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }
}

/// Trait that must be implemented by a type which want to use the Visitor pattern to visit a
/// `Stmt` statement of the Malis lanaguage
impl StmtVisitor<Result<(), ResolverError>> for Resolver {
    fn visit_expr_stmt(&mut self, stmt: &Expr) -> Result<(), ResolverError> {
        self.resolve_expr(stmt)
    }

    fn visit_print_stmt(&mut self, stmt: &Expr) -> Result<(), ResolverError> {
        self.resolve_expr(stmt)
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), ResolverError> {
        // We spilt variable initialization into 2 steps: declaring and defining.
        self.declare(stmt.identifier());
        if let Some(expr) = &stmt.expr() {
            self.resolve_expr(expr)?;
        }
        self.define(stmt.identifier());
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Result<(), ResolverError> {
        // A block begins a new scope
        self.begin_scope();
        // It resolves the statement inside it
        self.resolve(stmts)?;
        // And finished the scope afterwards
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), ResolverError> {
        // In an if statement we resolve 1 expression for the condition
        self.resolve_expr(&stmt.condition)?;
        // And then we resolve the then branch
        self.resolve_stmt(&stmt.then_branch)?;
        // optional 3rd expression which is the else branch. Here we do not care about control flow
        // and we resolve any branch
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), ResolverError> {
        // Resolve the condition of the while
        self.resolve_expr(&stmt.condition)?;
        // Resolve the body/statemet of the while
        self.resolve_stmt(&stmt.stmt)
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), ResolverError> {
        // If return also comes with a value to be returned
        if let Some(value) = stmt.expr() {
            // We return it
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_function(&mut self, function: &FunctionDeclaration) -> Result<(), ResolverError> {
        // Functions both bind names and introduce a scope. When a function is declared, the name
        // of the function is bound in the current scope where the function is declared. And when
        // we step into the function's body, we also bind its parameters to the new scope introduced
        // by the function's body.
        self.declare(&function.name);
        // We define the function eagerly, just after declaration. This enables a function to call
        // itself and do recursion.
        self.define(&function.name);
        self.resolve_function(function)
    }
}
