use crate::compiler::{
    instructions::Instructions, optimization::constant_folding::constant_folding,
};

pub fn optimize(code: Vec<Instructions>) -> Vec<Instructions> {
    let code = constant_folding(code);
    code
}
