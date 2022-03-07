use derive_new::new;

use crate::expr::*;
use crate::token::LiteralToken;
use crate::token_type::TokenType;

#[derive(new)]
struct AstPrinter;

impl AstPrinter {
    fn print_binary_expr(e: Binary) -> String {
        format!(
            "({} {} {})",
            e.operator.lexeme,
            Self::print(e.left),
            Self::print(e.right)
        )
    }

    fn print_grouping_expr(e: Grouping) -> String {
        format!("(group {})", Self::print(e.expression))
    }

    fn print_literal_expr(e: Literal) -> String {
        match &e.value {
            Some(LiteralToken::Number(v)) => format!("{}", v),
            Some(LiteralToken::String(v)) => format!("{}", v),
            None => "nil".to_string(),
        }
    }

    fn print_unary_expr(e: Unary) -> String {
        format!("({} {})", e.operator.lexeme, Self::print(e.right))
    }

    fn print(e: Expr) -> String {
        match e {
            Expr::Binary(b) => Self::print_binary_expr(*b),
            Expr::Grouping(g) => Self::print_grouping_expr(*g),
            Expr::Literal(l) => Self::print_literal_expr(*l),
            Expr::Unary(u) => Self::print_unary_expr(*u),
        }
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

        assert_eq!(AstPrinter::print(expr), "(* (- 123) (group 45.67))");
    }
}
