use crate::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;

#[derive(Debug)]
pub enum CompileError {

    CannotInferType {
        name: String
    },


    UndefinedType{
      undefined_type:String
    },
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

    VariableRecreation{
        name:String
    },
    ConstantWithoutValue{
      name:String
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