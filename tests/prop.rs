use proptest::prelude::*;

use tinylanguage::{eval, parse_expr, type_of, Type, Value};

fn value_type(value: &Value) -> Type {
    match value {
        Value::Int(_) => Type::Int,
        Value::Bool(_) => Type::Bool,
        Value::Closure { .. } => Type::Func(Box::new(Type::Int), Box::new(Type::Int)),
    }
}

proptest! {
    #[test]
    fn arithmetic_roundtrip(a in -500i64..500, b in -500i64..500) {
        let source = format!("{a} + {b} * 2");
        let expr = parse_expr(&source).expect("parse");
        let ty = type_of(&expr).expect("type");
        prop_assert_eq!(ty, Type::Int);
        let value = eval(&expr).expect("eval");
        prop_assert_eq!(value, Value::Int(a + b * 2));
    }

    #[test]
    fn if_expression_is_typed(flag in any::<bool>(), a in -100i64..100, b in -100i64..100) {
        let flag_str = if flag { "true" } else { "false" };
        let source = format!("if {flag_str} then {a} else {b}");
        let expr = parse_expr(&source).expect("parse");
        let ty = type_of(&expr).expect("type");
        prop_assert_eq!(ty, Type::Int);
        let value = eval(&expr).expect("eval");
        let expected = if flag { a } else { b };
        prop_assert_eq!(value, Value::Int(expected));
    }

    #[test]
    fn let_lambda_application(a in -50i64..50, b in -50i64..50) {
        let source = format!(
            "let x = {a} in let f = \\\n             y: Int -> x + y in f {b}"
        );
        let expr = parse_expr(&source).expect("parse");
        let ty = type_of(&expr).expect("type");
        prop_assert_eq!(ty, Type::Int);
        let value = eval(&expr).expect("eval");
        prop_assert_eq!(value, Value::Int(a + b));
        prop_assert_eq!(value_type(&value), Type::Int);
    }
}
