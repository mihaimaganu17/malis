pub mod function;
pub mod object;
pub mod malis_class;
pub mod visit;

pub use function::{MalisCallable, NativeFunction, UserFunction};
pub use object::MalisObject;
pub use malis_class::{MalisClass, MalisInstance};
use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    error::{ResolverError, RuntimeError},
    token::Token,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    // This is the global environment that is accessible at all times
    _globals: Rc<RefCell<Environment>>,
    // This is the current local environment that the interepreter executes in
    environment: Rc<RefCell<Environment>>,
    // Stores resolution information about variables and how many scopes we have to traverse
    // between the current scope (the one in which the variable is accessed) and the resolution
    // scope (the one that contains the value for the variable)
    locals: HashMap<String, usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let maybe_interpreter = Self::new();

        match maybe_interpreter {
            Ok(interpreter) => interpreter,
            Err(err) => {
                println!("Error: Native functions not available {}", err);
                Self {
                    ..Default::default()
                }
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Result<Self, RuntimeError> {
        // Define a new environment
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        let environment = globals.clone();

        // Create a new native function
        let clock = MalisObject::NativeFunction(Box::new(NativeFunction::new(
            "clock <native fn>".to_string(),
            0,
            |_interpreter, _arguments| {
                Ok(MalisObject::Number(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs_f32(),
                ))
            },
        )));

        globals.borrow_mut().define("clock".to_string(), clock)?;

        Ok(Self {
            _globals: globals,
            environment,
            locals: HashMap::new(),
        })
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements.iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn resolve(&mut self, expr: String, scope_level: usize) -> Result<(), ResolverError> {
        self.locals.insert(expr, scope_level);

        Ok(())
    }

    fn lookup_variable(&mut self, var: &Token) -> Result<MalisObject, ResolverError> {
        // If there is a distance, it means the variable was in an specific environment
        let object = if let Some(distance) = self.locals.get(&format!("{:p}", var)) {
            // We traverse `distance` environments in order to get the value
            self.environment
                .borrow()
                .get_at(*distance, var.lexeme())?
                .clone()
        } else {
            self._globals.borrow().get(var.lexeme())?.clone()
        };
        Ok(object)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<MalisObject, RuntimeError> {
        expr.walk(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.walk(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],
        parent_env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        // Executing a block requires creating a new environment, executing within that environment
        // and restoring the environment to its previous state

        // To prevent creating a cycle, we must take the value out of the parent environment.
        // Afterwards, we wrap it in a `Rc` as it is required in order to share it. We also wrap it
        // in a `RefCell` such that we obtain mutable state
        let parent_env_rc = Rc::new(RefCell::new(parent_env.take()));

        // Save the current environment assigned to the interpreter as `previous_env`.
        // This is used to prevent losing the top environment when executing an inner scope.
        // The environment for the current execution block becomes the parent environemnt, such
        // that we could access scope from the current block's scope and from the scope that
        // contains this block as well
        let previous_env = self
            .environment
            .replace(Environment::new(Some(parent_env_rc.clone())));

        // Start executing statements
        for stmt in stmts.iter() {
            // Execute statement
            let stmt_exec = self.execute(stmt);

            // If the statement is an error, we cannot return it just yet
            if stmt_exec.is_err() {
                // We must reverse the scope created above and replace the executing scope with
                // the scope we have before entering the block.
                // Order of operations is important. Replacing the current execution environment
                // first assures that there is not any other strong reference to the previous
                // environment
                self.environment.replace(previous_env);

                // We also replace the parent environment with the initial environment we passed
                // when entering the scope
                parent_env.replace(
                    Rc::into_inner(parent_env_rc)
                        .ok_or(RuntimeError::MultipleReferenceForEnclosingEnvironment)?
                        .into_inner(),
                );
                return stmt_exec;
            }
        }

        // We must reverse the scope created above and replace the executing scope with
        // the scope we have before entering the block.
        // Order of operations is important. Replacing the current execution environment
        // first assures that there is not any other strong reference to the previous
        // environment
        self.environment.replace(previous_env);
        // We also replace the parent environment with the initial environment we passed
        // when entering the scope
        parent_env.replace(
            Rc::into_inner(parent_env_rc)
                .ok_or(RuntimeError::MultipleReferenceForEnclosingEnvironment)?
                .into_inner(),
        );

        Ok(())
    }
}
