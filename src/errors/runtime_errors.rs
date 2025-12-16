use crate::ast::nodes::Node;

enum RuntimeErrorsType{
    CannotDivideBy0
}

pub struct RuntimeError{
    error_type:RuntimeErrorsType,
    wrong_part:dyn Node
}