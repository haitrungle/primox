use crate::expr::*;
use crate::token::LiteralToken;
use crate::token_type::TokenType;

trait Print {
    fn print(&self) -> String;
}

impl Print for Expr {
    fn print(&self) -> String {
        match self {
            Expr::Binary(e) => e.print(),
            Expr::Grouping(e) => e.print(),
            Expr::Literal(e) => e.print(),
            Expr::Unary(e) => e.print(),
        }
    }
}

impl Print for Binary {
    fn print(&self) -> String {
        format!(
            "({} {} {})",
            self.operator.lexeme,
            self.left.print(),
            self.right.print()
        )
    }
}

impl Print for Grouping {
    fn print(&self) -> String {
        format!("(group {})", self.expression.print())
    }
}

impl Print for Literal {
    fn print(&self) -> String {
        match &self.value {
            Some(LiteralToken::Number(v)) => format!("{}", v),
            Some(LiteralToken::String(v)) => format!("{}", v),
            None => "nil".to_string(),
        }
    }
}

impl Print for Unary {
    fn print(&self) -> String {
        format!("({} {})", self.operator.lexeme, self.right.print())
    }
}

#[cfg(test)]
mod test {
    use crate::token::{LiteralToken, Token};

    use super::*;

    #[test]
    fn test_print() {
        let expr: Expr = Binary::new(
            Unary::new(
                Token::new(TokenType::MINUS, "-", None, 1),
                Literal::new(Some(LiteralToken::Number(123.0))).into(),
            )
            .into(),
            Token::new(TokenType::STAR, "*", None, 1),
            Grouping::new(Literal::new(Some(LiteralToken::Number(45.67))).into()).into(),
        )
        .into();

        assert_eq!(expr.print(), "(* (- 123) (group 45.67))");
    }
}
