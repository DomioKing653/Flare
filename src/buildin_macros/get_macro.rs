use crate::buildin_macros::macros::{Macro, ProcessExitMacro, WriteLnMacro, WriteMacro};
use crate::errors::compiler::compiler_errors::CompileError;
use crate::errors::compiler::compiler_errors::CompileError::UnknownMacro;
use std::collections::HashMap;

pub struct MacroManager {
    pub macros: HashMap<String, Box<dyn Macro>>,
}

impl MacroManager {
    pub fn get_macro_mut(&mut self, name: &str) -> Result<&mut Box<dyn Macro>, CompileError> {
        self.macros.get_mut(name).ok_or(UnknownMacro {
            name: name.to_string(),
        })
    }
    pub fn new() -> Self {
        let mut register = Self {
            macros: HashMap::new(),
        };
        register.register("writeLn", WriteLnMacro);
        register.register("write", WriteMacro);
        register.register("processExit", ProcessExitMacro);
        register
    }
    pub fn register<M: Macro + 'static>(&mut self, name: &str, mac: M) {
        self.macros.insert(name.to_string(), Box::new(mac));
    }
}
