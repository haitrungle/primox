use derive_new::new;

use crate::expr::*;

#[derive(new)]
pub(crate) struct RpnPrinter;

impl RpnPrinter {
    pub(crate) fn print(e: Expr) -> String {
        match e {
            Expr::Binary(e) => Self::print_binary_expr(*e),
            Expr::Grouping(e) => Self::print_grouping_expr(*e),
            Expr::Literal(e) => Self::print_literal_expr(*e),
            Expr::Ternary(e) => Self::print_ternary_expr(*e),
            Expr::Unary(e) => Self::print_unary_expr(*e),
        }
    }

    fn print_binary_expr(e: Binary) -> String {
        format!(
            "{} {} {}",
            Self::print(e.left),
            Self::print(e.right),
            e.operator.lexeme,
        )
    }

    fn print_grouping_expr(e: Grouping) -> String {
        format!("{}", Self::print(e.expression))
    }

    fn print_literal_expr(e: Literal) -> String {
        match &e.value {
            Some(v) => v.print(),
            None => "nil".to_string(),
        }
    }

    fn print_ternary_expr(e: Ternary) -> String {
        format!(
            "{} {} {} ?:",
            Self::print(e.left),
            Self::print(e.mid),
            Self::print(e.right),
        )
    }

    fn print_unary_expr(e: Unary) -> String {
        format!("{}{}", e.operator.lexeme, Self::print(e.right))
    }
}

mod test {
    use crate::token::{LiteralToken, Token};
    use crate::token_type::TokenType;

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

        assert_eq!(RpnPrinter::print(expr), "-123 45.67 *");
    }
}
