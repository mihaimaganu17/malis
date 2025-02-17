pub mod function;
pub mod object;
pub mod visit;

use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    error::RuntimeError,
};
pub use function::{MalisCallable, NativeFunction, UserFunction};
pub use object::MalisObject;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    // This is the global environment that is accessible at all times
    globals: Rc<RefCell<Environment>>,
    // This is the current local environment that the interepreter executes in
    environment: Rc<RefCell<Environment>>,
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
            globals,
            environment,
        })
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements.iter() {
            self.execute(stmt)?;
        }
        Ok(())
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
        parent_env: Environment,
    ) -> Result<(), RuntimeError> {
        // Save the current environment assigned to the interpreter. This is used to prevent losing
        // the top environment when executing a inner scope
        // The environment for the current execution block becomes the parent environemnt, such
        // that we could access scope from the current block's scope and from the scope that
        // contains this block as well
        let _ = self
            .environment
            .replace(Environment::new(Some(Rc::new(RefCell::new(parent_env)))));

        for stmt in stmts {
            self.execute(stmt)?;
        }
        // Bring the initial environment back which contains the scope our interpreter had before
        // execution of this block. This resides in the enclosing field
        let env = self
            .environment
            .borrow_mut()
            .enclosing
            .take()
            .ok_or(RuntimeError::CannotAccessParentScope)?;
        // Replace our current environment wit the enclosing one. Here we make sure that the
        // enclosing environment has no other reference to itself, such that we can move it.
        self.environment.replace(
            Rc::into_inner(env)
                .ok_or(RuntimeError::MultipleReferenceForEnclosingEnvironment)?
                .into_inner(),
        );

        println!("Before exiting block {:#?}", self.environment);
        Ok(())
    }
}
