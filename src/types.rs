use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeError {
    pub message: String,
}

pub fn type_of(expr: &Expr) -> Result<Type, TypeError> {
    let mut env = HashMap::new();
    type_of_with_env(expr, &mut env)
}

fn type_of_with_env(expr: &Expr, env: &mut HashMap<String, Type>) -> Result<Type, TypeError> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Var(name) => env.get(name).cloned().ok_or_else(|| TypeError {
            message: format!("unbound variable {name}"),
        }),
        Expr::Let(name, value, body) => {
            let value_ty = type_of_with_env(value, env)?;
            let previous = env.insert(name.clone(), value_ty);
            let result = type_of_with_env(body, env);
            if let Some(prev) = previous {
                env.insert(name.clone(), prev);
            } else {
                env.remove(name);
            }
            result
        }
        Expr::If(cond, then_branch, else_branch) => {
            let cond_ty = type_of_with_env(cond, env)?;
            if cond_ty != Type::Bool {
                return Err(TypeError {
                    message: "condition must be Bool".to_string(),
                });
            }
            let then_ty = type_of_with_env(then_branch, env)?;
            let else_ty = type_of_with_env(else_branch, env)?;
            if then_ty != else_ty {
                return Err(TypeError {
                    message: "branches must have the same type".to_string(),
                });
            }
            Ok(then_ty)
        }
        Expr::Lambda {
            param,
            param_type,
            body,
        } => {
            let previous = env.insert(param.clone(), param_type.clone());
            let body_ty = type_of_with_env(body, env);
            if let Some(prev) = previous {
                env.insert(param.clone(), prev);
            } else {
                env.remove(param);
            }
            Ok(Type::Func(Box::new(param_type.clone()), Box::new(body_ty?)))
        }
        Expr::App(callee, arg) => {
            let callee_ty = type_of_with_env(callee, env)?;
            let arg_ty = type_of_with_env(arg, env)?;
            match callee_ty {
                Type::Func(param_ty, ret_ty) => {
                    if *param_ty != arg_ty {
                        return Err(TypeError {
                            message: format!(
                                "function expected {param_ty} but argument was {arg_ty}"
                            ),
                        });
                    }
                    Ok(*ret_ty)
                }
                _ => Err(TypeError {
                    message: "attempted to call a non-function".to_string(),
                }),
            }
        }
        Expr::BinOp(op, lhs, rhs) => {
            let lhs_ty = type_of_with_env(lhs, env)?;
            let rhs_ty = type_of_with_env(rhs, env)?;
            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul => {
                    if lhs_ty != Type::Int || rhs_ty != Type::Int {
                        return Err(TypeError {
                            message: "arithmetic expects Int".to_string(),
                        });
                    }
                    Ok(Type::Int)
                }
                BinOp::Eq => {
                    if lhs_ty != rhs_ty {
                        return Err(TypeError {
                            message: "equality expects matching types".to_string(),
                        });
                    }
                    Ok(Type::Bool)
                }
                BinOp::Lt => {
                    if lhs_ty != Type::Int || rhs_ty != Type::Int {
                        return Err(TypeError {
                            message: "comparison expects Int".to_string(),
                        });
                    }
                    Ok(Type::Bool)
                }
            }
        }
    }
}
