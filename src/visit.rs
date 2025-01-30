use crate::{
    ast::{Binary, Expr, Group, Literal, Stmt, Ternary, Unary, VarStmt},
    token::Token,
};

/// Trait that must be implemented by a type which want to use the Visitor pattern to visit an
/// `Expr` expression of the Malis lanaguage
pub trait ExprVisitor<T> {
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_ternary(&mut self, ternary: &Ternary) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_group(&mut self, group: &Group) -> T;
    fn visit_variable(&mut self, variable: &Token) -> T;
}

/// Trait that must be implemented by a type which want to use the Visitor pattern to visit a
/// `Stmt` statement of the Malis lanaguage
pub trait StmtVisitor<T> {
    fn visit_expr_stmt(&mut self, stmt: &Expr) -> T;
    fn visit_print_stmt(&mut self, stmt: &Expr) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
}

#[derive(Debug)]
pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_unary(&mut self, unary: &Unary) -> String {
        let lexeme = unary.operator.lexeme();
        let expr = unary.right.walk(self);
        self.parenthesize(lexeme, &[expr])
    }

    fn visit_binary(&mut self, binary: &Binary) -> String {
        let lexeme = binary.operator.lexeme();
        let expr1 = binary.left.walk(self);
        let expr2 = binary.right.walk(self);
        self.parenthesize(lexeme, &[expr1, expr2])
    }

    fn visit_ternary(&mut self, ternary: &Ternary) -> String {
        let lexeme = ternary.first_operator.lexeme();
        let variants = {
            let lexeme2 = ternary.second_operator.lexeme();
            let expr2 = ternary.second.walk(self);
            let expr3 = ternary.third.walk(self);
            self.parenthesize(lexeme2, &[expr2, expr3])
        };
        let condition = ternary.first.walk(self);
        self.parenthesize(lexeme, &[condition, variants])
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        format!("{:?}", literal.l_type)
    }

    fn visit_group(&mut self, group: &Group) -> String {
        let expr = group.expr.walk(self);
        self.parenthesize("group", &[expr])
    }

    fn visit_variable(&mut self, variable: &Token) -> String {
        let lexeme = variable.lexeme();
        self.parenthesize("var", &[lexeme])
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_expr_stmt(&mut self, stmt: &Expr) -> String {
        let expr = stmt.walk(self);
        self.parenthesize("expr_stmt", &[expr])
    }

    fn visit_print_stmt(&mut self, stmt: &Expr) -> String {
        let expr = stmt.walk(self);
        self.parenthesize("print_stmt", &[expr])
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> String {
        let id = self.visit_variable(stmt.identifier());
        let expr = if let Some(expr) = stmt.expr() {
            expr.walk(self)
        } else {
            "None".to_string()
        };
        self.parenthesize("var", &[id, expr])
    }
}

impl AstPrinter {
    // Wraps subexpressions stored in `exprs` in parenthasis with spaces between them
    fn parenthesize<S: AsRef<str>>(&mut self, name: &str, exprs: &[S]) -> String {
        let final_string =
            exprs
                .iter()
                .map(|e| e.as_ref())
                .fold(String::from(name), |mut acc, x| {
                    acc.push(' ');
                    acc.push_str(x);
                    acc
                });
        format!("({})", final_string)
    }

    pub fn print_expr(&mut self, expr: &Expr) -> String {
        expr.walk(self)
    }

    pub fn print_stmt(&mut self, statements: &[Stmt]) -> String {
        let statements = statements
            .iter()
            .map(|stmt| stmt.walk(self))
            .collect::<Vec<_>>();
        statements.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::AstPrinter;
    use crate::ast::{Binary, Expr, Group, Literal, LiteralType, Unary};
    use crate::token::{SingleChar, Token, TokenType};

    #[test]
    fn unary_test() {
        let unary_expr = Unary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "-"),
            right: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(1.72),
            })),
        };
        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print_expr(&Expr::Unary(unary_expr)))
    }

    #[test]
    fn binary_test() {
        let binary_expr = Binary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "*"),
            left: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(425.12),
            })),
            right: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(0.132),
            })),
        };
        let mut ast_printer = AstPrinter;
        println!(
            "Ast: {}",
            ast_printer.print_expr(&Expr::Binary(binary_expr))
        )
    }

    #[test]
    fn grouping_test() {
        let grouping_expr = Group {
            expr: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(32.0),
            })),
        };
        let mut ast_printer = AstPrinter;
        println!(
            "Ast: {}",
            ast_printer.print_expr(&Expr::Group(grouping_expr))
        )
    }

    #[test]
    fn nested_test() {
        let unary_expr = Unary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "-"),
            right: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(987.65),
            })),
        };
        let grouping_expr = Group {
            expr: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(123.0),
            })),
        };
        let binary_expr = Binary::new(
            Expr::Unary(unary_expr),
            Token::create(TokenType::SingleChar(SingleChar::Minus), "*"),
            Expr::Group(grouping_expr),
        );

        let mut ast_printer = AstPrinter;
        println!(
            "Ast: {}",
            ast_printer.print_expr(&Expr::Binary(binary_expr))
        )
    }
}
