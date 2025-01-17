use crate::ast::{Expr, Unary, Binary, Literal, Grouping};

pub trait Visitor<T> {
    fn visit_unary<E: Expr>(&mut self, unary: &Unary<E>) -> T;
    fn visit_binary<E1: Expr, E2: Expr>(&mut self, binary: &Binary<E1, E2>) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_grouping<E: Expr>(&mut self, grouping: &Grouping<E>) -> T;
}

#[derive(Debug)]
pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_unary<E: Expr>(&mut self, unary: &Unary<E>) -> String {
        if let Some(lexeme) = unary.operator.lexeme.get() {
            self.parenthesize(lexeme, &[&unary.right])
        } else {
            String::from("unknown_unary")
        }
    }

    fn visit_binary<E1: Expr, E2: Expr>(&mut self, binary: &Binary<E1, E2>) -> String {
        if let Some(lexeme) = binary.operator.lexeme.get() {
            self.parenthesize(lexeme, &[&binary.left, &binary.right])
        } else {
            String::from("unknown_binary")
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        format!("{:?}", literal.l_type)
    }

    fn visit_grouping<E: Expr>(&mut self, grouping: &Grouping<E>) -> String {
        self.parenthesize("group", &[&grouping.expr])
    }
}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter
    }

    // Wraps subexpressions stored in `exprs` in parenthasis with spaces between them
    fn parenthesize<E: Expr>(&mut self, name: &str, exprs: &[&E]) -> String {
        let final_string = exprs.as_ref().iter().map(|e| e.walk(self))
            .fold(String::from(name), |mut acc, x| {
                acc.push(' ');
                acc.push_str(&x);
                acc
            });
        format!("({})", final_string)
    }

    fn print<E: Expr>(&mut self, expr: E) -> String {
        expr.walk(self)
    }
}

#[cfg(test)]
mod tests {
    use super::AstPrinter;
    use crate::ast::{Literal, LiteralType, Unary, Binary, Grouping};
    use crate::token::{Token, TokenType, SingleChar};

    #[test]
    fn unary_test() {
        let unary_expr = Unary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "-"),
            right: Literal {
                l_type: LiteralType::Number(1.72),
            },
        };
        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(unary_expr))
    }

    #[test]
    fn binary_test() {
        let binary_expr = Binary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "*"),
            left: Literal {
                l_type: LiteralType::Number(425.12),
            },
            right: Literal {
                l_type: LiteralType::Number(0.132),
            },
        };
        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(binary_expr))
    }

    #[test]
    fn grouping_test() {
        let grouping_expr = Grouping {
            expr: Literal {
                l_type: LiteralType::Number(32.0),
            }
        };
        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(grouping_expr))
    }

    #[test]
    fn nested_test() {
        let unary_expr = Unary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "-"),
            right: Literal {
                l_type: LiteralType::Number(987.65),
            },
        };
        let grouping_expr = Grouping {
            expr: Literal {
                l_type: LiteralType::Number(123.0),
            }
        };
        let binary_expr = Binary {
            operator: Token::create(TokenType::SingleChar(SingleChar::Minus), "*"),
            left: unary_expr,
            right: grouping_expr,
        };

        let mut ast_printer = AstPrinter;
        println!("Ast: {}", ast_printer.print(binary_expr))
    }
}
