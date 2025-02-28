use crate::Interpreter;
use crate::{
    ast::{
        Binary, Call, ClassDeclaration, Expr, FunctionDeclaration, Group, IfStmt, Literal, Logical,
        ReturnStmt, Stmt, Ternary, Unary, VarStmt, WhileStmt, GetExpr,
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
pub struct Resolver<'a> {
    // Reference to the `Interpreter` used to store variable names and the scope level distance at
    // which their resolution is found.
    interpreter: &'a mut Interpreter,
    // Keeps track of all scopes in the form of a stack. Top most element is the innermost scope.
    // We use the key `String` as the name of the variable. The value is split in 2:
    // 1. First one flags that the variable was declared but not defined
    // 2. Second one defines that the variable was declared and defined but it is never used
    scopes: LinkedList<HashMap<String, (bool, bool)>>,
    // Keeps track if for this current point in time, the resolver is whithin a function scope or
    // not. This is used in order to prevent invalid `return` statements, as the ones which are not
    // inside a function.
    current_function: ResolverFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolverFunctionType {
    Function,
    None,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: LinkedList::new(),
            current_function: ResolverFunctionType::None,
        }
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(), ResolverError> {
        // Begin a new scope, the global scope
        self.begin_scope();
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        // End the scope before exiting
        self.end_scope();
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolverError> {
        stmt.walk(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolverError> {
        expr.walk(self)
    }

    fn resolve_local(&mut self, expr_addr: String, name: &Token) -> Result<(), ResolverError> {
        // Iterate through all the scopes from the innermost (top of the stack) to the outer most
        // (bottom of the stack)
        for (idx, scope) in self.scopes.iter().enumerate().rev() {
            // If we find the variable in one of the scopes
            if scope.contains_key(name.lexeme()) {
                // We resolve it, passing in the number of scopes between the current innermost
                // scope and the scope where the variable was found.
                return self
                    .interpreter
                    .resolve(expr_addr, self.scopes.len() - 1 - idx);
            }
        }
        Ok(())
    }

    fn resolve_function(
        &mut self,
        function: &FunctionDeclaration,
        func_type: ResolverFunctionType,
    ) -> Result<(), ResolverError> {
        // We first save the state of the current function
        let func_state = self.current_function.clone();
        // We then replace the state with the type sent in the function call
        self.current_function = func_type;
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
        // We revert the current function back to the state it was in before calling this
        // `resolve_function`
        self.current_function = func_state;
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
            // If the variable was already declared, the user should've just assigned to it.
            if current_scope.contains_key(name.lexeme()) {
                // At this point we have a double initialisation
                panic!("Already a variable with this name in this scoep {:?}", name);
            }
            // And insert the new declaration in this scope. Because we did not resolve the variable
            // yet, we insert it with a `false` flag in the scopes `HashMap`.
            current_scope.insert(name.lexeme().to_string(), (false, false));
        }
    }

    fn define(&mut self, name: &Token) {
        // At this point, initializer for the variable represented by name should have been run
        // and we mark it as such in the scope
        if let Some(current_scope) = self.scopes.back_mut() {
            current_scope.insert(name.lexeme().to_string(), (true, false));
        }
    }

    fn end_scope(&mut self) {
        // Pop the inner most scope
        if let Some(scope) = self.scopes.pop_back() {
            // Verify all the names defined in the scope
            for (key, (defined, accessed)) in scope.iter() {
                if defined == &true && accessed == &false {
                    panic!("Variable defined in this scope is not used {:?}", key);
                }
            }
        }
    }
}

impl ExprVisitor<Result<(), ResolverError>> for Resolver<'_> {
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
        if let Some(current_scope) = self.scopes.back_mut() {
            // If the variable is in this scope but it's initializer flag is false, it means it
            // was declared but not defined yet. We consider this an error and we report it.
            if current_scope.get(variable.lexeme()) == Some(&(false, false)) {
                return Err(ResolverError::NotInitialized(format!(
                    "Can't access local variable {} in it own initializer.",
                    variable
                )));
            } else {
                // We mark the variable as accessed
                current_scope.insert(variable.lexeme().to_string(), (true, true));
            }
        }
        // At this point, we know we should have a value for the variable and we resolve it
        self.resolve_local(format!("{:p}", variable), variable)?;
        Ok(())
    }

    fn visit_assign(&mut self, ident: &Token, expr: &Expr) -> Result<(), ResolverError> {
        self.resolve_expr(expr)?;
        self.resolve_local(format!("{:p}", expr), ident)?;
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

    fn visit_get(&mut self, _get: &GetExpr) -> Result<(), ResolverError> {
        Ok(())
    }
}

/// Trait that must be implemented by a type which want to use the Visitor pattern to visit a
/// `Stmt` statement of the Malis lanaguage
impl StmtVisitor<Result<(), ResolverError>> for Resolver<'_> {
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
        // We first check if we are in a function's scope
        if self.current_function == ResolverFunctionType::None {
            return Err(ResolverError::ReturnOutsideFunction(format!(
                "Can't return from top-level code: {:?}",
                stmt.keyword()
            )));
        }
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
        self.resolve_function(function, ResolverFunctionType::Function)
    }

    fn visit_class(&mut self, class: &ClassDeclaration) -> Result<(), ResolverError> {
        // The Malis resolver essentially sees this class as just a variable
        // Declare the class
        self.declare(&class.name);
        // Define the class
        self.define(&class.name);
        Ok(())
    }
}
