mod builtins;
mod value;

use super::drawer::DrawCmd;
use super::parser::{AstNode, Binop};
use builtins::get_builtins;
use std::collections::HashMap;
use std::iter::zip;
use value::*;

pub struct Interpreter {
    drawing: Vec<DrawCmd>,
    environment: HashMap<String, Value>,
}

impl Interpreter {
    fn new() -> Self {
        let mut environment = HashMap::new();
        get_builtins().into_iter().for_each(|(names, f)| {
            names.into_iter().for_each(|n| {
                environment.insert(n.to_string(), f.clone());
            });
        });
        Self {
            drawing: vec![],
            environment,
        }
    }

    fn eval_binop(v1: Value, op: &Binop, v2: Value) -> Value {
        match op {
            Binop::And => match (v1, v2) {
                (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 && b2),
                (_, _) => panic!(""),
            },
            Binop::Or => match (v1, v2) {
                (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 || b2),
                (_, _) => panic!(""),
            },
            Binop::Less => match (v1, v2) {
                (Value::Number(n1), Value::Number(n2)) => Value::Bool(n1 < n2),
                (_, _) => panic!(""),
            },
            Binop::LessEqual => match (v1, v2) {
                (Value::Number(n1), Value::Number(n2)) => Value::Bool(n1 <= n2),
                (_, _) => panic!(""),
            },
            Binop::Greater => match (v1, v2) {
                (Value::Number(n1), Value::Number(n2)) => Value::Bool(n1 > n2),
                (_, _) => panic!(""),
            },
            Binop::GreaterEqual => match (v1, v2) {
                (Value::Number(n1), Value::Number(n2)) => Value::Bool(n1 >= n2),
                (_, _) => panic!(""),
            },
            Binop::EqualEqual => match (v1, v2) {
                (Value::Number(n1), Value::Number(n2)) => Value::Bool(n1 == n2),
                (_, _) => panic!(""),
            },
            Binop::Add => match (v1, v2) {
                (Value::Number(b1), Value::Number(b2)) => Value::Number(b1 + b2),
                (_, _) => panic!(""),
            },
            Binop::Sub => match (v1, v2) {
                (Value::Number(b1), Value::Number(b2)) => Value::Number(b1 - b2),
                (_, _) => panic!(""),
            },
            Binop::Mul => match (v1, v2) {
                (Value::Number(b1), Value::Number(b2)) => Value::Number(b1 * b2),
                (_, _) => panic!(""),
            },
            Binop::Div => match (v1, v2) {
                (Value::Number(b1), Value::Number(b2)) => Value::Number(b1 / b2),
                (_, _) => panic!(""),
            },
        }
    }

    fn eval(&mut self, node: &AstNode) -> Value {
        match node {
            AstNode::String(s) => Value::String(s.clone()),
            AstNode::Number(n) => Value::Number(n.clone()),
            AstNode::Variable(x) => match self.environment.get(&x[..]) {
                Some(v) => (*v).clone(),
                None => panic!("unbound variable {:#?}", &x),
            },
            AstNode::List(elems) => {
                Value::List(elems.iter().map(|e| self.eval(e)).collect::<Vec<_>>())
            }
            AstNode::Binop { lhs, op, rhs } => {
                let v1 = self.eval(&lhs);
                let v2 = self.eval(&rhs);
                Self::eval_binop(v1, &op, v2)
            }
            AstNode::Call { f, args } => {
                let f = self.eval(&f);
                let args: Vec<Value> = args.iter().map(|a| self.eval(&a)).collect();
                match f {
                    Value::Function(f) => match f {
                        LogoFn::LangFn(f) => match f {
                            LangFn { arity, function } => function(self, args),
                        },
                        LogoFn::UserFn(f) => match f {
                            UserFn { params, body } => {
                                if args.len() != params.len() {
                                    panic!("number of args is different than number of params")
                                } else {
                                    let saved: Vec<(String, Option<Value>)> = zip(params, args)
                                        .map(|(x, v)| (x.clone(), self.environment.insert(x, v)))
                                        .collect();
                                    self.eval(&body);
                                    saved.into_iter().for_each(|(x, v)| {
                                        if let Some(v) = v {
                                            self.environment.insert(x, v);
                                        }
                                    });
                                    Value::Nothing
                                }
                            }
                        },
                    },
                    _ => panic!("can only call functions"),
                }
            }
            AstNode::If { condition, body } => {
                let c = self.eval(condition);
                match c {
                    Value::Bool(true) => self.eval(body),
                    Value::Bool(false) => Value::Nothing,
                    _ => panic!("guard type is not boolean"),
                }
            }
            AstNode::Loop { repeat, body } => {
                let r = self.eval(repeat);
                let saved = self.environment.get("repcount").cloned();
                if let Value::Number(n) = r {
                    for i in 1..=(n as i32) {
                        self.environment
                            .insert("repcount".to_string(), Value::Number((i as f64).clone()));
                        self.eval(body);
                    }
                } else {
                    panic!("repeat is not a number")
                }
                if let Some(v) = saved {
                    self.environment.insert("repcount".to_string(), v);
                }
                Value::Nothing
            }
            AstNode::ProcDef {
                proc_name,
                params,
                body,
            } => {
                let f = Value::Function(LogoFn::UserFn(UserFn {
                    params: params.clone(),
                    body: (**body).clone(),
                }));
                self.environment.insert((*proc_name).clone(), f);
                Value::Nothing
            }
            AstNode::Block(stmts) => {
                let mut ret = Value::Nothing;
                for stmt in stmts {
                    ret = self.eval(stmt);
                    if let Value::Return = ret {
                        break;
                    }
                }
                ret
            }
        }
    }
}

pub fn evaluate(source: &AstNode) -> Vec<DrawCmd> {
    let mut interpreter = Interpreter::new();
    interpreter.eval(source);
    interpreter.drawing
}

#[cfg(test)]
mod tests {
    use super::super::parser::parse_logo_source;
    use super::*;

    #[test]
    fn simple_forward() {
        let source = "forward 1 + 2";
        let ast = parse_logo_source(source);
        assert!(ast.is_ok());
        let ast = ast.unwrap();
        assert_eq!(evaluate(&ast), vec![DrawCmd::Forward(3.0)])
    }

    #[test]
    fn repeat_forward() {
        let source = "repeat 4 [ forward 1 ]";
        let ast = parse_logo_source(source);
        assert!(ast.is_ok());
        let ast = ast.unwrap();
        assert_eq!(
            evaluate(&ast),
            vec![
                DrawCmd::Forward(1.0),
                DrawCmd::Forward(1.0),
                DrawCmd::Forward(1.0),
                DrawCmd::Forward(1.0)
            ]
        )
    }

    #[test]
    fn procedure_with_stop() {
        let source = "to proc :x if :x < 1 [ stop ] fd 1 end proc 0";
        let ast = parse_logo_source(source);
        assert!(ast.is_ok());
        let ast = ast.unwrap();
        assert_eq!(evaluate(&ast), vec![])
    }
}
