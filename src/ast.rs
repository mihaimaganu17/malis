use crate::{
    error::{AstError, MalisError},
    token::{Keyword, Literal as LiteralToken, Token, TokenType},
    visit::{ExprVisitor, StmtVisitor},
};

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(VarStmt),
    Block(Vec<Stmt>),
}

impl AsRef<Stmt> for Stmt {
    fn as_ref(&self) -> &Stmt {
        self
    }
}

impl Stmt {
    pub fn walk<T, V: StmtVisitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Expr(expr) => {
                println!("Visiting expr");
                visitor.visit_expr_stmt(expr)
            }
            Stmt::Print(expr) => {
                println!("Visiting print");
                visitor.visit_print_stmt(expr)
            }
            Stmt::Var(var) => {
                println!("Visition var");
                visitor.visit_var_stmt(var) }
            Stmt::Block(stmts) => visitor.visit_block_stmt(stmts),
        }
    }
}

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

pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Group(Group),
    Literal(Literal),
    Ternary(Ternary),
    Var(Token),
    Assign(Token, Box<Expr>),
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
            Expr::Var(token) => {
                println!("Visitin expr var");
                visitor.visit_variable(token) }
            Expr::Assign(token, expr) => visitor.visit_assign(token, expr),
        }
    }
}

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

#[derive(Debug)]
pub struct Literal {
    pub l_type: LiteralType,
}

impl Literal {
    pub fn new(token: &Token) -> Result<Self, MalisError> {
        let l_type = if let Some(token) = token.t_type.get() {
            match token {
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
        } else {
            return Err(MalisError::NoneTokenType);
        };
        Ok(Self { l_type })
    }
}

impl From<LiteralType> for Literal {
    fn from(l_type: LiteralType) -> Self {
        Self { l_type }
    }
}

#[derive(Debug)]
pub enum LiteralType {
    Number(f32),
    LitString(String),
    True,
    False,
    Nil,
}

// Grouping matches any expression derivation inside a parenthasis -> "(" expression ")"
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
