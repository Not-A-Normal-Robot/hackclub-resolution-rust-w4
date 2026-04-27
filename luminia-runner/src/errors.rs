use core::fmt::Display;

use thiserror::Error;

use crate::{ast::Op, interpreter::Value};

#[derive(Clone, Debug, PartialEq, Error)]
pub enum ParseAndRunError<'input> {
    ParseError(ParseError<'input>),
    RuntimeError(RuntimeError),
}

impl Display for ParseAndRunError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "Parse error: {e}"),
            Self::RuntimeError(e) => write!(f, "Runtime error: {e}"),
        }
    }
}

impl<'i> From<ParseError<'i>> for ParseAndRunError<'i> {
    fn from(value: ParseError<'i>) -> Self {
        Self::ParseError(value)
    }
}

impl From<RuntimeError> for ParseAndRunError<'static> {
    fn from(value: RuntimeError) -> Self {
        Self::RuntimeError(value)
    }
}

pub type ParseError<'input> =
    lalrpop_util::ParseError<usize, crate::parser::Token<'input>, &'static str>;

#[derive(Clone, Debug, PartialEq, Error)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RuntimeErrorKind as Display>::fmt(&self.kind, f)
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum RuntimeErrorKind {
    DivByZero {
        numerator: f64,
    },
    InvalidOperation {
        lhs: Value,
        op: Op,
        rhs: Value,
    },
    UndefinedVariable {
        var_name: String,
    },
    UndefinedFunction {
        fn_name: String,
    },
    ParamMismatch {
        function_name: String,
        expected_arg_count: usize,
        given_arg_count: usize,
    },
    Panic {
        inner: Value,
    },
}

impl Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivByZero { numerator } => {
                write!(f, "attempt to divide {numerator:?} by zero")
            }
            Self::InvalidOperation { lhs, op, rhs } => {
                write!(f, "attempt to perform invalid operation: {lhs} {op} {rhs}")
            }
            Self::UndefinedVariable { var_name } => {
                write!(f, "variable `{var_name}` is undefined")
            }
            Self::UndefinedFunction { fn_name } => {
                write!(f, "function `{fn_name}` is undefined")
            }
            Self::ParamMismatch {
                function_name,
                expected_arg_count,
                given_arg_count,
            } => {
                write!(
                    f,
                    "function `{function_name}` expected {expected_arg_count} arguments, got {given_arg_count} arguments instead"
                )
            }
            Self::Panic { inner } => {
                write!(f, "panic: {inner}")
            }
        }
    }
}
