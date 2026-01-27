use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{BinOp, Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Closure {
        param: String,
        body: Box<Expr>,
        env: Env,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Env(Rc<HashMap<String, Value>>);

impl Env {
    fn empty() -> Self {
        Env(Rc::new(HashMap::new()))
    }

    fn extend(&self, name: String, value: Value) -> Self {
        let mut new_map = (*self.0).clone();
        new_map.insert(name, value);
        Env(Rc::new(new_map))
    }

    fn get(&self, name: &str) -> Option<Value> {
        self.0.get(name).cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    pub message: String,
}

pub fn eval(expr: &Expr) -> Result<Value, EvalError> {
    eval_with_env(expr, &Env::empty())
}

fn eval_with_env(expr: &Expr, env: &Env) -> Result<Value, EvalError> {
    match expr {
        Expr::Int(value) => Ok(Value::Int(*value)),
        Expr::Bool(value) => Ok(Value::Bool(*value)),
        Expr::Var(name) => env.get(name).ok_or_else(|| EvalError {
            message: format!("unbound variable {name}"),
        }),
        Expr::Let(name, value_expr, body) => {
            let value = eval_with_env(value_expr, env)?;
            let next_env = env.extend(name.clone(), value);
            eval_with_env(body, &next_env)
        }
        Expr::If(cond, then_branch, else_branch) => {
            let cond_value = eval_with_env(cond, env)?;
            match cond_value {
                Value::Bool(true) => eval_with_env(then_branch, env),
                Value::Bool(false) => eval_with_env(else_branch, env),
                _ => Err(EvalError {
                    message: "condition must be Bool".to_string(),
                }),
            }
        }
        Expr::Lambda {
            param,
            body,
            ..
        } => Ok(Value::Closure {
            param: param.clone(),
            body: body.clone(),
            env: env.clone(),
        }),
        Expr::App(callee, arg) => {
            let callee_value = eval_with_env(callee, env)?;
            let arg_value = eval_with_env(arg, env)?;
            match callee_value {
                Value::Closure { param, body, env: closure_env } => {
                    let next_env = closure_env.extend(param, arg_value);
                    eval_with_env(&body, &next_env)
                }
                _ => Err(EvalError {
                    message: "attempted to call a non-function".to_string(),
                }),
            }
        }
        Expr::BinOp(op, lhs, rhs) => {
            let lhs_value = eval_with_env(lhs, env)?;
            let rhs_value = eval_with_env(rhs, env)?;
            match (op, lhs_value, rhs_value) {
                (BinOp::Add, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                (BinOp::Sub, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                (BinOp::Mul, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                (BinOp::Eq, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l == r)),
                (BinOp::Eq, Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                (BinOp::Lt, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l < r)),
                _ => Err(EvalError {
                    message: "invalid operands".to_string(),
                }),
            }
        }
    }
}
