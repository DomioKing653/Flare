use crate::backend::compiler::byte_code::Compilable;
use std::fmt;
use std::fmt::{Formatter};

#[derive(Clone,PartialEq)]
pub enum CallType {
    Macro,
    Fn,
}

#[derive(Clone)]
pub struct ProgramNode {
    pub program_nodes: Vec<Box<dyn Compilable>>,
}

impl ProgramNode {
    pub fn new() -> Self {
        Self {
            program_nodes: Vec::new(),
        }
    }
}

impl fmt::Debug for ProgramNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
impl Default for ProgramNode {
    fn default() -> Self {
        Self::new()
    }
}


