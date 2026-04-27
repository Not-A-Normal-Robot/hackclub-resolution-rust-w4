use crate::{
    ast::{Expr, Op, Statement},
    errors::{RuntimeError, RuntimeErrorKind},
};
use core::fmt::{Debug, Display};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    StringVal(String),
    Bool(bool),
    Nil,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => <f64 as Debug>::fmt(n, f),
            Value::StringVal(s) => write!(f, "\"{s}\""),
            Value::Bool(b) => <bool as Debug>::fmt(b, f),
            Value::Nil => f.write_str("nil"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if *n == (*n as i64) as f64 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{n}")
                }
            }
            Value::StringVal(s) => f.write_str(s),
            Value::Bool(b) => <bool as Display>::fmt(b, f),
            Value::Nil => f.write_str("nil"),
        }
    }
}

type LuminiaBody = Vec<Statement>;
type NativeBody = Box<dyn Fn(&[Value]) -> Result<Value, RuntimeError>>;

pub enum FunctionBody {
    LuminiaBody(LuminiaBody),
    Native(NativeBody),
}

pub struct Function {
    pub params: Vec<String>,
    pub body: FunctionBody,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: Self::stdlib(),
        }
    }

    pub fn override_function(&mut self, fn_name: String, f: Function) -> Option<Function> {
        self.functions.insert(fn_name, f)
    }

    fn stdlib() -> HashMap<String, Function> {
        HashMap::from([
            (
                "print".into(),
                Function {
                    params: vec!["_".to_string()],
                    body: FunctionBody::Native(Box::new(|vals| {
                        if let Some(val) = vals.first() {
                            print!("{val}");
                        }
                        Ok(Value::Nil)
                    })),
                },
            ),
            (
                "eprint".into(),
                Function {
                    params: vec!["_".to_string()],
                    body: FunctionBody::Native(Box::new(|vals| {
                        if let Some(val) = vals.first() {
                            eprint!("{val}");
                        }
                        Ok(Value::Nil)
                    })),
                },
            ),
            (
                "println".into(),
                Function {
                    params: vec!["_".to_string()],
                    body: FunctionBody::Native(Box::new(|vals| {
                        if let Some(val) = vals.first() {
                            println!("{val}");
                        }
                        Ok(Value::Nil)
                    })),
                },
            ),
            (
                "eprintln".into(),
                Function {
                    params: vec!["_".to_string()],
                    body: FunctionBody::Native(Box::new(|vals| {
                        if let Some(val) = vals.first() {
                            eprintln!("{val}");
                        }
                        Ok(Value::Nil)
                    })),
                },
            ),
            (
                "panic".into(),
                Function {
                    params: vec!["_".to_string()],
                    body: FunctionBody::Native(Box::new(|vals| {
                        Err(RuntimeError {
                            kind: RuntimeErrorKind::Panic {
                                inner: vals.first().cloned().unwrap_or(Value::Nil),
                            },
                        })
                    })),
                },
            ),
        ])
    }

    pub fn run(&mut self, program: Vec<Statement>) -> Result<(), RuntimeError> {
        for stmt in &program {
            self.exec_statement(stmt)?;
        }
        Ok(())
    }

    fn exec_statement(&mut self, stmt: &Statement) -> Result<Option<Value>, RuntimeError> {
        match stmt {
            Statement::Assign(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.variables.insert(name.clone(), val);
                Ok(None)
            }
            Statement::If(cond, body, else_body) => {
                let val = self.eval_expr(cond)?;
                if self.is_truthy(&val) {
                    for s in body {
                        if let Some(ret) = self.exec_statement(s)? {
                            return Ok(Some(ret));
                        }
                    }
                } else if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        if let Some(ret) = self.exec_statement(s)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Statement::While(cond, body) => {
                loop {
                    let val = self.eval_expr(cond)?;
                    if !self.is_truthy(&val) {
                        break;
                    }
                    for s in body {
                        if let Some(ret) = self.exec_statement(s)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Statement::Def(name, params, body) => {
                self.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: FunctionBody::LuminiaBody(body.clone()),
                    },
                );
                Ok(None)
            }
            Statement::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(Some(val))
            }
            Statement::ExprStatement(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::StringLiteral(s) => Ok(Value::StringVal(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Variable(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| RuntimeError {
                    kind: RuntimeErrorKind::UndefinedVariable {
                        var_name: name.clone(),
                    },
                }),
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binop(l, op, r)
            }
            Expr::Call(name, args) => {
                let evaluated_args: Result<Vec<Value>, RuntimeError> =
                    args.iter().map(|a| self.eval_expr(a)).collect();
                let evaluated_args = evaluated_args?;
                self.call_function(name, evaluated_args)
            }
        }
    }

    fn eval_binop(&self, lhs: Value, op: &Op, rhs: Value) -> Result<Value, RuntimeError> {
        match (lhs, op, rhs) {
            (Value::Number(a), Op::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), Op::Sub, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), Op::Mul, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), Op::Div, Value::Number(b)) => {
                if b == 0.0 {
                    Err(RuntimeError {
                        kind: RuntimeErrorKind::DivByZero { numerator: a },
                    })
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Number(a), Op::Eq, Value::Number(b)) => Ok(Value::Bool(a == b)),
            (Value::Number(a), Op::Lt, Value::Number(b)) => Ok(Value::Bool(a < b)),
            (Value::Number(a), Op::Gt, Value::Number(b)) => Ok(Value::Bool(a > b)),
            (Value::StringVal(a), Op::Add, Value::StringVal(b)) => {
                Ok(Value::StringVal(format!("{a}{b}")))
            }
            (Value::StringVal(a), Op::Eq, Value::StringVal(b)) => Ok(Value::Bool(a == b)),
            (lhs, op, rhs) => Err(RuntimeError {
                kind: RuntimeErrorKind::InvalidOperation {
                    lhs,
                    op: op.clone(),
                    rhs,
                },
            }),
        }
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, RuntimeError> {
        let func = self.functions.get(name).ok_or_else(|| RuntimeError {
            kind: RuntimeErrorKind::UndefinedFunction {
                fn_name: name.to_string(),
            },
        })?;

        if args.len() != func.params.len() {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::ParamMismatch {
                    function_name: name.to_string(),
                    expected_arg_count: func.params.len(),
                    given_arg_count: args.len(),
                },
            });
        }

        let luminia_body = match &func.body {
            FunctionBody::LuminiaBody(b) => b.clone(),
            FunctionBody::Native(b) => {
                return b(&args);
            }
        };

        // Save the current variables so we can restore them after the call.
        let old_vars = self.variables.clone();

        for (param, arg) in func.params.clone().into_iter().zip(args) {
            self.variables.insert(param, arg);
        }

        let mut result = Value::Nil;
        for stmt in &luminia_body {
            if let Some(ret) = self.exec_statement(stmt)? {
                result = ret;
                break;
            }
        }

        self.variables = old_vars;
        Ok(result)
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::StringVal(s) => !s.is_empty(),
            Value::Nil => false,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
