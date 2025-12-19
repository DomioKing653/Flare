use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;

pub enum CompileError {
    TypeMismatch {
        expected: ComptimeValueType,
        found: ComptimeValueType,
    },

    InvalidBinaryOp {
        op: &'static str,
        left: ComptimeValueType,
        right: ComptimeValueType,
    },

    UndefinedVariable {
        name: String,
    },

    ConstReassignment {
        name: String,
    },

    DivisionByZero,

    UninitializedVariable {
        name: String,
    },

    ExpectedType {
        expected: ComptimeValueType,
        found: ComptimeValueType,
    },
}