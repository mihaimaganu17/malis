use crate::ast::{Expr, Unary, Binary, Literal, Group};

pub trait Visitor<T> {
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_group(&mut self, group: &Group) -> T;
}

#[derive(Debug)]
pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_unary(&mut self, unary: &Unary) -> String {
        if let Some(lexeme) = unary.operator.lexeme.get() {
            let expr = unary.right.walk(self);
            self.parenthesize(lexeme, &[expr])
        } else {
            String::from("unknown_unary")
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> String {
        if let Some(lexeme) = binary.operator.lexeme.get() {
            let expr1 = binary.left.walk(self);
            let expr2 = binary.right.walk(self);
            self.parenthesize(lexeme, &[expr1, expr2])
        } else {
            String::from("unknown_binary")
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        format!("{:?}", literal.l_type)
    }

    fn visit_group(&mut self, group: &Group) -> String {
        let expr = group.expr.walk(self);
        self.parenthesize("group", &[expr])
    }
}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter
    }

    // Wraps subexpressions stored in `exprs` in parenthasis with spaces between them
    fn parenthesize<S: AsRef<str>>(&mut self, name: &str, exprs: &[S]) -> String {
        let final_string = exprs.iter().map(|e| e.as_ref())
            .fold(String::from(name), |mut acc, x| {
                acc.push(' ');
                acc.push_str(&x);
                acc
            });
        format!("({})", final_string)
    }

    fn print(&mut self, expr: Expr) -> String {
        expr.walk(self)
    }
}

#[cfg(test)]
mod tests {
    use super::AstPrinter;
    use crate::ast::{Literal, LiteralType, Unary, Binary, Group, Expr};
    use crate::token::{Token, TokenType, SingleChar};

    #[test]
    fn unary_test() {
        let unary_expr = Unary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "-"),
            right: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(1.72),
            })),
        };
        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(Expr::Unary(unary_expr)))
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
        println!("Ast: {}", ast_printer.print(Expr::Binary(binary_expr)))
    }

    #[test]
    fn grouping_test() {
        let grouping_expr = Group {
            expr: Box::new(Expr::Literal(Literal {
                l_type: LiteralType::Number(32.0),
            }))
        };
        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(Expr::Group(grouping_expr)))
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
            }))
        };
        let binary_expr = Binary::new(
            Expr::Unary(unary_expr),
            Token::create(TokenType::SingleChar(SingleChar::Minus), "*"),
            Expr::Group(grouping_expr),
        );

        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(Expr::Binary(binary_expr)))
    }
}
