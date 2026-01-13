use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType;
use crate::backend::errors::compiler::compiler_errors::CompileError;
use crate::backend::compiler::comptime_variable_checker::comptime_value_for_check::ComptimeValueType::{
    Bool, Int, StringValue, Void, Float
};
use crate::backend::compiler::comptime_variable_checker::functions::Function;
use crate::backend::errors::compiler::compiler_errors::CompileError::UndefinedType;
use std::collections::HashMap;

pub struct CompileContext {
    pub variables: HashMap<String, ComptimeVariable>,
    pub functions: HashMap<String, Function>,
    pub scopes:Vec<HashMap<String,ComptimeVariable>>
}
impl CompileContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            scopes:vec![HashMap::new()]
        }
    }
    pub fn get_type(type_to_identify: &str) -> Result<ComptimeValueType, CompileError> {
        match type_to_identify {
            "numb" => Ok(Int),
            "string" => Ok(StringValue),
            "bool" => Ok(Bool),
            "void" => Ok(Void),
            "flt" => Ok(Float),
            _ => Err(UndefinedType {
                undefined_type: type_to_identify.to_string(),
            }),
        }
    }
    pub fn exit_scope(&mut self) {
       self.scopes.pop().expect("Fatal error: stack undeflow at compilation!");
    }
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
        
    }
    pub fn add_variable(&mut self,name : String,variable:ComptimeVariable)->Result<(),CompileError> {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name){
            return Err(CompileError::VariableRecreation { name })
        }else {
            current_scope.insert(name, variable);
            Ok(())
        }
    }
    pub fn get_variable(&self,name:&str)->Option<&ComptimeVariable>{
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        None

    }
}

pub struct ComptimeVariable {
    pub value_type: ComptimeValueType,
    pub is_const: bool,
}
