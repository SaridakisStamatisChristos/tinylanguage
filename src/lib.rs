pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod types;

pub use ast::{BinOp, Expr, Type};
pub use eval::{eval, Value};
pub use parser::parse_expr;
pub use types::type_of;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_type_eval_simple() {
        let expr = parse_expr("let x = 2 in if true then x + 3 else 0").unwrap();
        let ty = type_of(&expr).unwrap();
        assert_eq!(ty, Type::Int);
        let value = eval(&expr).unwrap();
        assert_eq!(value, Value::Int(5));
    }
}
