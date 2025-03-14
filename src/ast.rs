use crate::{
    error::{AstError, MalisError},
    token::{Keyword, Literal as LiteralToken, Token, TokenType},
    visit::{ExprVisitor, StmtVisitor},
};

#[derive(Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(VarStmt),
    Block(Vec<Stmt>),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionDeclaration),
    Return(ReturnStmt),
    Class(ClassDeclaration),
}

impl AsRef<Stmt> for Stmt {
    fn as_ref(&self) -> &Stmt {
        self
    }
}

impl Stmt {
    pub fn walk<T, V: StmtVisitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr_stmt(expr),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::Var(var) => visitor.visit_var_stmt(var),
            Stmt::Block(stmts) => visitor.visit_block_stmt(stmts),
            Stmt::If(if_stmt) => visitor.visit_if_stmt(if_stmt),
            Stmt::While(while_stmt) => visitor.visit_while_stmt(while_stmt),
            Stmt::Function(func) => visitor.visit_function(func),
            Stmt::Return(return_stmt) => visitor.visit_return_stmt(return_stmt),
            Stmt::Class(class_declaration) => visitor.visit_class(class_declaration),
        }
    }
}

#[derive(Clone)]
pub struct VarStmt {
    identifier: Token,
    expr: Option<Expr>,
}

impl VarStmt {
    pub fn new(identifier: Token, expr: Option<Expr>) -> Self {
        Self { identifier, expr }
    }
    pub fn identifier(&self) -> &Token {
        &self.identifier
    }
    pub fn expr(&self) -> Option<&Expr> {
        self.expr.as_ref()
    }
}

#[derive(Clone)]
pub struct IfStmt {
    // Condition that evaluates to true or false
    pub condition: Expr,
    // Branch to be executed if the condition evaluated to `true`
    pub then_branch: Box<Stmt>,
    // Optional branch to be executed if the condition evaluated to `false`
    pub else_branch: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }
}

#[derive(Clone)]
pub struct WhileStmt {
    // Condition that evaluates to true or false
    pub condition: Expr,
    // Branch to be executed if the condition evaluated to `true`
    pub stmt: Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Expr, stmt: Stmt) -> Self {
        Self {
            condition,
            stmt: Box::new(stmt),
        }
    }
}

#[derive(Clone)]
pub struct ReturnStmt {
    keyword: Token,
    expr: Option<Expr>,
}

impl ReturnStmt {
    pub fn new(keyword: Token, expr: Option<Expr>) -> Self {
        Self { keyword, expr }
    }

    pub fn expr(&self) -> Option<&Expr> {
        self.expr.as_ref()
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }
}

#[derive(Clone)]
pub struct Lambda {
    pub parameters: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Lambda {
    pub fn new(parameters: Vec<Token>, body: Vec<Stmt>) -> Self {
        Lambda { parameters, body }
    }
}

#[derive(Clone)]
pub struct FunctionDeclaration {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl FunctionDeclaration {
    pub fn new(name: Token, parameters: Vec<Token>, body: Vec<Stmt>) -> Self {
        FunctionDeclaration {
            name,
            parameters,
            body,
        }
    }
}

#[derive(Clone)]
pub enum FunctionKind {
    Free,
    Method,
}

#[derive(Clone)]
pub struct ClassDeclaration {
    // Not all classes need to inherit from a superclass
    pub superclass: Option<Token>,
    // Name of the class
    pub name: Token,
    // A list of methods for the class
    pub methods: Vec<Stmt>,
}

impl ClassDeclaration {
    pub fn new(name: Token, methods: Vec<Stmt>, superclass: Option<Token>) -> Self {
        Self {
            name,
            methods,
            superclass,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Group(Group),
    Literal(Literal),
    Ternary(Ternary),
    Var(Token),
    Assign(Token, Box<Expr>),
    Logical(Logical),
    Call(Call),
    // State getter expresion on classes
    Get(GetExpr),
    // State setter expresion on classes
    Set(SetExpr),
    // Added self keyword to access current state and behaviour of class instances
    ClassSelf(Token),
    // `super` keyword expression that calls methods from the superclass
    SuperExpr(SuperExpr),
}

impl AsRef<Expr> for Expr {
    fn as_ref(&self) -> &Expr {
        self
    }
}

impl Expr {
    pub fn walk<T, V: ExprVisitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expr::Group(group) => visitor.visit_group(group),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Var(token) => visitor.visit_variable(token),
            Expr::Assign(token, expr) => visitor.visit_assign(token, expr),
            Expr::Logical(logical) => visitor.visit_logical(logical),
            Expr::Call(call) => visitor.visit_call(call),
            Expr::Get(get_expr) => visitor.visit_get(get_expr),
            Expr::Set(set_expr) => visitor.visit_set(set_expr),
            Expr::ClassSelf(class_self) => visitor.visit_self(class_self),
            Expr::SuperExpr(super_expr) => visitor.visit_super(super_expr),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Call {
    // Function to be called
    pub callee: Box<Expr>,
    // Parenthesis at which the arguments for the current function end
    pub paren: Token,
    // Arguments taken by the function
    pub arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Ternary {
    pub first: Box<Expr>,
    pub first_operator: Token,
    pub second: Box<Expr>,
    pub second_operator: Token,
    pub third: Box<Expr>,
}

impl Ternary {
    pub fn new(
        first: Expr,
        first_operator: Token,
        second: Expr,
        second_operator: Token,
        third: Expr,
    ) -> Self {
        Self {
            first: Box::new(first),
            first_operator,
            second: Box::new(second),
            second_operator,
            third: Box::new(third),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Literal {
    pub l_type: LiteralType,
}

impl Literal {
    pub fn new(token: &Token) -> Result<Self, MalisError> {
        let l_type = {
            match token.t_type() {
                TokenType::Literal(literal) => match literal {
                    LiteralToken::Number(value) => LiteralType::Number(*value),
                    LiteralToken::LitString(value) => LiteralType::LitString(value.clone()),
                },
                TokenType::Keyword(value) => match value {
                    Keyword::True => LiteralType::True,
                    Keyword::False => LiteralType::False,
                    Keyword::Nil => LiteralType::Nil,
                    _ => Err(AstError::NotALiteral)?,
                },
                _ => Err(AstError::NotALiteral)?,
            }
        };
        Ok(Self { l_type })
    }
}

impl From<LiteralType> for Literal {
    fn from(l_type: LiteralType) -> Self {
        Self { l_type }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralType {
    Number([u8; 4]),
    LitString(String),
    True,
    False,
    Nil,
}

// Grouping matches any expression derivation inside a parenthasis -> "(" expression ")"
#[derive(Clone, PartialEq, Eq)]
pub struct Group {
    pub expr: Box<Expr>,
}

impl Group {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct GetExpr {
    name: Token,
    object: Box<Expr>,
}

impl GetExpr {
    pub fn new(name: Token, object: Expr) -> Self {
        Self {
            name,
            object: Box::new(object),
        }
    }

    pub fn object(&self) -> &Expr {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SetExpr {
    // Object state to be set
    object: Box<Expr>,
    // Name of the variable
    name: Token,
    // Value to set the state to
    value: Box<Expr>,
}

impl SetExpr {
    pub fn new(object: Expr, name: Token, value: Expr) -> Self {
        Self {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }

    pub fn object(&self) -> &Expr {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Expr {
        &self.value
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SuperExpr {
    // This is the `super` keyword
    keyword: Token,
    // This is the identifier for the method of the superclass that we want to call
    method: Token,
}

impl SuperExpr {
    pub fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn method(&self) -> &Token {
        &self.method
    }
}
