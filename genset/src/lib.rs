use crate::gen::expr::{Accept, Binary, Expr, Grouping, Literal, Unary};
use std::borrow::Borrow;

mod gen;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &Vec<&Expr>) -> String {
        let mut buffer = String::new();

        buffer.push('(');
        buffer.push_str(name);
        for expr in exprs {
            buffer.push(' ');
            let s: String = expr.accept(self);
            buffer.push_str(s.as_str());
        }
        buffer.push(')');

        buffer
    }
}

impl gen::expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, binary: &Binary) -> String {
        let left = binary.left.borrow();
        let right = binary.right.borrow();
        self.parenthesize(binary.operator.lexeme.as_str(), &vec![left, right])
    }

    fn visit_grouping_expr(&self, grouping: &Grouping) -> String {
        let e = grouping.expression.borrow();
        self.parenthesize("group", &vec![e])
    }

    fn visit_literal_expr(&self, literal: &Literal) -> String {
        format!("{}", literal.value)
    }

    fn visit_unary_expr(&self, unary: &Unary) -> String {
        let right = unary.right.borrow();
        self.parenthesize(unary.operator.lexeme.as_str(), &vec![right])
    }
}

#[cfg(test)]
mod tests {
    use crate::gen::expr::{Binary, Expr, Grouping, Literal, Unary};
    use crate::AstPrinter;
    use shared::tokens::{LiteralValue, Token, TokenType};

    #[test]
    fn unary() {
        let a = AstPrinter {};

        let u = Expr::Unary(Unary {
            operator: Token::new(TokenType::PLUS, "+".to_string(), 1, LiteralValue::NoVal),
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Num(45),
            })),
        });
        let r = a.print(&u);

        assert_eq!(r, "(+ 45)");
    }

    #[test]
    fn grouping() {
        let a = AstPrinter {};

        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token::new(TokenType::MINUS, "-".to_string(), 1, LiteralValue::NoVal),
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Num(123),
                })),
            })),
            operator: Token::new(TokenType::STAR, "*".to_string(), 1, LiteralValue::NoVal),
            right: Box::new(Expr::Grouping(Grouping {
                expression: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::NumFloat(45.67),
                })),
            })),
        });

        let res = a.print(&expr);

        assert_eq!(res, "(* (- 123) (group 45.67))");
    }
}
