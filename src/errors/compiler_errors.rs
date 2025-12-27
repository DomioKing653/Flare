use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use thiserror::Error;

#[derive(Debug, Error)]

pub enum CompileError {
    #[error("Unknown macro: {name}")]
    UnknownMacro { name: String },

    #[error("Cannot infer type for {name}")]
    CannotInferType { name: String },

    #[error("Undefined type: {undefined_type}")]
    UndefinedType { undefined_type: String },

    #[error("Type mismatch: expected {expected:?}, found {found:?}")]
    TypeMismatch {
        expected: ComptimeValueType,
        found: ComptimeValueType,
    },

    #[error("Invalid binary operation: {op} between {left:?} and {right:?}")]
    InvalidBinaryOp {
        op: &'static str,
        left: ComptimeValueType,
        right: ComptimeValueType,
    },

    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },

    #[error("Variable {name} already exists")]
    VariableRecreation { name: String },

    #[error("Cannot have constant without value")]
    ConstantWithoutValue { name: String },
    #[error("Cannot reassign constant {name}")]
    ConstReassignment { name: String },

    #[error("Wrong macro argument count: expected {expected}, found {found}")]
    WrongMacroArgCount { expected: usize, found: usize },
}
